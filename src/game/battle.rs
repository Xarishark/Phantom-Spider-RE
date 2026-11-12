//! `G` (battle manager) and `i` (the enemy spider pool).

use super::ents::{sparkle, Ent, Kind};
use super::sfx::SFx;
use super::spider::Z;
use super::{hit, rnd, Game, Images, K_a};
use crate::gfx::{Graphics, Jex, OptExt, Res, NPE};

const G_P: usize = 20; // max weapons
const G_A: i32 = 5; // basic shots
const G_I: usize = 15; // eggs
const G_L: usize = 2; // webs

const CCE: Jex = Jex("ClassCastException");

/// `i`
pub struct Spiders {
    pub e: [Option<Z>; 4],
}

pub struct Battle {
    pub d: Vec<Option<Ent>>,
    pub q: Vec<Option<Ent>>,
    pub c: Vec<Option<Ent>>,
    pub f: Vec<Option<Ent>>,
    pub n: SFx,
    pub h: Spiders,
}

impl Battle {
    /// `new G()`
    pub fn new(img: &mut Images) -> Battle {
        let n = SFx::new(img);
        Z::load_images(img); // this.h.a()
        Battle {
            d: (0..G_P).map(|_| None).collect(),
            q: (0..G_A as usize).map(|_| None).collect(),
            c: (0..G_I).map(|_| None).collect(),
            f: (0..G_L).map(|_| None).collect(),
            n,
            h: Spiders { e: [None, None, None, None] },
        }
    }
}

impl Game {
    // ------------------------------------------------------------ class i
    fn z(&self, i: usize) -> Res<&Z> {
        self.bat.h.e[i].as_ref().ok_or(NPE)
    }
    fn zm(&mut self, i: usize) -> Res<&mut Z> {
        self.bat.h.e[i].as_mut().ok_or(NPE)
    }
    fn i_free(&self) -> i32 {
        self.bat.h.e.iter().position(|z| z.is_none()).map(|p| p as i32).unwrap_or(-1)
    }
    fn i_has(&self, i: usize) -> bool {
        self.bat.h.e[i].is_some()
    }
    /// `i.b(slot, life, speed, attack, defence, size)`
    fn i_spawn(&mut self, v1: i32, v2: i32, v3: i32, v4: i32, v5: i32, v6: i32) -> Res {
        let slot = self.bat.h.e.get_mut(v1 as usize).ok_or(crate::gfx::AIOOBE)?;
        let mut z = Z::new(&mut self.img);
        z.set_stats(v2, v3, v4, v5);
        z.set_size(v6);
        *slot = Some(z);
        Ok(())
    }
    fn i_slot(&mut self, i: i32) -> Res<&mut Z> {
        if i < 0 {
            return Err(crate::gfx::AIOOBE);
        }
        self.zm(i as usize)
    }
    /// `i.d(int)`: split / remove a spider whose life ran low
    fn i_split(&mut self, v1: usize) -> Res {
        let (v2, v3, p, m) = {
            let z = self.z(v1)?;
            (z.l, z.k, z.p, z.m)
        };
        let (d, c, g, b) = (self.st.i_d, self.st.i_c, self.st.i_g, self.st.i_b);
        if p == 2 {
            if m <= d / 2 {
                self.bat.h.e[v1] = None;
                let v4 = self.i_free();
                if v4 != -1 {
                    self.i_spawn(v4, d / 2, c, g, b, 1)?;
                }
                let v5 = self.i_free();
                if v5 != -1 {
                    self.i_spawn(v5, d / 2, c, g, b, 1)?;
                }
                let z = self.i_slot(v4)?;
                z.l = v2;
                z.k = v3;
                let z = self.i_slot(v5)?;
                z.l = v2;
                z.k = v3;
            }
        } else if p == 1 {
            if m <= d / 4 {
                self.bat.h.e[v1] = None;
                let v6 = self.i_free();
                if v6 != -1 {
                    self.i_spawn(v6, d / 4, c, g, b, 0)?;
                }
                let v7 = self.i_free();
                if v7 != -1 {
                    self.i_spawn(v7, d / 4, c, g, b, 0)?;
                }
                let z = self.i_slot(v6)?;
                z.l = v2;
                z.k = v3;
                let z = self.i_slot(v7)?;
                z.l = v2;
                z.k = v3;
            }
        } else if m <= 0 {
            self.bat.h.e[v1] = None;
        }
        Ok(())
    }

