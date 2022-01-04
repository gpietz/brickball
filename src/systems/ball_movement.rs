use std::borrow::BorrowMut;
use crate::prelude::*;

pub fn ball_movement(game_state: Res<GameState>,
                     keyboard_input: Res<Input<KeyCode>>,
                     mut ball_query: Query<(&Ball, &mut Transform)>,
                     mut paddle_query: Query<(&Transform, (With<Paddle>, Without<Ball>))>
) {
    if game_state.pause {
        return;
    }

    if let Ok((ball, mut ball_transform)) = ball_query.single_mut() {
        if game_state.direct_ball_movement {
            if keyboard_input.just_pressed(KeyCode::C)
                && keyboard_input.pressed(KeyCode::LShift) {
                ball_transform.translation.x = 0.;
                ball_transform.translation.y = 0.;
                println!("Ball centered.");
            }
            else {
                if keyboard_input.pressed(KeyCode::Up) {
                    ball_transform.translation.y += ball.velocity.x;
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    ball_transform.translation.y -= ball.velocity.x;
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    ball_transform.translation.x -= ball.velocity.y;
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    ball_transform.translation.x += ball.velocity.y;
                }
            }
            return;
        }

        if ball.sticking_on_paddle {
            if let Ok((paddle_transform, _)) = paddle_query.single_mut() {
                ball_transform.translation.x = paddle_transform.translation.x;
                ball_transform.translation.y = paddle_transform.translation.y + 20.;
            }
        }
        else {
            ball_transform.translation.x += ball.velocity.x;
            ball_transform.translation.y += ball.velocity.y;
        }
    }
}
