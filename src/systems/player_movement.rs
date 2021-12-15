use crate::prelude::*;

pub fn player_movement(keyboard_input: Res<Input<KeyCode>>,
    window_size: Res<WindowSize>,
    mut paddle_query: Query<(&MoveSpeed, &mut Position, &mut Transform, With<Paddle>)>,
    mut ball_query: Query<(&mut Ball, (With<Ball>, Without<Paddle>))>)
{
    if let Ok((speed, mut position, mut transform, _)) = paddle_query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            if let Ok((mut ball, _)) = ball_query.single_mut() {
                if ball.sticking_on_paddle {
                    ball.sticking_on_paddle = false;
                }
            }
        } else if keyboard_input.pressed(KeyCode::R) {
            if let Ok((mut ball, _)) = ball_query.single_mut() {
                if !ball.sticking_on_paddle {
                    ball.sticking_on_paddle = true;
                }
            }
        }

        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1
        } else if keyboard_input.pressed(KeyCode::Right) {
            1
        } else {
            return;
        };

        if !is_in_range(&position, dir, window_size.width) {
            return;
        }

        let delta = (dir as f32) * speed.0 * TIME_STEP;
        transform.translation.x += delta;
        position.x += delta;
    }
}

fn is_in_range(pos: &Position, direction: i32, window_width: f32) -> bool {
    return if direction < 0 {
        ((pos.x - 100.) - 10.) > -(window_width / 2.)
    }
    else {
        ((pos.x + 100.) + 10.) < (window_width / 2.)
    };
}
