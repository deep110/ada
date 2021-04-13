//! The `color` module is a utility module for defining RGBA colors

/// defines RGBA color
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    /// red component
    pub r: u8,
    /// green component
    pub g: u8,
    /// blue component
    pub b: u8,
    /// alpha component
    pub a: u8,
}

impl Color {
    /// creates a new RGBA color from rgba values
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    /// creates a new RGBA color from rgb values and alpha 255 
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
}

#[macro_export]
macro_rules! color {
    { $A:expr, $B:expr, $C:expr } => {
        Color {r: $A, g: $B, b: $C, a:255}
    };
    { $A:expr, $B:expr, $C:expr, $D:expr } => {
        Color {r: $A, g: $B, b: $C, a:$D}
    };
}

pub const BLACK: Color = color!(0, 0, 0);

pub const WHITE: Color = color!(255, 255, 255);

pub const RED: Color = color!(255, 0, 0);

pub const GREEN: Color = color!(0, 255, 0);

pub const BLUE: Color = color!(0, 0, 255);
