//! `PhantomSpider` (the MIDlet): menus, forms, commands and save data.
//! SMS challenges (`H`, `n`, the phone-number forms) cannot work on a PC and
//! are replaced with a notice.

use super::{Cur, Eye, Game};
use crate::gfx::Res;
use crate::lang;
use crate::lcdui::{Cmd, Item, Screen, Tag};
use crate::rms::W;
use std::collections::HashMap;

const STORE: &str = "Phoenix7";

#[derive(Default)]
pub struct Midlet {
    /// a mission is in progress (Continue)
    pub z: bool,
    /// restore the saved mission when entering L
    pub w: bool,
    pub K: W,
    pub i: HashMap<String, String>,
    /// name of the fire key, used in the instructions
    pub r: String,
}

/// `s.substring(0, s.indexOf(" "))` parsed as int
fn peek(s: &str) -> Result<i32, ()> {
    let i = s.find(' ').ok_or(())?;
    s[..i].trim_start_matches('+').parse().map_err(|_| ())
}
/// `s = s.substring(s.indexOf(" ") + 1)`
fn adv(s: &mut String) {
    if let Some(i) = s.find(' ') {
        *s = s[i + 1..].to_string();
    }
}
fn take(s: &mut String) -> Result<i32, ()> {
    let v = peek(s)?;
    adv(s);
    Ok(v)
}
fn parse_all(s: &str) -> Result<i32, ()> {
    s.trim_start_matches('+').parse().map_err(|_| ())
}
fn set(arr: &mut [i32], i: i32, v: i32) -> Result<(), ()> {
    *arr.get_mut(i as usize).ok_or(())? = v;
    Ok(())
}

impl Game {
    fn rec(&self, k: i32) -> String {
        self.ps.i.get(&k.to_string()).cloned().unwrap_or_else(|| "null".into())
    }

    /// `a(String)`: load the save
    pub fn ps_load(&mut self) {
        self.ps.K.open(STORE);
        let v2 = self.ps.K.count();
        if v2 > 0 {
            for (i, r) in self.ps.K.read_all(v2).into_iter().enumerate() {
                if let Some(r) = r {
                    self.ps.i.insert(i.to_string(), r);
                }
            }
        }
        self.ps.K.close();
        if v2 > 0 {
            let _ = self.ps_parse();
        }
    }

