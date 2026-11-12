//! Phantom Spider: native PC port of the 2004 Nokia Series 40 J2ME game.
//! SDL3 provides the window, keyboard and gamepad input, rumble and audio.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod assets;
mod audio;
mod font;
mod game;
mod gfx;
mod lang;
mod lcdui;
mod rms;

use audio::Audio;
use game::Game;
use gfx::{Graphics, SCREEN_H, SCREEN_W};
use lcdui::{K_CLEAR, K_DOWN, K_FIRE, K_LEFT, K_RIGHT, K_SOFT_L, K_SOFT_R, K_UP};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

struct Opts {
    scale: u32,
    fullscreen: bool,
    soundfont: Option<PathBuf>,
    script: Option<PathBuf>,
    out: PathBuf,
    headless: bool,
    mute: bool,
    rumble: bool,
    lang: String,
}

fn parse_args() -> Opts {
    let mut o = Opts {
        scale: 5,
        fullscreen: false,
        soundfont: None,
        script: None,
        out: PathBuf::from("."),
        headless: false,
        mute: false,
        rumble: true,
        lang: "xx".into(),
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--scale" => o.scale = it.next().and_then(|v| v.parse().ok()).unwrap_or(5).clamp(1, 16),
            "--fullscreen" => o.fullscreen = true,
            "--soundfont" => o.soundfont = it.next().map(PathBuf::from),
            "--script" => o.script = it.next().map(PathBuf::from),
            "--out" => o.out = it.next().map(PathBuf::from).unwrap_or_default(),
            "--headless" => o.headless = true,
            "--mute" => o.mute = true,
            "--no-rumble" => o.rumble = false,
            "--lang" => o.lang = it.next().unwrap_or_else(|| "xx".into()),
            "--help" | "-h" => {
                println!(
                    "phantom-spider [--scale N] [--fullscreen] [--soundfont file.sf2] [--mute] [--no-rumble]\n\
                     \n  Keyboard: arrows/WASD move, Enter/Space fire/select, Esc menu/back,\n\
                     \x20 Backspace clear, Tab left soft key, 0-9 keypad digits, M mute, F11 fullscreen\n\
                     Gamepad: D-pad/left stick move, A/X/RT fire/select, B/Start menu/back, Y/Back left soft key"
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }
    o
}

fn exe_dir() -> PathBuf {
    std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| PathBuf::from("."))
}

fn find_soundfont(o: &Opts) -> Option<PathBuf> {
    if let Some(p) = &o.soundfont {
        return Some(p.clone());
    }
    for dir in [exe_dir(), PathBuf::from(".")] {
        let p = dir.join("soundfont.sf2");
        if p.exists() {
            return Some(p);
        }
        if let Ok(rd) = std::fs::read_dir(&dir) {
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().map(|x| x.eq_ignore_ascii_case("sf2")).unwrap_or(false) {
                    return Some(p);
                }
            }
        }
    }
    None
}

fn save_png(path: &Path, fb: &[u32], scale: usize) {
    let Ok(f) = std::fs::File::create(path) else { return };
    let (w0, h0) = (SCREEN_W as usize, SCREEN_H as usize);
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), (w0 * scale) as u32, (h0 * scale) as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let Ok(mut w) = enc.write_header() else { return };
    let mut data = Vec::with_capacity(fb.len() * 3 * scale * scale);
    for y in 0..h0 * scale {
        for x in 0..w0 * scale {
            let p = fb[(y / scale) * w0 + x / scale];
            data.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
        }
    }
    let _ = w.write_image_data(&data);
}

// ----------------------------------------------------------------- input
/// Logical MIDP keys pressed by several physical sources are ref-counted so
/// releasing one source doesn't release the key while another still holds it.
struct Input {
    held: HashMap<i32, u32>,
    /// physical source id -> key code
    sources: HashMap<u64, i32>,
    repeat_at: HashMap<i32, f64>,
}

const SRC_KEY: u64 = 1 << 40;
const SRC_PAD: u64 = 2 << 40;
const SRC_AXIS: u64 = 3 << 40;

