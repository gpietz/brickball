use crate::prelude::*;

pub fn ball_movement(game_state: Res<GameState>,
                     keyboard_input: Res<Input<KeyCode>>,
                     window_size: Res<WindowSize>,
                     mut ball_query: Query<(&mut Ball, &mut Transform, &mut Sprite)>,
                     mut paddle_query: Query<(&Transform, &Sprite, (With<Paddle>, Without<Ball>))>) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        print_ball_paddle_coordinates(&mut ball_query, &mut paddle_query);
    }

    if game_state.pause && !game_state.direct_ball_movement {
        return;
    }

    if let Ok((mut ball, mut ball_transform, ball_sprite)) = ball_query.single_mut() {
        if game_state.direct_ball_movement {
            if keyboard_input.just_pressed(KeyCode::C)
                && keyboard_input.pressed(KeyCode::LShift) {
                ball_transform.translation.x = 0.;
                ball_transform.translation.y = 0.;
                println!("Ball centered.");
            }
            else {
                if keyboard_input.pressed(KeyCode::Up) {
                    ball_transform.translation.y += f32::abs(ball.velocity.y);
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    ball_transform.translation.y -= f32::abs(ball.velocity.y);
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    ball_transform.translation.x -= f32::abs(ball.velocity.x);
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    ball_transform.translation.x += f32::abs(ball.velocity.x);
                }
            }
            return;
        }

        if ball.sticking_on_paddle {
            if let Ok((paddle_transform, paddle_sprite, _)) = paddle_query.single_mut() {
                ball_transform.translation.x = paddle_transform.translation.x;
                ball_transform.translation.y = paddle_transform.translation.y + 20.;
            }
        }
        else {
            let ball_x = ball_transform.translation.x;
            let ball_y = ball_transform.translation.y;
            let ball_diff = 20. + 10.; // Wall width + half of ball radius

            if ball_y > (window_size.get_top_boundary() - ball_diff) {
                ball.velocity.y = -ball.velocity.y;
            }
            if ball_x > (window_size.get_right_boundary() - ball_diff) {
                ball.velocity.x = -ball.velocity.x;
            }
            if ball_y < (window_size.get_bottom_boundary() - ball_diff) {
                ball.velocity.y = -ball.velocity.y;
            }
            if ball_x < (window_size.get_left_boundary() + ball_diff) {
                ball.velocity.x = -ball.velocity.x;
            }

            if let Ok((paddle_transform, paddle_sprite, _)) = paddle_query.single_mut() {
                if ball.velocity.y < 0. && is_paddle_collide(&ball_transform, &ball_sprite, &paddle_transform, &paddle_sprite) {
                    ball.velocity.y = -ball.velocity.y;
                }
            }

            ball_transform.translation.x += ball.velocity.x;
            ball_transform.translation.y += ball.velocity.y;
        }
    }
}

fn is_paddle_collide(ball_transform: &Transform,
                     ball_sprite: &Sprite,
                     paddle_transform: &Transform,
                     paddle_sprite: &Sprite) -> bool {
    let ball_x : f32 = ball_transform.translation.x - (ball_sprite.size.x / 2.);
    let ball_y : f32 = ball_transform.translation.y;
    let paddle_x1 : f32 = paddle_transform.translation.x - (paddle_sprite.size.x / 2.);
    let paddle_x2 : f32 = paddle_x1 + paddle_sprite.size.x;
    let paddle_y : f32 = paddle_transform.translation.y + paddle_sprite.size.y;
    ball_y <= paddle_y && ball_x >= paddle_x1 && ball_x <= paddle_x2
}

fn print_ball_paddle_coordinates(mut ball_query: &mut Query<(&mut Ball, &mut Transform, &mut Sprite)>,
                                 mut paddle_query: &mut Query<(&Transform, &Sprite, (With<Paddle>, Without<Ball>))>) {
    if let Ok((_, ball_transform, _)) = ball_query.single_mut() {
        if let Ok((paddle_transform, _, _)) = paddle_query.single_mut() {
            let ball_x : f32 = ball_transform.translation.x;
            let ball_y : f32 = ball_transform.translation.y;
            let paddle_x : f32 = paddle_transform.translation.x;
            let paddle_y : f32 = paddle_transform.translation.y + 10.;
            println!("X: {} - {} | Y: {} - {}", ball_x, paddle_x, ball_y, paddle_y);
        }
    }
}
