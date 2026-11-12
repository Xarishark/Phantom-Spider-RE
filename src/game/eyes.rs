//! `J` (eye collection), `v` (eye) and `j` (pilot stats).

use super::{load, Eye, Game};
use crate::midp::gfx::{Graphics, Img, Res, AIOOBE, LEFT, TOP};
use crate::midp::lang;

const TL: i32 = TOP | LEFT;

impl Eye {
    /// `new v(type, level)`
    pub fn new(b: i32, d: i32) -> Eye {
        let a = match b {
            0 => lang::s(86),
            1 => lang::s(54),
            2 => lang::s(50),
            3 => lang::s(43),
            4 => lang::s(38),
            5 => lang::s(39),
            6 => lang::s(87),
            7 => lang::s(55),
            8 => lang::s(51),
            9 => lang::s(44),
            _ => String::new(),
        };
        Eye { b, d, a }
    }
}

/// `v.a(Graphics, type, x, y, level)` (static)
pub fn v_draw(g: &mut Graphics, vc: &[Img; 10], v1: i32, v2: i32, v3: i32, v4: i32) -> Res {
    g.set_clip(v2, v3, 19, 20);
    if v1 > 4 {
        g.draw_image(&vc[5], v2 - (v1 - 5) * 19, v3, TL)?;
    } else {
        let img = vc.get(v1.max(0) as usize).ok_or(AIOOBE)?;
        if v1 < 0 {
            return Err(AIOOBE);
        }
        g.draw_image(img, v2 - v4 / 60 * 19, v3, TL)?;
    }
    g.reset_clip();
    Ok(())
}

impl Game {
    pub fn eye(&self, i: i32) -> Res<&Eye> {
        if i < 0 {
            return Err(AIOOBE);
        }
        self.eyes.get(i as usize).ok_or(AIOOBE)
    }
    pub fn eye_mut(&mut self, i: i32) -> Res<&mut Eye> {
        if i < 0 {
            return Err(AIOOBE);
        }
        self.eyes.get_mut(i as usize).ok_or(AIOOBE)
    }

    /// `v.a()` static loader
    pub fn v_load(&mut self) {
        for i in 0..6 {
            if i > 4 {
                self.img.v_c[5] = load("/eyes.png");
            } else {
                self.img.v_c[i] = load(&format!("/attribute/eye/eye{i}.png"));
            }
        }
    }

    /// `new J()` -> `J.b()`
    pub fn J_new(&mut self) {
        self.v_load();
        self.st.J_a = 0;
    }

    /// `J.a(int, int)`: add an eye
    pub fn J_add(&mut self, v1: i32, v2: i32) -> bool {
        if self.st.J_a >= 30 {
            return false;
        }
        let v2 = v2.min(599);
        self.eyes.push(Eye::new(v1, v2));
        self.st.J_a += 1;
        true
    }

    /// `J.a()`: clear
    pub fn J_clear(&mut self) {
        self.eyes.clear();
        self.st.J_a = 0;
    }

    /// `J.a(Graphics, idx, x, y)`
    pub fn J_draw(&self, g: &mut Graphics, v2: i32, v3: i32, v4: i32) -> Res {
        if v2 != -1 {
            let e = self.eye(v2)?;
            if e.b > 4 {
                g.set_clip(v3, v4, 19, 20);
                g.draw_image(&self.img.v_c[5], v3 - (e.b - 5) * 19, v4, TL)?;
                g.reset_clip();
            } else {
                g.set_clip(v3, v4, 19, 20);
                let img = self.img.v_c.get(e.b.max(0) as usize).ok_or(AIOOBE)?;
                g.draw_image(img, v3 - e.d / 60 * 19, v4, TL)?;
                g.reset_clip();
            }
        }
        Ok(())
    }

    /// `J.b(int)`: eye name
    pub fn J_name(&self, i: i32) -> Res<String> {
        Ok(self.eye(i)?.a.clone())
    }

    /// `J.a(int[], int)`: remove eyes, remapping the equipped indices
    pub fn J_remove(&mut self, v1: &[i32], v2: usize) {
        let mut v3 = 0;
        let mut v4 = 0;
        let mut v5 = 0;
        let mut keep: Vec<Eye> = Vec::new();
        for v7 in 0..self.eyes.len() as i32 {
            let v8 = !v1[..v2].contains(&v7);
            if v7 == self.st.j_f {
                v4 = v3;
            }
            if v7 == self.st.I_a {
                v5 = v3;
            }
            if v8 {
                let e = &self.eyes[v7 as usize];
                keep.push(Eye::new(e.b, e.d));
                v3 += 1;
            }
        }
        for &r in &v1[..v2] {
            if self.st.j_f == r {
                self.st.j_f = -1;
            }
            if self.st.I_a == r {
                self.st.I_a = -1;
            }
        }
        if self.st.j_f != -1 {
            self.st.j_f = v4;
        }
        if self.st.I_a != -1 {
            self.st.I_a = v5;
        }
        self.J_clear();
        for e in keep {
            self.J_add(e.b, e.d);
        }
    }