impl Input {
    fn new() -> Input {
        Input { held: HashMap::new(), sources: HashMap::new(), repeat_at: HashMap::new() }
    }
    fn press(&mut self, src: u64, code: i32, game: &mut Game, font: &font::Font) {
        if self.sources.contains_key(&src) {
            return;
        }
        self.sources.insert(src, code);
        let n = self.held.entry(code).or_insert(0);
        *n += 1;
        if *n == 1 {
            game.key_pressed(code, false, font);
            self.repeat_at.insert(code, 380.0);
        }
    }
    fn release(&mut self, src: u64, game: &mut Game, font: &font::Font) {
        let Some(code) = self.sources.remove(&src) else { return };
        if let Some(n) = self.held.get_mut(&code) {
            *n = n.saturating_sub(1);
            if *n == 0 {
                self.held.remove(&code);
                self.repeat_at.remove(&code);
                game.key_released(code);
                // The phone could only hold one direction; on a keyboard or stick the
                // player often rolls between directions. In the shooter, releasing one
                // direction while another is still held keeps moving the other way
                // instead of stopping (the original keyReleased zeroes movement).
                if game.in_shooter() && matches!(code, K_UP | K_DOWN | K_LEFT | K_RIGHT) {
                    if let Some(&other) = [K_UP, K_DOWN, K_LEFT, K_RIGHT].iter().find(|c| self.held.contains_key(c)) {
                        game.key_pressed(other, false, font);
                    }
                }
            }
        }
    }
    fn release_all(&mut self, game: &mut Game, font: &font::Font) {
        let srcs: Vec<u64> = self.sources.keys().copied().collect();
        for s in srcs {
            self.release(s, game, font);
        }
    }
    /// Auto-repeat directions on menus/forms (canvases ignore repeats).
    fn tick(&mut self, dt: f64, game: &mut Game, font: &font::Font) {
        if !game.screen_is_form() {
            return;
        }
        for code in [K_UP, K_DOWN, K_LEFT, K_RIGHT, K_CLEAR] {
            if let Some(t) = self.repeat_at.get_mut(&code) {
                *t -= dt;
                if *t <= 0.0 {
                    *t = 110.0;
                    game.key_pressed(code, true, font);
                }
            }
        }
    }
}

fn map_key(k: sdl3::keyboard::Keycode) -> Option<i32> {
    use sdl3::keyboard::Keycode as K;
    Some(match k {
        K::Up | K::W => K_UP,
        K::Down | K::S => K_DOWN,
        K::Left | K::A => K_LEFT,
        K::Right | K::D => K_RIGHT,
        K::Return | K::Return2 | K::Space | K::KpEnter => K_FIRE,
        K::Escape => K_SOFT_R,
        K::Tab | K::F1 => K_SOFT_L,
        K::Backspace | K::Delete => K_CLEAR,
        K::_0 | K::Kp0 => 48,
        K::_1 | K::Kp1 => 49,
        K::_2 | K::Kp2 => 50,
        K::_3 | K::Kp3 => 51,
        K::_4 | K::Kp4 => 52,
        K::_5 | K::Kp5 => 53,
        K::_6 | K::Kp6 => 54,
        K::_7 | K::Kp7 => 55,
        K::_8 | K::Kp8 => 56,
        K::_9 | K::Kp9 => 57,
        _ => return None,
    })
}

fn map_button(b: sdl3::gamepad::Button) -> Option<i32> {
    use sdl3::gamepad::Button as B;
    Some(match b {
        B::DPadUp => K_UP,
        B::DPadDown => K_DOWN,
        B::DPadLeft => K_LEFT,
        B::DPadRight => K_RIGHT,
        B::South | B::West => K_FIRE,
        B::East | B::Start => K_SOFT_R,
        B::North | B::Back => K_SOFT_L,
        B::LeftShoulder => K_LEFT,
        B::RightShoulder => K_RIGHT,
        _ => return None,
    })
}

