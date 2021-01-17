// forbid unsafe code
#![forbid(unsafe_code)]

mod canvas;
pub mod color;
mod point;
mod shape;

pub use canvas::Canvas;
pub use color::*;
pub use point::Point;
