use crate::prelude::*;

const WALL_WIDTH : f32 = 20.;

/// Spawn the walls around the play field.
pub fn walls_spawn(windows: ResMut<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    let half_window_height = window.height() / 2.0;
    let half_window_width = window.width() / 2.0;
    let half_wall_width = WALL_WIDTH / 2.0;

    // Top
    add_wall(0.0, half_window_height - half_wall_width, window.width(), WALL_WIDTH, &mut commands);
    // Left
    add_wall(0.0 - half_window_width + half_wall_width, 0.0, WALL_WIDTH, window.height(), &mut commands);
    // Right
    add_wall(half_window_width - half_wall_width, 0.0, WALL_WIDTH, window.height(), &mut commands);
}

fn add_wall(x: f32, y: f32, width: f32, height: f32, commands: &mut Commands) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 100.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            color: Color::rgb(0.5, 0.5, 0.5),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Wall);
}
