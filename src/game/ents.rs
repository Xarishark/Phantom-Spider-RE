//! The abstract class `F` and its subclasses (projectiles, protectors,
//! enemy eggs and webs). Java field shadowing is preserved explicitly:
//! e.g. `q.j`/`f.j` are separate from `B.j`, and `a.a`/`a.h` shadow `F.a`/`F.h`.
//!
//! 60 fps model: the original `a()` (one update per 150 ms frame) is split
//! into [`Ent::beat`] (discrete logic, run once per original frame, which
//! also decides this beat's velocity) and [`Ent::sub`] (moves a fraction of
//! that velocity every 1/60 s). [`Ent::update`] does both at once for code
//! that still runs at the original rate (the tutorial).

use super::{load, part, Images, St};
use crate::midp::gfx::{Graphics, Res, LEFT, TOP};

const TL: i32 = TOP | LEFT;

/// `a.m` orbit table
pub const PATH: [[i32; 2]; 13] = [
    [37, 10], [32, 0], [27, -5], [16, -10], [-5, -5], [-10, 0], [-15, 10], [-10, 20], [-5, 25], [16, 30], [27, 25],
    [32, 20], [32, 20],
];

/// Fields declared in class `a` (protector), inherited by `N` and `O`.
#[derive(Clone, Copy)]
pub struct Prot {
    pub i: i32,
    pub c: i32,
    pub a: i32,
    pub h: i32,
    /// orbit offset at the start of this beat and at the next one (60 fps)
    pub o0: [i32; 2],
    pub o1: [i32; 2],
}

impl Prot {
    fn offset(&self) -> [i32; 2] {
        PATH[((self.i + self.c * 2) % 12) as usize]
    }
    /// One original frame of orbit progress (`i++` with wrap).
    fn advance(&mut self) {
        self.o0 = self.offset();
        self.i = if self.i < 11 { self.i + 1 } else { 0 };
        self.o1 = self.offset();
    }
}

pub enum Kind {
    /// `A`: no weapon
    A,
    /// `B`: missile
    B { j: i32 },
    /// `q extends B` (own shadowing `j`)
    Q { bj: i32, qj: i32 },
    /// `f extends B` (own `j`, `i`, and two `b` burns)
    Fm { bj: i32, fj: i32, i: bool, k: Box<[Ent; 2]> },
    /// `g`: basic shot
    G,
    /// `y`: laser
    Y,
    /// `o`: flame
    O,
    /// `b extends o`: burn
    Burn { c: i32 },
    /// `a`: protector
    Prot(Prot),
    /// `N extends a`
    ProtN { p: Prot, j: Box<[Ent; 3]>, l: bool, k: i32 },
    /// `O extends a`
    ProtO { p: Prot, l: bool, k: i32 },
    /// `d`: flame earth
    D,
    /// `e`: spider egg
    Egg { i: i32, c: i32, j: i32 },
    /// `t`: spider web. `c`: 0/-1 = still the projectile sprite, else web
    Web { c: i32, i: bool },
}

pub struct Ent {
    pub g: i32,
    pub e: i32,
    pub f: i32,
    pub b: i32,
    pub a: i32,
    pub h: i32,
    pub d: bool,
    pub kind: Kind,
    /// velocity for the current beat (pixels per original frame)
    pub vx: i32,
    pub vy: i32,
    /// launched since the last beat; gets its first beat on the next sub-step
    pub fresh: bool,
}

fn base(b: i32, kind: Kind) -> Ent {
    Ent { g: 0, e: 0, f: 0, b, a: 0, h: 0, d: false, kind, vx: 0, vy: 0, fresh: false }
}

