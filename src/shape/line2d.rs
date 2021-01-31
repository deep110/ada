use crate::canvas::Canvas;
use crate::shape::Shape;
use crate::Color;

pub struct Line2D {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line2D {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Line2D { x1, y1, x2, y2 }
    }
}

impl Shape for Line2D {
    fn draw(&self, canvas: &mut Canvas, color: &Color) {
        draw_line(self.x1, self.y1, self.x2, self.y2, canvas, color);
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color) {
        draw_line(self.x1, self.y1, self.x2, self.y2, canvas, color);
    }
}

pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, canvas: &mut Canvas, color: &Color) {
    for i in x1..x2 {
        canvas.draw_point(i as usize, y1 as usize, color);
    }
}
