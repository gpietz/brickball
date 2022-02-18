use crate::prelude::*;
use bevy_prototype_lyon::prelude::*;

// TODO Add to ball.rs??
pub fn ball_collision_field_system(
    game_state: Res<GameState>,
    mut commands: Commands,
    ball_query: Query<(&Transform, &Sprite, &Ball), (With<Ball>, Without<CollisionField>)>,
    mut field_query: Query<(Entity, &mut Transform), (With<CollisionField>, Without<Ball>)>
) {
    if let Ok((ball_transform, ball_sprite, ball)) = ball_query.get_single() {
        if let Ok((field_entity, mut field_transform)) = field_query.get_single_mut() {
            if game_state.show_ball_collision_field {
                field_transform.translation.x = ball_transform.translation.x;
                field_transform.translation.y = ball_transform.translation.y;
            } else {
                commands.entity(field_entity).despawn();
            }
        } else if game_state.show_ball_collision_field {
            let collision_rect = shapes::Rectangle {
                extents: ball_sprite.custom_size.unwrap(),
                origin: RectangleOrigin::Center
            };
            let builder = GeometryBuilder::new().add(&collision_rect);
            let draw_mode = DrawMode::Outlined {
                fill_mode: FillMode::color(Color::NONE),
                outline_mode: StrokeMode::new(Color::RED, 1.0)
            };

            commands.spawn_bundle(builder.build(draw_mode, Transform {
                translation: Vec3::new(
                    ball_transform.translation.x,
                    ball_transform.translation.y,
                    200.0
                ),
                ..Default::default()
            }))
            .insert(CollisionField);
        }
    }
}
