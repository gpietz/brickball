use crate::prelude::*;

pub fn ball_spawn(windows: ResMut<Windows>, mut commands: Commands, materials: Res<Materials>) {
    let window = windows.get_primary().unwrap();
    let ypos = -(window.height() / 2.) + 50.;
    commands.spawn_bundle(SpriteBundle {
        material: materials.ball_material.clone(),
        transform: Transform {
            translation: Vec3::new(0., ypos , 10.),
            ..Default::default()
        },
        sprite: Sprite::new(Vec2::new(20., 20.)),
        ..Default::default()
    })
    .insert(Ball::default())
    .insert(Position { x: 0., y: ypos })
    .insert(MoveSpeed::new(150.));
}
