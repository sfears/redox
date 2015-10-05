/// A color
#[derive(Copy, Clone)]
pub struct Color {
    pub data: u32,
}

impl Color {
    /// Create a new color from RGB
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { data: 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32) }
    }

    /// Set the alpha
    pub fn alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { data: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32) }
    }
}
