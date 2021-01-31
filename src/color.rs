/// struct for color
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    // red component
    pub r: u8,

    pub g: u8,

    pub b: u8,

    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
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
