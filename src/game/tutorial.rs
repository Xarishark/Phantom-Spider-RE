//! `u`: animated instruction pages.

use super::ents::Ent;
use super::eyes::v_draw;
use super::kcanvas::k_strings_J;
use super::sfx::SFx;
use super::spider::{Plane, Z};
use super::{hit, rnd, Game, Images};
use crate::gfx::{Graphics, Res, LEFT, TOP};
use crate::lang;

const TL: i32 = TOP | LEFT;
const BL: i32 = 32 | LEFT;

pub struct Tut {
    pub c: Plane,
    pub n: Z,
    pub k: Ent,
    pub f: Ent,
    pub j: SFx,
}

impl Tut {
    /// `new u(midlet, k)`
    pub fn new(img: &mut Images) -> Tut {
        let mut c = Plane::new(img);
        let n = Z::new(img);
        // `new G()` inside u only loads images
        let _ = super::battle::Battle::new(img);
        c.l = 46;
        c.k = 80;
        c.set_stats(100, 5, 20, 20);
        let mut k = Ent::new_egg(5);
        k.load(img);
        let mut f = Ent::new_g(13);
        f.load(img);
        let j = SFx::new(img);
        Tut { c, n, k, f, j }
    }
}

impl Game {
    /// `u.a(Graphics)`
    pub fn u_paint(&mut self, g: &mut Graphics) -> Res {
        let v2 = [lang::s(86), lang::s(54), lang::s(50), lang::s(87)];
        let v3 = [lang::s(20), lang::s(21)];
        let b = self.st.u_b;
        if b == 2 {
            g.set_color(0, 0, 0);
            g.fill_rect(0, 0, 128, 128);
            self.k_spider_screen(g)?;
            v_draw(g, &self.img.v_c, 0, 103, 27, 1)?;
            g.draw_image(&self.img.k_e, 78, 23, TL)?;
            if self.st.u_h < 16 {
                g.set_color(255, 0, 0);
                if self.st.u_h % 4 == 0 {
                    g.fill_triangle(110, 51, 105, 56, 115, 56);
                }
                self.st.u_h += 1;
            } else {
                self.st.u_h = 0;
                self.st.u_m = true;
                self.st.k_W = 26;
            }
        } else if b == 1 {
            g.set_color(0, 0, 0);
            g.fill_rect(0, 0, 128, 128);
            self.k_pilot(g, -1)?;
            v_draw(g, &self.img.v_c, 0, 102, 25, 1)?;
            g.draw_image(&self.img.k_e, 77, 22, TL)?;
            if self.st.u_h < 30 {
                self.st.u_h += 1;
                let h = self.st.u_h;
                g.set_color(255, 0, 0);
                if h < 4 {
                    g.fill_triangle(7, 26, 2, 31, 12, 31);
                } else if h < 8 {
                    g.fill_triangle(52, 51, 52, 61, 57, 56);
                } else if h < 12 {
                    g.fill_triangle(52, 62, 52, 74, 57, 69);
                } else if h < 16 {
                    g.fill_triangle(52, 77, 52, 87, 57, 82);
                } else if h < 20 {
                    g.fill_triangle(52, 90, 52, 100, 57, 94);
                } else if h < 30 {
                    g.fill_triangle(111, 48, 106, 53, 116, 53);
                }
            } else {
                self.st.u_h = 0;
                self.st.u_m = true;
                self.st.k_W = 26;
            }
            g.set_color(11, 120, 0);
            g.draw_string(&lang::s(67), 3, 111, TL);
            g.set_color(24, 255, 0);
            g.draw_string(&lang::s(67), 2, 110, TL);
        } else if b == 0 {
            if self.st.u_h == 0 {
                self.tut.k.fire(41, 15, 0);
                self.tut.f.fire(52, 70, 1);
            }
            g.set_color(0, 0, 0);
            g.fill_rect(0, 0, 128, 128);
            g.draw_image(&self.img.k_ag[0], 0, 0, TL)?;
            g.draw_image(&self.img.k_ag[1], 0, 128, BL)?;
            self.tut.n.set_size(1);
            self.tut.n.l = 41;
            self.tut.n.k = 5;
            let (nx, ny) = (self.tut.n.l + rnd(5), self.tut.n.k + rnd(5));
            self.tut.n.draw_at(g, &self.img, nx, ny)?;
            self.k_score(g, 100)?;
            v_draw(g, &self.img.v_c, 0, 26, 5, 1)?;
            g.draw_image(&self.img.k_e, 1, 1, TL)?;
            g.set_color(255, 0, 213);
            g.fill_rect(17, 120, 10, 5);
            self.tut.c.l = 46 + rnd(5);
            self.tut.c.k = 80 + rnd(5);
            self.tut.c.draw(g)?;
            g.draw_image(&self.img.k_a, 0, 117, TL)?;
            self.tut.k.draw(g, &self.img)?;
            self.tut.f.draw(g, &self.img)?;
            self.tut.j.draw(g, &self.img, &self.st)?;
            let lh = self.st.L_H;
            self.tut.k.update(lh, &self.st);
            self.tut.f.update(lh, &self.st);
            let t = &self.tut;
            if hit(t.f.g, t.f.e, 1, 1, t.n.l, t.n.k, 50, 42) {
                let (x, y) = (self.tut.f.g, self.tut.f.e);
                self.tut.j.add(&mut self.st, 0, x, y, 3);
            }
            let t = &self.tut;
            if hit(t.k.g, t.k.e, 1, 1, t.c.l, t.c.k, 22, 20) {
                // the original passes (e, g) here, swapped
                let (x, y) = (self.tut.k.e, self.tut.k.g);
                self.tut.j.add(&mut self.st, 0, x, y, 3);
            }
            if !self.tut.f.d {
                let (x, y) = (self.tut.c.l + 6, self.tut.c.k - 10);
                self.tut.f.fire(x, y, 0);
            }
            if !self.tut.k.d {
                let (x, y) = (self.tut.n.l + 20, self.tut.n.k + 10);
                self.tut.k.fire(x, y, 0);
            }
            if self.st.u_h < 15 {
                self.st.u_h += 1;
            } else {
                self.st.u_h = 0;
                self.st.u_l = 0;
                self.st.u_m = true;
                self.st.k_W = 26;
            }
        } else if b == 3 {
            let d = self.st.u_d;
            self.k_eyelist(g, 0, 0, d, &v2[(d % 2) as usize].clone(), 1, 10, 1)?;
            let (h, l) = (self.st.u_h, self.st.u_l);
            if h <= 30 {
                v_draw(g, &self.img.v_c, 0, 10, 15, 1)?;
                v_draw(g, &self.img.v_c, 1, 31, 15, 1)?;
                v_draw(g, &self.img.v_c, 2, 52, 15, 1)?;
                self.k_eyesel(g, 8, 14, 1, 0, l % 3)?;
                self.k_eyesel(g, 28, 14, 1, 1, l % 3)?;
                self.k_eyesel(g, 49, 14, 1, 2, l % 3)?;
                g.set_color(0, 0, 0);
                g.fill_rect(23, 84, 98, 16);
                g.set_color(255, 255, 255);
                g.draw_string(&format!("{}: {}", lang::s(82), v2[(l % 3) as usize]), 26, 88, TL);
                g.draw_image(&self.img.k_ad, 73, 52, TL)?;
                self.k_num3(g, 111, 43, 1)?;
                v_draw(g, &self.img.v_c, l % 3, 77, 55, l % 3)?;
            } else if h < 34 {
                v_draw(g, &self.img.v_c, 0, 10, 15, 1)?;
                v_draw(g, &self.img.v_c, 1, 31, 15, 1)?;
                v_draw(g, &self.img.v_c, 2, 52, 15, 1)?;
                self.k_eyesel(g, 8, 14, 1, 0, l % 3)?;
                self.k_eyesel(g, 28, 14, 1, 1, l % 3)?;
                self.k_eyesel(g, 49, 14, 1, 2, l % 3)?;
                g.draw_image(&self.img.k_A, 9, 15, TL)?;
                g.draw_image(&self.img.k_A, 29, 15, TL)?;
                g.set_color(0, 0, 0);
                g.fill_rect(23, 84, 98, 16);
                g.set_color(255, 255, 255);
                g.draw_string(&v3[0], 26, 88, TL);
                g.draw_image(&self.img.k_ad, 73, 52, TL)?;
                self.k_level2(g, 111, 43, 1)?;
                v_draw(g, &self.img.v_c, 1, 77, 55, 1)?;
            } else {
                v_draw(g, &self.img.v_c, 2, 10, 15, 1)?;
                v_draw(g, &self.img.v_c, 6, 31, 15, 1)?;
                self.k_eyesel(g, 8, 14, 1, 0, 0)?;
                self.k_eyesel(g, 28, 14, 1, 1, 0)?;
                g.set_color(0, 0, 0);
                g.fill_rect(23, 84, 98, 16);
                g.set_color(255, 255, 255);
                g.draw_string(&format!("{}: {}", lang::s(82), v2[2]), 26, 88, TL);
                g.draw_image(&self.img.k_ad, 73, 52, TL)?;
                self.k_level2(g, 111, 43, 1)?;
                v_draw(g, &self.img.v_c, 2, 77, 55, 2)?;
            }
            if self.st.u_d == 1 {
                g.set_color(255, 204, 4);
                g.draw_rect(10, 59, 51, 20);
                g.set_color(255, 255, 255);
                let jj = k_strings_J();
                let h = self.st.u_h;
                g.draw_string(if h % 4 != 0 && h % 4 != 1 { &jj[1] } else { &jj[0] }, 18, 65, TL);
                Self::k_chevron(g, 1, 67);
            }
            if self.st.u_h < 40 {
                self.st.u_h += 1;
            } else {
                self.st.u_h = 0;
                self.st.u_l = 0;
                self.st.u_d = 1;
                self.st.u_m = true;
                self.st.k_W = 26;
            }
            let h = self.st.u_h;
            if h < 22 {
                if h % 4 == 0 {
                    if self.st.u_l < 4 && h < 16 {
                        self.st.u_l += 1;
                    } else {
                        self.st.u_l = 0;
                    }
                    if h >= 16 {
                        self.st.u_d = 1;
                    }
                } else {
                    g.set_color(255, 0, 0);
                    if h < 16 {
                        g.fill_triangle(118, 30, 113, 35, 122, 35);
                    } else {
                        g.fill_triangle(47, 70, 42, 75, 53, 75);
                    }
                }
            } else if h < 26 {
                self.st.u_d = 0;
            } else if h < 28 {
                g.set_color(255, 0, 0);
                g.fill_triangle(47, 60, 42, 65, 53, 65);
            }
        } else if b == 4 {
            self.k_vs(g, -1, 100, 1, 10)?;
            v_draw(g, &self.img.v_c, 1, 18, 14, 100)?;
            g.set_color(255, 0, 0);
            let h = self.st.u_h;
            if h < 4 {
                g.fill_triangle(39, 50, 34, 55, 44, 55);
            } else if h < 8 {
                g.fill_triangle(27, 33, 22, 38, 32, 38);
            } else if h < 12 {
                g.fill_triangle(58, 15, 53, 20, 63, 20);
            } else if h < 16 {
                g.fill_triangle(114, 50, 109, 55, 119, 55);
            } else if h < 20 {
                g.fill_triangle(91, 33, 86, 38, 96, 38);
            } else if h < 24 {
                g.fill_triangle(117, 15, 112, 20, 122, 20);
            }
            if h < 28 {
                self.st.u_h += 1;
            } else {
                self.st.u_h = 0;
                self.st.k_W = 23;
            }
        }
        Ok(())
    }
}
