use crate::prelude::*;

pub struct LevelColor {
    pub color_palette: u8,
    pub symbol: char,
    pub color: Color,
}

impl LevelColor {
    pub fn new(color_palette: u8, symbol:  char, colors: [u8; 3]) -> Self {
        let color = Color::rgb(f32::from(colors[0]) / 255.0,
                               f32::from(colors[1]) / 255.0,
                               f32::from(colors[2]) / 255.0);
        Self {
            color_palette,
            symbol,
            color
        }
    }
}
