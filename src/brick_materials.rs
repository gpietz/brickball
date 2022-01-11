use std::collections::HashMap;
use bevy::prelude::*;

pub struct BrickMaterials {
    pastel_colors: HashMap<char, Handle<ColorMaterial>>,
    arkanoid_colors: HashMap<char, Handle<ColorMaterial>>,
    vintage_colors: HashMap<char, Handle<ColorMaterial>>
}

impl BrickMaterials {
    pub fn new(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        let mut pastel_colors = HashMap::new();
        pastel_colors.insert('y', create_material(&mut materials, [255, 247, 165]));
        pastel_colors.insert('p', create_material(&mut materials, [255, 165, 224]));
        pastel_colors.insert('b', create_material(&mut materials, [165, 179, 255]));
        pastel_colors.insert('g', create_material(&mut materials, [191, 255, 165]));
        pastel_colors.insert('o', create_material(&mut materials, [255, 203, 165]));
        pastel_colors.insert('x', create_material(&mut materials, [56, 56, 56]));
        let mut arkanoid_colors = HashMap::new();
        arkanoid_colors.insert('w', create_material(&mut materials, [252, 252, 252]));
        arkanoid_colors.insert('o', create_material(&mut materials, [252, 116, 96]));
        arkanoid_colors.insert('l', create_material(&mut materials, [60, 188, 252]));
        arkanoid_colors.insert('g', create_material(&mut materials, [128, 208, 16]));
        arkanoid_colors.insert('r', create_material(&mut materials, [216, 40, 0]));
        arkanoid_colors.insert('b', create_material(&mut materials, [0, 112, 236]));
        arkanoid_colors.insert('p', create_material(&mut materials, [252, 116, 180]));
        arkanoid_colors.insert('y', create_material(&mut materials, [252, 152, 56]));
        arkanoid_colors.insert('s', create_material(&mut materials, [188, 188, 188]));
        arkanoid_colors.insert('d', create_material(&mut materials, [240, 188, 60]));
        let vintage_colors = HashMap::new();
        Self {
            pastel_colors,
            arkanoid_colors,
            vintage_colors
        }
    }

    pub fn get_material(&self, level: u8, brick_symbol: char) -> Handle<ColorMaterial> {
        let brick_symbol = make_lowercase(brick_symbol);
        let material_map = self.get_material_map(level);
        if !material_map.contains_key(&brick_symbol) {
            panic!("Symbol '{}' not found in color map for level {}!", brick_symbol, level);
        }
        let lh = material_map.get(&brick_symbol).unwrap();
        (*lh).clone()
    }

    fn get_material_map(&self, level: u8) -> &HashMap<char, Handle<ColorMaterial>> {
        match level {
            1 => &self.pastel_colors,
            2 => &self.arkanoid_colors,
            3 => &self.arkanoid_colors,
            4 => &self.arkanoid_colors,
            5 => &self.pastel_colors,
            _ => panic!("Color map not found for level {}!", level)
        }
    }
}

fn create_material(materials: &mut ResMut<Assets<ColorMaterial>>, colors: [u8; 3]) -> Handle<ColorMaterial> {
    let color_material = Color::rgb(f32::from(colors[0]) / 255.,
                                    f32::from(colors[1]) / 255.,
                                    f32::from(colors[2]) / 255.);
    materials.add(color_material.into())
}

// Sometimes this languages drives me nuts .....
fn make_lowercase(chr: char) -> char {
    let chr_lowercase : String = chr.to_lowercase().collect();
    chr_lowercase.chars().next().unwrap()
}

// Discussion about to_lowercase:
// https://stackoverflow.com/questions/35716159/what-is-the-motivation-of-rusts-tolowercase
