use crate::{errors, shape, Color, Result};

/// Container for drawing the shapes
pub struct Canvas<'a> {
    /// height of canvas
    height: usize,
    /// width of canvas
    width: usize,
    /// buffer to write pixel values
    buffer: &'a mut [u8],

    h_minus_1: usize,
    w_minus_1: usize,
}

impl<'a> Canvas<'a> {
    /// creates a new canvas. Takes:
    ///
    /// ```
    /// width: width of the canvas
    /// height: height of the canvas
    /// buffer: unsigned int array of sufficient length to write pixel values
    /// ```
    ///
    /// Canvas does not own the buffer
    ///
    /// Returns unsupported error if buffer is not of sufficient length
    pub fn new(width: usize, height: usize, buffer: &'a mut [u8]) -> Result<Canvas<'a>> {
        if buffer.len() < 4 * width * height {
            return errors::unsupported_error("Insufficient buffer length");
        }
        Ok(Canvas {
            width: width,
            height: height,
            buffer: buffer,
            w_minus_1: width-1,
            h_minus_1: height-1,
        })
    }

    /// Draw the shape within the bounds in the canvas
    pub fn draw(&mut self, shape: &dyn shape::Shape, color: &Color, is_filled: bool) {
        if is_filled {
            shape.draw_filled(self, color)
        } else {
            shape.draw(self, color)
        }
    }

    /// fill the canvas buffer with specified color
    pub fn clear(&mut self, color: &Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.draw_point_internal(i, j, color);
            }
        }
    }

    /// Get the pixel's color value at specified coordinate
    #[inline]
    pub fn get_color(&self, x: i32, y: i32) -> &[u8] {
        // TODO: take care of mapping from user's coordinate plane to canvas
        // Use origin shifting
        let si = (x as usize + y as usize * self.width) * 4;

        return &self.buffer[si..si + 4];
    }

    #[inline(always)]
    pub(crate) fn draw_point(&mut self, x: i32, y: i32, color: &Color) {
        // TODO: take care of mapping from user's coordinate plane to canvas
        // Use origin shifting
        self.draw_point_internal(x as usize, y as usize, color)
    }

    #[inline(always)]
    fn draw_point_internal(&mut self, x: usize, y: usize, color: &Color) {
        if x > self.w_minus_1 || y > self.h_minus_1 {
            return;
        }
        let si = (x + y * self.width) * 4;
        self.buffer[si] = color.r;
        self.buffer[si + 1] = color.g;
        self.buffer[si + 2] = color.b;
        self.buffer[si + 3] = color.a;
    }
}
