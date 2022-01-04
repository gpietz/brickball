use crate::prelude::*;

/// Spawn player sprite.
pub fn paddle_spawn(windows: ResMut<Windows>, mut commands: Commands, materials: Res<Materials>) {
    let window = windows.get_primary().unwrap();
    let bottom = -window.height() / 2.;
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_material.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 30., 10.),
            ..Default::default()
        },
        sprite: Sprite::new(Vec2::new(200.0, 20.)),
        ..Default::default()
    })
    .insert(Paddle::default());
}
