use crate::ColorMode;

pub struct Canvas<'a> {
    // height of canvas
    height: usize,
    // width of canvas
    width: usize,

    stride: usize,

    buffer: &'a mut [u8],

    color_mode: ColorMode,
}

impl <'a> Canvas<'a> {
    pub fn new(width: usize, height: usize, buffer: &'a mut [u8]) -> Canvas<'a> {
        Canvas {
            width: width,
            height: height,
            stride: 4,
            buffer: buffer,
            color_mode: ColorMode::RGBA,
        }
    }

    pub fn new_with_color_mode(width: usize, height: usize, buffer: &'a mut [u8], color_mode: ColorMode) -> Canvas<'a> {
        let stride = if color_mode == ColorMode::RGB {
            3
        } else {
            4
        };

        Canvas {
            width: width,
            height: height,
            stride: stride,
            buffer: buffer,
            color_mode: color_mode,
        }
    }

    pub fn draw(&mut self) {

    }

    pub fn clear(&mut self) {
        
    }
}
