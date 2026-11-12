//! High-level LCDUI screens (List, Form, Alert) drawn into the 128x128
//! framebuffer in a style matching the game.

use crate::midp::gfx::{Graphics, HCENTER, LEFT, RIGHT, SCREEN_H, SCREEN_W, TOP};
use crate::midp::lang;

// MIDP / Nokia key codes
pub const K_UP: i32 = -1;
pub const K_DOWN: i32 = -2;
pub const K_LEFT: i32 = -3;
pub const K_RIGHT: i32 = -4;
pub const K_FIRE: i32 = -5;
pub const K_SOFT_L: i32 = -6;
pub const K_SOFT_R: i32 = -7;
pub const K_CLEAR: i32 = -8;

/// `Canvas.getGameAction`
pub fn game_action(key: i32) -> i32 {
    match key {
        K_UP | 50 => 1,
        K_DOWN | 56 => 6,
        K_LEFT | 52 => 2,
        K_RIGHT | 54 => 5,
        K_FIRE | 53 => 8,
        49 => 9,
        51 => 10,
        55 => 11,
        57 => 12,
        _ => 0,
    }
}

/// The midlet's Command objects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cmd {
    /// N: Select (SCREEN)
    N,
    /// F: Back (BACK)
    F,
    /// a: Exit (BACK)
    A,
    /// s: Submit (SCREEN)
    S,
    /// H: OK (SCREEN)
    H,
    /// b: Next (SCREEN)
    B,
    /// d: Cancel (BACK)
    D,
}

impl Cmd {
    pub fn label(self) -> String {
        lang::s(match self {
            Cmd::N => 16,
            Cmd::F => 6,
            Cmd::A => 9,
            Cmd::S => 81,
            Cmd::H => 15,
            Cmd::B => 14,
            Cmd::D => 7,
        })
    }
    pub fn is_back(self) -> bool {
        matches!(self, Cmd::F | Cmd::A | Cmd::D)
    }
}

/// Identity of a screen, standing in for `var2 == this.O` comparisons.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    /// main menu List O
    O,
    /// eye menu List q
    QList,
    /// new-game confirm Form I
    I,
    /// instructions Form Q
    QForm,
    /// challenge Form t
    T,
    /// Alerts D, m, y, x
    AlertD,
    AlertM,
    AlertY,
    AlertX,
    /// Form C (waiting)
    C,
    /// Form f (accept challenge)
    FForm,
    /// cheat Forms p, l, J, B
    P,
    LForm,
    J,
    B,
    /// Port-only notice
    Info,
}

#[derive(Clone)]
pub enum Item {
    Text(String),
    Field { label: String, text: String, max: usize },
    Choice { label: String, opts: Vec<String>, sel: usize },
    Gauge(String),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    List,
    Form,
    Alert,
}

#[derive(Clone)]
pub struct Screen {
    pub tag: Tag,
    pub kind: Kind,
    pub title: String,
    pub items: Vec<Item>,
    pub opts: Vec<String>,
    pub sel: usize,
    pub cmds: Vec<Cmd>,
    pub select_cmd: Option<Cmd>,
    focus: usize,
    scroll: i32,
    pub timeout_ms: i32,
}

const TITLE_H: i32 = 12;
const SOFT_H: i32 = 11;
const ROW_H: i32 = 11;
const LINE_H: i32 = 10;
const BODY_Y: i32 = TITLE_H + 2;
const BODY_H: i32 = SCREEN_H - TITLE_H - SOFT_H - 3;