// ----------------------------------------------------------------- script
/// Headless test driver: `wait ms`, `press code`, `release code`,
/// `tap code`, `shot file.png`, `quit`. Codes are MIDP key codes or names
/// (up, down, left, right, fire, softl, softr, clear).
fn run_script(game: &mut Game, g: &mut Graphics, script: &Path, out: &Path, font: &font::Font) {
    let mut input = Input::new();
    let text = std::fs::read_to_string(script).unwrap_or_default();
    let _ = std::fs::create_dir_all(out);
    let step = 1000.0 / 60.0;
    let code_of = |s: &str| -> i32 {
        match s {
            "up" => K_UP,
            "down" => K_DOWN,
            "left" => K_LEFT,
            "right" => K_RIGHT,
            "fire" => K_FIRE,
            "softl" => K_SOFT_L,
            "softr" => K_SOFT_R,
            "clear" => K_CLEAR,
            _ => s.parse().unwrap_or(0),
        }
    };
    let mut buf = Vec::new();
    let mut t_ms = 0.0f64;
    let mut advance = |game: &mut Game, g: &mut Graphics, ms: f64| {
        let mut t = 0.0;
        while t < ms {
            game.update(step, g);
            game.audio.render((audio::RATE as f64 * step / 1000.0) as usize, &mut buf);
            t += step;
        }
    };
    for line in text.lines().flat_map(|l| l.split(';')) {
        let mut p = line.split_whitespace();
        let Some(cmd) = p.next() else { continue };
        let arg = p.next().unwrap_or("");
        match cmd {
            "wait" => {
                let ms: f64 = arg.parse().unwrap_or(0.0);
                t_ms += ms;
                advance(game, g, ms)
            }
            "press" => input.press(SRC_KEY | (code_of(arg) as u32 as u64), code_of(arg), game, font),
            "release" => input.release(SRC_KEY | (code_of(arg) as u32 as u64), game, font),
            "tap" => {
                let c = code_of(arg);
                game.key_pressed(c, false, font);
                advance(game, g, 60.0);
                game.key_released(c);
                advance(game, g, 60.0);
            }
            "shot" => {
                game.update(0.0, g);
                save_png(&out.join(arg), &g.fb, 3);
                println!("shot {arg}");
            }
            "stat" => println!(
                "stat {} t={} H={} score={} life={} spiders={} level={}",
                arg,
                t_ms,
                game.st.L_H,
                game.st.j_d,
                game.lc.s.m,
                game.bat.h.e.iter().filter(|z| z.is_some()).count(),
                game.st.j_b
            ),
            "pos" => println!("pos {} x={} y={} dir={} speed={}", arg, game.lc.s.l, game.lc.s.k, game.lc.s.f, game.lc.s.e),
            "quit" => break,
            _ => {}
        }
        if game.quit {
            break;
        }
    }
}

// ----------------------------------------------------------------- main
fn main() {
    let o = parse_args();
    if let Err(msg) = rms::init_data_dir() {
        eprintln!("Phantom Spider: {msg}");
        if !o.headless && o.script.is_none() {
            let _ = sdl3::messagebox::show_simple_message_box(
                sdl3::messagebox::MessageBoxFlag::ERROR,
                "Phantom Spider",
                &msg,
                None,
            );
        }
        std::process::exit(1);
    }
    lang::init(&o.lang);
    let font = Rc::new(font::Font::new());
    let mut g = Graphics::new(font.clone());

    let sf = find_soundfont(&o).and_then(|p| Audio::load_soundfont(&p));
    if sf.is_none() && !o.headless {
        eprintln!("No SoundFont found (soundfont.sf2 next to the exe); music is silent.");
    }
    let mut au = Audio::new(if o.headless { None } else { sf });
    au.muted = o.mute;
    let mut game = Game::new(au, "5");

    if let Some(script) = &o.script {
        run_script(&mut game, &mut g, script, &o.out, &font);
        game.ps_destroy();
        return;
    }
    if let Err(e) = run(&o, &mut game, &mut g, &font) {
        eprintln!("fatal: {e}");
        game.ps_destroy();
        std::process::exit(1);
    }
}

