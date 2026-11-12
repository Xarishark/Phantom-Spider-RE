//! `r` (base actor), `z` (enemy tarantula) and `l` (the pilot's ship).

use super::{load, part, rnd, Images};
use crate::midp::gfx::{Graphics, Res, Sprite, NPE};

/// `r()` constructor: loads the static `r.g` and builds the sprite.
fn r_ctor(img: &mut Images) -> Option<Sprite> {
    img.r_g = load("/spider/spiderheada.png");
    img.r_g.as_ref().map(|g| {
        let (w, h) = (g.w / 2, g.h / 2);
        Sprite::new(img.r_g.clone(), w, h)
    })
}

/// `z extends r`. Note `z` declares its own `b`/`o` (shadowing `r.b`/`r.o`).
pub struct Z {
    // r fields
    pub m: i32,
    pub e: i32,
    pub h: i32,
    pub a: i32,
    pub l: i32,
    pub k: i32,
    pub n: Option<Sprite>,
    // z fields
    pub f: i32,
    pub p: i32,
    pub b: i32,
    pub o: i32,
    pub i: i32,
    pub c: i32,
    /// 60 fps: retreat velocity for this beat, leg wobble sampled per beat,
    /// sub-steps left to show the health bar after a hit
    pub rvx: i32,
    pub rvy: i32,
    pub jit: [i32; 6],
    pub bar: i32,
}

impl Z {
    pub fn new(img: &mut Images) -> Z {
        Z { m: 0, e: 0, h: 0, a: 0, l: 0, k: 0, n: r_ctor(img), f: 0, p: 2, b: 0, o: 0, i: -1, c: 0, rvx: 0, rvy: 0, jit: [0; 6], bar: 0 }
    }

    /// `r.a(int,int,int,int)`
    pub fn set_stats(&mut self, m: i32, e: i32, h: i32, a: i32) {
        self.m = m;
        self.e = e;
        self.h = h;
        self.a = a;
    }

    /// `z.c()` static image loader
    pub fn load_images(img: &mut Images) {
        img.z_d[2] = load("/spider/spiderheada.png");
        img.z_j[2] = load("/spider/spiderlega.png");
        img.z_d[1] = load("/spider/spiderbodyb.png");
        img.z_j[1] = load("/spider/spiderlegb.png");
        img.z_d[0] = load("/spider/spiderbodyc.png");
        img.z_j[0] = load("/spider/spiderlegc.png");
    }

    /// `b()` (move by `e` per original frame) spread over sub-step `k` of `n`,
    /// plus this beat's retreat dash and the health-bar timer.
    pub fn sub(&mut self, k: i32, n: i32) {
        let d = part(self.e, k, n);
        match self.f {
            1 => {
                if self.l + self.e + self.b < 128 {
                    self.l += d;
                }
            }
            2 => {
                if self.l - self.e > 0 {
                    self.l -= d;
                }
            }
            3 => {
                if self.k > 0 {
                    self.k -= d;
                }
            }
            4 => {
                if self.k + self.e + self.o < 128 {
                    self.k += d;
                }
            }
            _ => {}
        }
        self.l += part(self.rvx, k, n);
        self.k += part(self.rvy, k, n);
        if self.bar > 0 {
            self.bar -= 1;
        }
    }

    /// Per-beat leg wobble (the original rolled it on every draw).
    pub fn roll_jitter(&mut self) {
        for j in self.jit.iter_mut() {
            *j = rnd(5);
        }
    }

    /// `a()`
    fn bob(&mut self, lh: i32) {
        if lh != 3 {
            self.f = if self.i < 4 { 4 } else { 3 };
            if self.i > 8 {
                self.i = -2;
            }
            self.i += 1;
        }
    }

