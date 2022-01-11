use std::collections::HashMap;
use bevy::prelude::*;

pub struct BrickMaterials {
    pastel_colors: HashMap<char, Handle<ColorMaterial>>,
    arkanoid_colors: HashMap<char, Handle<ColorMaterial>>,
    vintage_colors: HashMap<char, Handle<ColorMaterial>>,
    level9_colors: HashMap<char, Handle<ColorMaterial>>,
    level10_colors: HashMap<char, Handle<ColorMaterial>>,
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
        let mut vintage_colors = HashMap::new();
        vintage_colors.insert('a', create_material(&mut materials, [239, 210, 121]));
        vintage_colors.insert('b', create_material(&mut materials, [149, 203, 233]));
        vintage_colors.insert('c', create_material(&mut materials, [2, 71, 105]));
        vintage_colors.insert('d', create_material(&mut materials, [175, 215, 117]));
        vintage_colors.insert('e', create_material(&mut materials, [44, 87, 0]));
        vintage_colors.insert('f', create_material(&mut materials, [222, 157, 127]));
        vintage_colors.insert('g', create_material(&mut materials, [127, 157, 222]));
        vintage_colors.insert('h', create_material(&mut materials, [0, 87, 44]));
        vintage_colors.insert('i', create_material(&mut materials, [117, 215, 175]));
        vintage_colors.insert('j', create_material(&mut materials, [105, 71, 2]));
        vintage_colors.insert('k', create_material(&mut materials, [233, 203, 149]));
        vintage_colors.insert('l', create_material(&mut materials, [121, 210, 239]));
        let mut level9_colors = HashMap::new();
        level9_colors.insert('b', create_material(&mut materials, [17, 17, 17]));
        level9_colors.insert('w', create_material(&mut materials, [238, 238, 238]));
        level9_colors.insert('c', create_material(&mut materials, [236, 113, 80]));
        level9_colors.insert('s', create_material(&mut materials, [179, 58, 47]));
        let mut level10_colors = HashMap::new();
        level10_colors.insert('r', create_material(&mut materials, [216, 0, 0]));
        level10_colors.insert('b', create_material(&mut materials, [112, 104, 0]));
        level10_colors.insert('o', create_material(&mut materials, [248, 171, 0]));
        level10_colors.insert('f', create_material(&mut materials, [248, 56, 0]));
        level10_colors.insert('w', create_material(&mut materials, [255, 255, 255]));
        level10_colors.insert('e', create_material(&mut materials, [255, 224, 168]));
        Self {
            pastel_colors,
            arkanoid_colors,
            vintage_colors,
            level9_colors,
            level10_colors
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
            6 => &self.vintage_colors,
            7 => &self.vintage_colors,
            8 => &self.pastel_colors,
            9 => &self.level9_colors,
            10 => &self.level10_colors,
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
