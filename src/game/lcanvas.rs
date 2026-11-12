//! `L`: the shooting-game canvas.
//!
//! The original did all of its logic inside `paint()`, once per 150 ms
//! repaint (100 ms from pilot level 100). This port runs it at 60 fps:
//! [`Game::L_tick`] is called every 1/60 s; the first sub-step of each
//! original frame runs [`Game::L_beat`] (that frame's discrete logic) and
//! every sub-step runs [`Game::L_sub`] (movement, projectile hits).
//! [`Game::L_draw`] draws the current state without changing it.

use super::spider::Plane;
use super::{eyes::v_draw, load, part, rnd, Cur, Eye, Game, Images, Snd};
use crate::midp::gfx::{Graphics, OptExt, Res, HCENTER, LEFT, TOP};
use crate::midp::lang;
use crate::midp::lcdui::{game_action, K_FIRE, K_SOFT_L, K_SOFT_R};

const TL: i32 = TOP | LEFT;
const TC: i32 = TOP | HCENTER;
const BL: i32 = 32 | LEFT;

pub struct LCanvas {
    /// `this.i.a`: repaint thread paused when 1
    pub i_a: i32,
    pub s: Plane,
    pub J: Snd,
    /// cheat code buffer
    pub G: String,
    /// invincibility cheat
    pub d: bool,
    pub r: i32,
    pub m: i32,
    pub n: i32,
    pub u: i32,
    pub t: i32,
    pub x: bool,
    /// 60 fps: a scripted motion (intro, fly-out, banner) runs this beat
    pub mv: bool,
}

impl LCanvas {
    pub fn new(img: &mut Images) -> LCanvas {
        LCanvas { i_a: 0, s: Plane::new(img), J: Snd::default(), G: String::new(), d: false, r: 0, m: 0, n: 0, u: 0, t: 0, x: false, mv: false }
    }
}

impl Game {
    /// Rest of `new L(...)` after the fields
    pub fn L_ctor(&mut self) {
        super::rnd_init();
        self.st.E_h = 1;
        self.st.L_D = false;
        self.img.L_A = load("/menu/umenubar.png");
        self.img.L_F = load("/menu/lmenubar.png");
        self.img.L_o = load("/bgEffect/01.png");
        self.img.L_j = load("/explore03.png");
        self.st.L_H = 0;
        self.st.L_k = 0;
        self.lc.d = false;
    }

    /// `m()`
    pub fn L_m(&mut self) {
        self.ps.z = true;
        self.st.L_H = 0;
        self.st.L_k = 0;
        self.lc.d = false;
        self.sk = 0;
    }

    /// `k()`: true if L's images are unloaded
    pub fn L_unloaded(&self) -> bool {
        self.img.L_K.is_none()
    }

    /// `d()`: pause (resume to the saved state)
    pub fn L_d(&mut self) {
        self.lc.i_a = 1;
        self.sk = 0;
        if self.st.j_g == 2 {
            self.st.L_H = 3;
        } else {
            self.st.L_H = self.st.j_g;
            if self.st.L_H == 4 || self.st.L_H == 7 {
                self.st.L_D = self.st.j_k;
                self.lc.m = self.st.j_l;
                self.lc.n = self.st.j_h;
                self.st.L_M = self.st.j_j;
                self.st.L_a = self.st.j_c;
            }
        }
    }

    /// `b()`: unpause
    pub fn L_b(&mut self) {
        self.lc.i_a = 0;
        self.sk = 0;
        self.lc.J.tune(&mut self.audio, 3);
        self.st.L_H = self.st.j_g;
    }

    /// `a(int)`: qualifies for the high score table
    fn L_is_high(&self, v1: i32) -> bool {
        self.st.ps_u.iter().any(|&u| v1 > u)
    }

