use crate::{shape, Color, Result};

/// Container for drawing the shapes
pub struct Canvas {
    /// height of canvas
    height: usize,
    /// width of canvas
    width: usize,

    h_minus_1: usize,
    w_minus_1: usize,
}

impl Canvas {
    /// creates a new canvas. Takes:
    ///
    /// ```ignore
    /// width: width of the canvas
    /// height: height of the canvas
    /// ```
    ///
    /// Canvas does not create a internal buffer, it takes a buffer during draw
    pub fn new(width: usize, height: usize) -> Result<Canvas> {
        Ok(Canvas {
            width: width,
            height: height,
            w_minus_1: width - 1,
            h_minus_1: height - 1,
        })
    }

    /// Draw the shape within the bounds in the canvas
    ///
    /// Provide a mutable buffer to fill the values. It assumes color mode to
    /// RGBA.
    pub fn draw(&mut self, shape: &dyn shape::Shape, color: &Color, buffer: &mut [u8]) {
        if shape.is_filled() {
            shape.draw_filled(self, color, buffer)
        } else {
            shape.draw(self, color, buffer)
        }
    }

    /// fill the canvas buffer with specified color
    pub fn clear(&mut self, color: &Color, buffer: &mut [u8]) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.draw_point_internal(i, j, color, buffer);
            }
        }
    }

    /// Get the pixel's color value at specified coordinate
    #[inline]
    pub fn get_color<'a>(&self, x: i32, y: i32, buffer: &'a mut [u8]) -> &'a [u8] {
        // TODO: take care of mapping from user's coordinate plane to canvas
        // Use origin shifting
        let si = (x as usize + y as usize * self.width) * 4;

        return &(buffer[si..si + 4]);
    }

    #[inline(always)]
    pub(crate) fn draw_point(&mut self, x: i32, y: i32, color: &Color, buffer: &mut [u8]) {
        // TODO: take care of mapping from user's coordinate plane to canvas
        // Use origin shifting
        self.draw_point_internal(x as usize, y as usize, color, buffer)
    }

    #[inline(always)]
    fn draw_point_internal(&mut self, x: usize, y: usize, color: &Color, buffer: &mut [u8]) {
        if x > self.w_minus_1 || y > self.h_minus_1 {
            return;
        }
        let si = (x + y * self.width) * 4;
        buffer[si] = color.r;
        buffer[si + 1] = color.g;
        buffer[si + 2] = color.b;
        buffer[si + 3] = color.a;
    }
}
