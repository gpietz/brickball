use crate::prelude::*;

pub fn paddle_movement_system(keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut game_state: ResMut<GameState>,
    mut paddle_query: Query<(&mut Transform, &mut Paddle, &Sprite)>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<Paddle>)>
) {
    if let Ok((mut paddle_transform, mut paddle, paddle_sprite)) = paddle_query.get_single_mut() {
        // check cursor keys for direction
        let mut velocity = 0.0;
        if !game_state.direct_ball_movement && !game_state.pause {
            velocity = if keyboard_input.pressed(KeyCode::Left) {
                -1.0
            } else if keyboard_input.pressed(KeyCode::Right) {
                1.0
            } else {
                0.0
            }
        }

        // move paddle
        if velocity != 0.0 {
            let delta = paddle.velocity * PADDLE_SPEED * TIME_STEP;
            if is_in_range(&windows, &paddle_sprite, &mut paddle_transform, delta) {
                paddle_transform.translation.x += delta;
                paddle.velocity = velocity;
            }
            else {
                paddle.velocity = 0.0;
            }
        } else {
            paddle.velocity = 0.0;
        }

        // move ball with paddle if it's stuck on it
        if game_state.paddle_owns_ball && !game_state.direct_ball_movement {
            let mut ball_transform = ball_query.single_mut();
            let paddle_size = paddle_sprite.custom_size.unwrap();
            ball_transform.translation.x = paddle_transform.translation.x;
            ball_transform.translation.y = paddle_transform.translation.y + paddle_size.y - 3.0;
        }
    }
}

// pub fn player_movement(keyboard_input: Res<Input<KeyCode>>,
//     window_size: Res<WindowSize>,
//     mut game_state: ResMut<GameState>,
//     ,
//     mut ball_query: Query<(&mut Ball, With<Ball>, Without<Paddle>)>) {
//
//     if let Ok(mut transform) = paddle_query.get_single_mut() {
//         if let Ok((mut ball, _, _)) = ball_query.get_single_mut() {
//             if keyboard_input.pressed(KeyCode::Space) && !game_state.pause {
//                 if ball.sticking_on_paddle {
//                     ball.sticking_on_paddle = false;
//                 }
//             } else if keyboard_input.pressed(KeyCode::R) {
//                 if keyboard_input.pressed(KeyCode::LShift) {
//                     transform.translation.x = 0.;
//                 }
//                 if !ball.sticking_on_paddle {
//                     ball.sticking_on_paddle = true;
//                 }
//             }
//
//             if keyboard_input.just_pressed(KeyCode::Plus)
//                 || keyboard_input.just_pressed(KeyCode::NumpadAdd) {
//                     if keyboard_input.pressed(KeyCode::LShift) {
//                         game_state.activate_next_level();
//                     } else {
//                         ball.change_speed(1.);
//                     }
//             } else if keyboard_input.just_pressed(KeyCode::Minus)
//                 || keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
//                     if keyboard_input.pressed(KeyCode::LShift) {
//                         game_state.activate_previous_level();
//                     } else {
//                         ball.change_speed(-1.);
//                     }
//             }
//         }
//
//         // pause state
//         if keyboard_input.just_pressed(KeyCode::P)  {
//             game_state.pause = !game_state.pause;
//             if game_state.pause {
//                 println!("*** PAUSE ACTIVATED ***");
//             } else {
//                 println!("*** PAUSE DEACTIVATED ***");
//             }
//         };
//
//         // left/right paddle movement
//         if !game_state.direct_ball_movement {
//             let dir = if keyboard_input.pressed(KeyCode::Left) {
//                 -1
//             } else if keyboard_input.pressed(KeyCode::Right) {
//                 1
//             } else {
//                 return;
//             };
//
//             if !is_in_range(&transform, dir, window_size.width)
//                 || game_state.pause {
//                 return;
//             }
//
//             let delta = (dir as f32) * PADDLE_SPEED * TIME_STEP;
//             transform.translation.x += delta;
//         }
//     }
// }

fn is_in_range(windows: &Res<Windows>, sprite: &Sprite, transform: &mut Transform, delta: f32) -> bool {
    let window = windows.get_primary().unwrap();
    let window_hw = window.width() / 2.0;
    let paddle_hw = sprite.custom_size.unwrap().x / 2.0;
    return if delta < 0.0 {
        ((transform.translation.x - paddle_hw) - WALL_WIDTH) + delta > -window_hw
    }
    else {
        ((transform.translation.x + paddle_hw) + WALL_WIDTH) + delta < window_hw
    };
}
