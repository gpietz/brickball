use crate::prelude::*;

pub fn ball_spawn(windows: ResMut<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    let ypos: f32 = -(window.height() / 2.) + 50.;
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., ypos , 101.),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(20.0, 20.0)),
            color: Color::GREEN,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Ball::default());
}
