//! Port of the obfuscated J2ME game classes.
//!
//! Naming: every Java static `Cls.fld` lives in [`St`] / [`Images`] as
//! `Cls_fld` so the code can be checked line by line against
//! `reference/java/`. Instance state lives in the per-object structs.

#![allow(non_snake_case)]

pub mod battle;
pub mod ents;
pub mod eyes;
pub mod kcanvas;
pub mod lcanvas;
pub mod midlet;
pub mod sfx;
pub mod spider;
pub mod tutorial;

use crate::assets;
use crate::audio::Audio;
use crate::gfx::{Img, Res};
use crate::lcdui::Screen;
use std::cell::Cell;

// ---------------------------------------------------------------- class m
thread_local! {
    static RNG: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
}

/// `m.a()`: seed from the clock like `new Random()`.
pub fn rnd_init() {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    RNG.with(|r| r.set(t | 1));
}

/// `m.a(int)`: `Math.abs(random.nextInt() % n)`
pub fn rnd(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    RNG.with(|r| {
        let mut x = r.get();
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        r.set(x);
        let v = (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 33) as i32;
        (v % n).abs()
    })
}

/// `m.a(8 ints)`: rectangle overlap
#[allow(clippy::too_many_arguments)]
pub fn hit(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32, a6: i32, a7: i32) -> bool {
    a0 < a4 + a6 && a0 + a2 > a4 && a1 < a5 + a7 && a1 + a3 > a5
}

/// `K.a(4 ints)`: damage formula
pub fn K_a(v0: i32, v1: i32, v2: i32, v3: i32) -> i32 {
    let v4 = v0 / 10;
    let v5 = v1 / 10;
    if v4 > v5 {
        v2 - (v3 + v4 - v5)
    } else {
        v2 - v3
    }
}

pub fn load(p: &str) -> Img {
    assets::image(p)
}

// ---------------------------------------------------------------- images
/// Every static `Image` field of every class.
#[derive(Default)]
pub struct Images {
    // k
    pub k_Y: Img,
    pub k_j: Img,
    pub k_ah: Img,
    pub k_ad: Img,
    pub k_e: Img,
    pub k_a: Img,
    pub k_l: Img,
    pub k_k: Img,
    pub k_I: Img,
    pub k_H: Img,
    pub k_i: Img,
    pub k_V: Img,
    pub k_aj: Img,
    pub k_ag: [Img; 2],
    pub k_h: Img,
    pub k_ai: Img,
    pub k_z: Img,
    pub k_A: Img,
    pub k_O: Img,
    pub k_G: Img,
    pub k_p: Img,
    // L
    pub L_z: Img,
    pub L_y: Img,
    pub L_h: Img,
    pub L_l: Img,
    pub L_q: Img,
    pub L_E: Img,
    pub L_K: Img,
    pub L_A: Img,
    pub L_c: Img,
    pub L_F: Img,
    pub L_o: Img,
    pub L_f: Img,
    pub L_j: Img,
    // v (eyes)
    pub v_c: [Img; 10],
    // weapons
    pub B_l: Img,
    pub o_i: Img,
    pub b_i: Img,
    pub g_c: Img,
    pub y_c: Img,
    pub a_n: Img,
    pub N_n: Img,
    pub O_n: Img,
    pub d_i: Img,
    pub e_k: Img,
    pub t_j: [Img; 3],
    // s (explosions)
    pub s_f: [Img; 4],
    // z / r / l
    pub z_d: [Img; 3],
    pub z_j: [Img; 3],
    pub r_g: Img,
    pub l_i: Img,
}

