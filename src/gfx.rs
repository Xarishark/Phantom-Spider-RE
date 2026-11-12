//! Minimal re-implementation of the MIDP 2.0 `Graphics`, `Image` and
//! `game.Sprite` classes, rendering into a 128x128 ARGB framebuffer.

use crate::font::Font;
use std::rc::Rc;

pub const SCREEN_W: i32 = 128;
pub const SCREEN_H: i32 = 128;

/// A Java exception that escaped. Paint/key handlers return it the way the
/// original code relied on `try { ... } catch (Exception e)` to abort a frame.
#[derive(Debug, Clone, Copy)]
pub struct Jex(#[allow(dead_code)] pub &'static str);
pub type Res<T = ()> = Result<T, Jex>;
pub const NPE: Jex = Jex("NullPointerException");
pub const AIOOBE: Jex = Jex("ArrayIndexOutOfBoundsException");

pub trait OptExt<T> {
    fn npe(&self) -> Res<&T>;
    fn npe_mut(&mut self) -> Res<&mut T>;
}
impl<T> OptExt<T> for Option<T> {
    fn npe(&self) -> Res<&T> {
        self.as_ref().ok_or(NPE)
    }
    fn npe_mut(&mut self) -> Res<&mut T> {
        self.as_mut().ok_or(NPE)
    }
}

// Anchor constants
pub const HCENTER: i32 = 1;
pub const VCENTER: i32 = 2;
pub const LEFT: i32 = 4;
pub const RIGHT: i32 = 8;
pub const TOP: i32 = 16;
pub const BOTTOM: i32 = 32;
pub const BASELINE: i32 = 64;

pub struct Image {
    pub w: i32,
    pub h: i32,
    /// ARGB8888
    pub px: Vec<u32>,
}
pub type Img = Option<Rc<Image>>;

impl Image {
    pub fn from_png(data: &[u8]) -> Option<Image> {
        let mut dec = png::Decoder::new(std::io::Cursor::new(data));
        dec.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = dec.read_info().ok()?;
        let mut buf = vec![0u8; reader.output_buffer_size()?];
        let info = reader.next_frame(&mut buf).ok()?;
        let (w, h) = (info.width as usize, info.height as usize);
        let bpp = match info.color_type {
            png::ColorType::Grayscale => 1,
            png::ColorType::GrayscaleAlpha => 2,
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            png::ColorType::Indexed => return None,
        };
        let line = info.line_size;
        let mut px = Vec::with_capacity(w * h);
        for y in 0..h {
            let row = &buf[y * line..y * line + w * bpp];
            for x in 0..w {
                let p = &row[x * bpp..x * bpp + bpp];
                let (r, g, b, a) = match bpp {
                    1 => (p[0], p[0], p[0], 255),
                    2 => (p[0], p[0], p[0], p[1]),
                    3 => (p[0], p[1], p[2], 255),
                    _ => (p[0], p[1], p[2], p[3]),
                };
                px.push((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32);
            }
        }
        Some(Image { w: w as i32, h: h as i32, px })
    }
}

pub struct Graphics {
    pub fb: Vec<u32>,
    color: u32,
    clip: (i32, i32, i32, i32), // x0, y0, x1, y1 (exclusive)
    pub font: Rc<Font>,
}

#[inline]
fn blend(dst: u32, src: u32) -> u32 {
    let a = src >> 24;
    if a == 255 {
        return src | 0xff00_0000;
    }
    if a == 0 {
        return dst;
    }
    let ia = 255 - a;
    let ch = |s: u32| -> u32 { (((src >> s) & 0xff) * a + ((dst >> s) & 0xff) * ia) / 255 };
    0xff00_0000 | ch(16) << 16 | ch(8) << 8 | ch(0)
}

impl Graphics {
    pub fn new(font: Rc<Font>) -> Self {
        Graphics {
            fb: vec![0xff00_0000; (SCREEN_W * SCREEN_H) as usize],
            color: 0xff00_0000,
            clip: (0, 0, SCREEN_W, SCREEN_H),
            font,
        }
    }

    pub fn set_color(&mut self, r: i32, g: i32, b: i32) {
        self.color = 0xff00_0000 | ((r as u32 & 0xff) << 16) | ((g as u32 & 0xff) << 8) | (b as u32 & 0xff);
    }
    pub fn set_color_rgb(&mut self, rgb: u32) {
        self.color = 0xff00_0000 | (rgb & 0xff_ffff);
    }

    pub fn set_clip(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let x0 = x.max(0);
        let y0 = y.max(0);
        let x1 = (x.saturating_add(w)).min(SCREEN_W);
        let y1 = (y.saturating_add(h)).min(SCREEN_H);
        self.clip = (x0, y0, x1.max(x0), y1.max(y0));
    }
    pub fn reset_clip(&mut self) {
        self.clip = (0, 0, SCREEN_W, SCREEN_H);
    }

    #[inline]
    fn put(&mut self, x: i32, y: i32, c: u32) {
        let (x0, y0, x1, y1) = self.clip;
        if x >= x0 && x < x1 && y >= y0 && y < y1 {
            let i = (y * SCREEN_W + x) as usize;
            self.fb[i] = blend(self.fb[i], c);
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w <= 0 || h <= 0 {
            return;
        }
        let (cx0, cy0, cx1, cy1) = self.clip;
        let xa = x.max(cx0);
        let ya = y.max(cy0);
        let xb = x.saturating_add(w).min(cx1);
        let yb = y.saturating_add(h).min(cy1);
        let c = self.color;
        for yy in ya..yb {
            let row = (yy * SCREEN_W) as usize;
            for xx in xa..xb {
                self.fb[row + xx as usize] = c;
            }
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w < 0 || h < 0 {
            return;
        }
        let c = self.color;
        for xx in x..=x + w {
            self.put(xx, y, c);
            self.put(xx, y + h, c);
        }
        for yy in y..=y + h {
            self.put(x, yy, c);
            self.put(x + w, yy, c);
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let c = self.color;
        let (mut x, mut y) = (x1, y1);
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.put(x, y, c);
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        let pts = [(x1, y1), (x2, y2), (x3, y3)];
        let ymin = y1.min(y2).min(y3);
        let ymax = y1.max(y2).max(y3);
        let c = self.color;
        for y in ymin..=ymax {
            let mut lo = f32::MAX;
            let mut hi = f32::MIN;
            for i in 0..3 {
                let (ax, ay) = pts[i];
                let (bx, by) = pts[(i + 1) % 3];
                if (y < ay.min(by)) || (y > ay.max(by)) {
                    continue;
                }
                if ay == by {
                    lo = lo.min(ax.min(bx) as f32);
                    hi = hi.max(ax.max(bx) as f32);
                } else {
                    let t = (y - ay) as f32 / (by - ay) as f32;
                    let xi = ax as f32 + t * (bx - ax) as f32;
                    lo = lo.min(xi);
                    hi = hi.max(xi);
                }
            }
            if lo <= hi {
                for x in lo.round() as i32..=hi.round() as i32 {
                    self.put(x, y, c);
                }
            }
        }
    }

    /// Only full ellipses (0..360) are used by the game.
    pub fn fill_arc(&mut self, x: i32, y: i32, w: i32, h: i32, _sa: i32, _aa: i32) {
        let (rx, ry) = (w as f32 / 2.0, h as f32 / 2.0);
        let (cx, cy) = (x as f32 + rx, y as f32 + ry);
        let c = self.color;
        for yy in y..y + h {
            for xx in x..x + w {
                let nx = (xx as f32 + 0.5 - cx) / rx;
                let ny = (yy as f32 + 0.5 - cy) / ry;
                if nx * nx + ny * ny <= 1.0 {
                    self.put(xx, yy, c);
                }
            }
        }
    }

    pub fn draw_arc(&mut self, x: i32, y: i32, w: i32, h: i32, _sa: i32, _aa: i32) {
        let (rx, ry) = (w as f32 / 2.0, h as f32 / 2.0);
        let (cx, cy) = (x as f32 + rx, y as f32 + ry);
        let c = self.color;
        let steps = ((w + h) * 4).max(16);
        let mut last = (i32::MIN, i32::MIN);
        for i in 0..steps {
            let a = i as f32 / steps as f32 * std::f32::consts::TAU;
            let px = (cx + rx * a.cos()).round() as i32;
            let py = (cy + ry * a.sin()).round() as i32;
            if (px, py) != last {
                self.put(px, py, c);
                last = (px, py);
            }
        }
    }

    pub fn draw_string(&mut self, s: &str, x: i32, y: i32, anchor: i32) {
        let font = self.font.clone();
        let w = font.width(s);
        let mut x = x;
        let mut y = y;
        if anchor & HCENTER != 0 {
            x -= w / 2;
        } else if anchor & RIGHT != 0 {
            x -= w;
        }
        if anchor & BOTTOM != 0 {
            y -= font.height;
        } else if anchor & BASELINE != 0 {
            y -= font.ascent;
        }
        let c = self.color;
        font.draw(s, x, y, |px, py| self.put(px, py, c));
    }

    /// `drawImage`. Throws NPE (returns Err) for a null image like Java does.
    pub fn draw_image(&mut self, img: &Img, x: i32, y: i32, anchor: i32) -> Res {
        let img = img.as_ref().ok_or(NPE)?;
        let mut x = x;
        let mut y = y;
        if anchor & HCENTER != 0 {
            x -= img.w / 2;
        } else if anchor & RIGHT != 0 {
            x -= img.w;
        }
        if anchor & VCENTER != 0 {
            y -= img.h / 2;
        } else if anchor & BOTTOM != 0 {
            y -= img.h;
        }
        self.blit(img, 0, 0, img.w, img.h, 0, x, y);
        Ok(())
    }

    /// Blit a region of `img` with a MIDP Sprite transform; (dx,dy) is the
    /// top-left of the transformed region.
    pub fn blit(&mut self, img: &Image, sx: i32, sy: i32, sw: i32, sh: i32, transform: i32, dx: i32, dy: i32) {
        let swap = matches!(transform, 4 | 5 | 6 | 7);
        let (dw, dh) = if swap { (sh, sw) } else { (sw, sh) };
        let (cx0, cy0, cx1, cy1) = self.clip;
        let xa = dx.max(cx0);
        let ya = dy.max(cy0);
        let xb = (dx + dw).min(cx1);
        let yb = (dy + dh).min(cy1);
        for py in ya..yb {
            let ty = py - dy;
            for px in xa..xb {
                let tx = px - dx;
                let (ux, uy) = match transform {
                    0 => (tx, ty),
                    1 => (tx, sh - 1 - ty),
                    2 => (sw - 1 - tx, ty),
                    3 => (sw - 1 - tx, sh - 1 - ty),
                    4 => (ty, tx),
                    5 => (ty, sh - 1 - tx),
                    6 => (sw - 1 - ty, tx),
                    _ => (sw - 1 - ty, sh - 1 - tx),
                };
                let ix = sx + ux;
                let iy = sy + uy;
                if ix < 0 || iy < 0 || ix >= img.w || iy >= img.h {
                    continue;
                }
                let s = img.px[(iy * img.w + ix) as usize];
                if s >> 24 == 0 {
                    continue;
                }
                let i = (py * SCREEN_W + px) as usize;
                self.fb[i] = blend(self.fb[i], s);
            }
        }
    }
}

/// `javax.microedition.lcdui.game.Sprite` (only what the game uses).
pub struct Sprite {
    img: Img,
    fw: i32,
    fh: i32,
    frame: i32,
    ref_x: i32,
    ref_y: i32,
    x: i32,
    y: i32,
    t: i32,
}

impl Sprite {
    pub fn new(img: Img, fw: i32, fh: i32) -> Sprite {
        Sprite { img, fw: fw.max(1), fh: fh.max(1), frame: 0, ref_x: 0, ref_y: 0, x: 0, y: 0, t: 0 }
    }
    fn frame_count(&self) -> i32 {
        match &self.img {
            Some(i) => (i.w / self.fw) * (i.h / self.fh),
            None => 0,
        }
    }
    pub fn width(&self) -> i32 {
        if matches!(self.t, 4..=7) { self.fh } else { self.fw }
    }
    pub fn height(&self) -> i32 {
        if matches!(self.t, 4..=7) { self.fw } else { self.fh }
    }
    pub fn set_image(&mut self, img: &Img, fw: i32, fh: i32) -> Res {
        let i = img.as_ref().ok_or(NPE)?;
        if fw <= 0 || fh <= 0 || i.w % fw != 0 || i.h % fh != 0 {
            return Err(Jex("IllegalArgumentException"));
        }
        self.img = img.clone();
        self.fw = fw;
        self.fh = fh;
        if self.frame >= self.frame_count() {
            self.frame = 0;
        }
        Ok(())
    }
    pub fn set_frame(&mut self, f: i32) -> Res {
        if f < 0 || f >= self.frame_count() {
            return Err(Jex("IndexOutOfBoundsException"));
        }
        self.frame = f;
        Ok(())
    }
    pub fn define_reference_pixel(&mut self, x: i32, y: i32) {
        self.ref_x = x;
        self.ref_y = y;
    }
    fn tpx(&self, x: i32, y: i32, t: i32) -> i32 {
        match t {
            0 | 1 => x,
            2 | 3 => self.fw - x - 1,
            5 | 7 => self.fh - y - 1,
            _ => y, // 4, 6
        }
    }
    fn tpy(&self, x: i32, y: i32, t: i32) -> i32 {
        match t {
            0 | 2 => y,
            1 | 3 => self.fh - y - 1,
            5 | 4 => x,
            _ => self.fw - x - 1, // 6, 7
        }
    }
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn set_ref_pixel_position(&mut self, x: i32, y: i32) {
        self.x = x - self.tpx(self.ref_x, self.ref_y, self.t);
        self.y = y - self.tpy(self.ref_x, self.ref_y, self.t);
    }
    pub fn set_transform(&mut self, t: i32) {
        let (rx, ry) = (self.ref_x, self.ref_y);
        self.x = self.x + self.tpx(rx, ry, self.t) - self.tpx(rx, ry, t);
        self.y = self.y + self.tpy(rx, ry, self.t) - self.tpy(rx, ry, t);
        self.t = t;
    }
    pub fn paint(&self, g: &mut Graphics) -> Res {
        let img = self.img.as_ref().ok_or(NPE)?;
        let per_row = (img.w / self.fw).max(1);
        let sx = (self.frame % per_row) * self.fw;
        let sy = (self.frame / per_row) * self.fh;
        g.blit(img, sx, sy, self.fw, self.fh, self.t, self.x, self.y);
        Ok(())
    }
}