    /// Spider struck: show its health bar, apply damage, add score.
    fn spider_hit(&mut self, g: &mut Graphics, v5: usize, atk: i32, hit_n: i32, score: i32) -> Res {
        self.z(v5)?.health_bar(g); // h.b(Graphics, v5)
        let z = self.zm(v5)?;
        z.m = K_a(atk, z.a, z.m, hit_n);
        self.st.j_d += score;
        Ok(())
    }

    // ------------------------------------------------------------ class G
    /// `G.c()`
    pub fn G_reset_fx(&mut self) {
        self.bat.n.reset();
    }

    /// `G.b(Graphics)`: draw spiders
    pub fn G_draw_spiders(&mut self, g: &mut Graphics) -> Res {
        let lh = self.st.L_H;
        for v2 in 0..4 {
            if let Some(z) = self.bat.h.e[v2].as_mut() {
                z.draw(g, &self.img, lh)?;
            }
        }
        Ok(())
    }

    /// `G.e()`: save spiders into `x`
    pub fn G_save_spiders(&mut self) {
        let mut v1 = 0;
        for v2 in 0..4 {
            if let Some(z) = &self.bat.h.e[v2] {
                let s = &mut self.st;
                s.x_h[v1] = z.l;
                s.x_g[v1] = z.k;
                s.x_i[v1] = z.m;
                s.x_a[v1] = z.a;
                s.x_e[v1] = z.h;
                s.x_c[v1] = z.e;
                s.x_k[v1] = z.p;
                s.x_d[v1] = z.c;
                s.x_j[v1] = z.i;
                v1 += 1;
            }
        }
        self.st.x_f = v1 as i32;
    }

    /// `G.d()`: save eggs and webs into `p`
    pub fn G_save_eggs(&mut self) {
        for v1 in 0..G_I {
            if let Some(c) = &self.bat.c[v1] {
                self.st.p_e[v1] = c.g;
                self.st.p_d[v1] = c.e;
                self.st.p_f[v1] = match c.kind {
                    Kind::Egg { c, .. } => c,
                    _ => 0,
                };
                self.st.p_a[v1] = c.d as i32;
            }
        }
        for v2 in 0..G_L {
            if let Some(f) = &self.bat.f[v2] {
                self.st.p_c[v2] = f.g;
                self.st.p_b[v2] = f.e;
                self.st.p_g[v2] = f.d as i32;
                self.st.p_j[v2] = f.f;
                if let Kind::Web { c, i } = f.kind {
                    self.st.p_h[v2] = i as i32;
                    self.st.p_i[v2] = c;
                }
            }
        }
    }

    /// `G.b()`: save basic shots into `c`
    pub fn G_save_basic(&mut self) {
        for v1 in 0..self.st.G_r as usize {
            if let Some(q) = &self.bat.q[v1] {
                self.st.c_a[v1] = q.g;
                self.st.c_c[v1] = q.e;
                self.st.c_b[v1] = q.d as i32;
            }
        }
    }

