//! Resources from the original JAR, embedded at build time.

use crate::midp::gfx::{Image, Img};
use std::rc::Rc;

include!(concat!(env!("OUT_DIR"), "/assets_gen.rs"));

/// `getResourceAsStream`
pub fn get(path: &str) -> Option<&'static [u8]> {
    FILES.iter().find(|(p, _)| *p == path).map(|(_, d)| *d)
}

/// `Image.createImage(path)`; None stands for the IOException / null result.
pub fn image(path: &str) -> Img {
    let img = get(path).and_then(Image::from_png).map(Rc::new);
    if img.is_none() {
        eprintln!("missing image {path}");
    }
    img
}
