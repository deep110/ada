mod line2d;
mod rectangle2d;

use crate::canvas::Canvas;
use crate::Color;

pub trait Shape {
    fn draw(&self, canvas: &mut Canvas, color: &Color);

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color);
}

pub use line2d::{Line2D, draw_line2d};
pub use rectangle2d::{Rectangle2D, draw_rect2d};