fn run(o: &Opts, game: &mut Game, g: &mut Graphics, font: &font::Font) -> Result<(), String> {
    use sdl3::event::{Event, WindowEvent};
    use sdl3::pixels::PixelFormat;

    sdl3::hint::set("SDL_JOYSTICK_ALLOW_BACKGROUND_EVENTS", "1");
    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let pads = sdl.gamepad().map_err(|e| e.to_string())?;

    let mut wb = video.window("Phantom Spider", 128 * o.scale, 128 * o.scale);
    wb.resizable().position_centered().high_pixel_density();
    let mut window = wb.build().map_err(|e| e.to_string())?;
    if let Some(icon) = assets::get("/icon.png").and_then(gfx::Image::from_png) {
        if let Ok(mut s) = sdl3::surface::Surface::new(icon.w as u32, icon.h as u32, PixelFormat::ARGB8888) {
            let pitch = s.pitch() as usize;
            s.with_lock_mut(|buf: &mut [u8]| {
                for y in 0..icon.h as usize {
                    for x in 0..icon.w as usize {
                        let p = icon.px[y * icon.w as usize + x].to_le_bytes();
                        buf[y * pitch + x * 4..y * pitch + x * 4 + 4].copy_from_slice(&p);
                    }
                }
            });
            window.set_icon(&s);
        }
    }
    if o.fullscreen {
        let _ = window.set_fullscreen(true);
    }
    let mut canvas = window.into_canvas();
    unsafe {
        sdl3::sys::render::SDL_SetRenderVSync(canvas.raw(), 1);
    }
    canvas
        .set_logical_size(128, 128, sdl3::sys::render::SDL_LOGICAL_PRESENTATION_INTEGER_SCALE)
        .map_err(|e| e.to_string())?;
    let tc = canvas.texture_creator();
    let mut tex = tc.create_texture_streaming(PixelFormat::ARGB8888, 128, 128).map_err(|e| e.to_string())?;
    tex.set_scale_mode(sdl3::render::ScaleMode::Nearest);

    // audio (optional)
    let audio_sub = sdl.audio().ok();
    let stream = audio_sub.as_ref().and_then(|a| {
        let spec = sdl3::audio::AudioSpec::new(Some(audio::RATE), Some(2), Some(sdl3::audio::AudioFormat::f32_sys()));
        let s = a.default_playback_device().open_device_stream(Some(&spec)).ok()?;
        let _ = s.resume();
        Some(s)
    });
    if stream.is_none() {
        eprintln!("audio unavailable; continuing silently");
    }
    let mut abuf: Vec<f32> = Vec::new();

    let mut pads_open: HashMap<u32, sdl3::gamepad::Gamepad> = HashMap::new();
    let mut axis_state: HashMap<(u32, u8), i32> = HashMap::new();
    let mut input = Input::new();
    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    let mut last = Instant::now();

    while !game.quit {
        for ev in events.poll_iter() {
            match ev {
                Event::Quit { .. } => {
                    game.ps_destroy();
                    game.quit = true;
                }
                Event::Window { win_event: WindowEvent::FocusLost, .. } => input.release_all(game, font),
                Event::KeyDown { keycode: Some(k), keymod, repeat, scancode, .. } => {
                    use sdl3::keyboard::{Keycode, Mod};
                    let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
                    if k == Keycode::F11 || (alt && k == Keycode::Return) {
                        if !repeat {
                            let w = canvas.window_mut();
                            let fs = !matches!(w.fullscreen_state(), sdl3::video::FullscreenType::Off);
                            let _ = w.set_fullscreen(!fs);
                        }
                        continue;
                    }
                    if k == Keycode::M && !repeat && !game.screen_is_form() {
                        game.audio.muted = !game.audio.muted;
                        continue;
                    }
                    if matches!(k, Keycode::Equals | Keycode::Plus | Keycode::KpPlus) {
                        game.audio.volume = (game.audio.volume + 0.25).min(4.0);
                        continue;
                    }
                    if matches!(k, Keycode::Minus | Keycode::KpMinus) {
                        game.audio.volume = (game.audio.volume - 0.25).max(0.0);
                        continue;
                    }
                    if let Some(code) = map_key(k) {
                        let src = SRC_KEY | scancode.map(|s| s as u64).unwrap_or(k as u64 & 0xffff_ffff);
                        if repeat {
                            if game.screen_is_form() && matches!(code, 48..=57) {
                                game.key_pressed(code, true, font);
                            }
                        } else {
                            input.press(src, code, game, font);
                        }
                    }
                }
                Event::KeyUp { keycode: Some(k), scancode, .. } => {
                    if map_key(k).is_some() {
                        let src = SRC_KEY | scancode.map(|s| s as u64).unwrap_or(k as u64 & 0xffff_ffff);
                        input.release(src, game, font);
                    }
                }
                Event::GamepadAdded { which, .. } => {
                    if let Ok(p) = pads.open(which) {
                        pads_open.insert(which.into(), p);
                    }
                }
                Event::GamepadRemoved { which, .. } => {
                    let which: u32 = which.into();
                    pads_open.remove(&which);
                    // drop anything this pad was holding
                    let srcs: Vec<u64> = input
                        .sources
                        .keys()
                        .copied()
                        .filter(|s| (s & 0xff_ffff_ffff) >> 8 == which as u64 && s >> 40 >= 2)
                        .collect();
                    for s in srcs {
                        input.release(s, game, font);
                    }
                }
                Event::GamepadButtonDown { which, button, .. } => {
                    let which: u32 = which.into();
                    if let Some(code) = map_button(button) {
                        input.press(SRC_PAD | (which as u64) << 8 | button as u64, code, game, font);
                    }
                }
                Event::GamepadButtonUp { which, button, .. } => {
                    let which: u32 = which.into();
                    if map_button(button).is_some() {
                        input.release(SRC_PAD | (which as u64) << 8 | button as u64, game, font);
                    }
                }
                Event::GamepadAxisMotion { which, axis, value, .. } => {
                    let which: u32 = which.into();
                    use sdl3::gamepad::Axis;
                    let v = value as i32;
                    let (id, neg, pos): (u8, i32, i32) = match axis {
                        Axis::LeftX => (0, K_LEFT, K_RIGHT),
                        Axis::LeftY => (1, K_UP, K_DOWN),
                        Axis::TriggerRight => (2, 0, K_FIRE),
                        Axis::TriggerLeft => (3, 0, K_SOFT_L),
                        _ => continue,
                    };
                    // hysteresis: engage at 50%, release at 30%
                    let prev = *axis_state.get(&(which, id)).unwrap_or(&0);
                    let now = if v > 16000 || (prev > 0 && v > 9800) {
                        1
                    } else if v < -16000 || (prev < 0 && v < -9800) {
                        -1
                    } else {
                        0
                    };
                    if now != prev {
                        let src = SRC_AXIS | (which as u64) << 8 | id as u64;
                        input.release(src, game, font);
                        let code = if now > 0 { pos } else if now < 0 { neg } else { 0 };
                        if code != 0 {
                            input.press(src, code, game, font);
                        }
                        axis_state.insert((which, id), now);
                    }
                }
                _ => {}
            }
        }
        if game.quit {
            break;
        }

        let now = Instant::now();
        let dt = (now - last).as_secs_f64() * 1000.0;
        last = now;
        let dt = dt.min(250.0);
        input.tick(dt, game, font);
        game.update(dt, g);

        if let Some((lo, hi, ms)) = game.rumble.take() {
            if o.rumble {
                for p in pads_open.values_mut() {
                    let _ = p.set_rumble(lo, hi, ms);
                }
            }
        }

        // keep ~80ms of audio queued
        if let Some(s) = &stream {
            let queued = s.queued_bytes().unwrap_or(0).max(0) as usize / 8;
            let target = (audio::RATE as usize) * 80 / 1000;
            if queued < target {
                game.audio.render(target - queued, &mut abuf);
                let _ = s.put_data_f32(&abuf);
            }
        } else {
            game.audio.render((audio::RATE as f64 * dt / 1000.0) as usize, &mut abuf);
        }

        let bytes: Vec<u8> = g.fb.iter().flat_map(|p| p.to_le_bytes()).collect();
        tex.update(None, &bytes, 128 * 4).map_err(|e| e.to_string())?;
        canvas.set_draw_color(sdl3::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&tex, None, None).map_err(|e| e.to_string())?;
        canvas.present();
        if dt < 4.0 {
            std::thread::sleep(Duration::from_millis(2));
        }
    }
    Ok(())
}
