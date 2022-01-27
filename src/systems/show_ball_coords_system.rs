use crate::prelude::*;

pub fn show_ball_coords_system(
    game_state: Res<GameState>,
    game_assets: Res<GameAssets>,
    mut query: Query<(Entity, &mut Text), With<BallCoordsDisplay>>,
    mut commands: Commands,
    ball_query: Query<&Transform, With<Ball>>
) {
    // ---------------------
    // Spawning & Despawning
    // ---------------------

    let query_result = query.get_single_mut();
    if query_result.is_err() && game_state.show_ball_coordinates {
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Pos: ".to_string(),
                        style: TextStyle {
                            font: game_assets.debug_font.clone(),
                            font_size: 16.0,
                            color: Color::YELLOW
                        }
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: game_assets.debug_font.clone(),
                            font_size: 16.0,
                            color: Color::YELLOW
                        }
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BallCoordsDisplay);
    } else if query_result.is_ok() && !game_state.show_ball_coordinates {
        let (entity, _) = query_result.unwrap();
        commands.entity(entity).despawn();
    } else if query_result.is_ok() {
        // -------------------------------
        // Update ball coordinates display
        // -------------------------------
        let (_, mut text) = query_result.unwrap();
        if let Ok(ball_transform) = ball_query.get_single() {
            text.sections[1].value = format!("{}, {}",
                                             ball_transform.translation.x,
                                             ball_transform.translation.y);
        }
    }
}