    /// `G.l()`: save weapons into `C`
    pub fn G_save_weapons(&mut self) {
        for v1 in 0..self.st.G_e as usize {
            let Some(d) = &self.bat.d[v1] else { continue };
            let s = &mut self.st;
            s.C_p[v1] = d.g;
            s.C_n[v1] = d.e;
            s.C_o[v1] = d.f;
            s.C_g[v1] = d.d as i32;
            match &d.kind {
                Kind::Q { qj, .. } => s.C_l[v1] = *qj,
                Kind::ProtO { p, l, k, .. } => {
                    s.C_d[v1] = p.i;
                    s.C_k[v1] = *l as i32;
                    s.C_f[v1] = *k;
                }
                Kind::ProtN { p, j, l, k } => {
                    s.C_k[v1] = *l as i32;
                    s.C_d[v1] = p.i;
                    s.C_f[v1] = *k;
                    for v3 in 0..3 {
                        s.C_c[v3][v1] = j[v3].g;
                        s.C_a[v3][v1] = j[v3].e;
                        s.C_i[v3][v1] = j[v3].d as i32;
                    }
                }
                Kind::Fm { fj, i, k, .. } => {
                    s.C_l[v1] = *fj;
                    s.C_k[v1] = *i as i32;
                    for v2 in 0..2 {
                        s.C_j[v2][v1] = k[v2].g;
                        s.C_h[v2][v1] = k[v2].e;
                        s.C_m[v2][v1] = k[v2].d as i32;
                        s.C_b[v2][v1] = k[v2].f;
                    }
                }
                Kind::Prot(p) => s.C_d[v1] = p.i,
                Kind::B { j } => s.C_l[v1] = *j,
                _ => {}
            }
        }
    }

    /// `G.g()`: restore weapons from `C`
    pub fn G_load_weapons(&mut self) -> Res {
        for v1 in 0..self.st.G_e as usize {
            let s = &self.st;
            let d = self.bat.d[v1].npe_mut()?;
            d.g = s.C_p[v1];
            d.e = s.C_n[v1];
            d.f = s.C_o[v1];
            d.d = s.C_g[v1] == 1;
            match &mut d.kind {
                Kind::Q { qj, .. } => *qj = s.C_l[v1],
                Kind::ProtO { p, l, k, .. } => {
                    p.i = s.C_d[v1];
                    *l = s.C_k[v1] == 1;
                    *k = s.C_f[v1];
                }
                Kind::ProtN { p, j, l, k } => {
                    *l = s.C_k[v1] == 1;
                    p.i = s.C_d[v1];
                    *k = s.C_f[v1];
                    for v3 in 0..3 {
                        j[v3].g = s.C_c[v3][v1];
                        j[v3].e = s.C_a[v3][v1];
                        j[v3].d = s.C_i[v3][v1] == 1;
                    }
                }
                Kind::Fm { fj, i, k, .. } => {
                    *fj = s.C_l[v1];
                    *i = s.C_k[v1] == 1;
                    for v2 in 0..2 {
                        k[v2].g = s.C_j[v2][v1];
                        k[v2].e = s.C_h[v2][v1];
                        k[v2].d = s.C_m[v2][v1] == 1;
                        k[v2].f = s.C_b[v2][v1];
                    }
                }
                Kind::Prot(p) => p.i = s.C_d[v1],
                Kind::B { j } => *j = s.C_l[v1],
                _ => {}
            }
        }
        Ok(())
    }

    /// `G.f()`: restore basic shots
    pub fn G_load_basic(&mut self) -> Res {
        for v1 in 0..self.st.G_r as usize {
            let q = self.bat.q[v1].npe_mut()?;
            q.g = self.st.c_a[v1];
            q.e = self.st.c_c[v1];
            q.d = self.st.c_b[v1] == 1;
        }
        Ok(())
    }

    /// `G.j()`: restore eggs and webs
    pub fn G_load_eggs(&mut self) -> Res {
        for v1 in 0..G_I {
            let c = self.bat.c[v1].npe_mut()?;
            c.g = self.st.p_e[v1];
            c.e = self.st.p_d[v1];
            match &mut c.kind {
                Kind::Egg { c: dir, .. } => *dir = self.st.p_f[v1],
                _ => return Err(CCE),
            }
            c.d = self.st.p_a[v1] == 1;
        }
        for v2 in 0..G_L {
            let f = self.bat.f[v2].npe_mut()?;
            f.g = self.st.p_c[v2];
            f.e = self.st.p_b[v2];
            f.d = self.st.p_g[v2] == 1;
            f.f = self.st.p_j[v2];
            match &mut f.kind {
                Kind::Web { c, i } => {
                    *c = self.st.p_i[v2];
                    *i = self.st.p_h[v2] == 1;
                }
                _ => return Err(CCE),
            }
        }
        Ok(())
    }

