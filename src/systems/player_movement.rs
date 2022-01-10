use crate::prelude::*;

pub fn player_movement(keyboard_input: Res<Input<KeyCode>>,
    window_size: Res<WindowSize>,
    mut game_state: ResMut<GameState>,
    mut paddle_query: Query<(&mut Transform, With<Paddle>)>,
    mut ball_query: Query<(&mut Ball, (With<Ball>, Without<Paddle>))>)
{
    if let Ok((mut transform, _)) = paddle_query.single_mut() {
        if let Ok((mut ball, _)) = ball_query.single_mut() {
            if keyboard_input.pressed(KeyCode::Space) && !game_state.pause {
                if ball.sticking_on_paddle {
                    ball.sticking_on_paddle = false;
                }
            } else if keyboard_input.pressed(KeyCode::R) {
                if keyboard_input.pressed(KeyCode::LShift) {
                    transform.translation.x = 0.;
                }
                if !ball.sticking_on_paddle {
                    ball.sticking_on_paddle = true;
                }
            } else if keyboard_input.just_pressed(KeyCode::F1) {
                game_state.direct_ball_movement = !game_state.direct_ball_movement;
                if game_state.direct_ball_movement {
                    println!("*** DIRECT BALL ACTIVATED ENABLED ****");
                } else {
                    println!("*** DIRECT BALL ACTIVATED DISABLED ****");
                }
            }

            if keyboard_input.just_pressed(KeyCode::Plus)
                || keyboard_input.just_pressed(KeyCode::NumpadAdd) {
                    if keyboard_input.pressed(KeyCode::LShift) {
                        game_state.current_level[0] += 1;
                    } else {
                        ball.change_speed(1.);
                    }
            } else if keyboard_input.just_pressed(KeyCode::Minus)
                || keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
                    if keyboard_input.pressed(KeyCode::LShift) {
                        game_state.current_level[0] -= 0;
                    } else {
                        ball.change_speed(-1.);
                    }
            }
        }

        // pause state
        if keyboard_input.just_pressed(KeyCode::P)  {
            game_state.pause = !game_state.pause;
            if game_state.pause {
                println!("*** PAUSE ACTIVATED ***");
            } else {
                println!("*** PAUSE DEACTIVATED ***");
            }
        };

        // left/right paddle movement
        if !game_state.direct_ball_movement {
            let dir = if keyboard_input.pressed(KeyCode::Left) {
                -1
            } else if keyboard_input.pressed(KeyCode::Right) {
                1
            } else {
                return;
            };

            if !is_in_range(&transform, dir, window_size.width)
                || game_state.pause {
                return;
            }

            let delta = (dir as f32) * PADDLE_SPEED * TIME_STEP;
            transform.translation.x += delta;
        }
    }
}

fn is_in_range(transform: &Transform, direction: i32, window_width: f32) -> bool {
    return if direction < 0 {
        ((transform.translation.x - 100.) - 30.) > -(window_width / 2.)
    }
    else {
        ((transform.translation.x + 100.) + 30.) < (window_width / 2.)
    };
}
