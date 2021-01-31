mod line2d;

use crate::canvas::Canvas;
use crate::Color;

pub trait Shape {
    fn draw(&self, canvas: &mut Canvas, color: &Color);

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color);
}

pub use line2d::*;
