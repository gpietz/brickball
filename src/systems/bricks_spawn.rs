use std::borrow::Borrow;
use crate::prelude::*;
use crate::levels::*;

pub const BRICK_SIZE: [u8; 2] = [38, 25];

pub fn bricks_spawn(brick_materials : Res<BrickMaterials>,
                    window_size     : Res<WindowSize>,
                    mut game_state  : ResMut<GameState>,
                    mut commands    : Commands,
                    brick_query     : Query<(Entity), (With<Brick>)>) {
    if !is_update_required(&game_state) {
        return;
    }

    // Remove all existing bricks from the play field
    for brick_entity in brick_query.iter() {
        commands.entity(brick_entity).despawn();
    }

    let next_level = get_next_level(&game_state);
    let level_data = get_level_data(next_level);
    if level_data.is_none() {
        panic!("No level data found for level {}!", next_level);
    }

    let mut line_nr = 0;
    for data in level_data.unwrap() {
        insert_brick_line(next_level, line_nr, data, &window_size, &brick_materials, &mut commands);
        line_nr += 1;
    }

    game_state.update_level(next_level);
}

fn is_update_required(game_state: &ResMut<GameState>) -> bool {
    game_state.current_level[0] == 0 || game_state.current_level[0] != game_state.current_level[1]
}

fn get_next_level(game_state: &ResMut<GameState>) -> u8 {
    let next_level = game_state.current_level[0];
    return if next_level > 0 {
        next_level
    } else {
        1
    }
}

fn insert_brick_line(level: u8, line_nr: u8, data: String,
                     window_size     : &Res<WindowSize>,
                     brick_materials : &Res<BrickMaterials>,
                     mut commands    : &mut Commands) {
    // calculate first brick position
    let mut x_pos: f32 = 55.;
    let mut y_pos: f32 = -30.;
    y_pos -= (f32::from(BRICK_SIZE[1]) + 2.) * (f32::from(line_nr) + 1.);

    x_pos = window_size.transform_x(x_pos);
    y_pos = window_size.transform_y(y_pos);

    for brick_char in data.chars() {
        if brick_char != ' ' {
            let brick_material = brick_materials.get_material(level, brick_char);
            let hits_required = if brick_char.is_ascii_uppercase() { 2 } else { 1 };
            add_brick(x_pos, y_pos, brick_material, &mut commands, hits_required);
        }
        x_pos += f32::from(BRICK_SIZE[0]) + 2.;
    }
}

fn add_brick(x: f32, y: f32,
             material: Handle<ColorMaterial>,
             mut commands: &mut Commands,
             hits_required: u8) {
    let width  = f32::from(BRICK_SIZE[0]);
    let height = f32::from(BRICK_SIZE[1]);
    commands.spawn_bundle(SpriteBundle {
        material,
        transform: Transform {
            translation: Vec3::new(x, y, 110.0),
            ..Default::default()
        },
        sprite: Sprite::new(Vec2::new(width, height)),
        ..Default::default()
    })
    .insert(Collider::Brick)
    .insert(Brick {
        hits_required,
        ..Default::default()
    });
}