    fn ps_parse(&mut self) -> Result<(), ()> {
        if parse_all(&self.rec(0))? == 1 {
            self.ps.z = true;
            self.ps.w = true;
        } else {
            self.ps.z = false;
            self.ps.w = false;
        }
        let mut v40 = self.rec(1);
        let mut v5 = self.rec(2);
        let s = &mut self.st;
        s.j_f = take(&mut v40)?;
        s.j_b = take(&mut v40)?;
        s.j_n = take(&mut v40)?;
        s.j_a = take(&mut v40)?;
        s.j_i = take(&mut v40)?;
        s.j_e = take(&mut v40)?;
        s.j_o = take(&mut v40)?;
        s.j_m = take(&mut v40)?;
        s.j_d = if self.ps.z { peek(&v40)? } else { 0 };
        adv(&mut v40);
        s.j_g = take(&mut v40)?;
        s.j_k = peek(&v40)? != 0;
        adv(&mut v40);
        s.j_l = take(&mut v40)?;
        s.j_h = take(&mut v40)?;
        s.j_j = peek(&v40)?;
        s.I_a = take(&mut v5)?;
        s.I_c = peek(&v5)?;
        let v6 = parse_all(&self.rec(3))?;
        let mut v5 = self.rec(4);
        let mut v7 = self.rec(5);
        for _ in 0..v6 {
            let e = Eye::new(peek(&v5)?, peek(&v7)?);
            self.eyes.push(e);
            adv(&mut v5);
            adv(&mut v7);
        }
        self.st.J_a = self.eyes.len() as i32;

        let _ = (|| -> Result<(), ()> {
            self.st.ps_u[0] = parse_all(&self.rec(6))?;
            self.st.ps_u[1] = parse_all(&self.rec(7))?;
            self.st.ps_u[2] = parse_all(&self.rec(8))?;
            self.st.ps_G = parse_all(&self.rec(9))? == 0;
            Ok(())
        })();

        if self.ps.z {
            self.st.x_f = parse_all(&self.rec(10))?;
            let mut r: Vec<String> = (11..20).map(|k| self.rec(k)).collect();
            for v14 in 0..self.st.x_f {
                let s = &mut self.st;
                set(&mut s.x_i, v14, take(&mut r[0])?)?;
                set(&mut s.x_a, v14, take(&mut r[1])?)?;
                set(&mut s.x_e, v14, take(&mut r[2])?)?;
                set(&mut s.x_c, v14, take(&mut r[3])?)?;
                set(&mut s.x_k, v14, take(&mut r[4])?)?;
                set(&mut s.x_d, v14, take(&mut r[5])?)?;
                set(&mut s.x_h, v14, take(&mut r[6])?)?;
                set(&mut s.x_g, v14, take(&mut r[7])?)?;
                set(&mut s.x_j, v14, take(&mut r[8])?)?;
            }
            let mut r: Vec<String> = (20..26).map(|k| self.rec(k)).collect();
            for v86 in 0..2 {
                let s = &mut self.st;
                let _ = (|| -> Result<(), ()> {
                    s.p_c[v86] = take(&mut r[0])?;
                    s.p_b[v86] = take(&mut r[1])?;
                    s.p_g[v86] = take(&mut r[2])?;
                    s.p_j[v86] = take(&mut r[3])?;
                    s.p_h[v86] = take(&mut r[4])?;
                    s.p_i[v86] = take(&mut r[5])?;
                    Ok(())
                })();
            }
            let mut r: Vec<String> = (26..30).map(|k| self.rec(k)).collect();
            for v87 in 0..15 {
                let s = &mut self.st;
                let _ = (|| -> Result<(), ()> {
                    s.p_e[v87] = take(&mut r[0])?;
                    s.p_d[v87] = take(&mut r[1])?;
                    s.p_f[v87] = take(&mut r[2])?;
                    s.p_a[v87] = take(&mut r[3])?;
                    Ok(())
                })();
            }
            let mut r: Vec<String> = (30..33).map(|k| self.rec(k)).collect();
            for v88 in 0..5 {
                let s = &mut self.st;
                let _ = (|| -> Result<(), ()> {
                    s.c_a[v88] = take(&mut r[0])?;
                    s.c_c[v88] = take(&mut r[1])?;
                    s.c_b[v88] = take(&mut r[2])?;
                    Ok(())
                })();
            }
            let mut r: Vec<String> = (33..59).map(|k| self.rec(k)).collect();
            for v31 in 0..20 {
                let s = &mut self.st;
                let _ = (|| -> Result<(), ()> {
                    s.C_p[v31] = take(&mut r[0])?;
                    s.C_n[v31] = take(&mut r[1])?;
                    s.C_g[v31] = take(&mut r[2])?;
                    s.C_d[v31] = take(&mut r[3])?;
                    s.C_o[v31] = take(&mut r[4])?;
                    s.C_l[v31] = take(&mut r[5])?;
                    s.C_k[v31] = take(&mut r[6])?;
                    s.C_f[v31] = take(&mut r[7])?;
                    s.C_e[v31] = take(&mut r[8])?;
                    s.C_c[0][v31] = take(&mut r[9])?;
                    s.C_c[1][v31] = take(&mut r[10])?;
                    s.C_c[2][v31] = take(&mut r[11])?;
                    s.C_a[0][v31] = take(&mut r[12])?;
                    s.C_a[1][v31] = take(&mut r[13])?;
                    s.C_a[2][v31] = take(&mut r[14])?;
                    s.C_i[0][v31] = take(&mut r[15])?;
                    s.C_i[1][v31] = take(&mut r[16])?;
                    s.C_i[2][v31] = take(&mut r[17])?;
                    s.C_j[0][v31] = take(&mut r[18])?;
                    s.C_j[1][v31] = take(&mut r[19])?;
                    s.C_h[0][v31] = take(&mut r[20])?;
                    s.C_h[1][v31] = take(&mut r[21])?;
                    s.C_m[0][v31] = take(&mut r[22])?;
                    s.C_m[1][v31] = take(&mut r[23])?;
                    s.C_b[0][v31] = take(&mut r[24])?;
                    s.C_b[1][v31] = take(&mut r[25])?;
                    Ok(())
                })();
            }
        }
        Ok(())
    }

