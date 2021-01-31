use crate::{shape, Color};

pub struct Canvas<'a> {
    // height of canvas
    height: usize,
    // width of canvas
    width: usize,

    buffer: &'a mut [u8],
}

impl<'a> Canvas<'a> {
    pub fn new(width: usize, height: usize, buffer: &'a mut [u8]) -> Canvas<'a> {
        Canvas {
            width: width,
            height: height,
            buffer: buffer,
        }
    }

    pub fn draw(&mut self, shape: &dyn shape::Shape, color: Color, is_filled: bool) {
        if is_filled {
            shape.draw_filled(self, color)
        } else {
            shape.draw(self, color)
        }
    }

    pub fn clear(&mut self, color: &Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.draw_point(i, j, color);
            }
        }
    }

    #[inline(always)]
    pub(crate) fn draw_point(&mut self, x: usize, y: usize, color: &Color) {
        let si = (x * self.width + y) * 4;
        self.buffer[si] = color.r;
        self.buffer[si + 1] = color.g;
        self.buffer[si + 2] = color.b;
        self.buffer[si + 3] = color.a;
    }
}