    /// `a()`: insert j.d into the high score table
    pub fn L_add_high(&mut self) {
        let mut v1 = 0;
        while v1 < 3 {
            if self.st.j_d > self.st.ps_u[v1] {
                let s = self.st.j_d;
                for v3 in (v1 + 1..3).rev() {
                    self.st.ps_u[v3] = self.st.ps_u[v3 - 1];
                }
                self.st.ps_u[v1] = s;
                v1 = 1000;
            }
            v1 += 1;
        }
    }

    /// `c()`: load one image per frame (loading bar)
    fn L_load_step(&mut self) {
        let im = &mut self.img;
        match self.st.L_k {
            0 => im.L_z = load("/attribute/equip/equipped.png"),
            1 => im.L_y = load("/attribute/eyeshine01.png"),
            2 => im.L_h = load("/attribute/no.png"),
            3 => im.L_l = load("/attribute/equip/lplane01.png"),
            4 => im.L_q = load("/type.png"),
            5 => {
                im.L_E = load("/no02.png");
                im.L_f = load("/attribute/eyesel.png");
            }
            6 => {
                im.L_K = load("/attribute/equip/enbar.png");
                im.L_c = load("/attribute/scorebox.png");
            }
            _ => {}
        }
        self.st.L_k += 1;
    }

    /// `f()` (static): unload
    pub fn L_unload(&mut self) {
        let im = &mut self.img;
        im.L_f = None;
        im.L_z = None;
        im.L_y = None;
        im.L_h = None;
        im.L_l = None;
        im.L_q = None;
        im.L_E = None;
        im.L_K = None;
        im.L_c = None;
    }

    /// `l()`: new mission
    fn L_new_mission(&mut self) -> Res {
        self.G_init_spiders()?;
        self.lc.s.l = 60;
        self.lc.s.k = 128;
        let s = &self.st;
        self.lc.s.set_stats(s.j_n, s.j_e, s.j_i, s.j_a);
        self.L_setup_weapons()?;
        self.G_reset_fx();
        Ok(())
    }

    fn L_setup_weapons(&mut self) -> Res {
        if self.st.j_f != -1 {
            let e = self.eye(self.st.j_f)?.clone();
            self.G_init_weapons(e.b, self.st.j_b, e.d)?;
            self.G_init_eggs();
        } else {
            self.G_init_eggs();
            self.G_init_weapons(self.st.j_f, self.st.j_b, 0)?;
        }
        Ok(())
    }

    /// `e()`: restore a saved mission
    fn L_restore(&mut self) -> Res {
        self.lc.s.l = self.st.j_o;
        self.lc.s.k = self.st.j_m;
        let s = &self.st;
        self.lc.s.set_stats(s.j_n, s.j_e, s.j_i, s.j_a);
        for v1 in 0..self.st.x_f as usize {
            let s = &self.st;
            let a = (v1 as i32, s.x_i[v1], s.x_c[v1], s.x_e[v1], s.x_a[v1], s.x_k[v1], s.x_d[v1], s.x_h[v1], s.x_g[v1]);
            self.G_restore_spider(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8)?;
        }
        self.L_setup_weapons()?;
        self.G_load_eggs()?;
        self.G_load_basic()?;
        self.G_load_weapons()
    }

    /// `b(Graphics)`
    fn L_clear(&self, g: &mut Graphics) {
        g.set_color(0, 0, 0);
        g.fill_rect(0, 0, 128, 128);
    }

    /// `j()`: snapshot the mission into statics (pause/save)
    pub fn L_save(&mut self) {
        self.ps.w = true;
        let s = &mut self.st;
        s.j_o = self.lc.s.l;
        s.j_m = self.lc.s.k;
        s.j_n = self.lc.s.m;
        s.j_e = self.lc.s.e;
        s.j_i = self.lc.s.h;
        s.j_a = self.lc.s.a;
        s.j_g = s.L_H;
        s.j_k = s.L_D;
        s.j_l = self.lc.m;
        s.j_h = self.lc.n;
        s.j_j = s.L_M;
        s.j_c = s.L_a;
        self.G_save_spiders();
        self.G_save_eggs();
        self.G_save_basic();
        self.G_save_weapons();
    }

