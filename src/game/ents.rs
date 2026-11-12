//! The abstract class `F` and its subclasses (projectiles, protectors,
//! enemy eggs and webs). Java field shadowing is preserved explicitly:
//! e.g. `q.j`/`f.j` are separate from `B.j`, and `a.a`/`a.h` shadow `F.a`/`F.h`.

use super::{load, Images, St};
use crate::gfx::{Graphics, Res, LEFT, TOP};

const TL: i32 = TOP | LEFT;

/// `a.m` orbit table
const PATH: [[i32; 2]; 13] = [
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
    ProtO { p: Prot, l: bool, k: i32, j: i32 },
    /// `d`: flame earth
    D,
    /// `e`: spider egg
    Egg { i: i32, c: i32, j: i32 },
    /// `t`: spider web
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
}

fn base(b: i32, kind: Kind) -> Ent {
    Ent { g: 0, e: 0, f: 0, b, a: 0, h: 0, d: false, kind }
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
        (e, Prot { i: 0, c: v1, a: 9, h: 9 })
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
        e.kind = Kind::ProtO { p, l: false, k: 0, j: 0 };
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
    pub fn prot_mut(&mut self) -> Option<&mut Prot> {
        match &mut self.kind {
            Kind::Prot(p) | Kind::ProtN { p, .. } | Kind::ProtO { p, .. } => Some(p),
            _ => None,
        }
    }

    /// `B.a()` movement shared by B, q and (with f's own fields) f
    fn missile_step(&mut self, side: i32, reset_speed: i32) -> i32 {
        let mut side = side;
        if self.e < 0 {
            self.d = false;
            self.b = reset_speed;
        } else {
            self.b += 1;
            if self.f < 5 {
                self.e += self.b;
            } else {
                self.e -= self.b;
            }
            self.f += 1;
            if side != 0 {
                if side < 0 {
                    side += 1;
                    self.g -= self.b / 2;
                } else {
                    side -= 1;
                    self.g += self.b / 2;
                }
            }
        }
        side
    }

    fn prot_step(&mut self) {
        let (i, c) = match self.prot() {
            Some(p) => (p.i, p.c),
            None => return,
        };
        let idx = ((i + c * 2) % 12) as usize;
        self.g += PATH[idx][0];
        self.e += PATH[idx][1];
        if let Some(p) = self.prot_mut() {
            if p.i < 11 {
                p.i += 1;
            } else {
                p.i = 0;
            }
        }
    }

    /// `a()`: per-frame update
    pub fn update(&mut self, lh: i32, st: &St) {
        match &mut self.kind {
            Kind::A => {}
            Kind::B { j } | Kind::Q { bj: j, .. } => {
                if lh != 3 {
                    let side = *j;
                    let ns = self.missile_step(side, st.B_c);
                    if let Kind::B { j } | Kind::Q { bj: j, .. } = &mut self.kind {
                        *j = ns;
                    }
                }
            }
            Kind::Fm { .. } => {
                if lh != 3 {
                    let (fi, fj) = match &self.kind {
                        Kind::Fm { i, fj, .. } => (*i, *fj),
                        _ => unreachable!(),
                    };
                    if !fi {
                        // f.c is a never-assigned static => 0
                        let ns = self.missile_step(fj, 0);
                        if let Kind::Fm { fj, .. } = &mut self.kind {
                            *fj = ns;
                        }
                    } else {
                        let mut done = false;
                        if let Kind::Fm { k, i, .. } = &mut self.kind {
                            for b in k.iter_mut() {
                                b.update(lh, st);
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
            }
            Kind::G | Kind::Y => {
                if lh != 3 {
                    if self.e < 0 {
                        self.d = false;
                    } else {
                        self.e -= self.b;
                    }
                }
            }
            Kind::O | Kind::D => {
                if lh != 3 {
                    if self.e < 0 {
                        self.d = false;
                    } else {
                        self.f += 1;
                        self.e -= self.b;
                    }
                }
            }
            Kind::Burn { .. } => {
                if lh != 3 {
                    if self.f < 3 {
                        self.f += 1;
                    } else {
                        self.f = 0;
                        self.d = false;
                    }
                }
            }
            Kind::Prot(_) => {
                if lh != 3 {
                    self.prot_step();
                }
            }
            Kind::ProtN { .. } => {
                if lh != 3 {
                    let l = matches!(self.kind, Kind::ProtN { l: true, .. });
                    if !l {
                        let (i, c) = self.prot().map(|p| (p.i, p.c)).unwrap();
                        let idx = ((i + c * 2) % 12) as usize;
                        self.g += PATH[idx][0];
                        self.e += PATH[idx][1];
                    } else {
                        let (g, e) = (self.g, self.e);
                        if let Kind::ProtN { j, l, k, .. } = &mut self.kind {
                            let mut idle = 0;
                            for y in j.iter_mut() {
                                if y.d {
                                    y.update(lh, st);
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
                    }
                }
                if lh != 3 {
                    if let Some(p) = self.prot_mut() {
                        if p.i < 11 {
                            p.i += 1;
                        } else {
                            p.i = 0;
                        }
                    }
                }
            }
            Kind::ProtO { .. } => {
                if lh != 3 {
                    let l = matches!(self.kind, Kind::ProtO { l: true, .. });
                    if !l {
                        self.prot_step();
                    } else if let Kind::ProtO { p, l, k, .. } = &mut self.kind {
                        if *k < 40 {
                            *k += 1;
                        } else {
                            *k = 0;
                            *l = false;
                        }
                        if p.i < 11 {
                            p.i += 1;
                        } else {
                            p.i = 0;
                        }
                    }
                }
            }
            Kind::Egg { i, c, .. } => {
                if lh != 3 {
                    if self.e <= 128 && self.e >= 0 {
                        self.e += *c * self.b;
                        self.g += *i;
                    } else {
                        self.d = false;
                    }
                }
            }
            Kind::Web { c, .. } => {
                if lh != 3 {
                    if self.f == 0 {
                        self.e += self.b;
                    } else if self.f < 20 {
                        self.f += 1;
                    } else {
                        self.f = 0;
                        *c = 0;
                        self.d = false;
                    }
                }
                if self.e > 128 {
                    self.d = false;
                }
            }
        }
    }

    /// `a(Graphics)`
    pub fn draw(&mut self, g: &mut Graphics, img: &Images) -> Res {
        match &mut self.kind {
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
                    for b in k.iter_mut() {
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
                    for y in j.iter_mut() {
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
            Kind::ProtO { l, j, .. } => {
                if *l {
                    *j = self.g + 7 - 15;
                    let j = *j;
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
                        if *c == 0 {
                            g.set_clip(self.g + 12, self.e, 23, 47);
                            g.draw_image(&img.t_j[1], self.g + 12, self.e, TL)?;
                            *c += 1;
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
                    if *c > 0 {
                        *c -= 1;
                    } else {
                        self.f = 0;
                        *c = 0;
                        *i = false;
                    }
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
    }

    /// `a(int)`
    pub fn set_j(&mut self, v: i32) {
        if let Kind::Egg { j, .. } = &mut self.kind {
            *j = v;
        }
    }

    /// `a.a(int, int)` (and the N/O overrides): follow the pilot
    pub fn follow(&mut self, x: i32, y: i32) {
        let locked = matches!(self.kind, Kind::ProtN { l: true, .. } | Kind::ProtO { l: true, .. });
        if !locked {
            self.g = x;
            self.e = y;
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
