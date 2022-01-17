use bevy_prototype_lyon::prelude::*;
use crate::prelude::*;

pub fn test_circle_system(game_state: Res<GameState>,
    mut commands: Commands,
    query: Query<Entity, With<TestCircle>>
) {
    let result = query.get_single();
    if result.is_ok() && game_state.test_circle_active
        || result.is_err() && !game_state.test_circle_active {
        return;
    }

    if game_state.test_circle_active {
        let circle = shapes::Circle { radius: 100.0, center: Vec2::new(0.0, 0.0) };
        let builder = GeometryBuilder::new().add(&circle);
        let draw_mode = DrawMode::Fill(FillMode {
            color: Color::RED,
            options: FillOptions::DEFAULT
        });

        commands.spawn_bundle(builder.build(draw_mode, Transform {
            translation: Vec3::new(0.0, 0.0, 50.0),
            ..Default::default()
        }))
        .insert(TestCircle::default());
    } else {
        let entity = result.unwrap();
        commands.entity(entity).despawn();
    }
}