impl Ent {
    pub fn new_a() -> Ent {
        base(0, Kind::A)
    }
    pub fn new_b(v: i32, st: &mut St) -> Ent {
        st.B_c = v;
        base(v, Kind::B { j: 0 })
    }
    pub fn new_q(v: i32, st: &mut St) -> Ent {
        st.B_c = v;
        base(v, Kind::Q { bj: 0, qj: 0 })
    }
    pub fn new_f(v: i32, st: &mut St) -> Ent {
        st.B_c = v;
        let k = Box::new([Ent::new_burn(v, 0), Ent::new_burn(v, 1)]);
        base(v, Kind::Fm { bj: 0, fj: 0, i: false, k })
    }
    pub fn new_g(v: i32) -> Ent {
        base(v, Kind::G)
    }
    pub fn new_y(v: i32) -> Ent {
        base(v, Kind::Y)
    }
    pub fn new_o(v: i32) -> Ent {
        base(v, Kind::O)
    }
    pub fn new_burn(v1: i32, v2: i32) -> Ent {
        base(v1, Kind::Burn { c: v2 })
    }
    fn prot_base(v1: i32, v2: i32) -> (Ent, Prot) {
        let mut e = base(v2, Kind::A);
        e.d = true;
        e.g = -10;
        e.e = -10;
        let mut p = Prot { i: 0, c: v1, a: 9, h: 9, o0: [0; 2], o1: [0; 2] };
        p.o0 = p.offset();
        p.o1 = p.o0;
        (e, p)
    }
    pub fn new_prot(v1: i32, v2: i32) -> Ent {
        let (mut e, p) = Self::prot_base(v1, v2);
        e.kind = Kind::Prot(p);
        e
    }
    pub fn new_n(v1: i32, v2: i32) -> Ent {
        let (mut e, p) = Self::prot_base(v1, v2);
        e.kind = Kind::ProtN { p, j: Box::new([Ent::new_y(v2), Ent::new_y(v2), Ent::new_y(v2)]), l: false, k: 0 };
        e
    }
    pub fn new_O(v1: i32, v2: i32) -> Ent {
        let (mut e, mut p) = Self::prot_base(v1, v2);
        e.b = v2;
        p.i = 0;
        p.c = v1;
        e.d = true;
        e.kind = Kind::ProtO { p, l: false, k: 0 };
        e
    }
    pub fn new_d(v: i32) -> Ent {
        base(v, Kind::D)
    }
    pub fn new_egg(v: i32) -> Ent {
        let mut e = base(v, Kind::Egg { i: 0, c: 0, j: 0 });
        e.a = 4;
        e.h = 4;
        e
    }
    pub fn new_web(v: i32) -> Ent {
        let mut e = base(v, Kind::Web { c: 0, i: false });
        e.a = 40;
        e.h = 40;
        e
    }

    /// Protector-family fields (`((a)x).a`, `.h`, `.i`); None for other kinds.
    pub fn prot(&self) -> Option<&Prot> {
        match &self.kind {
            Kind::Prot(p) | Kind::ProtN { p, .. } | Kind::ProtO { p, .. } => Some(p),
            _ => None,
        }
    }
    /// `B.a()` for B, q and f: returns the new sideways counter.
    fn missile_beat(&mut self, side: i32, reset_speed: i32) -> i32 {
        let mut side = side;
        self.vx = 0;
        self.vy = 0;
        if self.e < 0 {
            self.d = false;
            self.b = reset_speed;
        } else {
            self.b += 1;
            self.vy = if self.f < 5 { self.b } else { -self.b };
            self.f += 1;
            if side != 0 {
                if side < 0 {
                    side += 1;
                    self.vx = -(self.b / 2);
                } else {
                    side -= 1;
                    self.vx = self.b / 2;
                }
            }
        }
        side
    }