impl Screen {
    fn base(tag: Tag, kind: Kind, title: String) -> Screen {
        Screen {
            tag,
            kind,
            title,
            items: Vec::new(),
            opts: Vec::new(),
            sel: 0,
            cmds: Vec::new(),
            select_cmd: None,
            focus: 0,
            scroll: 0,
            timeout_ms: 0,
        }
    }
    /// `new List(title, IMPLICIT, items, null)`
    pub fn list(tag: Tag, title: String, opts: Vec<String>) -> Screen {
        let mut s = Self::base(tag, Kind::List, title);
        s.opts = opts;
        s
    }
    pub fn form(tag: Tag, title: String) -> Screen {
        Self::base(tag, Kind::Form, title)
    }
    /// `new Alert(null, text, null, INFO)` with a timeout
    pub fn alert(tag: Tag, text: String, timeout_ms: i32) -> Screen {
        let mut s = Self::base(tag, Kind::Alert, String::new());
        s.items.push(Item::Text(text));
        s.timeout_ms = timeout_ms;
        s
    }
    pub fn add_command(&mut self, c: Cmd) {
        if !self.cmds.contains(&c) {
            self.cmds.push(c);
        }
    }
    pub fn set_select_command(&mut self, c: Cmd) {
        self.select_cmd = Some(c);
    }
    pub fn append(&mut self, item: Item) {
        self.items.push(item);
    }
    pub fn set_selected_index(&mut self, i: i32) {
        if i >= 0 && (i as usize) < self.opts.len() {
            self.sel = i as usize;
        }
    }
    pub fn selected_index(&self) -> i32 {
        self.sel as i32
    }
    /// TextField / ChoiceGroup accessors by item index
    pub fn field_text(&self, idx: usize) -> String {
        match self.items.get(idx) {
            Some(Item::Field { text, .. }) => text.clone(),
            _ => String::new(),
        }
    }
    pub fn choice_sel(&self, idx: usize) -> i32 {
        match self.items.get(idx) {
            Some(Item::Choice { sel, .. }) => *sel as i32,
            _ => -1,
        }
    }

    fn left_cmd(&self) -> Option<Cmd> {
        if self.kind == Kind::List {
            if let Some(c) = self.select_cmd {
                return Some(c);
            }
        }
        self.cmds.iter().copied().find(|c| !c.is_back())
    }
    fn right_cmd(&self) -> Option<Cmd> {
        self.cmds.iter().copied().find(|c| c.is_back())
    }

    /// Focus stops in a form: (item index, option index)
    fn stops(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (i, it) in self.items.iter().enumerate() {
            match it {
                Item::Field { .. } => v.push((i, 0)),
                Item::Choice { opts, .. } => {
                    for o in 0..opts.len() {
                        v.push((i, o));
                    }
                }
                _ => {}
            }
        }
        v
    }

    /// Handle a key press; returns a command to dispatch to commandAction.
    /// `Err(())` means "dismiss alert".
    pub fn key(&mut self, key: i32, font: &crate::midp::font::Font) -> Result<Option<Cmd>, ()> {
        match self.kind {
            Kind::Alert => {
                if matches!(key, K_FIRE | K_SOFT_L | K_SOFT_R) {
                    return Err(());
                }
                Ok(None)
            }
            Kind::List => {
                let n = self.opts.len();
                match key {
                    K_UP if n > 0 => self.sel = (self.sel + n - 1) % n,
                    K_DOWN if n > 0 => self.sel = (self.sel + 1) % n,
                    K_FIRE | K_SOFT_L => return Ok(self.left_cmd()),
                    K_SOFT_R => return Ok(self.right_cmd()),
                    _ => {}
                }
                Ok(None)
            }
            Kind::Form => {
                let stops = self.stops();
                match key {
                    K_UP | K_DOWN => {
                        if stops.is_empty() {
                            let total = self.layout(font).len() as i32 * LINE_H;
                            let max = (total - BODY_H).max(0);
                            let d = if key == K_UP { -LINE_H * 3 } else { LINE_H * 3 };
                            self.scroll = (self.scroll + d).clamp(0, max);
                        } else {
                            let n = stops.len();
                            self.focus = if key == K_UP { (self.focus + n - 1) % n } else { (self.focus + 1) % n };
                        }
                    }
                    K_FIRE => {
                        if let Some(&(i, o)) = stops.get(self.focus) {
                            if let Some(Item::Choice { sel, .. }) = self.items.get_mut(i) {
                                *sel = o;
                                return Ok(None);
                            }
                        }
                        return Ok(self.left_cmd());
                    }
                    K_SOFT_L => return Ok(self.left_cmd()),
                    K_SOFT_R => return Ok(self.right_cmd()),
                    K_CLEAR | K_LEFT => {
                        if let Some(&(i, _)) = stops.get(self.focus) {
                            if let Some(Item::Field { text, .. }) = self.items.get_mut(i) {
                                text.pop();
                            }
                        }
                    }
                    48..=57 => {
                        if let Some(&(i, _)) = stops.get(self.focus) {
                            if let Some(Item::Field { text, max, .. }) = self.items.get_mut(i) {
                                if text.len() < *max {
                                    text.push(char::from_u32(key as u32).unwrap());
                                }
                            }
                        }
                    }
                    _ => {}
                }
                Ok(None)
            }
        }
    }

