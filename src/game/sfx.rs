//! `s`: explosion/hit effects, with `M` slots.

use super::{load, Images, St};
use crate::gfx::{Graphics, Res, LEFT, TOP};

#[derive(Default, Clone, Copy)]
pub struct M {
    pub a: i32,
    pub c: i32,
    pub b: i32,
}

pub struct SFx {
    pub d: [M; 10],
}

const SZ: [[i32; 2]; 4] = [[14, 18], [24, 24], [17, 24], [12, 13]];
const FR: [i32; 3] = [3, 2, 2];

impl SFx {
    /// `s()`: loads the static images and creates the slots
    pub fn new(img: &mut Images) -> SFx {
        Self::load(img);
        SFx { d: [M::default(); 10] }
    }

    /// `b()`
    pub fn load(img: &mut Images) {
        img.s_f[0] = load("/explore01.png");
        img.s_f[1] = load("/waterlife.png");
        img.s_f[2] = load("/burn.png");
        img.s_f[3] = load("/explore02.png");
    }

    /// `a(type, x, y, frames)`
    pub fn add(&mut self, st: &mut St, v1: i32, v2: i32, v3: i32, v4: i32) {
        let e = st.s_e as usize;
        if self.d[e].b == 0 {
            self.d[e] = M { a: v2, c: v3, b: v4 };
            st.s_e += 1;
        }
        if st.s_e >= 10 {
            st.s_e = 0;
        }
        st.s_c = v1;
    }

    /// `a()`
    pub fn reset(&mut self) {
        for m in self.d.iter_mut() {
            m.b = 0;
        }
    }

    /// `a(Graphics)`
    pub fn draw(&mut self, g: &mut Graphics, img: &Images, st: &St) -> Res {
        let c = st.s_c;
        for m in self.d.iter_mut() {
            if m.b > 0 {
                // M.a()
                if st.L_H != 3 && m.b > 0 {
                    m.b -= 1;
                }
                if c == 1 {
                    let s = SZ[1];
                    g.set_clip(m.a, m.c, s[0], s[1]);
                    g.draw_image(&img.s_f[1], m.a - s[0] * (m.b % FR[1]), m.c, TOP | LEFT)?;
                    g.reset_clip();
                } else if c == 10 {
                    let s = SZ[3];
                    g.set_clip(m.a, m.c, s[0], s[1]);
                    g.draw_image(&img.s_f[3], m.a - s[0] * (m.b % 3), m.c, TOP | LEFT)?;
                    g.reset_clip();
                } else {
                    let ci = c as usize;
                    let s = *SZ.get(ci).ok_or(crate::gfx::AIOOBE)?;
                    g.set_clip(m.a, m.c, s[0], s[1]);
                    // `b + 1 % n` parses as `b + (1 % n)` == b + 1
                    g.draw_image(img.s_f.get(ci).ok_or(crate::gfx::AIOOBE)?, m.a - s[0] * (m.b + 1), m.c, TOP | LEFT)?;
                    g.reset_clip();
                }
            }
        }
        Ok(())
    }
}