    /// `g()`: mission failed; lose the equipped eye
    fn L_lose_eye(&mut self) -> Res {
        if self.st.j_f != -1 {
            let f = self.st.j_f;
            self.J_remove(&[f], 1);
            if self.st.j_f == self.st.I_a {
                self.st.I_a = -1;
                self.st.I_c = -1;
            }
            self.st.j_f = -1;
        }
        self.j_b()
    }

    /// `i()`: award the reward eye once
    fn L_award(&mut self) -> Res {
        if self.st.L_D {
            let v1 = rnd(5);
            let v2 = if self.st.j_b < 300 { 1 + rnd(10) } else { 1 + rnd(20) };
            if self.eyes.len() < 30 {
                self.J_add(v1, v2);
            }
            self.lc.m = v1;
            self.lc.n = v2;
            if self.st.j_f != -1 {
                let f = self.st.j_f;
                self.eye_mut(f)?.d += 1;
                self.J_cap(f)?;
            }
            self.st.j_b += 1;
            self.j_clamp();
            self.j_b()?;
        }
        self.st.L_D = false;
        Ok(())
    }

    fn L_failed(&mut self) -> Res {
        self.L_lose_eye()?;
        self.st.L_e = 0;
        self.st.L_H = 5;
        Ok(())
    }

    // ------------------------------------------------------------ 60 fps

    /// Sub-steps per original frame at the current repaint period.
    pub fn sub_n(&self) -> i32 {
        ((self.st.h_b as f64) * 60.0 / 1000.0).round().max(1.0) as i32
    }

    /// One 1/60 s step of the shooting game.
    pub fn L_tick(&mut self) {
        let n = self.sub_n();
        self.sn = n;
        if self.sk >= n {
            self.sk = 0;
        }
        let k = self.sk;
        if k == 0 {
            let _ = self.L_beat();
        }
        if matches!(self.cur, Cur::L) && self.lc.i_a == 0 {
            let _ = self.L_sub(k, n);
        }
        self.sk = (k + 1) % n;
    }

