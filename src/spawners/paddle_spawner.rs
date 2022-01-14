use crate::prelude::*;

/// Spawn player sprite.
pub fn paddle_spawn(windows: ResMut<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    let bottom = -window.height() / 2.0;
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., bottom + 30., 10.),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0, 20.0)),
            color: Color::WHITE,
                ..Default::default()
        },
        ..Default::default()
    })
    .insert(Paddle::default());
}