    /// `J.a(int)`: cap level at 599
    pub fn J_cap(&mut self, i: i32) -> Res {
        let e = self.eye_mut(i)?;
        if e.d > 599 {
            e.d = 599;
        }
        Ok(())
    }

    /// `J.a(v, v)`: can these two eyes be combined?
    pub fn J_can_combine(a: &Eye, b: &Eye) -> bool {
        if a.b == b.b {
            true
        } else if a.b < 6 && b.b < 6 {
            if (a.b - b.b).abs() == 1 {
                a.b <= 4 && b.b <= 4
            } else {
                (a.b == 0 && b.b == 4) || (a.b == 4 && b.b == 0)
            }
        } else {
            false
        }
    }

    /// `J.b(v, v)`: combination result
    pub fn J_combine(a: &Eye, b: &Eye) -> Eye {
        let mut v4 = 0;
        let mut v5 = 0;
        if a.b != b.b {
            v4 = a.b + b.b;
        }
        match v4 {
            0 => {
                v4 = a.b;
                v5 = a.d + b.d;
            }
            1 => {
                // falls through into case 2 (break) in the original
                v4 = 6;
                v5 = a.d.max(b.d);
            }
            3 => {
                v4 = 7;
                v5 = a.d.max(b.d);
            }
            4 => {
                v4 = 5;
                v5 = a.d.max(b.d);
            }
            5 => {
                v4 = 8;
                v5 = a.d.max(b.d);
            }
            7 => {
                v4 = 9;
                v5 = a.d.max(b.d);
            }
            _ => {}
        }
        if v5 > 599 {
            v5 = 599;
        }
        Eye::new(v4, v5)
    }

    /// `J.a(4 ints)`: challenge result 0 win / 1 lose / 2 draw
    pub fn J_battle(v1: i32, v2: i32, v3: i32, v4: i32) -> i32 {
        let a = Eye::new(v1, v2);
        let b = Eye::new(v3, v4);
        if Self::J_can_combine(&a, &b) {
            if a.d - b.d > 0 {
                0
            } else if a.d - b.d < 0 {
                1
            } else {
                2
            }
        } else if (a.b == 4 && b.b == 1)
            || (a.b == 0 && b.b == 2)
            || (a.b == 1 && b.b == 3)
            || (a.b == 2 && b.b == 4)
            || (a.b == 3 && b.b == 0)
        {
            0
        } else {
            1
        }
    }

    // ------------------------------------------------------------ class j
    /// `j.a()`
    pub fn j_clamp(&mut self) {
        let s = &mut self.st;
        if s.j_a > 90 {
            s.j_a = 90;
        }
        if s.j_i > 90 {
            s.j_i = 100;
        }
        if s.j_e > 9 {
            s.j_e = 9;
        }
        if s.j_n > 440 {
            s.j_n = 440;
        }
        if s.j_b > 599 {
            s.j_b = 599;
        }
    }

    /// `j.b()`: recompute pilot stats from level and equipped eye
    pub fn j_b(&mut self) -> Res {
        let f = self.st.j_f;
        let b = self.st.j_b;
        self.st.j_n = 100 + 20 * b / 30;
        self.st.j_a = 10 + 3 * b / 20 + self.j_bonus_def(f)?;
        self.st.j_i = 10 + 3 * b / 20 + self.j_bonus_atk(f)?;
        self.st.j_e = 3 + 3 * b / 200 + self.j_bonus_spd(f)?;
        self.j_clamp();
        Ok(())
    }

    /// `j.c(int)`: attack bonus
    pub fn j_bonus_atk(&self, v0: i32) -> Res<i32> {
        let mut v1 = 0;
        if v0 != -1 {
            let e = self.eye(v0)?;
            match e.b {
                0 | 1 | 5 | 8 => v1 += 2 * e.d / 20,
                2 => v1 += 2 * e.d / 20, // falls through to break
                6 | 7 => v1 += 5 * e.d / 20,
                _ => {}
            }
        }
        Ok(v1)
    }

    /// `j.b(int)`: defence bonus
    pub fn j_bonus_def(&self, v0: i32) -> Res<i32> {
        let mut v1 = 0;
        if v0 != -1 {
            let e = self.eye(v0)?;
            if matches!(e.b, 4 | 5 | 9) {
                v1 += 2 * e.d / 20;
            }
        }
        Ok(v1)
    }

    /// `j.a(int)`: speed bonus (always 0 in the original)
    pub fn j_bonus_spd(&self, v0: i32) -> Res<i32> {
        if v0 != -1 {
            self.eye(v0)?;
        }
        Ok(0)
    }
}
