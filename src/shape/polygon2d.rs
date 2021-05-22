use super::line2d::draw_line2d;
use crate::canvas::Canvas;
use crate::shape::Shape;
use crate::Color;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Polygon2D<'a> {
    xi: &'a [i32],
    yi: &'a [i32],
}

impl<'a> Polygon2D<'a> {
    pub fn new(xi: &'a [i32], yi: &'a [i32]) -> Self {
        return Polygon2D { xi, yi };
    }
}

impl<'a> Shape for Polygon2D<'a> {
    fn draw(&self, canvas: &mut Canvas, color: &Color, buffer: &mut [u8]) {
        draw_polygon2d(self.xi, self.yi, canvas, color, buffer);
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color, buffer: &mut [u8]) {
        draw_polygon2d_filled(self.xi, self.yi, canvas, color, buffer);
    }
    fn is_filled(&self) -> bool {
        false
    }
}

fn dist_sq(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)
}

/// To find orientation of ordered triplet (p, q, r).
///
/// Returns if p, q and r are:
///
/// 0 --> Colinear |
/// 1 --> Clockwise |
/// 2 --> Counterclockwise |
fn orientation(px: i32, py: i32, qx: i32, qy: i32, rx: i32, ry: i32) -> i32 {
    let val = (qy - py) * (rx - qx) - (qx - px) * (ry - qy);

    if val == 0 {
        // colinear
        return 0;
    }
    // clock or counter-clock wise
    return if val > 0 { 1 } else { 2 };
}

fn create_convex_hull(xi: &[i32], yi: &[i32]) -> Vec<usize> {
    let num_points = xi.len();
    let mut ipoints = vec![0usize; num_points];
    let mut start_index = 0;
    let mut hull: Vec<usize> = Vec::new();

    // Step 1: Find the bottom most point, i.e point with least y coordinate
    for i in 0..num_points {
        if yi[i] < yi[start_index] || (yi[i] == yi[start_index] && xi[i] < xi[start_index]) {
            ipoints[i] = start_index;
            start_index = i;
        } else {
            ipoints[i] = i;
        }
    }
    ipoints[0] = start_index;

    // Step 2: Arrange the points in increasing order of angle with x axis relative
    // to bottom most point
    ipoints[1..].sort_by(|a, b| {
        let o = orientation(
            xi[start_index],
            yi[start_index],
            xi[*a],
            yi[*a],
            xi[*b],
            yi[*b],
        );
        if o == 0 {
            return if dist_sq(xi[start_index], yi[start_index], xi[*b], yi[*b])
                >= dist_sq(xi[start_index], yi[start_index], xi[*b], yi[*b])
            {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        return if o == 2 {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    });

    // Step 3: Remove two or more points which make same angle with initial index
    let mut m: usize = 1;
    let mut k: usize = 1;
    while k < num_points {
        // Keep removing i while angle of i and i+1 is same with respect to p0
        while k < num_points - 1
            && orientation(
                xi[start_index],
                yi[start_index],
                xi[ipoints[k]],
                yi[ipoints[k]],
                xi[ipoints[k + 1]],
                yi[ipoints[k + 1]],
            ) == 0
        {
            k = k + 1;
        }
        ipoints[m] = ipoints[k];
        m = m + 1;
        k = k + 1;
    }
    if m < 4 {
        return hull;
    };

    // Step 4: Now iterate over sorted array and only add anti-clock wise angles to
    // final hull
    hull.push(ipoints[0]);
    hull.push(ipoints[1]);
    hull.push(ipoints[2]);

    for i in 3..m {
        // Keep removing top while the angle formed by
        // points next-to-top, top, and points[i] makes
        // a non-left turn
        while orientation(
            xi[hull[hull.len() - 2]],
            yi[hull[hull.len() - 2]],
            xi[hull[hull.len() - 1]],
            yi[hull[hull.len() - 1]],
            xi[ipoints[i]],
            yi[ipoints[i]],
        ) != 2
        {
            hull.pop();
        }
        hull.push(ipoints[i]);
    }

    return hull;
}

/// Renders a convex polygon formed by the given points. It calculates the convex hull using [Graham Scan Algorithm](https://en.wikipedia.org/wiki/Graham_scan)
///
/// The provided list of points should be an open path, i.e first and last
/// should not be same
pub fn draw_polygon2d(
    xi: &[i32],
    yi: &[i32],
    canvas: &mut Canvas,
    color: &Color,
    buffer: &mut [u8],
) {
    if xi.len() != yi.len() {
        return;
    }

    let hull = create_convex_hull(xi, yi);

    let hull_len = hull.len() - 1;
    for i in 0..hull_len {
        draw_line2d(
            xi[hull[i]],
            yi[hull[i]],
            xi[hull[i + 1]],
            yi[hull[i + 1]],
            canvas,
            color,
            buffer,
        );
    }
    // draw closing line from last index to first index
    draw_line2d(
        xi[hull[hull_len]],
        yi[hull[hull_len]],
        xi[hull[0]],
        yi[hull[0]],
        canvas,
        color,
        buffer,
    );
}

pub fn draw_polygon2d_filled(
    _xi: &[i32],
    _yi: &[i32],
    _canvas: &mut Canvas,
    _color: &Color,
    _buffer: &mut [u8],
) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    const WHITE: [u8; 4] = [255, 255, 255, 255];

    #[test]
    fn test_polygon_correct_convex_hull() {
        let mut buffer = vec![0u8; 4 * WIDTH * HEIGHT];
        let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();

        let xi: [i32; 10] = [127, 243, 62, 110, 93, 193, 135, 70, 258, 248];
        let yi: [i32; 10] = [320, 15, 162, 54, 311, 314, 290, 10, 163, 155];

        draw_polygon2d(&xi, &yi, &mut canvas, &color::WHITE, &mut buffer[..]);

        assert_eq!(canvas.get_color(70, 10, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(243, 15, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(258, 163, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(193, 314, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(127, 320, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(93, 311, &mut buffer[..]), &WHITE);
        assert_eq!(canvas.get_color(62, 162, &mut buffer[..]), &WHITE);
    }
}
