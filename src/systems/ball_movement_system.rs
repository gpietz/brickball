use crate::prelude::*;
use crate::rectangle::Rectangle;
use crate::systems::ball_collision_context::BallCollisionContext;
use bevy::sprite::collide_aabb::Collision;

const BALL_HITS_PEDDLE: &str = "impact1";
const BALL_HITS_BRICK:  &str = "impact2";
const BALL_KILLS_BRICK: &str = "impact3";
const BALL_HITS_WALL:   &str = "impact1";

static MOVEMENT_PRECISION: f32 = 1.0;
const PLAY_SOUND_ENABLED: bool = true;

pub fn ball_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    game_state: Res<GameState>,
    mut ev_reader: EventReader<GameCommandEvent>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    paddle_query: Query<
        (&Transform, &Sprite),
        (With<Paddle>, Without<Ball>, Without<Wall>, Without<Brick>),
    >,
    wall_query: Query<
        (&mut Transform, &mut Sprite),
        (Without<Ball>, Without<Paddle>, Without<Brick>),
    >,
    mut bricks_query: Query<
        (Entity, &mut Transform, &mut Sprite, &mut Brick),
        (Without<Wall>, Without<Paddle>, Without<Ball>),
    >,
    mut commands: Commands,
    mut ev_play_sound: EventWriter<PlaySoundEvent>,
) {
    if game_state.pause && !game_state.direct_ball_movement {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut brick_collision_ids = Vec::new();
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
                apply_direct_ball_transform(&window, &mut ball_transform, |_, mut y| {
                    (*y) += f32::abs(ball.velocity.y);
                });
            }
            if keyboard_input.pressed(KeyCode::Down) {
                apply_direct_ball_transform(&window, &mut ball_transform, |_, mut y| {
                    (*y) -= f32::abs(ball.velocity.y);
                });
            }
            if keyboard_input.pressed(KeyCode::Left) {
                apply_direct_ball_transform(&window, &mut ball_transform, |mut x, _| {
                    (*x) -= f32::abs(ball.velocity.x);
                });
            }
            if keyboard_input.pressed(KeyCode::Right) {
                apply_direct_ball_transform(&window, &mut ball_transform, |mut x, _| {
                    (*x) += f32::abs(ball.velocity.x);
                });
            }
            return;
        } else if !game_state.paddle_owns_ball {
            if keyboard_input.just_pressed(KeyCode::Minus)
                || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
            {
                ball.change_speed(-1.0);
            } else if keyboard_input.just_pressed(KeyCode::Plus)
                || keyboard_input.just_pressed(KeyCode::NumpadAdd)
            {
                ball.change_speed(1.0);
            }

            let mut ball_collision_context =
                BallCollisionContext::create_for_ball(&ball, &ball_transform);

            let mut collision_context = CollisionsContext {
                collision_info: Vec::new(),
                ball_velocity: ball.velocity,
                paddle: paddle_query.get_single().unwrap(),
                walls: Vec::from_iter(wall_query.iter()),
                bricks: Vec::from_iter(bricks_query.iter())
            };

            if !apply_ball_transform(
                &ball.velocity,
                &mut ball_transform,
                &mut collision_context,
                &mut ball_collision_context,
            ) {
                let ball_velocity_x = ball.velocity.x;
                let ball_velocity_y = ball.velocity.y;

                for info in collision_context.collision_info {
                    match info.collider {
                        Collider::Paddle => match info.direction {
                            Collision::Top | Collision::Left | Collision::Right => {
                                ball.velocity.y = -ball_velocity_y;
                                play_sound(&mut ev_play_sound, BALL_HITS_PEDDLE);
                            }
                            Collision::Bottom => {
                                play_sound(&mut ev_play_sound, BALL_HITS_PEDDLE);
                            }
                        },

                        Collider::Wall => match info.direction {
                            Collision::Top | Collision::Bottom => {
                                ball.velocity.y = -ball_velocity_y;
                                play_sound(&mut ev_play_sound, BALL_HITS_WALL);
                            }
                            Collision::Left | Collision::Right => {
                                ball.velocity.x = -ball_velocity_x;
                                play_sound(&mut ev_play_sound, BALL_HITS_WALL);
                            }
                        },

                        Collider::Brick => {
                            // prevents duplicate brick handling
                            if brick_collision_ids.contains(&info.brick_id) {
                                continue;
                            }
                            brick_collision_ids.push(info.brick_id);
                            match info.direction {
                                Collision::Top | Collision::Bottom => {
                                    ball.velocity.y = -ball_velocity_y;
                                    brick_collision_ids.push(info.brick_id);
                                }
                                Collision::Left | Collision::Right => {
                                    ball.velocity.x = -ball_velocity_x;
                                    brick_collision_ids.push(info.brick_id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut brick_ids_despawned = Vec::new();
    for brick_id in brick_collision_ids {
        let mut brick  = bricks_query.iter_mut().find(|(e, t, s, b)| b.id == brick_id).unwrap();
        if brick.3.hits_required > 0 {
            brick.3.hits_required -= 1;
        }
        if brick.3.hits_required == 0 {
            if !brick_ids_despawned.contains(&brick_id) {
                brick_ids_despawned.push(brick_id);
                commands.entity(brick.0).despawn();
                play_sound(&mut ev_play_sound, BALL_KILLS_BRICK);
            }
        } else {
            brick.2.color = Color::rgb(0.85, 0.85, 0.85);
            play_sound(&mut ev_play_sound, BALL_HITS_BRICK);
        }
    }
}

struct CollisionsContext<'a> {
    pub collision_info: Vec<BallCollisionInfo>,
    pub ball_velocity: Vec2,
    pub paddle: (&'a Transform, &'a Sprite),
    pub walls: Vec<(&'a Transform, &'a Sprite)>,
    pub bricks: Vec<(Entity, &'a Transform, &'a Sprite, &'a Brick)>,
}

fn create_float_iterator(start: f32, end: f32) -> FloatIterator {
    FloatIterator::new_with_step(
        start as f64,
        f64::abs(end as f64),
        MOVEMENT_PRECISION as f64,
    )
}

fn apply_ball_transform(
    ball_velocity: &Vec2,
    ball_transform: &mut Transform,
    mut collision_context: &mut CollisionsContext,
    ball_collision_context: &mut BallCollisionContext,
) -> bool {
    let mut no_collision = false;
    let mut transform = Vec3::from(ball_transform.translation);

    for f in create_float_iterator(ball_velocity.x, 1.0) {
        let transform_x = transform.x;
        transform.x = ball_transform.translation.x + f as f32;
        ball_collision_context.update_ball_position(&transform);
        if !check_collision(&ball_collision_context, &mut collision_context) {
            break;
        } else {
            transform.x = transform_x;
            no_collision = false;
            continue;
        }
    }

    for f in create_float_iterator(ball_velocity.y, 1.0) {
        let transform_y = transform.y;
        transform.y = ball_transform.translation.y + f as f32;
        ball_collision_context.update_ball_position(&transform);
        if !check_collision(&ball_collision_context, &mut collision_context) {
            break;
        } else {
            transform.y = transform_y;
            no_collision = false;
            continue;
        }
    }

    ball_transform.translation.x = transform.x;
    ball_transform.translation.y = transform.y;

    no_collision
}

/// Checks current ball position for any collisions.
///
/// If the ball collides with the peddle, a wall or a brick the result will be 'true',
/// otherwise 'false'.
fn check_collision(
    ball_collision_context: &BallCollisionContext,
    collision_context: &mut CollisionsContext,
) -> bool {
    // paddle collision check
    let paddle_collision = ball_collision_context
        .check_collision(collision_context.paddle.0, collision_context.paddle.1);

    // Checks current ball position for any collisions;
    if paddle_collision.is_some() {
        collision_context
            .collision_info
            .push(BallCollisionInfo::new(
                Collider::Paddle,
                paddle_collision.unwrap(),
            ));
        return true;
    }

    // wall collision check
    for wall in collision_context.walls.iter() {
        let wall_collision = ball_collision_context.check_collision(wall.0, wall.1);
        if wall_collision.is_some() {
            collision_context
                .collision_info
                .push(BallCollisionInfo::new(
                    Collider::Wall,
                    wall_collision.unwrap(),
                ));
            return true;
        }
    }

    // brick collision check
    let mut collision_brick_count = 0;
    for brick in collision_context.bricks.iter() {
        let brick_collision = ball_collision_context.check_collision(brick.1, brick.2);
        if brick_collision.is_some() {
            collision_context
                .collision_info
                .push(BallCollisionInfo::new_brick(
                    brick.3.id,
                    brick_collision.unwrap(),
                ));
            collision_brick_count += 1;
        }
    }

    if collision_brick_count > 0 {
        println!("Collision brick count: {}", collision_brick_count);
    }
    collision_brick_count > 0
}

fn apply_direct_ball_transform<T>(window: &Window, ball_transform: &mut Transform, transform: T)
where
    T: Fn(&mut f32, &mut f32),
{
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

//noinspection RsConstantConditionIf
fn play_sound(mut ev_play_sound: &mut EventWriter<PlaySoundEvent>, sound_name: &str) {
    if PLAY_SOUND_ENABLED {
        ev_play_sound.send(PlaySoundEvent::normal(sound_name))
    }
}
