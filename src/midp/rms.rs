//! `javax.microedition.rms.RecordStore` backed by files in the `save` folder
//! (see `paths`), wrapped the same way as the game's `w` helper class.

use std::fs;
use std::path::PathBuf;

fn data_dir() -> PathBuf {
    crate::paths::save_dir()
}

fn path(name: &str) -> PathBuf {
    data_dir().join(format!("{name}.rms"))
}

struct Store {
    name: String,
    records: Vec<Vec<u8>>,
}

impl Store {
    fn flush(&self) {
        let mut out = Vec::new();
        for r in &self.records {
            out.extend_from_slice(&(r.len() as u32).to_be_bytes());
            out.extend_from_slice(r);
        }
        let _ = fs::create_dir_all(data_dir());
        let p = path(&self.name);
        let tmp = p.with_extension("tmp");
        if fs::write(&tmp, &out).is_ok() {
            let _ = fs::rename(&tmp, &p);
        }
    }
}

/// The game's `w` class.
#[derive(Default)]
pub struct W {
    a: Option<Store>,
}

impl W {
    /// `b(String)`: openRecordStore(name, true)
    pub fn open(&mut self, name: &str) {
        let mut records = Vec::new();
        if let Ok(d) = fs::read(path(name)) {
            let mut i = 0;
            while i + 4 <= d.len() {
                let n = u32::from_be_bytes([d[i], d[i + 1], d[i + 2], d[i + 3]]) as usize;
                i += 4;
                if i + n > d.len() {
                    break;
                }
                records.push(d[i..i + n].to_vec());
                i += n;
            }
        }
        self.a = Some(Store { name: name.into(), records });
    }

    /// `a(int)`: read the first n records as writeUTF strings.
    pub fn read_all(&self, n: i32) -> Vec<Option<String>> {
        let mut v = vec![None; n.max(0) as usize];
        if let Some(s) = &self.a {
            for (i, r) in s.records.iter().enumerate().take(v.len()) {
                if r.len() >= 2 {
                    let len = u16::from_be_bytes([r[0], r[1]]) as usize;
                    v[i] = Some(String::from_utf8_lossy(&r[2..(2 + len).min(r.len())]).into_owned());
                }
            }
        }
        v
    }

    /// `a(String)`: addRecord(writeUTF(s))
    pub fn add(&mut self, s: &str) {
        if let Some(st) = &mut self.a {
            let b = s.as_bytes();
            let mut r = Vec::with_capacity(b.len() + 2);
            r.extend_from_slice(&(b.len() as u16).to_be_bytes());
            r.extend_from_slice(b);
            st.records.push(r);
        }
    }

    /// `b()`: getNumRecords, -1 if not open
    pub fn count(&self) -> i32 {
        self.a.as_ref().map(|s| s.records.len() as i32).unwrap_or(-1)
    }

    /// `c(String)`: deleteRecordStore
    pub fn delete(&mut self, name: &str) {
        if self.a.as_ref().map(|s| s.name == name).unwrap_or(false) {
            return; // can't delete an open store
        }
        let _ = fs::remove_file(path(name));
    }

    /// `a()`: closeRecordStore
    pub fn close(&mut self) {
        if let Some(s) = self.a.take() {
            s.flush();
        }
    }
}
