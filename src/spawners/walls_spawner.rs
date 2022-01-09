use crate::prelude::*;

const WALL_WIDTH : f32 = 20.;

/// Spawn the walls around the play field.
pub fn walls_spawn(windows: ResMut<Windows>, materials: Res<Materials>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    // Top
    add_wall(0.0, 0.0 + (window.height() / 2.) - (WALL_WIDTH / 2.), window.width(), WALL_WIDTH,
             &mut commands, materials.wall_material.clone());
    // Left
    add_wall(0.0 - (window.width() / 2.) + (WALL_WIDTH / 2.), 0.0, WALL_WIDTH, window.height(),
             &mut commands, materials.wall_material.clone());
    // Right
    add_wall( 0.0 + (window.width() / 2.) - (WALL_WIDTH / 2.), 0., WALL_WIDTH, window.height(),
             &mut commands, materials.wall_material.clone());
}

fn add_wall(x: f32, y: f32, width: f32, height: f32,
            commands: &mut Commands,
            material: Handle<ColorMaterial>) {
    commands.spawn_bundle(SpriteBundle {
        material,
        transform: Transform {
            translation: Vec3::new(x, y, 100.0),
            ..Default::default()
        },
        sprite: Sprite::new(Vec2::new(width, height)),
        ..Default::default()
    })
    .insert(Wall);
}
