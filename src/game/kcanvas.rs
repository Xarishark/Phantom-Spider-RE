//! `k`: the menu/eyes canvas (splash, eye list, combine, pilot & spider
//! info, high scores, tutorial host).

use super::eyes::v_draw;
use super::spider::{Plane, Z};
use super::{load, rnd, Eye, Game, Images, Snd};
use crate::midp::gfx::{Graphics, Res, HCENTER, LEFT, TOP};
use crate::midp::lang;
use crate::midp::lcdui::{game_action, K_FIRE, K_SOFT_R};

const TL: i32 = TOP | LEFT;
const TC: i32 = TOP | HCENTER;
const BL: i32 = 32 | LEFT;

pub struct KCanvas {
    pub F: Snd,
    pub w: Z,
    pub M: Plane,
    pub d: [i32; 2],
    pub L: String,
    pub x: bool,
    pub D: bool,
    pub C: bool,
    pub Q: String,
    pub v: String,
    pub t: String,
}

impl KCanvas {
    pub fn new(img: &mut Images) -> KCanvas {
        KCanvas {
            F: Snd::default(),
            w: Z::new(img),
            M: Plane::new(img),
            d: [0; 2],
            L: String::new(),
            x: false,
            D: false,
            C: false,
            Q: String::new(),
            v: String::new(),
            t: String::new(),
        }
    }
}

fn k_strings_K() -> [String; 2] {
    [lang::s(37), lang::s(45)]
}
pub fn k_strings_J() -> [String; 2] {
    [lang::s(66), lang::s(76)]
}

impl Game {
    /// Rest of the `k` constructor
    pub fn k_ctor(&mut self) {
        self.st.k_W = self.st.K_b;
        if self.st.k_W == 0 {
            self.img.k_i = load("/frontpage.png");
        } else {
            self.k_load();
        }
        // u = new J()
        self.J_new();
        self.st.E_h = 0;
        super::rnd_init();
        self.kc.x = false;
        // this.g = new u(...) is constructed by Game::new
    }

    /// `f()`
    pub fn k_f(&mut self) {
        self.k_load();
        self.kc.x = false;
        self.st.k_W = self.st.K_b;
    }

    /// `a()`: load images
    pub fn k_load(&mut self) {
        Z::load_images(&mut self.img);
        let im = &mut self.img;
        im.k_Y = load("/bgpattern01.png");
        im.k_j = load("/type.png");
        im.k_ah = load("/attribute/eyesel.png");
        im.k_ad = load("/attribute/eyeshine01.png");
        im.k_e = load("/attribute/equip/equipped.png");
        im.k_a = load("/attribute/equip/enbar.png");
        im.k_l = load("/attribute/equip/lv.png");
        im.k_k = load("/attribute/equip/spdf.png");
        im.k_I = load("/attribute/equip/showbar.png");
        im.k_H = load("/attribute/equip/lplane01.png");
        im.k_ag[0] = load("/menu/lmenubar.png");
        im.k_ag[1] = load("/menu/umenubar.png");
        im.k_V = load("/mission/spiderpattern01.png");
        im.k_aj = load("/no02.png");
        im.k_h = load("/attribute/no.png");
        im.k_ai = load("/attribute/scorebox.png");
        im.k_A = load("/cSel.png");
        im.k_O = load("/light01.png");
        im.k_G = load("/bgEffect/01.png");
        im.k_p = load("/attribute/equip/eyeselbase.png");
    }

    /// `e()`: true if k's images are unloaded
    pub fn k_unloaded(&self) -> bool {
        self.img.k_G.is_none()
    }

    /// `c()` (static): unload
    pub fn k_unload(&mut self) {
        let im = &mut self.img;
        im.k_G = None;
        im.k_Y = None;
        im.k_j = None;
        im.k_ah = None;
        im.k_z = None;
        im.k_e = None;
        im.k_ad = None;
        im.k_a = None;
        im.k_l = None;
        im.k_k = None;
        im.k_I = None;
        im.k_H = None;
        im.k_aj = None;
        im.k_h = None;
        im.k_ai = None;
        im.k_A = None;
        im.k_p = None;
    }

    /// `b()`: scroll the background
    fn k_scroll(&mut self) {
        if self.st.k_b < 26 {
            self.st.k_b += 1;
        } else {
            self.st.k_b = 0;
        }
    }

    /// `b(Graphics, x, y, frame)`: background tile
    fn k_tile(&self, g: &mut Graphics, v2: i32, v3: i32, v4: i32) -> Res {
        g.set_clip(v2, v3, 26, 26);
        g.draw_image(&self.img.k_Y, v2 - v4 * 26, v3, TL)?;
        g.reset_clip();
        Ok(())
    }

    /// `d(Graphics)`: animated background
    fn k_bg(&self, g: &mut Graphics) -> Res {
        g.set_color(16, 0, 0);
        g.fill_rect(0, 0, 128, 128);
        let b = self.st.k_b;
        let mut v2 = 0;
        while v2 < 148 {
            let mut v3 = 0;
            while v3 < 148 {
                self.k_tile(g, v3 - b, v2 - b, 1)?;
                self.k_tile(g, v3, v2, 0)?;
                v3 += 26;
            }
            v2 += 26;
        }
        Ok(())
    }

    /// `a(Graphics, 5 ints)`: eye selection frame
    pub fn k_eyesel(&self, g: &mut Graphics, v2: i32, v3: i32, v4: i32, v5: i32, v6: i32) -> Res {
        let (mut v2, mut v3, mut v4) = (v2, v3, v4);
        if v5 == v6 {
            v4 = 0;
            v2 -= 1;
            v3 -= 1;
        }
        g.set_clip(v2, v3, 23, 23);
        g.draw_image(&self.img.k_ah, v2 - 23 * v4, v3, TL)?;
        g.reset_clip();
        Ok(())
    }

    /// `d(Graphics, x, y, dir)`: white arrow
    fn k_arrow(g: &mut Graphics, v2: i32, v3: i32, v4: i32) {
        g.set_color(255, 255, 255);
        if v4 == 0 {
            g.fill_triangle(v2, v3 + 2, v2 + 3, v3 - 1, v2 + 3, v3 + 6);
        } else if v4 == 1 {
            g.fill_triangle(v2 - 1, v3 - 1, v2 - 1, v3 + 6, v2 + 3, v3 + 2);
        }
    }

