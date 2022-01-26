use crate::prelude::*;
use crate::rectangle::Rectangle;

pub fn ball_movement_system(keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    game_state: Res<GameState>,
    mut ev_reader: EventReader<GameCommandEvent>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>
) {
    if game_state.pause && !game_state.direct_ball_movement {
        return;
    }

    let window = windows.get_primary().unwrap();
    if let Ok((mut ball, mut ball_transform)) = ball_query.get_single_mut() {

        // direct ball movement (using cursor keys)
        if game_state.direct_ball_movement {

            // ball centering (Shift + C)
            if has_game_command(&mut ev_reader, GameCommand::CenterBall) {
                ball_transform.translation.x = 0.;
                ball_transform.translation.y = 0.;
                println!("Ball centered.");
            }

            // cursor keys for movement
            if keyboard_input.pressed(KeyCode::Up) {
                apply_ball_transform(&window, &mut ball_transform, |_, mut y| {
                    (*y) += f32::abs(ball.velocity.y);
                });
            }
            if keyboard_input.pressed(KeyCode::Down) {
                apply_ball_transform(&window, &mut ball_transform, |_, mut y| {
                    (*y) -= f32::abs(ball.velocity.y);
                });
            }
            if keyboard_input.pressed(KeyCode::Left) {
                apply_ball_transform(&window, &mut ball_transform, |mut x, _| {
                    (*x) -= f32::abs(ball.velocity.x);
                });
            }
            if keyboard_input.pressed(KeyCode::Right) {
                apply_ball_transform(&window, &mut ball_transform, |mut x, _| {
                    (*x) += f32::abs(ball.velocity.x);
                });
            }
            return;
        }
        else {
            if keyboard_input.just_pressed(KeyCode::Minus)
                || keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
                ball.change_speed(-1.0);
            }
            else if keyboard_input.just_pressed(KeyCode::Plus)
                || keyboard_input.just_pressed(KeyCode::NumpadAdd) {
                ball.change_speed(1.0);
            }

            ball_transform.translation.x += ball.velocity.x;
            ball_transform.translation.y += ball.velocity.y;
        }
    }
}

fn apply_ball_transform<F>(window: &Window, ball_transform: &mut Transform, transform: F)
    where F : Fn(&mut f32, &mut f32) {
    let mut x_pos = ball_transform.translation.x;
    let mut y_pos = ball_transform.translation.y;
    transform(&mut x_pos, &mut y_pos);
    if is_ball_in_range(&window, x_pos, y_pos) {
        ball_transform.translation.x = x_pos;
        ball_transform.translation.y = y_pos;
    }
}

fn is_ball_in_range(window: &Window, x: f32, y: f32) -> bool {
    let rect = Rectangle::create_from_window(&window);
    rect.is_inside_ref(&x, &y)
}