// ---------------------------------------------------------------- statics
pub struct St {
    // E
    pub E_f: String,
    pub E_c: String,
    pub E_i: bool,
    pub E_j: i32,
    pub E_b: i32,
    pub E_e: bool,
    pub E_d: bool,
    pub E_g: String,
    pub E_a: i32,
    pub E_h: i32,
    // I
    pub I_a: i32,
    pub I_c: i32,
    pub I_b: i32,
    // K
    pub K_b: i32,
    pub K_a: bool,
    // c
    pub c_a: [i32; 5],
    pub c_c: [i32; 5],
    pub c_b: [i32; 5],
    // C
    pub C_p: [i32; 20],
    pub C_n: [i32; 20],
    pub C_g: [i32; 20],
    pub C_d: [i32; 20],
    pub C_o: [i32; 20],
    pub C_l: [i32; 20],
    pub C_k: [i32; 20],
    pub C_f: [i32; 20],
    pub C_e: [i32; 20],
    pub C_c: [[i32; 20]; 3],
    pub C_a: [[i32; 20]; 3],
    pub C_i: [[i32; 20]; 3],
    pub C_j: [[i32; 20]; 2],
    pub C_h: [[i32; 20]; 2],
    pub C_m: [[i32; 20]; 2],
    pub C_b: [[i32; 20]; 2],
    // p
    pub p_c: [i32; 2],
    pub p_b: [i32; 2],
    pub p_g: [i32; 2],
    pub p_j: [i32; 2],
    pub p_h: [i32; 2],
    pub p_i: [i32; 2],
    pub p_e: [i32; 15],
    pub p_d: [i32; 15],
    pub p_f: [i32; 15],
    pub p_a: [i32; 15],
    // x
    pub x_f: i32,
    pub x_i: [i32; 4],
    pub x_a: [i32; 4],
    pub x_e: [i32; 4],
    pub x_c: [i32; 4],
    pub x_k: [i32; 4],
    pub x_d: [i32; 4],
    pub x_j: [i32; 4],
    pub x_h: [i32; 4],
    pub x_g: [i32; 4],
    // j (pilot stats & saved game state)
    pub j_o: i32,
    pub j_m: i32,
    pub j_f: i32,
    pub j_b: i32,
    pub j_n: i32,
    pub j_a: i32,
    pub j_i: i32,
    pub j_e: i32,
    pub j_d: i32,
    pub j_g: i32,
    pub j_k: bool,
    pub j_l: i32,
    pub j_h: i32,
    pub j_j: i32,
    pub j_c: i32,
    // i (enemy stats)
    pub i_d: i32,
    pub i_c: i32,
    pub i_g: i32,
    pub i_b: i32,
    pub i_f: i32,
    pub i_a: i32,
    // h
    pub h_b: i32,
    // J
    pub J_a: i32,
    // l
    pub l_j: bool,
    pub l_d: i32,
    pub l_p: i32,
    // L
    pub L_H: i32,
    pub L_M: i32,
    pub L_a: i32,
    pub L_e: i32,
    pub L_v: i32,
    pub L_D: bool,
    pub L_k: i32,
    // k
    pub k_b: i32,
    pub k_y: i32,
    pub k_m: i32,
    pub k_W: i32,
    pub k_c: i32,
    pub k_B: i32,
    pub k_U: i32,
    pub k_E: i32,
    pub k_X: i32,
    pub k_q: i32,
    pub k_T: i32,
    pub k_Z: i32,
    pub k_P: i32,
    pub k_r: i32,
    pub k_ab: String,
    pub k_S: String,
    // G
    pub G_r: i32,
    pub G_e: i32,
    pub G_k: i32,
    pub G_b: i32,
    pub G_o: i32,
    pub G_g: i32,
    pub G_m: i32,
    pub G_s: i32,
    pub G_j: i32,
    // s
    pub s_e: i32,
    pub s_c: i32,
    // B
    pub B_c: i32,
    // u
    pub u_h: i32,
    pub u_d: i32,
    pub u_l: i32,
    pub u_m: bool,
    pub u_b: i32,
    // PhantomSpider
    pub ps_u: [i32; 3],
    pub ps_G: bool,
}

impl Default for St {
    fn default() -> St {
        St {
            E_f: String::new(),
            E_c: String::new(),
            E_i: false,
            E_j: 1,
            E_b: 2,
            E_e: false,
            E_d: false,
            E_g: String::new(),
            E_a: 0,
            E_h: 0,
            I_a: -1,
            I_c: 1,
            I_b: -1,
            K_b: 0,
            K_a: false,
            c_a: [0; 5],
            c_c: [0; 5],
            c_b: [0; 5],
            C_p: [0; 20],
            C_n: [0; 20],
            C_g: [0; 20],
            C_d: [0; 20],
            C_o: [0; 20],
            C_l: [0; 20],
            C_k: [0; 20],
            C_f: [0; 20],
            C_e: [0; 20],
            C_c: [[0; 20]; 3],
            C_a: [[0; 20]; 3],
            C_i: [[0; 20]; 3],
            C_j: [[0; 20]; 2],
            C_h: [[0; 20]; 2],
            C_m: [[0; 20]; 2],
            C_b: [[0; 20]; 2],
            p_c: [0; 2],
            p_b: [0; 2],
            p_g: [0; 2],
            p_j: [0; 2],
            p_h: [0; 2],
            p_i: [0; 2],
            p_e: [0; 15],
            p_d: [0; 15],
            p_f: [0; 15],
            p_a: [0; 15],
            x_f: 0,
            x_i: [0; 4],
            x_a: [0; 4],
            x_e: [0; 4],
            x_c: [0; 4],
            x_k: [0; 4],
            x_d: [0; 4],
            x_j: [0; 4],
            x_h: [0; 4],
            x_g: [0; 4],
            j_o: 0,
            j_m: 0,
            j_f: -1,
            j_b: 1,
            j_n: 100,
            j_a: 10,
            j_i: 10,
            j_e: 3,
            j_d: 0,
            j_g: 0,
            j_k: false,
            j_l: 0,
            j_h: 0,
            j_j: 0,
            j_c: 0,
            i_d: 500,
            i_c: 5,
            i_g: 90,
            i_b: 60,
            i_f: 1,
            i_a: 0,
            h_b: 150,
            J_a: 0,
            l_j: false,
            l_d: 0,
            l_p: 0,
            L_H: 0,
            L_M: 0,
            L_a: 128,
            L_e: 0,
            L_v: 0,
            L_D: false,
            L_k: 0,
            k_b: 0,
            k_y: 0,
            k_m: 0,
            k_W: 0,
            k_c: 0,
            k_B: 0,
            k_U: 0,
            k_E: 0,
            k_X: 0,
            k_q: 0,
            k_T: 0,
            k_Z: 0,
            k_P: 0,
            k_r: 0,
            k_ab: String::new(),
            k_S: String::new(),
            G_r: 0,
            G_e: 0,
            G_k: 0,
            G_b: 0,
            G_o: 0,
            G_g: 0,
            G_m: 0,
            G_s: 0,
            G_j: 0,
            s_e: 0,
            s_c: 0,
            B_c: 0,
            u_h: 0,
            u_d: 0,
            u_l: 0,
            u_m: false,
            u_b: 0,
            ps_u: [0; 3],
            ps_G: true,
        }
    }
}

