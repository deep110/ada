// forbid unsafe code
#![forbid(unsafe_code)]

mod canvas;
pub mod color;
mod point;
pub mod shape;

pub use canvas::Canvas;
pub use color::Color;
pub use point::Point;
