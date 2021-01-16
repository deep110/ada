use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A 2d point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    /// x-coordinate.
    pub x: f32,
    /// y-coordinate.
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_new() {
        let point = Point::new(10., 10.);
        assert_eq!(point.x, 10f32);
        assert_eq!(point.y, 10f32);
    }

    #[test]
    fn point_add_new() {
        let point1 = Point::new(10., 10.);
        let point2 = Point::new(2., 10.);

        let added_point = point1 + point2;
        assert_eq!(added_point.x, 12.);
        assert_eq!(added_point.y, 20.);
    }

    #[test]
    fn point_add_inplace() {
        let point1 = Point::new(10., 10.);
        let mut point2 = Point::new(2., 10.);

        point2 += point1;
        assert_eq!(point2.x, 12.);
        assert_eq!(point2.y, 20.);
    }

    #[test]
    fn point_sub_new() {
        let point1 = Point::new(10., 10.);
        let point2 = Point::new(2., 10.);

        let sub_point = point1 - point2;
        assert_eq!(sub_point.x, 8.);
        assert_eq!(sub_point.y, 0.);
    }

    #[test]
    fn point_sub_inplace() {
        let point1 = Point::new(10., 10.);
        let mut point2 = Point::new(2., 10.);

        point2 -= point1;
        assert_eq!(point2.x, -8.);
        assert_eq!(point2.y, 0.);
    }
}