// ---------------------------------------------------------------- class D
/// `D`: wraps one MMAPI Player.
#[derive(Default)]
pub struct Snd {
    pub c: Option<usize>,
    pub a: i32,
}

impl Snd {
    /// `a()`: close
    pub fn close(&mut self, au: &mut Audio) {
        if let Some(c) = self.c {
            au.close(c);
        }
    }
    /// `b()`: getState() == CLOSED (NPE if never created)
    pub fn is_closed(&self, au: &Audio) -> Res<bool> {
        let c = self.c.ok_or(crate::gfx::NPE)?;
        Ok(au.state(c) == crate::audio::CLOSED)
    }
    fn create(&mut self, au: &mut Audio, path: &str) {
        self.c = assets::get(path).and_then(|d| au.create(d));
        if let Some(c) = self.c {
            au.realize(c);
            au.prefetch(c);
        }
    }
    fn play(&mut self, au: &mut Audio, path: &str, v3: i32) {
        // D.b is always true (sound on)
        if let (true, Some(c)) = (v3 == self.a, self.c) {
            if au.state(c) != crate::audio::CLOSED {
                au.stop(c);
                if v3 == 1 || v3 == 3 {
                    au.set_loop_count(c, -1);
                }
                au.start(c);
                return;
            }
        }
        self.close(au);
        self.create(au, path);
        if let Some(c) = self.c {
            if v3 == 1 || v3 == 3 {
                au.set_loop_count(c, -1);
            }
            au.start(c);
            self.a = v3;
        }
    }
    /// `a(int)`
    pub fn tune(&mut self, au: &mut Audio, n: i32) {
        match n {
            0 => self.play(au, "/sound/opening.mid", 0),
            1 => self.play(au, "/sound/instruction.mid", 1),
            2 => self.play(au, "/sound/highscore.mid", 2),
            3 => self.play(au, "/sound/background.mid", 3),
            _ => {}
        }
    }
}

// ---------------------------------------------------------------- display
pub enum Cur {
    None,
    K,
    L,
    Scr(Box<Screen>),
}

impl Cur {
    pub fn is_canvas(&self) -> bool {
        matches!(self, Cur::K | Cur::L)
    }
}

/// Eye (`v`)
#[derive(Clone)]
pub struct Eye {
    pub b: i32,
    pub d: i32,
    pub a: String,
}

pub struct Game {
    pub img: Images,
    pub st: St,
    /// `J.c`
    pub eyes: Vec<Eye>,
    pub ps: midlet::Midlet,
    pub kc: kcanvas::KCanvas,
    pub lc: lcanvas::LCanvas,
    pub bat: battle::Battle,
    pub tut: tutorial::Tut,
    pub cur: Cur,
    pub after_alert: Option<Cur>,
    pub alert_left_ms: i32,
    pub show_paint: bool,
    pub quit: bool,
    pub audio: Audio,
    /// gamepad rumble requests (low, high, ms)
    pub rumble: Option<(u16, u16, u32)>,
    acc_ms: f64,
    blink_ms: f64,
}

impl Game {
    /// `Display.setCurrent`
    pub fn set_current(&mut self, d: Cur) {
        if let Cur::Scr(s) = &d {
            if s.kind == crate::lcdui::Kind::Alert {
                self.alert_left_ms = s.timeout_ms;
                let prev = std::mem::replace(&mut self.cur, Cur::None);
                if !matches!(&prev, Cur::Scr(p) if p.kind == crate::lcdui::Kind::Alert) {
                    self.after_alert = Some(prev);
                }
                self.cur = d;
                return;
            }
        }
        let same = matches!((&self.cur, &d), (Cur::K, Cur::K) | (Cur::L, Cur::L));
        if d.is_canvas() && !same {
            self.show_paint = true;
        }
        self.cur = d;
    }