    /// Lines of a form: (text, kind) where kind: 0 text, 1 field box (item, stop), 2 choice option
    fn layout(&self, font: &crate::midp::font::Font) -> Vec<(String, Option<usize>, u8)> {
        let mut out = Vec::new();
        let w = SCREEN_W - 8;
        let mut stop = 0;
        for it in &self.items {
            match it {
                Item::Text(t) => {
                    for l in font.wrap(t, w) {
                        out.push((l, None, 0));
                    }
                }
                Item::Gauge(t) => {
                    for l in font.wrap(t, w) {
                        out.push((l, None, 0));
                    }
                    out.push(("...".into(), None, 0));
                }
                Item::Field { label, text, .. } => {
                    for l in font.wrap(label, w) {
                        out.push((l, None, 0));
                    }
                    out.push((text.clone(), Some(stop), 1));
                    stop += 1;
                }
                Item::Choice { label, opts, sel } => {
                    for l in font.wrap(label, w) {
                        out.push((l, None, 0));
                    }
                    for (i, o) in opts.iter().enumerate() {
                        let mark = if i == *sel { "(o) " } else { "( ) " };
                        out.push((format!("{mark}{o}"), Some(stop), 2));
                        stop += 1;
                    }
                }
            }
        }
        out
    }

    pub fn draw(&mut self, g: &mut Graphics, blink: bool) {
        g.reset_clip();
        let font = g.font.clone();
        if self.kind == Kind::Alert {
            // Alert drawn as a framed box over a dark backdrop
            g.set_color(0, 0, 0);
            g.fill_rect(0, 0, SCREEN_W, SCREEN_H);
            let text = match &self.items[0] {
                Item::Text(t) => t.clone(),
                _ => String::new(),
            };
            let lines = font.wrap(&text, SCREEN_W - 16);
            let h = lines.len() as i32 * LINE_H + 10;
            let y0 = (SCREEN_H - h) / 2;
            g.set_color(88, 89, 172);
            g.fill_rect(4, y0, SCREEN_W - 8, h);
            g.set_color(0, 0, 0);
            g.fill_rect(5, y0 + 1, SCREEN_W - 10, h - 2);
            g.set_color(255, 0, 213);
            g.draw_rect(6, y0 + 2, SCREEN_W - 13, h - 5);
            g.set_color(255, 255, 255);
            for (i, l) in lines.iter().enumerate() {
                g.draw_string(l, SCREEN_W / 2, y0 + 5 + i as i32 * LINE_H, TOP | HCENTER);
            }
            return;
        }

        g.set_color(0, 0, 0);
        g.fill_rect(0, 0, SCREEN_W, SCREEN_H);
        // title bar
        g.set_color(31, 0, 108);
        g.fill_rect(0, 0, SCREEN_W, TITLE_H);
        g.set_color(88, 89, 172);
        g.fill_rect(0, TITLE_H - 1, SCREEN_W, 1);
        g.set_color(255, 255, 255);
        g.draw_string(&self.title, SCREEN_W / 2, 1, TOP | HCENTER);

        g.set_clip(0, BODY_Y, SCREEN_W, BODY_H);
        match self.kind {
            Kind::List => {
                let n = self.opts.len() as i32;
                let visible = BODY_H / ROW_H;
                let sel = self.sel as i32;
                if sel * ROW_H < self.scroll {
                    self.scroll = sel * ROW_H;
                }
                if (sel + 1) * ROW_H > self.scroll + visible * ROW_H {
                    self.scroll = (sel + 1) * ROW_H - visible * ROW_H;
                }
                for i in 0..n {
                    let y = BODY_Y + i * ROW_H - self.scroll;
                    if i == sel {
                        g.set_color(255, 0, 213);
                        g.fill_rect(2, y, SCREEN_W - 4, ROW_H);
                        g.set_color(255, 255, 255);
                    } else {
                        g.set_color(190, 190, 230);
                    }
                    g.draw_string(&self.opts[i as usize], 6, y + 1, TOP | LEFT);
                }
                if n > visible {
                    // scroll indicator
                    let track = BODY_H - 2;
                    let th = (track * visible / n).max(4);
                    let ty = BODY_Y + 1 + (track - th) * sel / (n - 1).max(1);
                    g.set_color(88, 89, 172);
                    g.fill_rect(SCREEN_W - 2, ty, 2, th);
                }
            }
            Kind::Form => {
                let lines = self.layout(&font);
                let stops = self.stops().len();
                if stops > 0 {
                    // keep focused stop visible
                    if let Some(idx) = lines.iter().position(|l| l.1 == Some(self.focus)) {
                        let y = idx as i32 * LINE_H;
                        if y < self.scroll {
                            self.scroll = (y - LINE_H).max(0);
                        }
                        if y + LINE_H + 2 > self.scroll + BODY_H {
                            self.scroll = y + LINE_H + 2 - BODY_H;
                        }
                    }
                }
                for (i, (text, stop, kind)) in lines.iter().enumerate() {
                    let y = BODY_Y + i as i32 * LINE_H - self.scroll;
                    let focused = *stop == Some(self.focus);
                    match kind {
                        1 => {
                            g.set_color(if focused { 255 } else { 88 }, if focused { 0 } else { 89 }, if focused { 213 } else { 172 });
                            g.draw_rect(3, y, SCREEN_W - 7, LINE_H - 1);
                            g.set_color(255, 255, 255);
                            let cursor = if focused && blink { "_" } else { "" };
                            g.draw_string(&format!("{text}{cursor}"), 6, y, TOP | LEFT);
                        }
                        2 => {
                            if focused {
                                g.set_color(255, 0, 213);
                                g.fill_rect(2, y, SCREEN_W - 4, LINE_H);
                            }
                            g.set_color(255, 255, 255);
                            g.draw_string(text, 6, y, TOP | LEFT);
                        }
                        _ => {
                            g.set_color(230, 230, 255);
                            g.draw_string(text, 4, y, TOP | LEFT);
                        }
                    }
                }
                let total = lines.len() as i32 * LINE_H;
                if total > BODY_H {
                    let track = BODY_H - 2;
                    let th = (track * BODY_H / total).max(4);
                    let ty = BODY_Y + 1 + (track - th) * self.scroll / (total - BODY_H).max(1);
                    g.set_color(88, 89, 172);
                    g.fill_rect(SCREEN_W - 2, ty, 2, th);
                }
            }
            Kind::Alert => {}
        }
        g.reset_clip();

        // soft-key bar
        let y = SCREEN_H - SOFT_H;
        g.set_color(31, 0, 108);
        g.fill_rect(0, y, SCREEN_W, SOFT_H);
        g.set_color(88, 89, 172);
        g.fill_rect(0, y, SCREEN_W, 1);
        g.set_color(255, 255, 255);
        if let Some(c) = self.left_cmd() {
            g.draw_string(&c.label(), 2, y + 1, TOP | LEFT);
        }
        if let Some(c) = self.right_cmd() {
            g.draw_string(&c.label(), SCREEN_W - 2, y + 1, TOP | RIGHT);
        }
    }
}