    /// Discrete part of the original `a()`: timers, animation frames,
    /// acceleration, and this beat's velocity.
    pub fn beat(&mut self, lh: i32, st: &St) {
        self.fresh = false;
        self.vx = 0;
        self.vy = 0;
        if lh == 3 {
            return;
        }
        match &mut self.kind {
            Kind::A => {}
            Kind::B { j } | Kind::Q { bj: j, .. } => {
                let side = *j;
                let ns = self.missile_beat(side, st.B_c);
                if let Kind::B { j } | Kind::Q { bj: j, .. } = &mut self.kind {
                    *j = ns;
                }
            }
            Kind::Fm { i, fj, .. } => {
                if !*i {
                    let side = *fj;
                    // f.c is a never-assigned static => 0
                    let ns = self.missile_beat(side, 0);
                    if let Kind::Fm { fj, .. } = &mut self.kind {
                        *fj = ns;
                    }
                } else {
                    let mut done = false;
                    if let Kind::Fm { k, i, .. } = &mut self.kind {
                        for b in k.iter_mut() {
                            b.beat(lh, st);
                        }
                        if !k[0].d && !k[1].d {
                            *i = false;
                            done = true;
                        }
                    }
                    if done {
                        self.d = false;
                    }
                }
            }
            Kind::G | Kind::Y => {
                if self.e < 0 {
                    self.d = false;
                } else {
                    self.vy = -self.b;
                }
            }
            Kind::O | Kind::D => {
                if self.e < 0 {
                    self.d = false;
                } else {
                    self.f += 1;
                    self.vy = -self.b;
                }
            }
            Kind::Burn { .. } => {
                if self.f < 3 {
                    self.f += 1;
                } else {
                    self.f = 0;
                    self.d = false;
                }
            }
            Kind::Prot(p) => p.advance(),
            Kind::ProtN { p, j, l, k } => {
                if *l {
                    let (g, e) = (self.g, self.e);
                    let mut idle = 0;
                    for y in j.iter_mut() {
                        if y.d {
                            y.beat(lh, st);
                        } else {
                            idle += 1;
                        }
                    }
                    if idle == 3 && *k > 10 {
                        *l = false;
                    }
                    if *k == 5 {
                        j[1].fire(g, e - 29, 0);
                    } else if *k == 10 {
                        j[2].fire(g, e - 29, 0);
                    }
                    *k += 1;
                }
                p.advance();
            }
            Kind::ProtO { p, l, k, .. } => {
                if *l {
                    if *k < 40 {
                        *k += 1;
                    } else {
                        *k = 0;
                        *l = false;
                    }
                }
                p.advance();
            }
            Kind::Egg { i, c, .. } => {
                if self.e <= 128 && self.e >= 0 {
                    self.vy = *c * self.b;
                    self.vx = *i;
                } else {
                    self.d = false;
                }
            }
            Kind::Web { c, i } => {
                if *i {
                    // caught-explosion countdown (was done while drawing)
                    if *c > 0 {
                        *c -= 1;
                    } else {
                        self.f = 0;
                        *c = 0;
                        *i = false;
                    }
                } else if *c == 0 {
                    *c = -1;
                } else if *c == -1 {
                    *c = 1;
                }
                if self.f == 0 {
                    self.vy = self.b;
                } else if self.f < 20 {
                    self.f += 1;
                } else {
                    self.f = 0;
                    if let Kind::Web { c, .. } = &mut self.kind {
                        *c = 0;
                    }
                    self.d = false;
                }
                if self.e > 128 {
                    self.d = false;
                }
            }
        }
    }

    /// Sub-step `k` of `n` within the current beat: move by that slice of
    /// the beat's velocity. Fresh launches get their first beat here.
    pub fn sub(&mut self, k: i32, n: i32, lh: i32, st: &St) {
        if self.fresh {
            self.beat(lh, st);
        }
        if lh == 3 {
            return;
        }
        self.g += part(self.vx, k, n);
        self.e += part(self.vy, k, n);
        if let Kind::ProtN { j, .. } = &mut self.kind {
            for y in j.iter_mut() {
                if y.d || y.fresh {
                    y.sub(k, n, lh, st);
                }
            }
        }
    }

    /// Whole original frame at once (code still running at 150 ms beats).
    pub fn update(&mut self, lh: i32, st: &St) {
        self.beat(lh, st);
        if lh != 3 {
            self.g += self.vx;
            self.e += self.vy;
        }
        if let Kind::ProtN { j, .. } = &mut self.kind {
            for y in j.iter_mut() {
                y.g += y.vx;
                y.e += y.vy;
            }
        }
    }

    /// `ProtO.j`: left end of the shield bar (computed when drawing originally)
    pub fn shield_x(&self) -> i32 {
        self.g + 7 - 15
    }