    /// `a(int,int)`: AI; returns weapon to fire (0 none, 1/3 egg, 2 web)
    pub fn think(&mut self, v1: i32, v2: i32, lh: i32, ig: i32) -> i32 {
        let mut v3 = 0;
        self.rvx = 0;
        self.rvy = 0;
        if lh != 3 {
            if self.i == -1 {
                match rnd(10) {
                    1 => self.f = if v1 < self.l { 2 } else { 1 },
                    2 => {
                        v3 = if self.p != 0 && self.c == 0 {
                            2
                        } else if v2 > self.k {
                            1
                        } else {
                            3
                        }
                    }
                    3 => {
                        if rnd(10) == 0 {
                            self.i = 0;
                        }
                    }
                    4 => {
                        if rnd(181 - ig) == 0 {
                            self.i = -2;
                        }
                    }
                    5 => v3 = if v2 > self.k { 1 } else { 3 },
                    _ => {}
                }
            } else if self.i == -2 {
                // Retreat: the original moved here *and* in b(); the dash is
                // now a per-beat velocity applied by sub(). The exit check
                // runs on the next beat, after that movement has happened.
                let exited = match self.c {
                    0 => self.k + self.o <= 0,
                    1 => self.l + self.b <= 0,
                    2 => self.l >= 128,
                    _ => false,
                };
                if exited {
                    self.i = -3;
                    v3 = 0;
                    self.c = rnd(3);
                    match self.c {
                        0 => {
                            self.k = -self.o;
                            self.l = rnd(128 - self.b);
                        }
                        1 => {
                            self.k = rnd(128 - self.o);
                            self.l = 128 + self.b;
                        }
                        2 => {
                            self.k = rnd(128 - self.o);
                            self.l = -self.b;
                        }
                        _ => {}
                    }
                } else {
                    match self.c {
                        0 => {
                            self.f = 3;
                            self.rvy = -self.e;
                        }
                        1 => {
                            self.f = 2;
                            self.rvx = -self.e;
                        }
                        2 => {
                            self.f = 1;
                            self.rvx = self.e;
                        }
                        _ => {}
                    }
                    v3 = if ig > 90 {
                        if v2 > self.k {
                            1
                        } else {
                            3
                        }
                    } else {
                        rnd(2)
                    };
                }
            } else if self.i == -3 {
                match self.c {
                    0 => {
                        self.f = 4;
                        if self.k >= 0 {
                            self.i = -1;
                            self.f = 0;
                        }
                    }
                    1 => {
                        self.f = 2;
                        if self.l <= 128 {
                            self.i = -1;
                            self.f = 0;
                        }
                    }
                    2 => {
                        self.f = 1;
                        if self.l >= 0 {
                            self.i = -1;
                            self.f = 0;
                        }
                    }
                    _ => {}
                }
            } else {
                self.bob(lh);
            }
        }
        v3
    }

    fn transform(&self) -> i32 {
        match self.c {
            1 => 5,
            2 => 6,
            _ => 0,
        }
    }

    /// `a(Graphics, frame, x, y, size)`: leg (exceptions swallowed)
    fn leg(&mut self, g: &mut Graphics, img: &Images, fr: i32, x: i32, y: i32, sz: usize) {
        const S: [[i32; 2]; 3] = [[12, 12], [22, 21], [28, 28]];
        let t = self.transform();
        let r: Res = (|| {
            let n = self.n.as_mut().ok_or(NPE)?;
            n.set_image(&img.z_j[sz], S[sz][0], S[sz][1])?;
            n.set_frame(fr)?;
            n.set_ref_pixel_position(x, y);
            n.set_transform(t);
            n.paint(g)
        })();
        let _ = r;
    }

    /// `b(Graphics, frame, x, y, size)`: body (exceptions propagate)
    fn body(&mut self, g: &mut Graphics, img: &Images, fr: i32, x: i32, y: i32, sz: usize) -> Res {
        const S: [[i32; 2]; 3] = [[8, 11], [15, 22], [20, 32]];
        let t = self.transform();
        let n = self.n.as_mut().ok_or(NPE)?;
        n.set_image(&img.z_d[sz], S[sz][0], S[sz][1])?;
        n.set_frame(fr)?;
        n.set_ref_pixel_position(x, y);
        n.set_transform(t);
        n.paint(g)
    }