    /// `G.b(x, y, w, h)`: does a spider touch the pilot?
    pub fn G_spider_touch(&mut self, v1: i32, v2: i32, v3: i32, v4: i32) -> bool {
        for v6 in 0..4 {
            if let Some(z) = &self.bat.h.e[v6] {
                if hit(z.l, z.k, z.b, z.o, v1, v2, v3, v4) {
                    self.bat.n.add(&mut self.st, 0, v1, v2, 3);
                    return true;
                }
            }
        }
        false
    }

    /// `G.k()`: spawn the big spider
    pub fn G_init_spiders(&mut self) -> Res {
        self.bat.h.e = [None, None, None, None];
        let (d, c, g, b) = (self.st.i_d, self.st.i_c, self.st.i_g, self.st.i_b);
        self.i_spawn(0, d, c, g, b, 2)
    }

    /// `G.a(9 ints)`: restore one spider
    #[allow(clippy::too_many_arguments)]
    pub fn G_restore_spider(&mut self, v1: i32, v2: i32, v3: i32, v4: i32, v5: i32, v6: i32, v7: i32, v8: i32, v9: i32) -> Res {
        self.i_spawn(v1, v2, v3, v4, v5, v6)?;
        let z = self.i_slot(v1)?;
        z.l = v8;
        z.k = v9;
        z.c = v7;
        Ok(())
    }

    /// `G.c(Graphics, x, y)`: spider movement & AI
    pub fn G_spider_ai(&mut self, v2: i32, v3: i32) -> Res {
        let (lh, ig) = (self.st.L_H, self.st.i_g);
        for v4 in 0..4 {
            if self.i_has(v4) {
                let z = self.zm(v4)?;
                z.step(lh);
                let v5 = z.think(v2, v3, lh, ig);
                if v5 != 0 {
                    let (l, k, b) = (z.l, z.k, z.b);
                    self.G_spawn_enemy_shot(v5 - 1, l, k, b)?;
                }
            }
        }
        Ok(())
    }

    /// `G.a(int)`: create eggs and webs
    pub fn G_init_eggs(&mut self) {
        let v2 = self.st.i_c;
        for v3 in 0..G_I {
            self.bat.c[v3] = Some(Ent::new_egg(v2));
        }
        if let Some(c) = self.bat.c[0].as_mut() {
            c.load(&mut self.img);
        }
        for v4 in 0..G_L {
            self.bat.f[v4] = Some(Ent::new_web(v2));
        }
        if let Some(f) = self.bat.f[0].as_mut() {
            f.load(&mut self.img);
        }
    }