    /// `b(String)`: write the save
    pub fn ps_save(&mut self) {
        self.ps.K.delete(STORE);
        self.ps.K.open(STORE);
        let mut recs: Vec<String> = Vec::new();
        let s = &self.st;
        recs.push(if self.ps.z { "1" } else { "0" }.into());
        recs.push(format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} ",
            s.j_f,
            s.j_b,
            s.j_n,
            s.j_a,
            s.j_i,
            s.j_e,
            s.j_o,
            s.j_m,
            s.j_d,
            s.j_g,
            if s.j_k { 1 } else { 0 },
            s.j_l,
            s.j_h,
            s.j_j,
            s.j_c
        ));
        recs.push(format!("{} {} ", s.I_a, s.I_c));
        recs.push(self.eyes.len().to_string());
        if !self.eyes.is_empty() {
            recs.push(self.eyes.iter().map(|e| format!("{} ", e.b)).collect());
            recs.push(self.eyes.iter().map(|e| format!("{} ", e.d)).collect());
        } else {
            recs.push("-1".into());
            recs.push("-1".into());
        }
        for u in s.ps_u {
            recs.push(u.to_string());
        }
        recs.push("1".into());
        if self.ps.z {
            recs.push(s.x_f.to_string());
            let n = s.x_f.clamp(0, 4) as usize;
            let row = |a: &[i32]| -> String { a[..n].iter().map(|v| format!("{v} ")).collect() };
            for a in [&s.x_i, &s.x_a, &s.x_e, &s.x_c, &s.x_k, &s.x_d, &s.x_h, &s.x_g, &s.x_j] {
                recs.push(row(a));
            }
            let all = |a: &[i32]| -> String { a.iter().map(|v| format!("{v} ")).collect() };
            for a in [&s.p_c, &s.p_b, &s.p_g, &s.p_j, &s.p_h, &s.p_i] {
                recs.push(all(a));
            }
            for a in [&s.p_e, &s.p_d, &s.p_f, &s.p_a] {
                recs.push(all(a));
            }
            for a in [&s.c_a, &s.c_c, &s.c_b] {
                recs.push(all(a));
            }
            for a in [&s.C_p, &s.C_n, &s.C_g, &s.C_d, &s.C_o, &s.C_l, &s.C_k, &s.C_f, &s.C_e] {
                recs.push(all(a));
            }
            for a in [
                &s.C_c[0], &s.C_c[1], &s.C_c[2], &s.C_a[0], &s.C_a[1], &s.C_a[2], &s.C_i[0], &s.C_i[1], &s.C_i[2],
                &s.C_j[0], &s.C_j[1], &s.C_h[0], &s.C_h[1], &s.C_m[0], &s.C_m[1], &s.C_b[0], &s.C_b[1],
            ] {
                recs.push(all(a));
            }
        }
        for r in &recs {
            self.ps.K.add(r);
        }
        self.ps.K.close();
    }

    /// `i()`: enter the shooting game
    pub fn ps_i(&mut self) {
        if !self.k_unloaded() {
            self.k_unload();
        }
        self.L_m();
        self.set_current(Cur::L);
    }

    /// `e()`: back to the menu canvas (reload its images)
    pub fn ps_e(&mut self) {
        if !self.L_unloaded() {
            self.L_unload();
        }
        self.k_f();
    }

    /// `p()`: high scores
    fn ps_p(&mut self) {
        self.kc.F.tune(&mut self.audio, 2);
        self.st.k_X = 0;
        self.st.K_b = 21;
        self.ps_e();
        self.st.k_y = 0;
        self.set_current(Cur::K);
    }

    /// `f()`: instructions
    fn ps_f(&mut self) {
        self.st.u_b = 0;
        self.st.u_l = 0;
        self.st.u_h = 0;
        self.st.K_b = 6;
        self.ps_e();
        self.kc.F.tune(&mut self.audio, 1);
        let b = self.st.u_b;
        self.ps_d(b);
    }

    fn ps_new_game_flow(&mut self) {
        self.st.j_d = 0;
        self.st.K_b = 8;
        self.ps_e();
        self.ps.z = false;
        self.ps.w = false;
        self.ps_a(0);
    }

    fn u_page_setup(&mut self) {
        match self.st.u_b {
            0 => {
                self.st.u_h = 0;
                self.st.u_l = 0;
            }
            1 => {
                self.st.u_h = 0;
                self.st.u_l = 0;
                self.st.u_d = 1;
            }
            2 => self.st.u_h = 0,
            3 => {
                self.st.u_h = 0;
                self.st.u_l = 0;
                self.st.u_d = 0;
            }
            4 => {
                self.st.u_h = 0;
                self.st.u_l = 1;
            }
            _ => {}
        }
        if !self.k_unloaded() {
            self.st.k_W = 6;
            self.set_current(Cur::K);
        } else {
            self.st.k_W = 6;
            self.ps_e();
        }
    }

    /// `commandAction(Command, Displayable)`
    pub fn ps_command(&mut self, c: Cmd, s: &Screen) {
        let _ = self.ps_command_inner(c, s);
    }

    fn ps_command_inner(&mut self, c: Cmd, s: &Screen) -> Res {
        let num = |idx: usize| -> Option<i32> {
            let t = s.field_text(idx);
            if t.is_empty() {
                None
            } else {
                t.parse().ok()
            }
        };
        match s.tag {
            Tag::O => {
                if c == Cmd::A {
                    self.ps_exit();
                } else if c == Cmd::N {
                    let sel = s.selected_index();
                    if self.st.ps_G {
                        match sel {
                            0 => {
                                self.st.k_q = 0;
                                self.ps_reset()?;
                                self.st.j_d = 0;
                                self.st.K_b = 8;
                                self.ps_e();
                                self.ps.z = false;
                                self.st.ps_G = false;
                                self.ps_a(0);
                            }
                            1 => {
                                self.st.k_q = 1;
                                self.ps_p();
                            }
                            2 => {
                                self.st.k_q = 2;
                                self.ps_f();
                            }
                            _ => {}
                        }
                    } else {
                        match sel {
                            0 => {
                                if self.ps.z && !self.L_unloaded() {
                                    self.L_d();
                                    self.set_current(Cur::L);
                                } else if self.ps.z && self.L_unloaded() {
                                    self.ps_i();
                                } else {
                                    self.ps_new_game_flow();
                                }
                                self.st.k_q = 0;
                            }
                            1 => {
                                self.ps_q();
                                self.st.k_q = 1;
                            }
                            2 => {
                                self.ps_p();
                                self.st.k_q = 2;
                            }
                            3 => {
                                self.ps_f();
                                self.st.k_q = 3;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Tag::I => {
                if c == Cmd::F {
                    self.ps_b(1);
                } else if c == Cmd::H {
                    self.ps_reset()?;
                    self.ps_new_game_flow();
                }
            }
            Tag::QList => {
                if c == Cmd::F {
                    self.ps_b(0);
                } else if c == Cmd::N {
                    match s.selected_index() {
                        0 => {
                            self.j_b()?;
                            self.k_roll_enemy()?;
                            self.st.k_W = 8;
                            self.st.k_y = 0;
                            self.set_current(Cur::K);
                            self.st.k_T = 0;
                        }
                        1 => {
                            self.st.k_m = 0;
                            self.st.k_W = 1;
                            self.set_current(Cur::K);
                            self.st.k_T = 1;
                        }
                        2 => {
                            self.st.k_m = self.st.j_f;
                            self.st.k_r = 0;
                            self.st.k_P = 0;
                            self.st.k_W = 4;
                            self.set_current(Cur::K);
                            self.st.k_T = 2;
                        }
                        3 => {
                            self.j_b()?;
                            self.st.k_W = 5;
                            self.set_current(Cur::K);
                            self.st.k_T = 3;
                        }
                        4 => {
                            if self.st.I_a == -1 {
                                self.ps_n();
                            } else {
                                self.st.k_y = 0;
                                self.ps_sms_unavailable();
                            }
                            self.st.k_T = 4;
                        }
                        _ => {}
                    }
                }
            }
            Tag::QForm => {
                if c == Cmd::F {
                    if self.st.u_b >= 0 {
                        self.st.u_b -= 1;
                    }
                    if self.st.u_b < 0 {
                        self.kc.F.close(&mut self.audio);
                        if self.st.ps_G {
                            self.ps_b(2);
                        } else {
                            self.ps_b(3);
                        }
                    } else {
                        self.u_page_setup();
                    }
                } else if c == Cmd::B || c == Cmd::H {
                    self.u_page_setup();
                }
            }
            Tag::P => {
                if c == Cmd::S {
                    if let Some(v) = num(1) {
                        if (1..=599).contains(&v) && self.eyes.len() < 30 {
                            let t = s.choice_sel(0);
                            self.k_add_eye(t, v);
                            self.set_current(Cur::K);
                        }
                    }
                } else if c == Cmd::F {
                    self.set_current(Cur::K);
                }
            }
            Tag::LForm => {
                if c == Cmd::S {
                    if let Some(v) = num(0) {
                        if (1..=599).contains(&v) {
                            self.st.j_b = v;
                            self.j_b()?;
                            self.set_current(Cur::K);
                        }
                    }
                } else if c == Cmd::F {
                    self.set_current(Cur::K);
                }
            }
            Tag::J => {
                if c == Cmd::S {
                    // field order: P (life), L (speed), k (attack), M (defence)
                    if let Some(v) = num(1).filter(|v| (1..=9).contains(v)) {
                        self.st.j_e = v;
                    }
                    if let Some(v) = num(3).filter(|v| (1..=90).contains(v)) {
                        self.st.j_a = v;
                    }
                    if let Some(v) = num(2).filter(|v| (1..=90).contains(v)) {
                        self.st.j_i = v;
                    }
                    if let Some(v) = num(0).filter(|v| (100..=440).contains(v)) {
                        self.st.j_n = v;
                    }
                }
                self.set_current(Cur::K);
            }
            Tag::B => {
                if c == Cmd::S {
                    if let Some(v) = num(1).filter(|v| (1..=9).contains(v)) {
                        self.st.i_c = v;
                    }
                    if let Some(v) = num(3).filter(|v| (1..=90).contains(v)) {
                        self.st.i_b = v * 2;
                    }
                    if let Some(v) = num(2).filter(|v| (1..=90).contains(v)) {
                        self.st.i_g = v * 2;
                    }
                    if let Some(v) = num(0).filter(|v| (100..=440).contains(v)) {
                        self.st.i_d = v;
                    }
                }
                self.set_current(Cur::K);
            }
            _ => {}
        }
        Ok(())
    }

    fn alert(&mut self, tag: Tag, text: String) {
        self.set_current(Cur::Scr(Box::new(Screen::alert(tag, text, 3000))));
    }

    /// `c()`
    pub fn ps_c(&mut self) {
        if !self.ps.z && self.st.k_c != 6 && self.st.k_c != 21 {
            let t = self.st.k_T;
            self.ps_a(t);
        } else {
            let q = self.st.k_q;
            self.ps_b(q);
        }
        self.alert(Tag::AlertD, lang::s(46));
    }

    /// `n()`: "Equip spider with eye"
    fn ps_n(&mut self) {
        self.alert(Tag::AlertD, lang::s(46));
    }

    /// Replacement for the SMS challenge form `l()`.
    fn ps_sms_unavailable(&mut self) {
        self.alert(Tag::Info, format!("{}: SMS is not available on PC", lang::s(22)));
    }

    /// `g()`: "Mission failed"
    pub fn ps_g(&mut self) {
        self.alert(Tag::AlertX, lang::s(62));
    }

    /// `e(int)`: challenge result
    pub fn ps_e_result(&mut self, r: i32) {
        let t = match r {
            0 => 88,
            1 => 89,
            _ => 42,
        };
        self.alert(Tag::AlertY, lang::s(t));
    }

    /// `b()`: cheat form "add eye"
    pub fn ps_cheat_add_eye(&mut self) {
        let types = [86, 34, 31, 29, 27, 28, 36, 35, 32, 30].iter().map(|&i| lang::s(i)).collect();
        let mut f = Screen::form(Tag::P, lang::s(33));
        f.append(Item::Choice { label: lang::s(49), opts: types, sel: 0 });
        f.append(Item::Field { label: lang::s(47), text: String::new(), max: 3 });
        f.add_command(Cmd::S);
        f.add_command(Cmd::F);
        self.set_current(Cur::Scr(Box::new(f)));
    }

    /// `m()`: cheat form "pilot level"
    pub fn ps_cheat_level(&mut self) {
        let mut f = Screen::form(Tag::LForm, lang::s(69));
        f.append(Item::Field { label: lang::s(68), text: String::new(), max: 3 });
        f.add_command(Cmd::S);
        f.add_command(Cmd::F);
        self.set_current(Cur::Scr(Box::new(f)));
    }

    fn stat_form(&mut self, tag: Tag, title: i32, labels: [i32; 4]) {
        let mut f = Screen::form(tag, lang::s(title));
        for l in labels {
            f.append(Item::Field { label: lang::s(l), text: String::new(), max: 3 });
        }
        f.add_command(Cmd::S);
        f.add_command(Cmd::F);
        self.set_current(Cur::Scr(Box::new(f)));
    }

    /// `h()`: cheat form "pilot information"
    pub fn ps_cheat_pilot(&mut self) {
        self.stat_form(Tag::J, 52, [56, 78, 18, 40]);
    }

    /// `d()`: cheat form "spider information"
    pub fn ps_cheat_spider(&mut self) {
        self.stat_form(Tag::B, 53, [57, 79, 19, 41]);
    }

    /// `s()`: reset the whole game
    fn ps_reset(&mut self) -> Res {
        self.ps.K.delete(STORE);
        self.st.j_f = -1;
        self.st.j_b = 1;
        self.j_b()?;
        self.J_new();
        self.J_clear();
        self.st.I_a = -1;
        self.st.I_c = 1;
        Ok(())
    }

    /// `q()`: "Starting a new game will reset all game settings"
    fn ps_q(&mut self) {
        let mut f = Screen::form(Tag::I, lang::s(0));
        f.append(Item::Text(lang::s(73)));
        f.add_command(Cmd::H);
        f.add_command(Cmd::F);
        self.set_current(Cur::Scr(Box::new(f)));
    }

    /// `d(int)`: instructions page
    pub fn ps_d(&mut self, v1: i32) {
        let texts = [lang::s1(1, &self.ps.r), lang::s(2), lang::s(3), lang::s(4), lang::s(5)];
        let Some(text) = texts.get(v1.max(0) as usize).filter(|_| v1 >= 0) else { return };
        let mut f = Screen::form(Tag::QForm, lang::s(12));
        f.append(Item::Text(text.clone()));
        f.add_command(if v1 == 5 { Cmd::H } else { Cmd::B });
        f.add_command(Cmd::F);
        self.set_current(Cur::Scr(Box::new(f)));
    }

    /// `b(int)`: main menu
    pub fn ps_b(&mut self, v1: i32) {
        let opts = if self.st.ps_G {
            vec![lang::s(13), lang::s(10), lang::s(11)]
        } else {
            vec![lang::s(8), lang::s(13), lang::s(10), lang::s(11)]
        };
        let mut l = Screen::list(Tag::O, lang::s(0), opts);
        l.set_select_command(Cmd::N);
        l.add_command(Cmd::A);
        l.set_selected_index(v1);
        self.set_current(Cur::Scr(Box::new(l)));
    }

    /// `a(int)`: eyes/mission menu
    pub fn ps_a(&mut self, v1: i32) {
        let opts = [80, 48, 83, 84, 23].iter().map(|&i| lang::s(i)).collect();
        let mut l = Screen::list(Tag::QList, lang::s(0), opts);
        l.set_select_command(Cmd::N);
        l.add_command(Cmd::F);
        l.set_selected_index(v1);
        self.set_current(Cur::Scr(Box::new(l)));
    }

    /// `k()`: exit
    pub fn ps_exit(&mut self) {
        self.cur = Cur::None;
        self.ps_destroy();
        self.quit = true;
    }

    /// `destroyApp(true)`
    pub fn ps_destroy(&mut self) {
        if self.ps.z {
            self.L_save();
        }
        self.ps_save();
    }
}
