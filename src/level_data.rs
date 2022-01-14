use std::collections::HashMap;
use std::slice::Iter;
use crate::prelude::*;

pub struct LevelData {
    pub color_palette: u8,
    pub level_data: Vec<String>
}

impl LevelData {
    pub fn new(color_palette: u8, level_data: Iter<'_, &str>) -> Self {
        let mut vec = Vec::new();
        for line in level_data {
            vec.push(line.to_string());
        }
        Self {
            color_palette,
            level_data: vec
        }
    }
}