    /// `a(Graphics)` without side effects
    pub fn draw(&self, g: &mut Graphics, img: &Images) -> Res {
        match &self.kind {
            Kind::A => {}
            Kind::B { .. } | Kind::Q { .. } => {
                g.set_clip(self.g, self.e, 4, 9);
                g.draw_image(&img.B_l, self.g - self.f % 2 * 4, self.e, TL)?;
                g.reset_clip();
            }
            Kind::Fm { i, k, .. } => {
                if !*i {
                    g.set_clip(self.g, self.e, 4, 9);
                    g.draw_image(&img.B_l, self.g - self.f % 2 * 4, self.e, TL)?;
                    g.reset_clip();
                } else {
                    for b in k.iter() {
                        if b.d {
                            b.draw(g, img)?;
                        }
                    }
                }
            }
            Kind::G => g.draw_image(&img.g_c, self.g + 2, self.e, TL)?,
            Kind::Y => g.draw_image(&img.y_c, self.g, self.e, TL)?,
            Kind::O => {
                g.set_clip(self.g, self.e, 8, 15);
                g.draw_image(&img.o_i, self.g - self.f % 2 * 8, self.e, TL)?;
                g.reset_clip();
            }
            Kind::Burn { c } => {
                g.set_clip(self.g, self.e, 17, 24);
                g.draw_image(&img.b_i, self.g - 17 * *c, self.e, TL)?;
                g.reset_clip();
            }
            Kind::Prot(_) => g.draw_image(&img.a_n, self.g, self.e, TL)?,
            Kind::ProtN { j, l, .. } => {
                if *l {
                    for y in j.iter() {
                        if y.d {
                            y.draw(g, img)?;
                        }
                    }
                    g.set_clip(self.g - 2, self.e, 3, 5);
                    g.draw_image(&img.N_n, self.g - 2, self.e, TL)?;
                    g.set_clip(self.g + 5, self.e, 3, 5);
                    g.draw_image(&img.N_n, self.g + 5 - 3, self.e, TL)?;
                    g.reset_clip();
                } else {
                    g.draw_image(&img.a_n, self.g, self.e, TL)?;
                }
            }
            Kind::ProtO { l, .. } => {
                if *l {
                    let j = self.shield_x();
                    g.set_color(214, 159, 0);
                    g.fill_rect(j, self.e + 1, 34, 3);
                    g.set_color(255, 255, 0);
                    g.fill_rect(j, self.e + 2, 34, 1);
                    g.set_clip(j - 3, self.e, 3, 5);
                    g.draw_image(&img.O_n, j - 3, self.e, TL)?;
                    g.set_clip(j + 34, self.e, 3, 5);
                    g.draw_image(&img.O_n, j + 34, self.e, TL)?;
                    g.reset_clip();
                } else {
                    g.draw_image(&img.a_n, self.g, self.e, TL)?;
                }
            }
            Kind::D => {
                g.set_clip(self.g - 12, self.e, 24, 24);
                g.draw_image(&img.d_i, self.g - 12 - self.f % 2 * 24, self.e, TL)?;
                g.reset_clip();
            }
            Kind::Egg { .. } => g.draw_image(&img.e_k, self.g, self.e, TL)?,
            Kind::Web { c, i } => {
                if !*i {
                    if self.f == 0 {
                        if *c <= 0 {
                            g.set_clip(self.g + 12, self.e, 23, 47);
                            g.draw_image(&img.t_j[1], self.g + 12, self.e, TL)?;
                        } else {
                            g.set_clip(self.g, self.e, 41, 41);
                            g.draw_image(&img.t_j[0], self.g, self.e, TL)?;
                        }
                    } else {
                        g.set_clip(self.g, self.e, 41, 41);
                        g.draw_image(&img.t_j[0], self.g - 41, self.e, TL)?;
                    }
                    g.reset_clip();
                } else {
                    g.draw_image(&img.t_j[2], self.g, self.e, TL)?;
                }
            }
        }
        Ok(())
    }

    /// `b()`: load images
    pub fn load(&mut self, img: &mut Images) {
        match &mut self.kind {
            Kind::A => {}
            Kind::B { .. } | Kind::Q { .. } => {
                if img.B_l.is_none() {
                    img.B_l = load("/missie.png");
                }
            }
            Kind::Fm { k, .. } => {
                if img.B_l.is_none() {
                    img.B_l = load("/missie.png");
                }
                k[0].load(img);
            }
            Kind::G => {
                if img.g_c.is_none() {
                    img.g_c = load("/basic.png");
                }
            }
            Kind::Y => {
                if img.y_c.is_none() {
                    img.y_c = load("/laser.png");
                }
            }
            Kind::O => {
                if img.o_i.is_none() {
                    img.o_i = load("/flame.png");
                }
            }
            Kind::Burn { .. } => img.b_i = load("/burn.png"),
            Kind::Prot(_) => img.a_n = load("/protector.png"),
            Kind::ProtN { j, .. } => {
                img.a_n = load("/protector.png");
                j[0].load(img);
                img.N_n = load("/protector02.png");
            }
            Kind::ProtO { .. } => {
                img.a_n = load("/protector.png");
                img.O_n = load("/protector02.png");
            }
            Kind::D => img.d_i = load("/flameearth.png"),
            Kind::Egg { .. } => {
                if img.e_k.is_none() {
                    img.e_k = load("/spiderEgg.png");
                }
            }
            Kind::Web { .. } => {
                if img.t_j[0].is_none() {
                    img.t_j[0] = load("/spiderCatch.png");
                }
                if img.t_j[1].is_none() {
                    img.t_j[1] = load("/enemyweapon01a.png");
                }
                if img.t_j[2].is_none() {
                    img.t_j[2] = load("/explore01.png");
                }
            }
        }
    }