    /// The discrete part of one original `paint()`.
    fn L_beat(&mut self) -> Res {
        match self.st.L_H {
            0 => {
                if self.st.L_k < 7 {
                    self.L_load_step();
                } else if self.ps.w {
                    self.L_restore()?;
                    self.ps.w = false;
                    self.L_d();
                } else {
                    self.L_new_mission()?;
                    self.st.L_H = 1;
                    self.st.j_g = 2;
                }
            }
            1 => {
                // `if (s.k > 85) s.k -= 5; else d();`
                self.lc.mv = self.lc.s.k > 85;
                if !self.lc.mv {
                    self.L_d();
                }
                if self.st.l_p > 0 {
                    self.st.l_p -= 1;
                }
            }
            2 => {
                let (x, y) = (self.lc.s.l, self.lc.s.k);
                // a spider sitting on the pilot hurts every original frame
                let (pb, po) = (self.lc.s.b, self.lc.s.o);
                if self.G_spider_touch(x, y, pb, po) {
                    self.rumble = Some((0x7000, 0x7000, 80));
                    if !self.lc.d {
                        self.lc.s.m -= 5;
                    }
                    if self.lc.s.m <= 0 {
                        return self.L_failed();
                    }
                }
                // a web on the pilot holds them for this frame
                if self.G_web_holds(x, y) {
                    self.lc.s.c = true;
                }
                self.lc.s.beat(&mut self.st.l_d);
                let atk = self.lc.s.h;
                let firing = self.st.l_j;
                self.G_beat(atk, x, y, firing)?;
                if self.st.l_p > 0 {
                    self.st.l_p -= 1;
                }
            }
            4 => {
                self.lc.J.close(&mut self.audio);
                self.L_award()?;
                match self.st.L_M {
                    0 => {
                        self.lc.mv = self.lc.s.k > 0;
                        if !self.lc.mv {
                            self.st.L_M = 1;
                        }
                    }
                    1 => {
                        self.lc.mv = self.st.L_a > -62;
                        if !self.lc.mv {
                            self.st.L_a = 0;
                            self.st.L_M = 2;
                        }
                    }
                    _ => {
                        self.st.L_a += 1;
                        if self.st.L_a > 20 {
                            if self.eyes.len() >= 30 {
                                self.st.L_H = 7;
                            } else {
                                if self.L_is_high(self.st.j_d) {
                                    self.L_add_high();
                                }
                                self.L_unload();
                                self.ps.z = false;
                                self.ps_e();
                                self.ps_a(0);
                            }
                        }
                    }
                }
            }
            5 => {
                self.lc.J.close(&mut self.audio);
                if self.st.L_e >= 16 {
                    self.st.L_e = 0;
                    self.st.L_H = 9;
                } else {
                    self.st.L_e += 1;
                    self.lc.u = rnd(5);
                    self.lc.t = rnd(5);
                }
            }
            6 => {
                self.L_new_mission()?;
                self.st.L_H = 2;
            }
            8 => {
                if self.st.L_k < 7 {
                    if self.st.L_k == 0 {
                        self.ps_e();
                    }
                    self.st.L_k += 1;
                } else {
                    self.ps_a(0);
                    self.st.L_e = 0;
                    self.st.L_k = 0;
                }
            }
            9 => {
                if self.L_is_high(self.st.j_d) {
                    self.L_add_high();
                    self.ps.z = false;
                } else {
                    self.L_unload();
                    self.ps.z = false;
                    self.st.L_e = 0;
                }
                self.st.L_H = 8;
                self.ps_g();
                self.st.L_k = 0;
            }
            _ => {}
        }
        Ok(())
    }

