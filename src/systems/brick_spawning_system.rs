use crate::prelude::*;
use crate::levels::*;

pub const BRICK_SIZE: [u8; 2] = [38, 25];

pub fn brick_spawning_system(levels: Res<Levels>,
    windows: Res<Windows>,
    mut ev_reader: EventReader<GameCommandEvent>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    brick_query: Query<Entity, With<Brick>>
) {
    let mut bricks_removed = false;

    if has_game_command(&mut ev_reader, GameCommand::RemoveBricks) {
        remove_all_bricks(&mut commands, &brick_query);
        bricks_removed = true;
    }

    if !is_update_required(&game_state) {
        return;
    }

    // Remove all existing bricks from the play field
    if !bricks_removed {
        remove_all_bricks(&mut commands, &brick_query);
    }

    let next_level = get_next_level(&game_state);
    let level_data = levels.get_level_data(next_level);

    let primary_window = windows.get_primary().unwrap();
    let mut line_nr = 0;
    for data in level_data {
        insert_brick_line(next_level, line_nr, data, &primary_window, &levels, &mut commands);
        line_nr += 1;
    }

    game_state.update_level(next_level);
    println!("Level activated: {}", next_level);
}

fn remove_all_bricks(commands: &mut Commands, brick_query: &Query<Entity, With<Brick>>) {
    for brick_entity in brick_query.iter() {
        commands.entity(brick_entity).despawn();
    }
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

fn insert_brick_line(level: u8, line_nr: u8, data: &String,
    window: &Window,
    levels: &Res<Levels>,
    mut commands: &mut Commands
) {
    // calculate first brick position
    let mut x_pos: f32 = 60.;
    let mut y_pos: f32 = -30.;
    y_pos -= (f32::from(BRICK_SIZE[1]) + 2.0) * (f32::from(line_nr) + 1.0);

    x_pos = transform_x(&window, x_pos);
    y_pos = transform_y(&window, y_pos);

    for brick_char in data.chars() {
        if brick_char != ' ' {
            let brick_color = levels.get_brick_color(level, brick_char);
            let hits_required = if brick_char.is_ascii_uppercase() { 2 } else { 1 };
            add_brick(x_pos, y_pos, brick_color, &mut commands, hits_required);
        }
        x_pos += f32::from(BRICK_SIZE[0]) + 2.0;
    }
}

fn add_brick(x: f32, y: f32, color: Color, mut commands: &mut Commands, hits_required: u8) {
    let width  = f32::from(BRICK_SIZE[0]);
    let height = f32::from(BRICK_SIZE[1]);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 10.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            color,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Collider::Brick)
    .insert(Brick {
        hits_required,
        ..Default::default()
    });
}
