// Embeds every file under assets/ into the executable.
use std::{env, fs, path::Path};

fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
    let mut entries: Vec<_> = fs::read_dir(dir).unwrap().flatten().collect();
    entries.sort_by_key(|e| e.path());
    for e in entries {
        let p = e.path();
        if p.is_dir() {
            walk(&p, root, out);
        } else {
            let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.push(rel);
        }
    }
}

fn main() {
    let root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");
    println!("cargo:rerun-if-changed=assets");
    let mut files = Vec::new();
    walk(&root, &root, &mut files);
    let mut src = String::from("pub static FILES: &[(&str, &[u8])] = &[\n");
    for f in &files {
        src.push_str(&format!(
            "    (\"/{f}\", include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/assets/{f}\"))),\n"
        ));
    }
    src.push_str("];\n");
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("assets_gen.rs");
    fs::write(out, src).unwrap();

    // Windows icon/resource is optional; nothing else to do.
}
