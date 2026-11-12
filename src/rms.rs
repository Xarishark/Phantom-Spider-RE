//! `javax.microedition.rms.RecordStore` backed by files in a `save` folder
//! next to the executable, wrapped the same way as the game's `w` helper class.

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

/// The per-user folder older builds saved to; still used as a fallback.
fn legacy_dir() -> PathBuf {
    let base = if cfg!(windows) {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
    };
    base.unwrap_or_else(|| PathBuf::from(".")).join("PhantomSpider")
}

/// `save/` next to the exe. Override with `PHANTOM_SPIDER_DATA`; if the exe's
/// folder isn't writable, falls back to the per-user folder. Saves from the
/// per-user folder are copied over once so existing progress carries along.
pub fn data_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        if let Some(d) = std::env::var_os("PHANTOM_SPIDER_DATA") {
            return PathBuf::from(d);
        }
        let Some(exe_dir) = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf())) else {
            return legacy_dir();
        };
        let dir = exe_dir.join("save");
        let probe = dir.join(".write-test");
        let writable = fs::create_dir_all(&dir).is_ok() && fs::write(&probe, b"").is_ok();
        let _ = fs::remove_file(&probe);
        if !writable {
            return legacy_dir();
        }
        // one-time migration from the per-user folder
        let has_saves = fs::read_dir(&dir)
            .map(|rd| rd.flatten().any(|e| e.path().extension().is_some_and(|x| x == "rms")))
            .unwrap_or(false);
        if !has_saves {
            if let Ok(rd) = fs::read_dir(legacy_dir()) {
                for e in rd.flatten() {
                    let p = e.path();
                    if p.extension().is_some_and(|x| x == "rms") {
                        if let Some(name) = p.file_name() {
                            let _ = fs::copy(&p, dir.join(name));
                        }
                    }
                }
            }
        }
        dir
    })
    .clone()
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