    /// `a(int)`: size
    pub fn set_size(&mut self, p: i32) {
        self.p = p;
        match p {
            0 => {
                self.b = 30;
                self.o = 27;
            }
            1 => {
                self.b = 50;
                self.o = 42;
            }
            2 => {
                self.b = 55;
                self.o = 40;
            }
            _ => {}
        }
    }

    /// `a(Graphics, x, y)`: medium spider facing down at a position
    pub fn draw_at(&mut self, g: &mut Graphics, img: &Images, x: i32, y: i32) -> Res {
        self.c = 0;
        self.leg(g, img, 2, x, y + 7, 1);
        self.leg(g, img, 3, x + 28, y + 7, 1);
        self.body(g, img, 1, x + 18, y, 1)?;
        self.body(g, img, 0, x + 18, y + 11, 1)?;
        self.leg(g, img, 0, x + 3, y + 20, 1);
        self.leg(g, img, 1, x + 32, y + 21, 1);
        self.leg(g, img, 1, x + 32, y + 21, 1);
        Ok(())
    }

    /// `d(Graphics)`
    pub fn draw_info(&mut self, g: &mut Graphics, img: &Images) -> Res {
        self.draw_at(g, img, 43, 41)
    }

    /// `b(Graphics)`: two spiders facing each other (VS screen)
    pub fn draw_vs(&mut self, g: &mut Graphics, img: &Images) -> Res {
        let (v2, v3) = (75, 54);
        self.c = 1;
        self.leg(g, img, 2, v2 + 20 + 10 + 5, v3 + 7, 1);
        self.leg(g, img, 3, v2 + 20 + 10 + 5, v3 + 20 + 7, 1);
        self.leg(g, img, 0, v2 + 15 + 5, v3 + 4 + 7, 1);
        self.leg(g, img, 1, v2 + 15 + 5, v3 + 25 + 7, 1);
        self.body(g, img, 1, v2 + 26 + 20 + 6, v3 + 10 + 7, 1)?;
        self.body(g, img, 0, v2 + 9 + 25 + 5, v3 + 10 + 7, 1)?;
        self.c = 2;
        let (v2, v3) = (9, 54);
        self.leg(g, img, 2, v2 + 15 - 5, v3 + 36 + 5 + 7, 1);
        self.leg(g, img, 3, v2 + 15 - 5, v3 + 10 + 8 + 7, 1);
        self.leg(g, img, 0, v2 + 30 - 5, v3 + 30 + 5 + 7, 1);
        self.leg(g, img, 1, v2 + 30 - 5, v3 + 10 + 5 + 7, 1);
        self.body(g, img, 1, v2 + 10 - 5 - 4, v3 + 11 + 10 + 3 + 7, 1)?;
        self.body(g, img, 0, v2 + 21 - 5 - 4, v3 + 11 + 10 + 3 + 7, 1)?;
        Ok(())
    }