    /// `c(int)`: (dead computation in the original)
    fn k_c(&self, v1: i32) -> Res {
        self.j_bonus_atk(v1)?;
        self.j_bonus_def(v1)?;
        self.j_bonus_spd(v1)?;
        Ok(())
    }

    /// `b(Graphics, int)`: pilot information
    pub fn k_pilot(&self, g: &mut Graphics, v2: i32) -> Res {
        let s = &self.st;
        g.draw_image(&self.img.k_ag[1], 0, 0, TL)?;
        g.draw_image(&self.img.k_p, 101, 25, TL)?;
        if v2 != -1 {
            self.J_draw(g, v2, 102, 25)?;
        }
        g.draw_image(&self.img.k_e, 77, 22, TL)?;
        g.draw_image(&self.img.k_l, 4, 12, TL)?;
        self.k_level2(g, 25, 17, s.j_b)?;
        g.draw_image(&self.img.k_I, 50, 43, TL)?;
        g.set_clip(17, 25, 62, 43);
        g.draw_image(&self.img.k_H, 17, 25, TL)?;
        g.reset_clip();
        let v3 = [s.j_n / 10, s.j_e + s.k_r, (s.j_a + s.k_P) / 10, (s.j_i + s.k_Z) / 10];
        for v4 in 0..4 {
            g.set_color(31, 0, 108);
            g.fill_rect(81, 57 + v4 * 13, 44, 5);
            g.set_color(255, 1, 213);
            if v4 == 0 {
                g.fill_rect(81, 57 + v4 * 13, v3[v4 as usize], 5);
            } else {
                g.fill_rect(81, 57 + v4 * 13, v3[v4 as usize] * 5, 5);
            }
        }
        g.draw_image(&self.img.k_a, 64, 53, TL)?;
        for v5 in 0..3 {
            g.draw_image(&self.img.k_a, 64, 53 + (v5 + 1) * 13, TL)?;
            g.set_clip(64, 68 + v5 * 13, 13, 9);
            g.draw_image(&self.img.k_k, 64 - v5 * 13, 68 + v5 * 13, TL)?;
            g.reset_clip();
        }
        g.draw_image(&self.img.k_ag[0], 0, 128, BL)
    }

