use crate::canvas::Canvas;
use crate::shape::Shape;
use crate::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    fn draw(&self, canvas: &mut Canvas, color: &Color, buffer: &mut [u8]) {
        draw_line2d(self.x1, self.y1, self.x2, self.y2, canvas, color, buffer);
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color, buffer: &mut [u8]) {
        draw_line2d(self.x1, self.y1, self.x2, self.y2, canvas, color, buffer);
    }

    fn is_filled(&self) -> bool {
        false
    }
}

/// Draws the line using [Bresenham's line Algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
///
/// It only involves integer calculations hence is fast than DDA
pub fn draw_line2d(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    canvas: &mut Canvas,
    color: &Color,
    buffer: &mut [u8],
) {
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
    let derror = ((my2 - my1) * 2).abs();
    let mut error = 0;
    let mut y = my1;
    for x in mx1..(mx2 + 1) {
        if steep {
            canvas.draw_point(y, x, color, buffer);
        } else {
            canvas.draw_point(x, y, color, buffer);
        }
        error += derror;
        if error > dx {
            y += if my2 > my1 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const WHITE: [u8; 4] = [255, 255, 255, 255];

    #[test]
    fn test_line_new() {
        let l = Line2D::new(0, 0, 10, 10);
        assert_eq!(l.x1, 0);
        assert_eq!(l.y1, 0);
        assert_eq!(l.x2, 10);
        assert_eq!(l.y2, 10);
    }

    #[test]
    fn test_line_slope_less_than_one() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        draw_line2d(0, 0, 4, 2, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(0, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(1, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(2, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(3, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(4, 2, &mut buffer[..]), &WHITE);
    }

    #[test]
    fn test_line_slope_one() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        draw_line2d(0, 0, 4, 4, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(0, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(1, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(2, 2, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(3, 3, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(4, 4, &mut buffer[..]), &WHITE);
    }

    #[test]
    fn test_line_slope_greater_than_one() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        draw_line2d(0, 0, 2, 4, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(0, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(0, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(1, 2, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(1, 3, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(2, 4, &mut buffer[..]), &WHITE);
    }

    #[test]
    fn test_line_slope_infinite() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        draw_line2d(0, 0, 0, 4, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(0, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(0, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(0, 2, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(0, 3, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(0, 4, &mut buffer[..]), &WHITE);
    }

    #[test]
    fn test_line_slope_zero() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        draw_line2d(0, 0, 4, 0, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(0, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(1, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(2, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(3, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(4, 0, &mut buffer[..]), &WHITE);
    }

    #[test]
    fn test_line_point_reorder() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        draw_line2d(4, 2, 0, 0, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(0, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(1, 0, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(2, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(3, 1, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(4, 2, &mut buffer[..]), &WHITE);
    }
}
