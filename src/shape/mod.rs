mod ellipse2d;
mod line2d;
mod rectangle2d;

use crate::canvas::Canvas;
use crate::Color;

pub trait Shape {
    fn draw(&self, canvas: &mut Canvas, color: &Color);

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color);
}

pub use ellipse2d::{draw_ellipse2d, draw_ellipse2d_filled, Ellipse2D};
pub use line2d::{draw_line2d, Line2D};
pub use rectangle2d::{draw_rect2d, draw_rect2d_filled, Rectangle2D};
