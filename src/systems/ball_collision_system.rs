use crate::BRICK_SIZE;
use crate::prelude::*;

const BALL_HITS_PEDDLE: &str = "impact1";
const BALL_HITS_BRICK:  &str = "impact2";
const BALL_KILLS_BRICK: &str = "impact3";
const BALL_HITS_WALL:   &str = "impact1";

pub fn ball_collision_system(windows: Res<Windows>,
    game_state: Res<GameState>,
    mut ev_reader: EventReader<GameCommandEvent>,
    mut ev_play_sound: EventWriter<PlaySoundEvent>,
    mut ball_query: Query<(&mut Ball, &mut Transform, &mut Sprite)>,
    mut paddle_query : Query<(&Transform, &Sprite), (With<Paddle>, Without<Ball>)>,
    mut bricks_query: Query<(Entity, &mut Sprite, &mut Transform, &mut Brick), (Without<Ball>, Without<Paddle>)>,
    mut commands: Commands
) {
    if has_game_command(&mut ev_reader, GameCommand::ShowCoordinates) {
        print_ball_paddle_coordinates(&mut ball_query, &mut paddle_query)
    }

    if game_state.pause || game_state.paddle_owns_ball {
        return;
    }

    if let Ok((mut ball, mut ball_transform, ball_sprite)) = ball_query.get_single_mut() {
        let window = windows.get_primary().unwrap();
        let ball_x = ball_transform.translation.x;
        let ball_y = ball_transform.translation.y;
        let ball_diff = WALL_WIDTH + 10.; // Wall width + half of ball radius

        // wall collision --------------------------------------------------------------------------
        // if ball_y > (window_top(&window) - ball_diff) {
        //     if !wall_collision {
        //         ball.velocity.y = -ball.velocity.y;
        //     }
        //     ball.wall_collision = true;
        // } else if ball_x > (window_right(&window) - ball_diff) {
        //     if !wall_collision {
        //         ball.velocity.x = -ball.velocity.x;
        //     }
        //     ball.wall_collision = true;
        // } else if ball_y < (window_bottom(&window) - ball_diff) {
        //     if !wall_collision {
        //         ball.velocity.y = -ball.velocity.y;
        //     }
        //     ball.wall_collision = true;
        // } else if ball_x < (window_left(&window) + ball_diff) {
        //     if !wall_collision {
        //         ball.velocity.x = -ball.velocity.x;
        //     }
        //     ball.wall_collision = true;
        // } else {
        //     ball.wall_collision = false;
        // }

        // if !wall_collision && ball.wall_collision {
        //     ev_play_sound.send(PlaySoundEvent::normal(BALL_HITS_WALL))
        // }

        // paddle collision ------------------------------------------------------------------------
        // let mut paddle_collision = false;
        // if let Ok((paddle_transform, paddle_sprite)) = paddle_query.get_single_mut() {
        //     if ball.velocity.y < 0. {
        //         if is_paddle_collide(&ball_transform, &ball_sprite,
        //                              &paddle_transform, &paddle_sprite) {
        //             ball.velocity.y = -ball.velocity.y;
        //             ball.clear_brick_velocity_change();
        //             paddle_collision = true;
        //             ev_play_sound.send(PlaySoundEvent::normal(BALL_HITS_PEDDLE))
        //         }
        //     }
        // }

        // brick collision -------------------------------------------------------------------------
        // if !wall_collision {
        //     let mut ball_rect = Rectangle::create_from_sprite(&ball_transform, &ball_sprite);
        //     for (brick_entity, mut brick_sprite, mut brick_transform, mut brick) in bricks_query.iter_mut() {
        //         if is_brick_collide(&ball_rect, &brick_transform)
        //             && check_brick_velocity_change(&mut ball, &ball_sprite, &ball_transform) {
        //             ball.update_brick_velocity_change(&ball_transform);
        //
        //             let mut ball_rect_2 = Rectangle::create_from(&ball_rect);
        //             // ball_rect_2.transform(ball.velocity.x, 0.);
        //             // if is_brick_collide(&ball_rect_2, &brick_transform) {
        //                 //ball.velocity.x = -ball.velocity.x;
        //             // }
        //
        //             // ball_rect_2.copy_from(&ball_rect);
        //             // ball_rect_2.transform(0., ball.velocity.y);
        //             // if is_brick_collide(&ball_rect_2, &brick_transform) {
        //                 ball.velocity.y = -ball.velocity.y;
        //                 println!("New velocity: {}", ball.velocity.y);
        //             //}
        //
        //             brick.hits_required -= 1;
        //             if brick.hits_required <= 0 {
        //                 commands.entity(brick_entity).despawn();
        //                 ev_play_sound.send(PlaySoundEvent::normal(BALL_KILLS_BRICK))
        //             } else {
        //                 brick_sprite.color = Color::rgb(0.85, 0.85, 0.85);
        //                 ev_play_sound.send(PlaySoundEvent::normal(BALL_HITS_BRICK))
        //             }
        //         }
        //     }
        // }
    }
}

/// Prints ball and paddle coordinates on the console.
fn print_ball_paddle_coordinates(
    ball_query: &mut Query<(&mut Ball, &mut Transform, &mut Sprite)>,
    paddle_query: &mut Query<(&Transform, &Sprite), (With<Paddle>, Without<Ball>)>
) {
    let (_, ball_transform, _) = ball_query.single_mut();
    let (paddle_transform, _) = paddle_query.single_mut();
    let ball_x : f32 = ball_transform.translation.x;
    let ball_y : f32 = ball_transform.translation.y;
    let paddle_x : f32 = paddle_transform.translation.x;
    let paddle_y : f32 = paddle_transform.translation.y + 10.;
    println!("X: {} - {} | Y: {} - {}", ball_x, paddle_x, ball_y, paddle_y);
}

// fn is_paddle_collide(ball_transform: &Transform,
//     ball_sprite: &Sprite,
//     paddle_transform: &Transform,
//     paddle_sprite: &Sprite) -> bool {
//     if ball_sprite.custom_size.is_none() || paddle_sprite.custom_size.is_none() {
//         return false;
//     }
//     let ball_size   = ball_sprite.custom_size.unwrap();
//     let paddle_size = paddle_sprite.custom_size.unwrap();
//     let ball_x      = ball_transform.translation.x - (ball_size.x / 2.0);
//     let ball_y      = ball_transform.translation.y;
//     let paddle_x1   = paddle_transform.translation.x - (paddle_size.x / 2.0);
//     let paddle_x2   = paddle_x1 + paddle_size.x;
//     let paddle_y    = paddle_transform.translation.y + paddle_size.y;
//     ball_y <= paddle_y && ball_x >= paddle_x1 && ball_x <= paddle_x2
// }

fn is_brick_collide(ball_rect: &Rectangle, brick_transform: &Transform) -> bool {
    let brick_width  = f32::from(BRICK_SIZE[0]);
    let brick_height = f32::from(BRICK_SIZE[1]);
    let brick_rect   = Rectangle::create_from_values(&brick_transform, brick_width, brick_height);
    ball_rect.intersects_with(&brick_rect)
}

fn check_brick_velocity_change(ball: &Ball, ball_sprite: &Sprite, transform: &Transform) -> bool {
    if ball.brick_velocity_change.is_none() {
        return true;
    }
    let ball_size = ball_sprite.custom_size.unwrap();
    let brick_velocity_change = ball.brick_velocity_change.unwrap();
    let distance = (brick_velocity_change.y - transform.translation.y).abs();
    distance > (ball_size.y * 1.5)
}
