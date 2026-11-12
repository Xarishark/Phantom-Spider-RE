//! Stand-ins for the phone APIs the game was written against (MIDP 2.0,
//! MMAPI, RMS and the Nokia UI extensions), implemented on a 128x128
//! software framebuffer. The SDL3 front end in `main.rs` drives them.

pub mod assets;
pub mod audio;
pub mod font;
pub mod gfx;
pub mod lang;
pub mod lcdui;
pub mod rms;
