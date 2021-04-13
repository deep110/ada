use super::line2d::draw_line2d;
use crate::canvas::Canvas;
use crate::shape::Shape;
use crate::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ellipse2D {
    xc: i32,
    yc: i32,
    width_radius: i32,
    height_radius: i32,
}

impl Ellipse2D {
    pub fn new(xc: i32, yc: i32, width_radius: i32, height_radius: i32) -> Self {
        Ellipse2D {
            xc,
            yc,
            width_radius,
            height_radius,
        }
    }
}

impl Shape for Ellipse2D {
    fn draw(&self, canvas: &mut Canvas, color: &Color) {
        draw_ellipse2d(
            self.xc,
            self.yc,
            self.width_radius,
            self.height_radius,
            canvas,
            color,
        );
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color) {
        draw_ellipse2d_filled(
            self.xc,
            self.yc,
            self.width_radius,
            self.height_radius,
            canvas,
            color,
        );
    }
}

/// Draws ellipse using [Mid Point Ellipse Algorithm](https://www.javatpoint.com/computer-graphics-midpoint-ellipse-algorithm)
///
/// For circle which is just a special case when major and minor axis are equal,
/// we move to more optimized [Midpoint Circle Algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm).
/// It exploits eight the fold symmetry in circles.
pub fn draw_ellipse2d(
    xc: i32,
    yc: i32,
    width_radius: i32,
    height_radius: i32,
    canvas: &mut Canvas,
    color: &Color,
) {
    // if major axis length and minor axis length is same, means it is a circle
    // circle have eight fold symmetry, so we can use more optimized algorithm
    if width_radius == height_radius {
        draw_circle(xc, yc, width_radius, canvas, color);
        return;
    }

    let wr2 = width_radius * width_radius;
    let hr2 = height_radius * height_radius;
    let mut x = -width_radius;
    let mut y = 0;
    let mut e2 = height_radius;
    let mut dx = (1 + 2 * x) * e2 * e2;
    let mut dy = x * x;
    let mut err = dx + dy;

    while x <= 0 {
        canvas.draw_point(xc + x, yc + y, color);
        canvas.draw_point(xc - x, yc + y, color);
        canvas.draw_point(xc + x, yc - y, color);
        canvas.draw_point(xc - x, yc - y, color);
        e2 = 2 * err;
        if e2 >= dx {
            x += 1;
            dx += 2 * hr2;
            err += dx;
        }

        if e2 <= dy {
            y += 1;
            dy += 2 * wr2;
            err += dy;
        }
    }

    while y < height_radius {
        y += 1;
        canvas.draw_point(xc, yc + y, color);
        canvas.draw_point(xc, yc - y, color);
    }
}

/// Draws the circle using [Midpoint Circle Algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm)
#[inline(always)]
fn draw_circle(xc: i32, yc: i32, radius: i32, canvas: &mut Canvas, color: &Color) {
    let mut x = 0i32;
    let mut y = radius;
    let mut d = 1 - radius;

    while x <= y {
        canvas.draw_point(xc + x, yc + y, color);
        canvas.draw_point(xc + y, yc + x, color);
        canvas.draw_point(xc - y, yc + x, color);
        canvas.draw_point(xc - x, yc + y, color);
        canvas.draw_point(xc - x, yc - y, color);
        canvas.draw_point(xc - y, yc - x, color);
        canvas.draw_point(xc + y, yc - x, color);
        canvas.draw_point(xc + x, yc - y, color);

        x += 1;
        if d < 0 {
            d += 2 * x + 3;
        } else {
            y -= 1;
            d += 2 * (x - y) + 5;
        }
    }
}

pub fn draw_ellipse2d_filled(
    xc: i32,
    yc: i32,
    width_radius: i32,
    height_radius: i32,
    canvas: &mut Canvas,
    color: &Color,
) {
    if width_radius == height_radius {
        draw_circle_filled(xc, yc, width_radius, canvas, color);
        return;
    }

    let wr2 = width_radius * width_radius;
    let hr2 = height_radius * height_radius;
    let mut x = -width_radius;
    let mut y = 0;
    let mut e2 = height_radius;
    let mut dx = (1 + 2 * x) * e2 * e2;
    let mut dy = x * x;
    let mut err = dx + dy;

    while x <= 0 {
        draw_line2d(xc + x, yc + y, xc - x, yc + y, canvas, color);
        draw_line2d(xc + x, yc - y, xc - x, yc - y, canvas, color);
        e2 = 2 * err;
        if e2 >= dx {
            x += 1;
            dx += 2 * hr2;
            err += dx;
        }

        if e2 <= dy {
            y += 1;
            dy += 2 * wr2;
            err += dy;
        }
    }

    while y < height_radius - 1 {
        y += 1;
        draw_line2d(xc, yc - y, xc, yc + y, canvas, color);
    }
}

#[inline(always)]
fn draw_circle_filled(xc: i32, yc: i32, radius: i32, canvas: &mut Canvas, color: &Color) {
    let mut x = 0i32;
    let mut y = radius;
    let mut d = 1 - radius;

    while x <= y {
        draw_line2d(xc + x, yc + y, xc - x, yc + y, canvas, color);
        draw_line2d(xc + y, yc + x, xc - y, yc + x, canvas, color);
        draw_line2d(xc + y, yc - x, xc - y, yc - x, canvas, color);
        draw_line2d(xc + x, yc - y, xc - x, yc - y, canvas, color);

        x += 1;
        if d < 0 {
            d += 2 * x + 3;
        } else {
            y -= 1;
            d += 2 * (x - y) + 5;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use test::Bencher;

    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    #[bench]
    fn bench_render_circle(b: &mut Bencher) {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut buffer[..]).unwrap();

        b.iter(|| draw_ellipse2d(200, 200, 200, 200, &mut canvas, &color::WHITE));
    }

    #[bench]
    fn bench_render_ellipse(b: &mut Bencher) {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut buffer[..]).unwrap();

        b.iter(|| draw_ellipse2d(200, 200, 199, 200, &mut canvas, &color::WHITE));
    }

    #[bench]
    fn bench_render_circle_filled(b: &mut Bencher) {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut buffer[..]).unwrap();

        b.iter(|| draw_ellipse2d_filled(200, 200, 200, 200, &mut canvas, &color::WHITE));
    }

    #[bench]
    fn bench_render_ellipse_filled(b: &mut Bencher) {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut buffer[..]).unwrap();

        b.iter(|| draw_ellipse2d_filled(200, 200, 199, 200, &mut canvas, &color::WHITE));
    }
}