    /// `a(int, int, int)`: launch
    pub fn fire(&mut self, v1: i32, v2: i32, v3: i32) {
        let moves = !matches!(self.kind, Kind::A | Kind::Prot(_) | Kind::ProtO { .. });
        match &mut self.kind {
            Kind::A | Kind::Prot(_) => {}
            Kind::B { j } | Kind::Q { bj: j, .. } => {
                self.g = v1;
                self.e = v2;
                self.d = true;
                self.f = 0;
                *j = v3;
            }
            Kind::Fm { bj, i, .. } => {
                self.g = v1;
                self.e = v2;
                self.d = true;
                self.f = 0;
                *bj = v3;
                *i = false;
            }
            Kind::G | Kind::Y => {
                self.g = v1;
                self.e = v2;
                self.d = true;
            }
            Kind::O | Kind::D | Kind::Burn { .. } => {
                self.g = v1;
                self.e = v2;
                self.d = true;
                self.f = 0;
            }
            Kind::ProtN { .. } => {
                self.g = v1;
                self.e = v2 - 5;
                let (g, e) = (self.g, self.e);
                if let Kind::ProtN { j, l, k, .. } = &mut self.kind {
                    *l = true;
                    j[0].fire(g, e - 29, v3);
                    *k = 0;
                }
            }
            Kind::ProtO { l, .. } => {
                self.g = v1;
                self.e = v2 - 10;
                *l = true;
                self.f = 10;
            }
            Kind::Egg { c, .. } => {
                self.g = v1;
                self.e = v2;
                self.d = true;
                *c = if v3 == 0 { 1 } else { -1 };
            }
            Kind::Web { c, .. } => {
                self.g = v1;
                self.e = v2;
                self.f = 0;
                *c = 0;
                self.d = true;
            }
        }
        self.vx = 0;
        self.vy = 0;
        self.fresh = moves && !matches!(self.kind, Kind::ProtN { .. });
    }

    /// `a(int)`
    pub fn set_j(&mut self, v: i32) {
        if let Kind::Egg { j, .. } = &mut self.kind {
            *j = v;
        }
    }

    /// `a.a(int, int)` (and the N/O overrides): follow the pilot. At 60 fps
    /// the orbit offset is blended between this beat's and the next one's.
    pub fn follow(&mut self, x: i32, y: i32, k: i32, n: i32) {
        let locked = matches!(self.kind, Kind::ProtN { l: true, .. } | Kind::ProtO { l: true, .. });
        if !locked {
            let (o0, o1) = match self.prot() {
                Some(p) => (p.o0, p.o1),
                None => ([0; 2], [0; 2]),
            };
            self.g = x + o0[0] + (o1[0] - o0[0]) * k / n.max(1);
            self.e = y + o0[1] + (o1[1] - o0[1]) * k / n.max(1);
            self.d = true;
        }
    }
}

/// `F.a(Graphics, 6 ints)`: charge sparkle
pub fn sparkle(g: &mut Graphics, v2: i32, v3: i32, v4: i32, _v5: i32, v6: i32, v7: i32) {
    let v9 = v2 + 3 + v4 / 2;
    g.set_color(59, 255, 238);
    g.fill_rect(v9 + v6, v3 + v7, 2, 2);
    g.fill_rect(v9 - v7, v3 - v6, 2, 2);
    g.fill_rect(v9 - v6, v3 + v7, 2, 2);
    g.fill_rect(v9 + v7, v3 - v6, 2, 2);
    g.fill_rect(v9 + v7, v3 + v6, 2, 2);
    g.fill_rect(v9 - v6, v3 - v7, 2, 2);
    g.fill_rect(v9 - v7, v3 + v6, 2, 2);
    g.fill_rect(v9 + v6, v3 - v7, 2, 2);
}