    pub fn dismiss_alert(&mut self) {
        let next = self.after_alert.take().unwrap_or(Cur::None);
        if next.is_canvas() {
            self.show_paint = true;
        }
        self.cur = next;
        if matches!(self.cur, Cur::None) {
            // nothing to return to; fall back to the main canvas
            self.cur = Cur::K;
            self.show_paint = true;
        }
    }

    /// Route END_OF_MEDIA to the owning `D` (playerUpdate -> this.a()).
    pub fn audio_events(&mut self) {
        for id in self.audio.take_ended() {
            if self.kc.F.c == Some(id) {
                self.kc.F.close(&mut self.audio);
            } else if self.lc.J.c == Some(id) {
                self.lc.J.close(&mut self.audio);
            } else {
                self.audio.close(id);
            }
        }
    }
}

impl Game {
    /// `startApp()`: build the MIDlet the way the original constructs it.
    pub fn new(audio: Audio, fire_key: &str) -> Game {
        let mut img = Images::default();
        let kc = kcanvas::KCanvas::new(&mut img);
        let tut = tutorial::Tut::new(&mut img);
        let bat = battle::Battle::new(&mut img);
        let lc = lcanvas::LCanvas::new(&mut img);
        let mut g = Game {
            img,
            st: St::default(),
            eyes: Vec::new(),
            ps: midlet::Midlet { r: fire_key.to_string(), ..Default::default() },
            kc,
            lc,
            bat,
            tut,
            cur: Cur::None,
            after_alert: None,
            alert_left_ms: 0,
            show_paint: false,
            quit: false,
            audio,
            rumble: None,
            acc_ms: 0.0,
            blink_ms: 0.0,
        };
        g.st.k_ab = "50001".into();
        g.k_ctor();
        g.J_new(); // static `u.a = new J()`
        g.L_ctor();
        g.ps_load();
        g.set_current(Cur::K);
        g
    }

    /// Key press from the platform (MIDP key code).
    pub fn key_pressed(&mut self, code: i32, repeat: bool, font: &crate::font::Font) {
        match &mut self.cur {
            Cur::K => {
                if !repeat {
                    self.k_key_pressed(code)
                }
            }
            Cur::L => {
                if !repeat {
                    self.L_key_pressed(code)
                }
            }
            Cur::Scr(s) => {
                let r = s.key(code, font);
                let snap = (**s).clone();
                match r {
                    Err(()) => self.dismiss_alert(),
                    Ok(Some(cmd)) => self.ps_command(cmd, &snap),
                    Ok(None) => {}
                }
            }
            Cur::None => {}
        }
    }

    pub fn key_released(&mut self, code: i32) {
        if let Cur::L = self.cur {
            self.L_key_released(code);
        }
    }

    pub fn screen_is_form(&self) -> bool {
        matches!(&self.cur, Cur::Scr(_))
    }

    fn paint_canvas(&mut self, g: &mut crate::gfx::Graphics, forced: bool) {
        match self.cur {
            Cur::K => self.k_paint(g),
            Cur::L => {
                if self.lc.i_a == 0 || forced {
                    self.L_paint(g)
                }
            }
            _ => {}
        }
        g.reset_clip();
    }

    /// Advance time by `dt_ms`: repaint thread (`h`), alert timeouts,
    /// audio events, and drawing of high-level screens.
    pub fn update(&mut self, dt_ms: f64, g: &mut crate::gfx::Graphics) {
        self.audio_events();
        if let Cur::Scr(s) = &self.cur {
            if s.kind == crate::lcdui::Kind::Alert {
                self.alert_left_ms -= dt_ms as i32;
                if self.alert_left_ms <= 0 {
                    self.dismiss_alert();
                }
            }
        }
        self.blink_ms += dt_ms;
        let period = self.st.h_b.max(10) as f64;
        self.acc_ms += dt_ms;
        if self.show_paint {
            self.show_paint = false;
            self.acc_ms = 0.0;
            self.paint_canvas(g, true);
        } else if self.acc_ms >= period {
            self.acc_ms -= period;
            if self.acc_ms > period * 2.0 {
                self.acc_ms = 0.0;
            }
            self.paint_canvas(g, false);
        }
        let blink = (self.blink_ms as i64 / 400) % 2 == 0;
        if let Cur::Scr(s) = &mut self.cur {
            s.draw(g, blink);
        }
    }
}

impl Game {
    /// The shooting canvas is showing.
    pub fn in_shooter(&self) -> bool {
        matches!(self.cur, Cur::L)
    }
}