    /// `G.a(type, x, y, w)`: a spider fires an egg (0/2) or a web (1)
    fn G_spawn_enemy_shot(&mut self, v1: i32, v2: i32, v3: i32, v4: i32) -> Res {
        match v1 {
            0 | 2 => {
                let gi = self.st.G_g as usize;
                let c = self.bat.c[gi].npe_mut()?;
                if !c.d {
                    if rnd(2) == 0 {
                        c.fire(v2 + v4 - 10, v3 + 30, v1);
                    } else {
                        c.fire(v2 + 10, v3 + 30, v1);
                    }
                    self.st.G_g += 1;
                    if self.st.G_g == G_I as i32 {
                        self.st.G_g = 0;
                    }
                }
            }
            1 => {
                let mi = self.st.G_m as usize;
                let f = self.bat.f[mi].npe_mut()?;
                let caught = matches!(f.kind, Kind::Web { i: true, .. });
                if !f.d && !caught {
                    let r = rnd(6) - 3;
                    f.fire(v2 + 10, v3 + 30, r);
                    self.st.G_m += 1;
                    if self.st.G_m == G_L as i32 {
                        self.st.G_m = 0;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// `G.d(Graphics, x, y)`: webs; true if the pilot is caught
    pub fn G_webs(&mut self, g: &mut Graphics, v2: i32, v3: i32) -> Res<bool> {
        let mut v4 = false;
        let lh = self.st.L_H;
        let s = self.st.G_s;
        for v5 in 0..G_L {
            let f = self.bat.f[v5].npe_mut()?;
            if f.d {
                f.update(lh, &self.st);
                f.draw(g, &self.img)?;
                if !v4 {
                    v4 = hit(f.g + 10, f.e + 10, f.a - 20, f.h - 20, v2 + 5, v3 + 5, 15, 15);
                    if v4 && f.f == 0 {
                        f.f = 1;
                    }
                }
                if s == 2 {
                    for v6 in 0..self.st.G_e as usize {
                        let (dg, de) = {
                            let d = self.bat.d[v6].npe()?;
                            (d.g, d.e)
                        };
                        let f = self.bat.f[v5].npe_mut()?;
                        if hit(f.g + 10, f.e + 10, f.a - 20, f.h - 20, dg, de, 4, 19) {
                            let (fg, fe) = (f.g, f.e);
                            f.d = false;
                            if let Kind::Web { c, .. } = &mut f.kind {
                                *c = 0;
                            }
                            f.f = 0;
                            self.bat.n.add(&mut self.st, 2, fg, fe, 3);
                        }
                    }
                }
            }
        }
        Ok(v4)
    }

    /// `G.i()`
    pub fn G_enemy_attack(&self) -> i32 {
        self.st.i_g
    }

    /// `G.b(Graphics, x, y)`: eggs; true if the pilot was hit
    pub fn G_eggs(&mut self, g: &mut Graphics, v2: i32, v3: i32) -> Res<bool> {
        let mut v4 = false;
        let lh = self.st.L_H;
        let s = self.st.G_s;
        for v5 in 0..G_I {
            let active = self.bat.c[v5].npe()?.d;
            if active && !v4 {
                {
                    let c = self.bat.c[v5].npe_mut()?;
                    v4 = hit(c.g, c.e, c.a, c.h, v2, v3, 15, 15);
                    if v4 {
                        c.d = false;
                    }
                }
                if v4 {
                    self.bat.n.add(&mut self.st, 0, v2, v3, 3);
                }
                let c = self.bat.c[v5].npe_mut()?;
                c.set_j(rnd(20));
                c.update(lh, &self.st);
                c.draw(g, &self.img)?;
                let (cg, ce, ca, ch) = (c.g, c.e, c.a, c.h);
                let mut kill = false;
                if s == 4 {
                    for v6 in 0..self.st.G_e as usize {
                        let d = self.bat.d[v6].npe()?;
                        let p = d.prot().ok_or(CCE)?;
                        if hit(d.g, d.e, p.a, p.h, cg, ce, ca, ch) {
                            kill = true;
                        }
                    }
                }
                if s == 9 || s == 5 {
                    for v7 in 0..self.st.G_e as usize {
                        let d = self.bat.d[v7].npe()?;
                        if s == 9 {
                            let Kind::ProtO { p, l, j, .. } = &d.kind else { return Err(CCE) };
                            if !*l {
                                if hit(d.g, d.e, p.a, p.h, cg, ce, ca, ch) {
                                    kill = true;
                                }
                            } else if hit(*j, d.e - 10, 37, 5, cg, ce, ca, ch) {
                                kill = true;
                            }
                        } else {
                            let Kind::ProtN { p, l, .. } = &d.kind else { return Err(CCE) };
                            if !*l && hit(d.g, d.e, p.a, p.a, cg, ce, ca, ch) {
                                kill = true;
                            }
                        }
                    }
                }
                if kill {
                    self.bat.c[v5].npe_mut()?.d = false;
                }
            }
        }
        Ok(v4)
    }

    /// `G.c(int, int)`: protectors follow the pilot
    pub fn G_follow(&mut self, v1: i32, v2: i32) -> bool {
        let s = self.st.G_s;
        if s != 4 && s != 9 && s != 5 {
            false
        } else {
            for v3 in 0..self.st.G_e as usize {
                if let Some(d) = self.bat.d[v3].as_mut() {
                    d.follow(v1, v2);
                }
            }
            true
        }
    }

    /// `G.b(Graphics, attack)`: update weapons and resolve hits on spiders
    pub fn G_weapons(&mut self, g: &mut Graphics, v2: i32) -> Res {
        let lh = self.st.L_H;
        let s = self.st.G_s;
        for v3 in 0..self.st.G_e as usize {
            if !self.bat.d[v3].npe()?.d {
                continue;
            }
            {
                let d = self.bat.d[v3].npe_mut()?;
                let _ = rnd(20); // d.a(m.a(20)) is a no-op for weapons
                d.update(lh, &self.st);
                d.draw(g, &self.img)?;
            }
            let mut v4 = true;
            for v5 in 0..4 {
                if !self.i_has(v5) {
                    continue;
                }
                v4 = false;
                let (zl, zk, zb, zo) = {
                    let z = self.z(v5)?;
                    (z.l, z.k, z.b, z.o)
                };
                if s != 7 && s != 5 && s != 8 {
                    let (dg, de) = {
                        let d = self.bat.d[v3].npe()?;
                        (d.g, d.e)
                    };
                    if hit(zl, zk, zb, zo, dg, de, 4, 19) {
                        if s != 0 && s != 4 && s != 9 {
                            self.bat.d[v3].npe_mut()?.d = false;
                        }
                        let t = if s != 6 { 0 } else { 1 };
                        self.bat.n.add(&mut self.st, t, dg, de, 3);
                        self.spider_hit(g, v5, v2, 3, 20)?;
                    }
                } else if s == 8 {
                    let (dg, de) = {
                        let d = self.bat.d[v3].npe()?;
                        (d.g, d.e)
                    };
                    if hit(zl, zk, zb, zo, dg - 12, de, 24, 24) {
                        self.bat.n.add(&mut self.st, 0, dg, de, 3);
                        self.bat.d[v3].npe_mut()?.d = false;
                        self.spider_hit(g, v5, v2, 3, 20)?;
                    }
                } else if s == 5 {
                    for v6 in 0..3 {
                        let hitpos = {
                            let d = self.bat.d[v3].npe()?;
                            let Kind::ProtN { j, l, .. } = &d.kind else { return Err(CCE) };
                            if *l && j[v6].d && hit(zl, zk, zb, zo, j[v6].g, j[v6].e, 4, 19) {
                                Some((j[v6].g, j[v6].e))
                            } else {
                                None
                            }
                        };
                        if let Some((jg, je)) = hitpos {
                            self.bat.n.add(&mut self.st, 0, jg, je, 3);
                            if let Kind::ProtN { j, .. } = &mut self.bat.d[v3].npe_mut()?.kind {
                                j[v6].d = false;
                            }
                            self.spider_hit(g, v5, v2, 3, 20)?;
                        }
                    }
                } else if s == 7 {
                    let (dg, de, burning, k0, k1) = {
                        let d = self.bat.d[v3].npe()?;
                        let Kind::Fm { i, k, .. } = &d.kind else { return Err(CCE) };
                        (d.g, d.e, *i, (k[0].d, k[0].g, k[0].e), (k[1].d, k[1].g, k[1].e))
                    };
                    if burning {
                        if k0.0 && hit(zl, zk, zb, zo, k0.1, k0.2, 4, 19) {
                            self.spider_hit(g, v5, v2, 3, 20)?;
                        }
                        if k1.0 && hit(zl, zk, zb, zo, k1.1, k1.2, 4, 19) {
                            self.spider_hit(g, v5, v2, 3, 20)?;
                        }
                    } else if hit(zl, zk, zb, zo, dg, de, 4, 19) {
                        if let Kind::Fm { i, k, .. } = &mut self.bat.d[v3].npe_mut()?.kind {
                            *i = true;
                            k[0].fire(dg, de, 0);
                            k[1].fire(dg + 12, de, 0);
                        }
                        self.bat.n.add(&mut self.st, 0, dg, de, 3);
                        self.spider_hit(g, v5, v2, 3, 20)?;
                    }
                }
                self.i_split(v5)?;
            }
            g.reset_clip();
            if v4 {
                self.mission_complete();
            }
        }
        Ok(())
    }

    fn mission_complete(&mut self) {
        self.st.L_v = rnd(5);
        self.st.L_a = 128;
        self.st.L_M = 0;
        self.st.L_D = true;
        self.st.L_H = 4;
    }

    /// `G.a(Graphics)`: explosions
    pub fn G_draw_fx(&mut self, g: &mut Graphics) -> Res {
        self.bat.n.draw(g, &self.img, &self.st)
    }

    /// `G.a(Graphics, attack)`: basic shots
    pub fn G_basic(&mut self, g: &mut Graphics, v2: i32) -> Res {
        let lh = self.st.L_H;
        for v4 in 0..self.st.G_r as usize {
            if !self.bat.q[v4].npe()?.d {
                continue;
            }
            {
                let q = self.bat.q[v4].npe_mut()?;
                q.update(lh, &self.st);
                q.draw(g, &self.img)?;
            }
            let mut v5 = true;
            for v6 in 0..4 {
                if !self.i_has(v6) {
                    continue;
                }
                v5 = false;
                let (qg, qe) = {
                    let q = self.bat.q[v4].npe()?;
                    (q.g, q.e)
                };
                let (zl, zk, zb, zo) = {
                    let z = self.z(v6)?;
                    (z.l, z.k, z.b, z.o)
                };
                if hit(zl, zk, zb, zo, qg + 2, qe, 8, 19) {
                    self.bat.q[v4].npe_mut()?.d = false;
                    self.z(v6)?.health_bar(g);
                    let z = self.zm(v6)?;
                    z.m = K_a(v2, z.a, z.m, 2);
                    self.bat.n.add(&mut self.st, 10, qg, qe, 3);
                    self.st.j_d += 5;
                }
                self.i_split(v6)?;
            }
            for v7 in 0..G_I {
                let (qg, qe) = {
                    let q = self.bat.q[v4].npe()?;
                    (q.g, q.e)
                };
                let c = self.bat.c[v7].npe_mut()?;
                if c.d && hit(c.g, c.e, c.a, c.h, qg + 2, qe, 8, 19) {
                    c.d = false;
                    self.bat.q[v4].npe_mut()?.d = false;
                }
            }
            if v5 {
                self.mission_complete();
            }
        }
        Ok(())
    }

    /// `G.a(Graphics, x, y)`: charge sparkle
    pub fn G_charge_fx(&mut self, g: &mut Graphics, v2: i32, v3: i32) -> Res {
        let s = self.st.G_s;
        let j = self.st.G_j;
        if ((s == 0 || s == 9) && j > 15) || ((s == 5 || s == 8) && j > 10) {
            self.bat.d[0].npe()?;
            let (a, b) = (rnd(5), rnd(5));
            sparkle(g, v2, v3, 15, 15, a, b);
        }
        Ok(())
    }

    /// `G.a(int, int)`: fire a basic shot
    pub fn G_fire_basic(&mut self, v1: i32, v2: i32) -> Res {
        let o = self.st.G_o as usize;
        let q = self.bat.q[o].npe_mut()?;
        if !q.d && self.st.l_d == 0 {
            q.fire(v1 + 6, v2, 9);
            self.st.G_o += 1;
            if self.st.G_o == self.st.G_r {
                self.st.G_o = 0;
            }
            self.st.l_d = 3;
        }
        Ok(())
    }

    /// `G.e(int, int)`: release a charged weapon
    pub fn G_release(&mut self, v1: i32, v2: i32) -> Res {
        let s = self.st.G_s;
        let e = self.st.G_e;
        if s == 0 {
            if self.st.G_j > 15 {
                for v3 in 0..e {
                    let b = self.st.G_b as usize;
                    let d = self.bat.d[b].npe_mut()?;
                    if !d.d {
                        d.fire(v1 + 6 + v3 * 10 - v3 * 5, v2, 9);
                        self.st.G_b += 1;
                        if self.st.G_b == e {
                            self.st.G_b = 0;
                        }
                    }
                }
                self.st.G_j = 0;
            }
        } else if s == 9 || s == 5 {
            let need = if s == 9 { 15 } else { 10 };
            if self.st.G_j > need {
                for v4 in 0..e as usize {
                    let d = self.bat.d[v4].npe_mut()?;
                    let locked = match &d.kind {
                        Kind::ProtO { l, .. } | Kind::ProtN { l, .. } => *l,
                        _ => return Err(CCE),
                    };
                    if !locked {
                        d.fire(v1, v2, 0);
                        self.st.G_j = 0;
                        break;
                    }
                }
            }
        } else if s == 8 && self.st.G_j > 10 {
            for v6 in 0..e as usize {
                let d = self.bat.d[v6].npe_mut()?;
                if !d.d {
                    d.fire(v1, v2, 0);
                    self.st.G_j = 0;
                    break;
                }
            }
        }
        Ok(())
    }

    /// `G.b(int, int)`: primary fire / charge
    pub fn G_fire(&mut self, v1: i32, v2: i32) -> Res {
        let s = self.st.G_s;
        if s != -1 {
            if s != 0 && s != 9 && s != 5 && s != 8 {
                let b = self.st.G_b as usize;
                let d = self.bat.d.get_mut(b).ok_or(crate::gfx::AIOOBE)?.npe_mut()?;
                if !d.d {
                    if s == 1 {
                        if b % 2 == 0 {
                            d.fire(v1 + 6, v2, rnd(10));
                        } else {
                            d.fire(v1 + 6, v2, -rnd(10));
                        }
                    } else {
                        d.fire(v1 + 6, v2, 0);
                    }
                    self.st.G_b += 1;
                    if self.st.G_b == self.st.G_e {
                        self.st.G_b = 0;
                    }
                }
            } else if self.st.G_j < 20 {
                self.st.G_j += 1;
            }
        }
        Ok(())
    }

    /// `G.b(type, slot, level)`: create one weapon
    fn G_make_weapon(&mut self, v1: i32, v2: usize, v3: i32) {
        let v4 = (3 + v3 / 20 / 2).min(15);
        let st = &mut self.st;
        let e = match v1 {
            0 => Some(Ent::new_y(15)),
            1 => Some(Ent::new_b(4, st)),
            2 => Some(Ent::new_o(6)),
            3 => Some(Ent::new_a()),
            4 => Some(Ent::new_prot(v2 as i32, v4)),
            5 => Some(Ent::new_n(v2 as i32, 4)),
            6 => Some(Ent::new_q(4, st)),
            7 => Some(Ent::new_f(4, st)),
            8 => Some(Ent::new_d(4)),
            9 => Some(Ent::new_O(v2 as i32, v4)),
            _ => None,
        };
        self.bat.d[v2] = e;
    }

    /// `G.a(type, level, eyeLevel)`: set up weapons for the equipped eye
    pub fn G_init_weapons(&mut self, v1: i32, v2: i32, v3: i32) -> Res {
        self.st.G_j = 0;
        self.st.G_k = v2;
        let mut v4 = 13;
        let e = match v1 {
            -1 => 0,
            0 | 1 | 2 | 4 | 5 | 6 | 7 => v3 / 100 + 1,
            3 => {
                v4 += 2 + v3 / 100;
                1
            }
            8 => {
                v4 += 2 + v3 / 100;
                v3 / 100 + 1
            }
            9 => {
                v4 += 2 + v3 / 100;
                (1 + v3 / 100).min(11)
            }
            _ => self.st.G_e,
        };
        self.st.G_e = e.min(G_P as i32);
        self.st.G_r = G_A;
        for v5 in 0..self.st.G_r as usize {
            self.bat.q[v5] = Some(Ent::new_g(v4));
        }
        if let Some(q) = self.bat.q[0].as_mut() {
            q.load(&mut self.img);
        }
        for v6 in 0..self.st.G_e as usize {
            self.bat.d[v6] = None;
            self.G_make_weapon(v1, v6, v3);
        }
        self.img.y_c = None;
        self.img.B_l = None;
        self.img.o_i = None;
        if v1 != -1 {
            if let Some(d) = self.bat.d[0].as_mut() {
                d.load(&mut self.img);
            } else {
                return Err(NPE);
            }
        }
        self.st.G_s = v1;
        Ok(())
    }
}

/// Unused-image helper kept for symmetry with `G.a()` / `G.h()` (backlight).
#[allow(dead_code)]
pub fn lights() {}