    /// `f(Graphics)`: "New mission" difficulty screen
    fn k_new_mission(&mut self, g: &mut Graphics) -> Res {
        self.k_bg(g)?;
        self.k_scroll();
        g.draw_image(&self.img.k_ag[1], 0, 0, TL)?;
        g.draw_image(&self.img.k_ag[0], 0, 128, BL)?;
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(65), 64, 23, TC);
        g.set_color(88, 89, 172);
        g.fill_rect(9, 40, 111, 62);
        g.set_color(0, 0, 0);
        g.fill_rect(10, 41, 109, 60);
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(61), 11, 43, TL);
        g.draw_string(&lang::s(63), 11, 54, TL);
        g.draw_string(&lang::s(64), 11, 65, TL);
        g.draw_string(&lang::s(58), 11, 76, TL);
        g.draw_string(&lang::s(60), 11, 87, TL);
        g.fill_rect(64, 43, 52, 7);
        g.fill_rect(64, 54, 52, 7);
        g.fill_rect(64, 65, 52, 7);
        g.fill_rect(64, 76, 52, 7);
        g.fill_rect(64, 87, 52, 7);
        g.set_color(246, 22, 247);
        let s = &self.st;
        g.fill_rect(64, 43, s.i_a, 7);
        g.fill_rect(64, 54, 52 * s.i_d / 900, 7);
        g.fill_rect(64, 65, 52 * (s.i_c / 2) * 90 / 900, 7);
        g.fill_rect(64, 76, 52 * s.i_g * 5 / 900, 7);
        g.fill_rect(64, 87, 52 * s.i_b * 5 / 900, 7);
        g.draw_image(&self.img.k_V, 89, 95, TL)
    }

    /// `a(Graphics, x, y, level)`: 2-digit pilot level
    pub fn k_level2(&self, g: &mut Graphics, v2: i32, v3: i32, v4: i32) -> Res {
        let v4 = v4 / 20 + 1;
        let d = [v4 / 10 % 10, v4 % 10];
        for v6 in 0..2 {
            g.set_clip(v2 + v6 * 7, v3, 6, 6);
            g.draw_image(&self.img.k_aj, v2 + v6 * 7 - d[v6 as usize] * 6, v3, TL)?;
        }
        g.reset_clip();
        Ok(())
    }

    /// `c(Graphics, x, y, n)`: 3-digit number
    pub fn k_num3(&self, g: &mut Graphics, v2: i32, v3: i32, v4: i32) -> Res {
        let d = [v4 / 100 % 10, v4 / 10 % 10, v4 % 10];
        for v6 in 0..3 {
            g.set_clip(v2 + v6 * 7 - 6, v3, 6, 6);
            g.draw_image(&self.img.k_aj, v2 + v6 * 7 - d[v6 as usize] * 6 - 6, v3, TL)?;
        }
        g.reset_clip();
        Ok(())
    }

    /// `a(Graphics, int)`: score box
    pub fn k_score(&self, g: &mut Graphics, v2: i32) -> Res {
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(74), 51, 0, TL);
        g.draw_image(&self.img.k_ai, 78, 2, TL)?;
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
            g.draw_image(&self.img.k_h, 81 + 6 * v4 - d * 5, 3, TL)?;
        }
        g.reset_clip();
        Ok(())
    }

    /// `d()` (static): roll the next mission's enemy stats
    pub fn k_roll_enemy(&mut self) -> Res {
        let v0 = self.st.j_b / 100 + 2;
        let v1 = if v0 < 20 {
            2
        } else if v0 < 50 {
            3
        } else {
            4
        };
        let v2 = (10 + rnd(v0)) * 2;
        let v3 = (10 + rnd(v0)) * 3;
        let v4 = (200 + rnd(v0)) * 3;
        let v5 = (rnd(v1) + 1) * 2;
        let mut v6 = self.st.j_e + v5;
        if self.st.j_f != -1 && self.eye(self.st.j_f)?.b == 3 {
            v6 = v6 / 4 + 1;
        }
        let mut v7 = (self.st.j_a + v2) * 2;
        let mut v8 = (self.st.j_i + v3) * 2;
        let mut v9 = self.st.j_n + v4;
        let mut v10 = v6 + v5;
        if self.st.j_b < 100 {
            v9 /= 4;
            self.st.h_b = 150;
        } else {
            self.st.h_b = 100;
        }
        if v10 > 18 {
            v10 = 18;
        }
        if v7 > 180 {
            v7 = 180;
            self.st.j_a -= 30;
        }
        if v8 > 180 {
            v8 = 180;
            self.st.j_i -= 30;
        }
        if v9 > 900 {
            v9 = 900;
        }
        let v11 = (52 * (v7 * 5 + v8 * 5 + v9 + v10 * 10) / 4500).min(52);
        let s = &mut self.st;
        s.i_d = v9;
        s.i_c = v10;
        s.i_g = v8;
        s.i_b = v7;
        s.i_f = s.j_b;
        s.i_a = v11;
        Ok(())
    }

    /// `e(Graphics)`: combine animation
    fn k_combine_anim(&mut self, g: &mut Graphics) -> Res {
        self.k_bg(g)?;
        self.k_scroll();
        g.draw_image(&self.img.k_ag[1], 0, 0, TL)?;
        self.k_score(g, self.st.j_d)?;
        self.k_eye_strip(g, 15, |s, g, v, x| {
            if v == s.kc.d[0] || v == s.st.k_m {
                g.draw_image(&s.img.k_A, x, 15, TL)?;
            }
            Ok(())
        })?;
        g.set_color(0, 0, 0);
        g.fill_rect(16, 83, 91, 18);
        g.set_color(255, 0, 254);
        g.draw_rect(16, 83, 91, 18);
        g.set_color(255, 255, 255);
        g.draw_string(&format!("{}: {}", lang::s(82), self.kc.L), 20, 88, TL);
        g.set_color(255, 255, 255);
        g.fill_rect(0, 36, 128, 48);
        let y = self.st.k_y;
        if y < 4 {
            g.set_color(0, 0, 0);
            g.fill_rect(0, 37, 128, 46);
            self.J_draw(g, self.kc.d[0], y * 35 / 3, 48)?;
            self.J_draw(g, self.st.k_m, 109 - y * 35 / 3, 48)?;
        } else {
            g.set_clip(0, 37, 128, 45);
            if y < 5 {
                g.draw_image(&self.img.k_O, 0, 37 - y % 2 * 45, TL)?;
            } else {
                g.draw_image(&self.img.k_O, 0, -8, TL)?;
            }
            g.reset_clip();
            if y < 5 {
                self.J_draw(g, self.kc.d[0], 35 + (y - 3) * 6, 48)?;
                self.J_draw(g, self.st.k_m, 73 - (y - 3) * 6, 48)?;
            } else {
                v_draw(g, &self.img.v_c, self.kc.d[1], 54, 48, 1)?;
            }
        }
        if self.st.k_y < 7 {
            self.st.k_y += 1;
        } else {
            self.st.k_y += 1;
            let v6 = [self.kc.d[0], self.st.k_m];
            let a = self.eye(v6[0])?.clone();
            let b = self.eye(v6[1])?.clone();
            let v3 = Self::J_combine(&a, &b);
            self.J_remove(&v6, 2);
            self.J_add(v3.b, v3.d);
            self.st.k_y = 0;
            self.st.k_W = 16;
            self.st.k_m = self.st.J_a - 1;
        }
        g.set_color(0, 0, 0);
        g.fill_rect(0, 117, 128, 11);
        g.draw_image(&self.img.k_ag[0], 0, 128, BL)
    }

    /// The scrolling 5-eye strip used by several screens. `extra` draws the
    /// per-slot overlay (selection marker) given (eye index, x).
    fn k_eye_strip(
        &self,
        g: &mut Graphics,
        y: i32,
        extra: impl Fn(&Game, &mut Graphics, i32, i32) -> Res,
    ) -> Res {
        let ja = self.st.J_a;
        let m = self.st.k_m;
        if ja < 6 {
            for v in 0..ja {
                self.J_draw(g, v, 9 + v * 21, y)?;
                extra(self, g, v, 9 + v * 21)?;
            }
        } else if m < 5 {
            for v in 0..5 {
                self.J_draw(g, v, 9 + v * 21, y)?;
                extra(self, g, v, 9 + v * 21)?;
            }
        } else {
            for v in m - 4..m + 1 {
                let x = 9 + (v - (m - 4)) * 21;
                self.J_draw(g, v, x, y)?;
                extra(self, g, v, x)?;
            }
        }
        Ok(())
    }

    /// `a(Graphics, 4 ints)`: challenge VS screen
    pub fn k_vs(&mut self, g: &mut Graphics, v2: i32, v3: i32, v4: i32, v5: i32) -> Res {
        self.k_scroll();
        self.k_bg(g)?;
        g.set_color(16, 0, 56);
        for v6 in 0..15 {
            g.draw_line(v6 * 9, 0, v6 * 9, 128);
            g.draw_line(0, v6 * 9, 128, v6 * 9);
        }
        g.set_color(0, 0, 0);
        g.fill_rect(0, 0, 128, 11);
        g.fill_rect(0, 117, 128, 11);
        g.draw_image(&self.img.k_ag[0], 0, 0, TL)?;
        g.draw_image(&self.img.k_ag[1], 0, 128, BL)?;
        g.draw_image(&self.img.k_j, 61, 2, TL)?;
        g.draw_image(&self.img.k_j, 2, 2, TL)?;
        self.J_draw(g, v2, 18, 14)?;
        v_draw(g, &self.img.v_c, v4, 78, 14, self.st.E_b)?;
        self.k_num3(g, 54, 7, v3)?;
        self.k_num3(g, 113, 7, v5)?;
        g.set_color(0, 0, 0);
        g.fill_rect(0, 36, 126, 15);
        g.set_color(255, 0, 237);
        g.draw_rect(0, 36, 126, 15);
        g.draw_line(57, 36, 57, 51);
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(70), 3, 40, TL);
        g.draw_string(&lang::s(26), 58, 40, TL);
        self.kc.w.set_size(1);
        self.kc.w.draw_vs(g, &self.img)?;
        g.set_color(28, 120, 0);
        g.draw_string(&lang::s(85), 65, 102, TC);
        g.set_color(60, 255, 0);
        g.draw_string(&lang::s(85), 64, 101, TC);
        Ok(())
    }

    /// `b(Graphics, int, int)`: eye list full (challenge win)
    fn k_full(&mut self, g: &mut Graphics, v2: i32, v3: i32) -> Res {
        self.k_scroll();
        self.k_bg(g)?;
        g.draw_image(&self.img.k_ag[0], 0, 0, TL)?;
        g.draw_image(&self.img.k_ag[1], 0, 128, BL)?;
        self.k_score(g, self.st.j_d)?;
        g.draw_image(&self.img.k_p, 25, 4, TL)?;
        v_draw(g, &self.img.v_c, v2, 26, 5, v3)?;
        g.draw_image(&self.img.k_e, 1, 1, TL)?;
        g.set_color(0, 0, 0);
        g.fill_rect(1, 33, 126, 41);
        g.set_color(255, 0, 237);
        g.draw_rect(1, 33, 126, 41);
        g.set_color(255, 255, 255);
        g.draw_string(&lang::s(91), 4, 38, TL);
        g.draw_string(&lang::s(75), 4, 50, TL);
        Self::k_arrow(g, 3, 87, 0);
        Self::k_arrow(g, 118, 87, 1);
        self.k_strip_sel(g, 78, 77)
    }

    /// Eye strip with selection frames (`J.a<6 / m<5 / else` pattern).
    fn k_strip_sel(&self, g: &mut Graphics, y: i32, sy: i32) -> Res {
        let ja = self.st.J_a;
        let m = self.st.k_m;
        if ja < 6 {
            for v4 in 0..ja {
                self.J_draw(g, v4, 9 + v4 * 21, y)?;
                self.k_eyesel(g, 8 + v4 * 21, sy, 1, v4, m)?;
            }
        } else if m < 5 {
            for v5 in 0..5 {
                self.J_draw(g, v5, 9 + v5 * 21, y)?;
                self.k_eyesel(g, 8 + v5 * 21, sy, 1, v5, m)?;
            }
        } else {
            for v6 in m - 4..m + 1 {
                self.J_draw(g, v6, 9 + (v6 - (m - 4)) * 21, y)?;
                self.k_eyesel(g, 92, sy, 1, v6, v6)?;
            }
        }
        Ok(())
    }

    /// `a(Graphics, int, int)`: green chevron
    pub fn k_chevron(g: &mut Graphics, v2: i32, v3: i32) {
        g.set_color(6, 255, 0);
        if v2 == 0 {
            g.fill_rect(51, v3, 1, 1);
            g.fill_rect(50, v3 + 1, 1, 1);
            g.fill_rect(53, v3 + 1, 1, 1);
            g.fill_rect(54, v3 + 2, 1, 1);
            g.fill_rect(49, v3 + 2, 1, 1);
        }
        if v2 == 1 {
            g.fill_rect(49, v3, 1, 1);
            g.fill_rect(54, v3, 1, 1);
            g.fill_rect(53, v3 + 1, 1, 1);
            g.fill_rect(50, v3 + 1, 1, 1);
            g.fill_rect(51, v3 + 2, 1, 1);
        }
    }

    /// `b(Graphics)`: eye strip at the top
    fn k_top_strip(&self, g: &mut Graphics) -> Res {
        self.k_strip_sel(g, 15, 14)
    }

    /// `a(Graphics, 7 ints + String)`: eye list screen
    #[allow(clippy::too_many_arguments)]
    pub fn k_eyelist(&mut self, g: &mut Graphics, _v2: i32, v3: i32, v4: i32, v5: &str, v6: i32, v7: i32, v8: i32) -> Res {
        self.k_bg(g)?;
        self.k_scroll();
        g.draw_image(&self.img.k_ag[1], 0, 0, TL)?;
        self.k_score(g, v7)?;
        Self::k_arrow(g, 0, 21, 0);
        Self::k_arrow(g, 120, 21, 1);
        g.set_color(0, 0, 0);
        g.fill_rect(3, 39, 69, 20);
        g.set_color(255, 204, 4);
        g.draw_rect(3, 39, 69, 20);
        let kk = k_strings_K();
        let jj = k_strings_J();
        if v8 == 3 {
            g.set_color(0, 0, 0);
            g.fill_rect(10, 59, 51, 20);
            g.set_color(255, 204, 4);
            g.draw_rect(10, 59, 51, 20);
            g.set_color(255, 255, 255);
            g.draw_string(jj.get(v3 as usize).ok_or(crate::midp::gfx::AIOOBE)?, 18, 65, TL);
            Self::k_chevron(g, 1, 67);
        } else {
            Self::k_chevron(g, 0, 47);
            Self::k_chevron(g, 1, 52);
        }
        if self.st.k_y < 20 {
            self.st.k_y += 1;
        } else {
            self.st.k_y = 0;
        }
        g.set_color(255, 255, 255);
        g.draw_string(kk.get(v4 as usize).ok_or(crate::midp::gfx::AIOOBE)?, 8, 45, TL);
        g.draw_image(&self.img.k_j, 60, 39, TL)?;
        g.set_color(0, 0, 0);
        g.fill_rect(22, 83, 100, 18);
        g.set_color(255, 0, 254);
        g.draw_rect(22, 83, 100, 18);
        if self.st.J_a > 0 {
            g.set_color(255, 255, 255);
            g.draw_string(&format!("{}: {}", lang::s(82), v5), 26, 88, TL);
            self.k_num3(g, 111, 43, v6)?;
            g.draw_image(&self.img.k_ad, 73, 52, TL)?;
            self.J_draw(g, self.st.k_m, 77, 55)?;
        }
        g.draw_image(&self.img.k_ag[0], 0, 128, BL)?;
        g.set_color(11, 120, 0);
        g.draw_string(&lang::s(48), 3, 108, TL);
        g.set_color(24, 255, 0);
        g.draw_string(&lang::s(48), 2, 107, TL);
        Ok(())
    }

    /// `c(Graphics)`: pilot information screen
    pub fn k_pilot_screen(&mut self, g: &mut Graphics) -> Res {
        self.k_bg(g)?;
        self.k_scroll();
        self.k_pilot(g, self.st.k_m)?;
        g.set_color(11, 120, 0);
        g.draw_string(&lang::s(67), 3, 111, TL);
        g.set_color(24, 255, 0);
        g.draw_string(&lang::s(67), 2, 110, TL);
        self.k_score(g, self.st.j_d)
    }

    /// `a(Graphics)`: spider information screen
    pub fn k_spider_screen(&mut self, g: &mut Graphics) -> Res {
        self.k_bg(g)?;
        self.k_scroll();
        self.k_score(g, self.st.j_d)?;
        g.set_color(11, 120, 0);
        g.draw_string(&lang::s(77), 3, 103, TL);
        g.set_color(24, 255, 0);
        g.draw_string(&lang::s(77), 2, 102, TL);
        self.kc.w.set_size(1);
        self.kc.w.l = 41;
        self.kc.w.k = 41;
        self.kc.w.draw_info(g, &self.img)?;
        g.draw_image(&self.img.k_p, 102, 26, TL)?;
        if self.st.I_a != -1 {
            self.J_draw(g, self.st.I_a, 102, 27)?;
        }
        g.draw_image(&self.img.k_e, 78, 23, TL)?;
        self.k_score(g, self.st.j_d)
    }

    /// `paint(Graphics)`
    pub fn k_paint(&mut self, g: &mut Graphics) {
        let _ = self.k_paint_inner(g);
    }

    fn k_back_target(&self) -> i32 {
        if !self.ps.z && self.st.k_c != 6 && self.st.k_c != 21 {
            24
        } else {
            23
        }
    }

    fn k_paint_inner(&mut self, g: &mut Graphics) -> Res {
        match self.st.k_W {
            0 => {
                g.draw_image(&self.img.k_i, 0, 0, TL)?;
                if self.st.k_y == 0 {
                    self.kc.F.tune(&mut self.audio, 0);
                }
                if self.kc.F.is_closed(&self.audio)? {
                    self.st.k_y = -1;
                    self.img.k_i = None;
                    self.k_load();
                    self.st.k_W = 23;
                }
                self.st.k_y += 1;
            }
            1 | 3 => {
                let (m, e, u, w, jd) = (self.st.k_m, self.st.k_E, self.st.k_U, self.st.k_W, self.st.j_d);
                if self.st.J_a > 0 {
                    let name = self.J_name(m)?;
                    let lvl = self.eye(m)?.d;
                    self.k_eyelist(g, m, e, u, &name, lvl, jd, w)?;
                } else {
                    self.k_eyelist(g, m, e, u, "", 0, jd, w)?;
                }
                self.k_top_strip(g)?;
            }
            2 => {
                if self.st.k_E == 0 {
                    self.k_pilot_screen(g)?;
                } else {
                    self.k_spider_screen(g)?;
                }
                if self.st.k_y < 10 {
                    self.st.k_y += 1;
                } else {
                    self.st.k_y = 0;
                    self.st.k_W = 24;
                }
            }
            4 => self.k_pilot_screen(g)?,
            5 => self.k_spider_screen(g)?,
            6 => self.u_paint(g)?,
            8 => {
                self.k_new_mission(g)?;
                if self.st.k_y < 10 {
                    self.st.k_y += 1;
                } else {
                    self.k_roll_enemy()?;
                    self.st.ps_G = false;
                    self.ps.z = false;
                    self.ps.w = false;
                    self.st.k_W = 25;
                    self.st.k_y = 0;
                }
            }
            13 => {
                self.k_bg(g)?;
                self.k_scroll();
                g.draw_image(&self.img.k_ag[1], 0, 0, TL)?;
                self.k_score(g, self.st.j_d)?;
                Self::k_arrow(g, 0, 21, 0);
                Self::k_arrow(g, 120, 21, 1);
                let ja = self.st.J_a;
                let m = self.st.k_m;
                let d0 = self.kc.d[0];
                if ja < 6 {
                    for v8 in 0..ja {
                        self.J_draw(g, v8, 9 + v8 * 21, 15)?;
                        if v8 == d0 {
                            g.draw_image(&self.img.k_A, 9 + v8 * 21, 15, TL)?;
                        }
                        self.k_eyesel(g, 8 + v8 * 21, 15, 1, v8, m)?;
                    }
                } else if m < 5 {
                    for v7 in 0..5 {
                        self.J_draw(g, v7, 9 + v7 * 21, 15)?;
                        if v7 == d0 {
                            g.draw_image(&self.img.k_A, 9 + v7 * 21, 15, TL)?;
                        }
                        self.k_eyesel(g, 8 + v7 * 21, 14, 1, v7, m)?;
                    }
                } else {
                    for v6 in m - 4..m + 1 {
                        let x = 9 + (v6 - (m - 4)) * 21;
                        self.J_draw(g, v6, x, 15)?;
                        if v6 == d0 {
                            g.draw_image(&self.img.k_A, x, 15, TL)?;
                        }
                        self.k_eyesel(g, 92, 14, 1, v6, v6)?;
                    }
                }
                g.set_color(0, 0, 0);
                g.fill_rect(3, 39, 69, 20);
                g.set_color(255, 204, 4);
                g.draw_rect(3, 39, 69, 20);
                g.set_color(255, 255, 255);
                g.draw_string(&k_strings_K()[0], 8, 45, TL);
                g.draw_image(&self.img.k_j, 60, 39, TL)?;
                g.set_color(0, 0, 0);
                g.fill_rect(0, 83, 126, 18);
                g.set_color(255, 0, 254);
                g.draw_rect(0, 83, 126, 18);
                if self.st.J_a > 0 {
                    g.set_color(255, 255, 255);
                    let ok = Self::J_can_combine(self.eye(d0)?, self.eye(m)?);
                    g.draw_string(&lang::s(if ok { 20 } else { 21 }), 64, 88, TC);
                    self.k_num3(g, 111, 43, self.eye(m)?.d)?;
                    g.draw_image(&self.img.k_ad, 73, 52, TL)?;
                    self.J_draw(g, m, 77, 55)?;
                }
                g.set_color(11, 120, 0);
                g.draw_string(&lang::s(48), 3, 108, TL);
                g.set_color(24, 255, 0);
                g.draw_string(&lang::s(48), 2, 107, TL);
                g.draw_image(&self.img.k_ag[0], 0, 128, BL)?;
            }
            14 => {
                let v2 = [self.kc.d[0], self.st.k_m];
                let a = self.eye(v2[0])?.clone();
                let b = self.eye(v2[1])?.clone();
                if Self::J_can_combine(&a, &b) {
                    let v13 = Self::J_combine(&a, &b);
                    self.kc.L = v13.a;
                    self.kc.d[1] = v13.b;
                }
                self.st.k_y = 0;
                self.st.k_W = 15;
            }
            15 => self.k_combine_anim(g)?,
            16 => {
                self.k_bg(g)?;
                self.k_scroll();
                g.draw_image(&self.img.k_ag[1], 0, 0, TL)?;
                self.k_score(g, self.st.j_d)?;
                let ja = self.st.J_a;
                let m = self.st.k_m;
                if ja < 6 {
                    for v12 in 0..ja {
                        self.J_draw(g, v12, 9 + v12 * 21, 15)?;
                        if v12 == ja - 1 {
                            g.draw_image(&self.img.k_A, 9 + v12 * 21, 15, TL)?;
                        }
                        self.k_eyesel(g, 8 + v12 * 21, 14, 1, v12, m)?;
                    }
                } else if m < 5 {
                    for v11 in 0..5 {
                        self.J_draw(g, v11, 9 + v11 * 21, 15)?;
                        if v11 == ja - 1 {
                            g.draw_image(&self.img.k_A, 9 + v11 * 21, 15, TL)?;
                        }
                        self.k_eyesel(g, 8 + v11 * 21, 14, 1, v11, m)?;
                    }
                } else {
                    for v10 in m - 4..m + 1 {
                        let x = 9 + (v10 - (m - 4)) * 21;
                        self.J_draw(g, v10, x, 15)?;
                        if v10 == ja - 1 {
                            g.draw_image(&self.img.k_A, x, 15, TL)?;
                        }
                        self.k_eyesel(g, 92, 14, 1, v10, v10)?;
                    }
                }
                g.draw_image(&self.img.k_j, 30, 39, TL)?;
                g.set_color(0, 0, 0);
                g.fill_rect(7, 83, 100, 18);
                g.set_color(255, 0, 254);
                g.draw_rect(7, 83, 100, 18);
                if self.st.J_a > 0 {
                    g.set_color(255, 255, 255);
                    g.draw_string(&format!("{}: {}", lang::s(82), self.J_name(m)?), 11, 88, TL);
                    self.k_num3(g, 81, 43, self.eye(m)?.d)?;
                    g.draw_image(&self.img.k_ad, 43, 52, TL)?;
                    self.J_draw(g, m, 47, 55)?;
                }
                g.set_color(11, 120, 0);
                g.draw_string(&lang::s(48), 3, 108, TL);
                g.set_color(24, 255, 0);
                g.draw_string(&lang::s(48), 2, 107, TL);
                g.draw_image(&self.img.k_ag[0], 0, 128, BL)?;
                if self.st.k_y < 15 {
                    self.st.k_y += 1;
                } else {
                    self.st.k_y = 0;
                    self.st.k_W = 1;
                }
            }
            17 => {
                self.k_scroll();
                self.k_bg(g)?;
                g.set_color(16, 0, 56);
                for v9 in 0..15 {
                    g.draw_line(v9 * 9, 0, v9 * 9, 128);
                    g.draw_line(0, v9 * 9, 128, v9 * 9);
                }
                g.set_color(0, 0, 0);
                g.fill_rect(0, 0, 128, 11);
                g.fill_rect(0, 117, 128, 11);
                g.draw_image(&self.img.k_ag[0], 0, 0, TL)?;
                g.draw_image(&self.img.k_ag[1], 0, 128, BL)?;
                g.set_color(0, 0, 0);
                g.fill_rect(9, 49, 111, 19);
                g.set_color(255, 204, 0);
                g.draw_rect(9, 49, 111, 19);
                g.set_color(255, 255, 255);
                g.draw_string(&lang::s(21), 64, 55, TC);
                if self.st.k_y < 15 {
                    self.st.k_y += 1;
                } else {
                    self.st.k_y = 0;
                    self.st.k_W = 1;
                }
            }
            18 => {
                if self.st.k_y < 20 {
                    let (a, c, j, b) = (self.st.I_a, self.st.I_c, self.st.E_j, self.st.E_b);
                    self.k_vs(g, a, c, j, b)?;
                    self.st.k_y += 1;
                } else {
                    let e = self.eye(self.st.I_a)?.clone();
                    let r = Self::J_battle(e.b, e.d, self.st.E_j, self.st.E_b);
                    self.ps_e_result(r);
                    self.st.k_W = 22;
                }
            }
            19 => {
                let (j, b) = (self.st.E_j, self.st.E_b);
                self.k_full(g, j, b)?;
            }
            21 => self.k_highscores(g)?,
            22 => {
                let e = self.eye(self.st.I_a)?.clone();
                match Self::J_battle(e.b, e.d, self.st.E_j, self.st.E_b) {
                    0 => {
                        if self.eyes.len() < 30 {
                            if !self.st.E_e {
                                let (j, b) = (self.st.E_j, self.st.E_b);
                                self.k_add_eye(j, b);
                            }
                            self.st.E_e = true;
                            self.st.E_i = false;
                            self.st.k_W = self.k_back_target();
                        } else {
                            self.st.k_m = 0;
                            self.st.k_W = 19;
                        }
                    }
                    1 => {
                        let a = self.st.I_a;
                        self.k_del_eye(a);
                        self.st.E_i = false;
                        self.st.k_W = self.k_back_target();
                    }
                    2 => {
                        self.st.E_i = false;
                        self.st.k_W = self.k_back_target();
                    }
                    _ => {}
                }
            }
            23 => {
                let q = self.st.k_q;
                self.ps_b(q);
            }
            24 => {
                let t = self.st.k_T;
                self.ps_a(t);
            }
            25 => self.ps_i(),
            26 => {
                if self.st.u_m {
                    self.st.u_b += 1;
                    self.st.u_m = false;
                }
                let b = self.st.u_b;
                self.ps_d(b);
            }
            27 => {
                self.ps_c();
                self.st.k_W = 28;
            }
            28 => self.st.k_W = self.k_back_target(),
            _ => {}
        }
        Ok(())
    }

    /// case 21: high scores
    fn k_highscores(&mut self, g: &mut Graphics) -> Res {
        g.set_color(0, 0, 0);
        g.fill_rect(0, 0, 128, 128);
        let mut v3 = 0;
        while v3 < 148 {
            let mut v4 = 0;
            while v4 < 148 {
                self.k_tile(g, v4, v3, 0)?;
                v4 += 26;
            }
            v3 += 26;
        }
        g.draw_image(&self.img.k_ag[0], 0, 0, TL)?;
        g.draw_image(&self.img.k_ag[1], 0, 128, BL)?;
        let x = self.st.k_X;
        let u = self.st.ps_u;
        let blue = |g: &mut Graphics, x0: i32, y: i32, w: i32| {
            g.set_color(0, 165, 255);
            g.fill_rect(x0, y, w, 1);
            g.fill_rect(x0, y + 12, w, 1);
        };
        // three rows, each sweeping in from the right
        let rows = [(0, 14, 18, 21), (10, 41, 45, 49), (20, 68, 73, 77)];
        let mut done_all = true;
        for (ri, &(base, py, ly, ty)) in rows.iter().enumerate() {
            let xx = x - base;
            if xx < 0 {
                done_all = false;
                break;
            }
            if xx == 0 || xx == 1 {
                self.kc.M.draw_at(g, 112, py);
            } else if xx == 2 || xx == 3 {
                blue(g, 112, ly, 16);
                self.kc.M.draw_at(g, 91, py);
            } else if xx == 4 || xx == 5 {
                blue(g, 76, ly, 52);
                self.kc.M.draw_at(g, 56, py);
            } else if xx == 6 || xx == 7 {
                blue(g, 37, ly, 91);
                self.kc.M.draw_at(g, 17, py);
            } else {
                blue(g, 0, ly, 128);
                g.set_color(255, 255, 255);
                g.draw_string(&u[ri].to_string(), 64, ty, TC);
                continue;
            }
            done_all = false;
            break;
        }
        if done_all {
            let y = self.st.k_y;
            if x == 30 || x == 31 {
                g.set_clip(63, 110, 62, 43);
                g.draw_image(&self.img.k_H, 63, 110, TL)?;
                g.reset_clip();
            } else if x == 32 || x == 33 {
                g.set_clip(63, 89, 62, 43);
                g.draw_image(&self.img.k_H, 63, 89, TL)?;
                g.reset_clip();
            } else if x >= 34 {
                g.set_clip(63, 75 + y % 2, 62, 43);
                g.draw_image(&self.img.k_H, 63, 75 + y % 2, TL)?;
                g.reset_clip();
            }
        }
        if self.st.k_X < 35 {
            self.st.k_X += 1;
        }
        if self.st.k_y < 10 {
            self.st.k_y += 1;
        } else {
            self.st.k_y = 0;
        }
        Ok(())
    }

    /// `a(int, int)`: add an eye
    pub fn k_add_eye(&mut self, v1: i32, v2: i32) {
        let _ = Eye::new(v1, v2);
        self.J_add(v1, v2);
    }

    /// `a(int)`: delete an eye
    fn k_del_eye(&mut self, v1: i32) {
        if v1 != -1 {
            self.J_remove(&[v1], 1);
        }
    }

    /// `a(int key, int code, String buf)`: 4-digit cheat matcher
    fn k_cheat(&mut self, v1: i32, v2: i32, which: u8) -> bool {
        let mut v4 = false;
        let mut v3 = match which {
            0 => std::mem::take(&mut self.kc.Q),
            1 => std::mem::take(&mut self.kc.t),
            _ => std::mem::take(&mut self.kc.v),
        };
        if !(48..=57).contains(&v1) {
            if v3.len() == 4 {
                if v3.parse::<i32>().ok() == Some(v2) {
                    self.st.k_y = 0;
                    v4 = true;
                }
                v3.clear();
            }
        } else {
            let dgt = v1 - 48;
            let want = match v3.len() {
                0 => Some(v2 / 1000 % 10),
                1 => Some(v2 / 100 % 10),
                2 => Some(v2 / 10 % 10),
                3 => Some(v2 % 10),
                _ => None,
            };
            if let Some(want) = want {
                if want == dgt {
                    v3.push_str(&dgt.to_string());
                    if v3.len() == 4 {
                        if v3.parse::<i32>().ok() == Some(v2) {
                            self.st.k_y = 0;
                            v4 = true;
                        }
                        v3.clear();
                    }
                } else {
                    v3.clear();
                }
            }
        }
        match which {
            0 => self.kc.Q = v3,
            1 => self.kc.t = v3,
            _ => self.kc.v = v3,
        }
        v4
    }

    /// `keyPressed`
    pub fn k_key_pressed(&mut self, v1: i32) {
        let _ = self.k_key_inner(v1);
    }

    fn k_key_inner(&mut self, v1: i32) -> Res {
        let v2 = game_action(v1);
        match self.st.k_W {
            0 => {
                self.st.k_y = 0;
                self.img.k_i = None;
                self.k_load();
                self.st.k_W = 23;
            }
            1 => {
                if v1 == K_FIRE {
                    if self.st.k_U == 1 && self.st.J_a > 0 {
                        self.k_c(self.st.k_m)?;
                        self.st.k_E = 0;
                        self.st.k_W = 3;
                    } else if self.st.k_U == 0 && self.st.J_a > 1 {
                        self.kc.d[0] = self.st.k_m;
                        self.st.k_m = if self.kc.d[0] == 0 { 1 } else { 0 };
                        self.st.k_W = 13;
                    }
                    self.kc.Q.clear();
                } else if v1 == K_SOFT_R {
                    self.kc.Q.clear();
                    self.st.k_W = 24;
                } else {
                    self.kc.x = self.k_cheat(v1, 1234, 0);
                    if self.kc.x {
                        self.ps_cheat_add_eye();
                    } else {
                        let ja = self.st.J_a;
                        match v2 {
                            1 | 6 => self.st.k_U = if self.st.k_U == 0 { 1 } else { 0 },
                            2 => {
                                if ja > 0 {
                                    self.st.k_m = if self.st.k_m > 0 { self.st.k_m - 1 } else { ja - 1 };
                                }
                            }
                            5 => {
                                if ja > 0 {
                                    self.st.k_m = if self.st.k_m < ja - 1 { self.st.k_m + 1 } else { 0 };
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            2 => {
                if v1 == K_SOFT_R {
                    self.st.k_y = 0;
                    self.st.k_W = 24;
                }
            }
            3 => {
                if v1 == K_FIRE {
                    if self.st.k_E == 0 {
                        self.st.j_f = self.st.k_m;
                        self.j_b()?;
                        self.st.k_W = 2;
                        self.st.k_U = 0;
                        self.st.k_y = 0;
                    } else if self.eye(self.st.k_m)?.b <= 4 {
                        let e = self.eye(self.st.k_m)?.clone();
                        self.st.I_a = self.st.k_m;
                        self.st.I_c = e.d;
                        self.st.I_b = e.b;
                        self.st.k_W = 2;
                        self.st.k_U = 0;
                        self.st.k_y = 0;
                    }
                } else if v1 == K_SOFT_R {
                    self.st.k_W = 1;
                } else if v2 == 6 || v1 == 56 {
                    if self.eye(self.st.k_m)?.b <= 4 {
                        self.st.k_E = if self.st.k_E == 0 { 1 } else { 0 };
                    } else {
                        self.st.k_E = 0;
                    }
                }
            }
            4 | 5 | 17 => {
                if v1 == K_SOFT_R {
                    self.st.k_W = 24;
                }
            }
            6 => {
                if v1 == K_FIRE {
                    if self.st.u_b < 4 {
                        self.st.u_m = true;
                        self.st.k_W = 26;
                    } else {
                        self.kc.F.close(&mut self.audio);
                        self.st.k_W = 23;
                    }
                } else if v1 == K_SOFT_R {
                    let b = self.st.u_b;
                    self.ps_d(b);
                }
            }
            8 => {
                if v1 == K_FIRE {
                    self.k_roll_enemy()?;
                    self.st.ps_G = false;
                    self.ps.z = false;
                    self.ps.w = false;
                    self.st.k_W = 25;
                } else if v1 == K_SOFT_R {
                    self.kc.Q.clear();
                    self.kc.v.clear();
                    self.st.k_W = 24;
                } else {
                    self.kc.x = self.k_cheat(v1, 1234, 0);
                    self.kc.C = self.k_cheat(v1, 2345, 1);
                    self.kc.D = self.k_cheat(v1, 6789, 2);
                    if self.kc.x {
                        self.kc.Q.clear();
                        self.kc.t.clear();
                        self.kc.v.clear();
                        self.ps_cheat_pilot();
                    } else if self.kc.C {
                        self.kc.Q.clear();
                        self.kc.t.clear();
                        self.kc.v.clear();
                        self.ps_cheat_level();
                    } else if self.kc.D {
                        self.kc.Q.clear();
                        self.kc.t.clear();
                        self.kc.v.clear();
                        self.ps_cheat_spider();
                    } else if v1 != 54 && v2 != 2 {
                        if v1 == 52 || v2 == 5 {
                            self.st.k_B = if self.st.k_B > 0 { self.st.k_B - 1 } else { 2 };
                        }
                    } else {
                        self.st.k_B = if self.st.k_B < 2 { self.st.k_B + 1 } else { 0 };
                    }
                }
            }
            13 => {
                if v1 == K_FIRE {
                    if Self::J_can_combine(self.eye(self.kc.d[0])?, self.eye(self.st.k_m)?) {
                        self.st.k_y = 0;
                        self.st.k_W = 14;
                    }
                } else if v1 == K_SOFT_R {
                    self.st.k_W = 1;
                    self.st.k_m = self.kc.d[0];
                } else {
                    let ja = self.st.J_a;
                    let d0 = self.kc.d[0];
                    match v2 {
                        2 => {
                            if ja > 0 {
                                self.st.k_m = if self.st.k_m > 0 { self.st.k_m - 1 } else { ja - 1 };
                                if self.st.k_m == d0 {
                                    self.st.k_m = if self.st.k_m > 0 { self.st.k_m - 1 } else { ja - 1 };
                                }
                            }
                        }
                        5 => {
                            if ja > 0 {
                                self.st.k_m = if self.st.k_m < ja - 1 { self.st.k_m + 1 } else { 0 };
                                if self.st.k_m == d0 {
                                    self.st.k_m = if self.st.k_m < ja - 1 { self.st.k_m + 1 } else { 0 };
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            16 => {
                if v1 == K_SOFT_R {
                    self.st.k_y = 0;
                    self.st.k_W = 1;
                }
            }
            19 => {
                if v1 == K_SOFT_R {
                    self.st.k_W = self.k_back_target();
                    self.st.E_i = false;
                } else if v1 == K_FIRE {
                    let m = self.st.k_m;
                    self.k_del_eye(m);
                    let (j, b) = (self.st.E_j, self.st.E_b);
                    self.k_add_eye(j, b);
                    self.st.E_i = false;
                    self.st.k_W = self.k_back_target();
                } else {
                    let ja = self.st.J_a;
                    match v2 {
                        2 => {
                            if ja > 0 {
                                self.st.k_m = if self.st.k_m > 0 { self.st.k_m - 1 } else { ja - 1 };
                            }
                        }
                        5 => {
                            if ja > 0 {
                                self.st.k_m = if self.st.k_m < ja - 1 { self.st.k_m + 1 } else { 0 };
                            }
                        }
                        _ => {}
                    }
                }
            }
            21 => {
                if v1 == K_SOFT_R {
                    self.st.k_W = 23;
                    self.kc.F.close(&mut self.audio);
                }
            }
            _ => {}
        }
        Ok(())
    }
}
