use super::line2d::draw_line2d;
use crate::canvas::Canvas;
use crate::shape::Shape;
use crate::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct QuadraticBezier2D {
    start: (i32, i32),
    end: (i32, i32),
    control: (i32, i32),
}

impl QuadraticBezier2D {
    pub fn new(start: (i32, i32), end: (i32, i32), control: (i32, i32)) -> Self {
        QuadraticBezier2D {
            start,
            end,
            control,
        }
    }
}

impl Shape for QuadraticBezier2D {
    fn draw(&self, canvas: &mut Canvas, color: &Color) {
        draw_quadratic_bezier2d(self.start, self.end, self.control, canvas, color);
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color) {
        draw_quadratic_bezier2d(self.start, self.end, self.control, canvas, color);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CubicBezier2D {
    start: (i32, i32),
    end: (i32, i32),
    control_a: (i32, i32),
    control_b: (i32, i32),
}

impl CubicBezier2D {
    pub fn new(
        start: (i32, i32),
        end: (i32, i32),
        control_a: (i32, i32),
        control_b: (i32, i32),
    ) -> Self {
        CubicBezier2D {
            start,
            end,
            control_a,
            control_b,
        }
    }
}

impl Shape for CubicBezier2D {
    fn draw(&self, canvas: &mut Canvas, color: &Color) {
        draw_cubic_bezier2d(
            self.start,
            self.end,
            self.control_a,
            self.control_b,
            canvas,
            color,
        );
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color) {
        draw_cubic_bezier2d(
            self.start,
            self.end,
            self.control_a,
            self.control_b,
            canvas,
            color,
        );
    }
}

/// Draws the Quadratic Bezier Curve using function from https://pomax.github.io/bezierinfo/#control
pub fn draw_quadratic_bezier2d(
    start: (i32, i32),
    end: (i32, i32),
    control: (i32, i32),
    canvas: &mut Canvas,
    color: &Color,
) {
    let quadratic_bezier_curve = |t: f32| {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let x = (start.0 as f32 * mt2) + (2.0 * control.0 as f32 * mt * t) + (end.0 as f32 * t2);
        let y = (start.1 as f32 * mt2) + (2.0 * control.1 as f32 * mt * t) + (end.1 as f32 * t2);
        // round to nearest pixel, to avoid ugly line artifacts
        (x.round() as i32, y.round() as i32)
    };

    let distance = |point_a: (i32, i32), point_b: (i32, i32)| {
        (((point_a.0 - point_b.0).pow(2) + (point_a.1 - point_b.1).pow(2)) as f32).sqrt()
    };

    // Approximate curve's length by adding distance between control points.
    let curve_length_bound: f32 = distance(start, control) + distance(control, end);

    // Use hyperbola function to give shorter curves a bias in number of line
    // segments.
    let num_segments: i32 = ((curve_length_bound.powi(2) + 800.0).sqrt() / 8.0) as i32;

    // Sample points along the curve and connect them with line segments.
    let t_interval = 1f32 / (num_segments as f32);
    let mut t1 = 0f32;
    let mut s1 = quadratic_bezier_curve(t1);
    for i in 0..num_segments {
        let t2 = (i as f32 + 1.0) * t_interval;
        let s2 = quadratic_bezier_curve(t2);
        draw_line2d(s1.0, s1.1, s2.0, s2.1, canvas, color);
        t1 = t2;
        s1 = quadratic_bezier_curve(t1);
    }
}

/// Draws the Cubic Bezier Curve using function from https://pomax.github.io/bezierinfo/#control
///
/// Source Code is taken from [imageproc library](https://github.com/image-rs/imageproc/blob/master/src/drawing/bezier.rs)
pub fn draw_cubic_bezier2d(
    start: (i32, i32),
    end: (i32, i32),
    control_a: (i32, i32),
    control_b: (i32, i32),
    canvas: &mut Canvas,
    color: &Color,
) {
    let cubic_bezier_curve = |t: f32| {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        let x = (start.0 as f32 * mt3)
            + (3.0 * control_a.0 as f32 * mt2 * t)
            + (3.0 * control_b.0 as f32 * mt * t2)
            + (end.0 as f32 * t3);
        let y = (start.1 as f32 * mt3)
            + (3.0 * control_a.1 as f32 * mt2 * t)
            + (3.0 * control_b.1 as f32 * mt * t2)
            + (end.1 as f32 * t3);
        // round to nearest pixel, to avoid ugly line artifacts
        (x.round() as i32, y.round() as i32)
    };

    let distance = |point_a: (i32, i32), point_b: (i32, i32)| {
        (((point_a.0 - point_b.0).pow(2) + (point_a.1 - point_b.1).pow(2)) as f32).sqrt()
    };

    // Approximate curve's length by adding distance between control points.
    let curve_length_bound: f32 =
        distance(start, control_a) + distance(control_a, control_b) + distance(control_b, end);

    let num_segments: i32 = ((curve_length_bound.powi(2) + 800.0).sqrt() / 8.0) as i32;

    // Sample points along the curve and connect them with line segments.
    let t_interval = 1f32 / (num_segments as f32);
    let mut t1 = 0f32;
    for i in 0..num_segments {
        let t2 = (i as f32 + 1.0) * t_interval;
        let s1 = cubic_bezier_curve(t1);
        let s2 = cubic_bezier_curve(t2);
        draw_line2d(s1.0, s1.1, s2.0, s2.1, canvas, color);
        t1 = t2;
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
    fn bench_render_quadratic_bezier(b: &mut Bencher) {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut buffer[..]).unwrap();

        b.iter(|| {
            draw_quadratic_bezier2d((10, 500), (500, 10), (40, 40), &mut canvas, &color::WHITE)
        });
    }

    #[bench]
    fn bench_render_cubic_bezier(b: &mut Bencher) {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT, &mut buffer[..]).unwrap();

        b.iter(|| {
            draw_cubic_bezier2d((110, 150), (210, 30), (25, 190), (210, 250), &mut canvas, &color::WHITE)
        });
    }
}
