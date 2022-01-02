use std::borrow::BorrowMut;
use crate::player_movement;
use crate::prelude::*;

pub fn ball_movement(game_state: Res<GameState>,
                     mut ball_query: Query<(&Ball, &mut Transform)>,
                     mut paddle_query: Query<(&Transform, (With<Paddle>, Without<Ball>))>
) {
    if game_state.pause {
        return;
    }

    if let Ok((ball, mut ball_transform)) = ball_query.single_mut() {
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

        //println!("ball: {} / {}", ball_transform.translation.x, ball_transform.translation.y);
    }
}