    /// Movement and projectile hits for sub-step `k` of `n`.
    fn L_sub(&mut self, k: i32, n: i32) -> Res {
        match self.st.L_H {
            1 => {
                if self.lc.mv {
                    self.lc.s.k -= part(5, k, n);
                }
            }
            2 => {
                self.lc.s.sub(k, n);
                let (x, y) = (self.lc.s.l, self.lc.s.k);
                let atk = self.lc.s.h;
                let hits = self.G_sub(k, n, atk, x, y)?;
                if hits.egg && self.st.L_H == 2 {
                    self.st.l_p = 10;
                    self.rumble = Some((0x5000, 0x9000, 120));
                    if !self.lc.d {
                        self.lc.s.m = super::K_a(self.G_enemy_attack(), self.lc.s.a, self.lc.s.m, 5);
                    }
                    if self.lc.s.m <= 0 {
                        self.st.K_a = false;
                        self.L_failed()?;
                    }
                }
            }
            4 => {
                if self.st.L_M == 0 && self.lc.mv {
                    self.lc.s.k -= part(10, k, n);
                } else if self.st.L_M == 1 && self.lc.mv {
                    self.st.L_a -= part(10, k, n);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Draw the current state (every display frame, also while paused).
    pub fn L_draw(&mut self, g: &mut Graphics) {
        let _ = self.L_draw_inner(g);
        g.reset_clip();
    }

    fn L_draw_inner(&mut self, g: &mut Graphics) -> Res {
        match self.st.L_H {
            0 | 8 => self.L_loading(g)?,
            1 => {
                self.L_clear(g);
                if self.st.l_p % 2 == 0 {
                    self.lc.s.draw(g)?;
                }
                self.L_score(g, self.st.j_d)?;
                self.J_draw(g, self.st.j_f, 26, 5)?;
                g.draw_image(&self.img.L_z, 1, 1, TL)?;
                g.set_color(0, 0, 0);
                g.fill_rect(0, 117, 128, 11);
                self.L_hud(g)?;
            }
            2 | 3 => {
                self.L_clear(g);
                g.draw_image(&self.img.L_A, 0, 0, TL)?;
                g.draw_image(&self.img.L_F, 0, 128, BL)?;
                self.G_draw(g)?;
                if self.st.l_p % 2 == 0 {
                    self.lc.s.draw(g)?;
                }
                g.reset_clip();
                self.G_draw_fx(g)?;
                let (x, y) = (self.lc.s.l, self.lc.s.k);
                self.G_charge_fx(g, x, y)?;
                self.L_score(g, self.st.j_d)?;
                self.J_draw(g, self.st.j_f, 26, 5)?;
                g.draw_image(&self.img.L_z, 1, 1, TL)?;
                if self.lc.d {
                    g.set_color(112, 40, 244);
                    g.fill_rect(0, 117, 128, 11);
                }
                self.L_hud(g)?;
            }
            4 => {
                self.L_complete_draw(g)?;
                self.L_hud(g)?;
            }
            5 => {
                self.L_clear(g);
                g.draw_image(&self.img.L_A, 0, 0, TL)?;
                g.draw_image(&self.img.L_F, 0, 128, BL)?;
                self.L_score(g, self.st.j_d)?;
                g.draw_image(&self.img.L_z, 1, 1, TL)?;
                g.draw_image(&self.img.L_K, 0, 117, TL)?;
                let lh = self.img.L_l.npe()?.h;
                g.set_clip(37, 47, 62, lh);
                g.draw_image(&self.img.L_l, 37, 55, TL)?;
                g.reset_clip();
                let (u, t, e) = (self.lc.u, self.lc.t, self.st.L_e);
                if e >= 2 {
                    if e < 8 {
                        g.set_clip(52 + u, 66 + t, 33, 35);
                        g.draw_image(&self.img.L_j, 52 + u, 66 + t, TL)?;
                    } else if e < 12 {
                        g.set_clip(40 + u, 54 + t, 44, 50);
                        g.draw_image(&self.img.L_j, 40 + u - 88, 54 + t, TL)?;
                    } else {
                        g.set_clip(38 + u, 50 + t, 43, 46);
                        g.draw_image(&self.img.L_j, 38 + u - 44, 50 + t, TL)?;
                    }
                    g.reset_clip();
                }
            }
            7 => {
                let (m, n) = (self.lc.m, self.lc.n);
                self.L_full(g, m, n)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// `c(Graphics)`: loading screen
    fn L_loading(&self, g: &mut Graphics) -> Res {
        let v2 = [0, 1, 2, 3, 4, 5, 6, 1];
        let k = self.st.L_k;
        self.L_clear(g);
        g.set_color(27, 0, 97);
        for v3 in 0..15 {
            g.draw_line(v3 * 9, 0, v3 * 9, 128);
            g.draw_line(0, v3 * 9, 128, v3 * 9);
        }
        g.set_color(0, 0, 0);
        g.fill_rect(0, 0, 128, 11);
        g.fill_rect(0, 117, 128, 11);
        g.draw_image(&self.img.L_A, 0, 0, TL)?;
        g.draw_image(&self.img.L_F, 0, 128, BL)?;
        g.set_color(88, 89, 172);
        g.draw_rect(35, 56, 84, 7);
        g.set_color(252, 0, 255);
        g.fill_rect(36, 58, k * 83 / 8, 3);
        g.set_color(0, 0, 0);
        g.fill_arc(12, 42, 25, 25, 0, 360);
        g.set_color(213, 170, 0);
        g.draw_arc(11, 41, 27, 27, 0, 360);
        g.set_clip(18, 47, 15, 15);
        g.draw_image(&self.img.L_o, 18 - v2[(k % 8).max(0) as usize] * 15, 47, TL)?;
        g.reset_clip();
        Ok(())
    }

    /// `a(Graphics, int, int, int)`: arrow
    fn L_arrow(g: &mut Graphics, v2: i32, v3: i32, v4: i32) {
        g.set_color(255, 255, 255);
        if v4 == 0 {
            g.fill_triangle(v2, v3 + 2, v2 + 3, v3 - 1, v2 + 3, v3 + 6);
        } else if v4 == 1 {
            g.fill_triangle(v2 - 1, v3 - 1, v2 - 1, v3 + 6, v2 + 3, v3 + 2);
        }
    }

    /// `a(Graphics, 5 ints)`: eye selection frame
    fn L_eyesel(&self, g: &mut Graphics, v2: i32, v3: i32, v4: i32, v5: i32, v6: i32) -> Res {
        let (mut v2, mut v3, mut v4) = (v2, v3, v4);
        if v5 == v6 {
            v4 = 0;
            v2 -= 1;
            v3 -= 1;
        }
        g.set_clip(v2, v3, 23, 23);
        g.draw_image(&self.img.L_f, v2 - 23 * v4, v3, TL)?;
        g.reset_clip();
        Ok(())
    }

    /// `a(Graphics, int, int)`: eye list full -> choose one to delete
    fn L_full(&self, g: &mut Graphics, v2: i32, v3: i32) -> Res {
        self.L_clear(g);
        g.draw_image(&self.img.L_A, 0, 0, TL)?;
        g.draw_image(&self.img.L_F, 0, 128, BL)?;
        self.L_score(g, self.st.j_d)?;
        v_draw(g, &self.img.v_c, v2, 26, 5, v3)?;
        g.draw_image(&self.img.L_z, 1, 1, TL)?;
        g.set_color(255, 0, 237);
        g.draw_rect(1, 33, 126, 41);
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(91), 4, 38, TL);
        g.draw_string(&lang::s(75), 4, 50, TL);
        Self::L_arrow(g, 3, 87, 0);
        Self::L_arrow(g, 118, 87, 1);
        let ja = self.st.J_a;
        let r = self.lc.r;
        if ja < 6 {
            for v4 in 0..ja {
                self.J_draw(g, v4, 9 + v4 * 21, 78)?;
                self.L_eyesel(g, 8 + v4 * 21, 77, 1, v4, r)?;
            }
        } else if r < 5 {
            for v5 in 0..5 {
                self.J_draw(g, v5, 9 + v5 * 21, 78)?;
                self.L_eyesel(g, 8 + v5 * 21, 77, 1, v5, r)?;
            }
        } else {
            for v6 in r - 4..r + 1 {
                self.J_draw(g, v6, 9 + (v6 - (r - 4)) * 21, 78)?;
                self.L_eyesel(g, 92, 77, 1, v6, v6)?;
            }
        }
        Ok(())
    }

    fn L_hud(&self, g: &mut Graphics) -> Res {
        g.set_color(255, 0, 213);
        g.fill_rect(17, 120, self.lc.s.m / 10, 5);
        g.draw_image(&self.img.L_K, 0, 117, TL)
    }

    /// `c(Graphics, int)`: sliding banner
    fn L_banner(g: &mut Graphics, v2: i32) {
        let w = 128 - (v2 + 39);
        g.set_color(160, 204, 227);
        g.fill_rect(v2 + 39, 51, w, 28);
        g.set_color(24, 159, 227);
        g.fill_rect(v2 + 39, 52, w, 25);
        g.set_color(9, 58, 83);
        g.fill_rect(v2 + 39, 56, w, 14);
        g.set_color(20, 118, 168);
        g.fill_rect(v2 + 39, 57, w, 12);
        g.set_color(16, 91, 128);
        g.fill_rect(v2 + 39, 60, w, 5);
        g.set_color(9, 58, 83);
        g.fill_rect(v2 + 39, 61, w, 1);
    }

    /// Drawing half of `a(Graphics)` (mission complete sequence)
    fn L_complete_draw(&mut self, g: &mut Graphics) -> Res {
        if self.st.L_M == 0 {
            self.L_clear(g);
            self.lc.s.draw(g)?;
        } else if self.st.L_M == 1 {
            self.L_clear(g);
            self.L_score(g, self.st.j_d)?;
            let a = self.st.L_a;
            Self::L_banner(g, a);
            g.set_clip(a, 47, 62, 43);
            g.draw_image(&self.img.L_l, a - 62, 47, TL)?;
            g.reset_clip();
            self.J_draw(g, self.st.j_f, 26, 5)?;
            g.draw_image(&self.img.L_z, 1, 1, TL)?;
        } else {
            self.L_clear(g);
            self.L_score(g, self.st.j_d)?;
            Self::L_banner(g, -62);
            self.J_draw(g, self.st.j_f, 26, 5)?;
            g.draw_image(&self.img.L_z, 1, 1, TL)?;
            if self.st.L_a < 10 {
                g.set_color(255, 255, 255);
                g.draw_string(&lang::s(59), 64, 60, TC);
            } else {
                let v2 = Eye::new(self.lc.m, self.lc.n);
                g.draw_image(&self.img.L_q, 38, 41, TL)?;
                g.set_color(255, 0, 254);
                g.draw_rect(22, 85, 100, 18);
                g.draw_image(&self.img.L_y, 51, 52, TL)?;
                v_draw(g, &self.img.v_c, v2.b, 55, 56, v2.d)?;
                g.draw_image(&self.img.L_F, 0, 128, BL)?;
                g.set_color(60, 225, 0);
                g.draw_string(&lang::s(90), 40, 29, TL);
                g.set_color(255, 255, 255);
                g.draw_string(&format!("{}: {}", lang::s(82), v2.a), 26, 90, TL);
                self.L_level3(g, v2.d)?;
            }
        }
        Ok(())
    }

    /// `a(Graphics, int)`: score box
    fn L_score(&self, g: &mut Graphics, v2: i32) -> Res {
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(74), 51, 0, TL);
        g.draw_image(&self.img.L_c, 78, 2, TL)?;
        let v2 = v2.min(9_999_999);
        let digits = [
            v2 / 1_000_000 % 10,
            v2 / 100_000 % 10,
            v2 / 10_000 % 10,
            v2 / 1000 % 10,
            v2 / 100 % 10,
            v2 / 10 % 10,
            v2 % 10,
        ];
        for (v4, d) in digits.iter().enumerate() {
            let v4 = v4 as i32;
            g.set_clip(81 + 6 * v4, 3, 5, 5);
            g.draw_image(&self.img.L_h, 81 + 6 * v4 - d * 5, 3, TL)?;
        }
        g.reset_clip();
        Ok(())
    }

    /// `b(Graphics, int)`: 3-digit level
    fn L_level3(&self, g: &mut Graphics, v2: i32) -> Res {
        let d = [v2 / 100 % 10, v2 / 10 % 10, v2 % 10];
        for v4 in 0..3 {
            g.set_clip(90 + v4 * 7 - 6, 45, 6, 6);
            g.draw_image(&self.img.L_E, 90 + v4 * 7 - d[v4 as usize] * 6 - 6, 45, TL)?;
        }
        g.reset_clip();
        Ok(())
    }

    /// `b(int)`: cheat code 1234 -> invincible
    fn L_cheat(&mut self, v1: i32) -> bool {
        let v2 = 1234;
        let mut v3 = false;
        let gs = &mut self.lc.G;
        if !(48..=57).contains(&v1) {
            if gs.len() == 4 {
                if gs.parse::<i32>().ok() == Some(v2) {
                    self.st.L_a = 0;
                    v3 = true;
                }
                gs.clear();
            }
        } else {
            let dgt = v1 - 48;
            let want = match gs.len() {
                0 => v2 / 1000 % 10,
                1 => v2 / 100 % 10,
                2 => v2 / 10 % 10,
                _ => v2 % 10,
            };
            if gs.len() <= 3 && want == dgt {
                gs.push_str(&dgt.to_string());
                if gs.len() == 4 {
                    if gs.parse::<i32>().ok() == Some(v2) {
                        self.st.L_a = 0;
                        self.lc.d = true;
                        v3 = true;
                    }
                    gs.clear();
                }
            } else if gs.len() <= 3 {
                gs.clear();
            }
        }
        v3
    }

    /// `keyReleased`
    pub fn L_key_released(&mut self, v1: i32) {
        let v2 = game_action(v1);
        if v2 != 8 && v1 != 53 {
            self.lc.s.f = 0;
        } else {
            let (x, y) = (self.lc.s.l, self.lc.s.k);
            let _ = self.G_release(x, y);
            self.st.l_j = false;
        }
    }

    fn L_finish_to_menu(&mut self) {
        self.lc.J.close(&mut self.audio);
        if self.L_is_high(self.st.j_d) {
            self.L_add_high();
        } else {
            self.L_unload();
        }
        self.ps.z = false;
        self.ps.w = false;
        self.ps_e();
        self.ps_a(0);
        self.st.L_e = 0;
    }

    /// `keyPressed`
    pub fn L_key_pressed(&mut self, v1: i32) {
        let _ = self.L_key_inner(v1);
    }

    fn L_key_inner(&mut self, v1: i32) -> Res {
        let v2 = game_action(v1);
        match self.st.L_H {
            2 => {
                self.lc.x = self.L_cheat(v1);
                if v1 == 53 {
                    self.st.l_j = true;
                } else if v1 == 52 {
                    self.lc.s.f = 2;
                } else if v1 == 54 {
                    self.lc.s.f = 1;
                } else if v1 == K_SOFT_R {
                    self.lc.J.close(&mut self.audio);
                    self.L_save();
                    self.ps_b(0);
                } else {
                    match v2 {
                        1 => self.lc.s.f = 3,
                        2 => self.lc.s.f = 2,
                        5 => self.lc.s.f = 1,
                        6 => self.lc.s.f = 4,
                        8 => self.st.l_j = true,
                        _ => {}
                    }
                }
            }
            3 => {
                if self.lc.i_a == 1 {
                    self.L_b();
                }
                if v1 == K_SOFT_R {
                    self.lc.J.close(&mut self.audio);
                    self.L_save();
                    self.ps_b(0);
                }
            }
            4 => {
                if self.lc.i_a == 1 {
                    self.L_b();
                }
                if v1 == K_SOFT_R && self.st.L_a > 20 {
                    self.L_finish_to_menu();
                }
            }
            5 => {
                if self.lc.i_a == 1 {
                    self.L_b();
                }
                if v1 == K_SOFT_R || v1 == K_SOFT_L {
                    self.L_finish_to_menu();
                }
            }
            7 => {
                if self.lc.i_a == 1 {
                    self.L_b();
                }
                if v1 == K_SOFT_R {
                    self.lc.J.close(&mut self.audio);
                    if self.L_is_high(self.st.j_d) {
                        self.L_add_high();
                    }
                    self.L_unload();
                    self.ps.z = false;
                    self.ps.w = false;
                    self.ps_e();
                    self.ps_a(0);
                } else if v1 == K_FIRE {
                    let r = self.lc.r;
                    self.J_remove(&[r], 1);
                    let (m, n) = (self.lc.m, self.lc.n);
                    self.J_add(m, n);
                    if self.L_is_high(self.st.j_d) {
                        self.L_add_high();
                    }
                    self.L_unload();
                    self.ps.z = false;
                    self.ps.w = false;
                    self.ps_e();
                    self.ps_a(0);
                } else {
                    let ja = self.st.J_a;
                    match v2 {
                        2 => {
                            if ja > 0 {
                                self.lc.r = if self.lc.r > 0 { self.lc.r - 1 } else { ja - 1 };
                            }
                        }
                        5 => {
                            if ja > 0 {
                                self.lc.r = if self.lc.r < ja - 1 { self.lc.r + 1 } else { 0 };
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