    /// `a(Graphics)`: main in-game draw
    pub fn draw(&mut self, g: &mut Graphics, img: &Images, lh: i32) -> Res {
        let (l, k) = (self.l, self.k);
        // leg wobble: sampled once per beat (roll_jitter), none while paused
        let jit = self.jit;
        let mut ji = 0usize;
        let mut j = |_: ()| {
            let v = if lh != 3 { jit[ji % 6] } else { 0 };
            ji += 1;
            v
        };
        match self.p {
            0 => {
                if self.c == 0 {
                    self.leg(g, img, 2, l + 1, k + 4 + j(()), 0);
                    self.leg(g, img, 3, l + 17, k + 4 + j(()), 0);
                    self.body(g, img, 1, l + 11, k + 1 + j(()), 0)?;
                    self.body(g, img, 0, l + 11, k + 9 + j(()), 0)?;
                    self.leg(g, img, 0, l + 2, k + 13 + j(()), 0);
                    self.leg(g, img, 1, l + 19, k + 13 + j(()), 0);
                } else if self.c == 1 {
                    self.leg(g, img, 2, l + 20 + j(()), k + 1, 0);
                    self.leg(g, img, 3, l + 20 + j(()), k + 17, 0);
                    self.leg(g, img, 0, l + 2 + 8 + j(()), k + 2, 0);
                    self.leg(g, img, 1, l + 2 + 8 + j(()), k + 19, 0);
                    self.body(g, img, 1, l + 18 + 5 + j(()), k + 10, 0)?;
                    self.body(g, img, 0, l + 7 + 10 + j(()), k + 10, 0)?;
                } else {
                    self.leg(g, img, 2, l + 8 + j(()), k + 19 + 8, 0);
                    self.leg(g, img, 3, l + 8 + j(()), k + 3 + 8, 0);
                    self.leg(g, img, 0, l + 18 + j(()), k + 18 + 8, 0);
                    self.leg(g, img, 1, l + 18 + j(()), k + 3 + 8, 0);
                    self.body(g, img, 1, l + 5 + j(()), k + 8 + 7, 0)?;
                    self.body(g, img, 0, l + 15 + j(()), k + 8 + 7, 0)?;
                }
            }
            1 => {
                if self.c == 0 {
                    self.leg(g, img, 2, l, k + 7 + j(()), 1);
                    self.leg(g, img, 3, l + 28, k + 7 + j(()), 1);
                    self.body(g, img, 1, l + 18, k + j(()), 1)?;
                    self.body(g, img, 0, l + 18, k + 11 + j(()), 1)?;
                    self.leg(g, img, 0, l + 3, k + 20 + j(()), 1);
                    self.leg(g, img, 1, l + 32, k + 21 + j(()), 1);
                } else if self.c == 1 {
                    self.leg(g, img, 2, l + 20 + 10 + 5 + j(()), k, 1);
                    self.leg(g, img, 3, l + 20 + 10 + 5 + j(()), k + 20, 1);
                    self.leg(g, img, 0, l + 15 + 5 + j(()), k + 4, 1);
                    self.leg(g, img, 1, l + 15 + 5 + j(()), k + 25, 1);
                    self.body(g, img, 1, l + 26 + 20 + j(()), k + 10, 1)?;
                    self.body(g, img, 0, l + 9 + 25 + 3 + j(()), k + 10, 1)?;
                } else {
                    self.leg(g, img, 2, l + 15 - 5 + j(()), k + 36 + 5, 1);
                    self.leg(g, img, 3, l + 15 - 5 + j(()), k + 10 + 8, 1);
                    self.leg(g, img, 0, l + 30 - 5 + j(()), k + 30 + 5, 1);
                    self.leg(g, img, 1, l + 30 - 5 + j(()), k + 10 + 5, 1);
                    self.body(g, img, 1, l + 10 - 5 + j(()), k + 11 + 10, 1)?;
                    self.body(g, img, 0, l + 21 - 5 + j(()), k + 11 + 10, 1)?;
                }
            }
            2 => {
                if self.c == 0 {
                    self.leg(g, img, 0, l + 6, k + 32 + j(()), 2);
                    self.leg(g, img, 1, l + 41, k + 32 + j(()), 2);
                    self.leg(g, img, 2, l, k + 12 + j(()), 2);
                    self.leg(g, img, 3, l + 40, k + 12 + j(()), 2);
                    self.body(g, img, 2, l + 24, k + j(()), 2)?;
                    self.body(g, img, 0, l + 24, k + 14 + j(()), 2)?;
                } else if self.c == 1 {
                    self.leg(g, img, 0, l + 25 - 5 + j(()), k + 6 + 5 - 15, 2);
                    self.leg(g, img, 1, l + 25 - 5 + j(()), k + 41 - 15, 2);
                    self.leg(g, img, 2, l + 30 + 20 - 10 + j(()), k + 5 - 15, 2);
                    self.leg(g, img, 3, l + 30 + 19 - 10 + j(()), k + 40 - 15, 2);
                    self.body(g, img, 2, l + 30 + 30 - 5 + j(()), k + 20 - 12, 2)?;
                    self.body(g, img, 0, l + 25 + 24 - 5 + j(()), k + 20 - 12, 2)?;
                } else if self.c == 2 {
                    self.leg(g, img, 0, l + 32 - 5 + j(()), k + 31 + 10, 2);
                    self.leg(g, img, 1, l + 32 - 5 + j(()), k + 15, 2);
                    self.leg(g, img, 2, l + 22 - 10 + j(()), k + 41 + 10, 2);
                    self.leg(g, img, 3, l + 23 - 10 + j(()), k + 15, 2);
                    self.body(g, img, 2, l - 5 + 15 - 5 + j(()), k + 11 + 13, 2)?;
                    self.body(g, img, 0, l + 29 - 10 + j(()), k + 12 + 13, 2)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// `c(Graphics)`: health bar
    pub fn health_bar(&self, g: &mut Graphics) {
        let v2 = [40, 50, 60];
        let (v3, v4) = (40, 5);
        g.set_color(0, 255, 0);
        g.fill_rect(self.l, self.k + v2[(self.p % 3) as usize], v3, v4);
        g.set_color(255, 0, 0);
        g.fill_rect(self.l + 1, self.k + v2[(self.p % 3) as usize] + 1, self.m * v3 / 1000, v4 - 2);
    }
}

/// `l extends r`: the pilot's ship.
pub struct Plane {
    pub m: i32,
    pub e: i32,
    pub h: i32,
    pub a: i32,
    pub l: i32,
    pub k: i32,
    pub b: i32,
    pub o: i32,
    pub n: Option<Sprite>,
    pub f: i32,
    pub c: bool,
    /// caught in a web: no movement for this beat
    pub frozen: bool,
}

impl Plane {
    pub fn new(img: &mut Images) -> Plane {
        let mut n = r_ctor(img);
        img.l_i = load("/plane01.png");
        if let Some(s) = n.as_mut() {
            if s.set_image(&img.l_i, 22, 20).is_ok() {
                let (w, h) = (s.width(), s.height());
                s.define_reference_pixel(w / 2, h / 2);
            }
        }
        Plane { m: 0, e: 10, h: 0, a: 0, l: 0, k: 0, b: 22, o: 20, n, f: 0, c: false, frozen: false }
    }

    pub fn set_stats(&mut self, m: i32, e: i32, h: i32, a: i32) {
        self.m = m;
        self.e = e;
        self.h = h;
        self.a = a;
    }

    /// `a(Graphics, x, y)` (high score screen, exceptions swallowed)
    pub fn draw_at(&mut self, g: &mut Graphics, x: i32, y: i32) {
        if let Some(n) = self.n.as_mut() {
            if n.set_frame(1).is_ok() {
                n.set_position(x, y);
                n.set_transform(6);
                let _ = n.paint(g);
            }
        }
    }

    /// `a(Graphics)`
    pub fn draw(&mut self, g: &mut Graphics) -> Res {
        let (fr, t) = match self.f {
            1 => (1, 2),
            2 => (1, 0),
            _ => (0, 0),
        };
        let (x, y) = if !self.c { (self.l, self.k) } else { (self.l + rnd(5) - 3, self.k + rnd(5) - 3) };
        let n = self.n.as_mut().ok_or(NPE)?;
        n.set_frame(fr)?;
        n.set_position(x, y);
        n.set_transform(t);
        n.paint(g)
    }

    /// Discrete part of `a()`: a web catch (`c`) freezes the pilot for this
    /// beat and pauses the fire cooldown `l_d`.
    pub fn beat(&mut self, l_d: &mut i32) {
        if self.c {
            self.c = false;
            self.frozen = true;
        } else {
            self.frozen = false;
            if *l_d > 0 {
                *l_d -= 1;
            }
        }
    }

    /// Movement part of `a()` for sub-step `k` of `n`. Reads the current
    /// direction every sub-step, so input takes effect immediately.
    pub fn sub(&mut self, k: i32, n: i32) {
        if self.frozen {
            return;
        }
        let d = part(self.e, k, n);
        match self.f {
            1 => {
                if self.l + self.e + 15 < 128 {
                    self.l += d;
                }
            }
            2 => {
                if self.l - self.e > 0 {
                    self.l -= d;
                }
            }
            3 => {
                if self.k - self.e > 0 {
                    self.k -= d;
                }
            }
            4 => {
                if self.k + self.e + 15 < 128 {
                    self.k += d;
                }
            }
            _ => {}
        }
    }
}
