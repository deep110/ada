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
        draw_line2d(self.x1, self.y1, self.x2, self.y2, canvas, color);
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color) {
        draw_line2d(self.x1, self.y1, self.x2, self.y2, canvas, color);
    }
}

/// Draws the line using [Bresenham's line Algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
///
/// It only involves integer calculations hence is fast than DDA
pub fn draw_line2d(x1: i32, y1: i32, x2: i32, y2: i32, canvas: &mut Canvas, color: &Color) {
    let mut mx1 = x1;
    let mut mx2 = x2;
    let mut my1 = y1;
    let mut my2 = y2;
    let mut steep = false;

    if (mx1 - mx2).abs() < (my1 - my2).abs() {
        std::mem::swap(&mut mx1, &mut my1);
        std::mem::swap(&mut mx2, &mut my2);
        steep = true;
    }
    if mx1 > mx2 {
        std::mem::swap(&mut mx1, &mut mx2);
        std::mem::swap(&mut my1, &mut my2);
    }
    let dx = mx2 - mx1;
    let dy = my2 - my1;
    let mut p = dy * 2 - dx;
    for x in mx1..mx2 {
        if steep {
            canvas.draw_point(my1 as usize, x as usize, color);
        } else {
            canvas.draw_point(x as usize, my1 as usize, color);
        }
        if p >= 0 {
            my1 = my1 + 1;
            p = p + 2 * dy - 2 * dx;
        } else {
            p = p + 2 * dy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_new() {
        let _ = Line2D::new(0, 0, 10, 10);
    }
}
