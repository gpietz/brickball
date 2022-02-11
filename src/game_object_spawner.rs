use crate::prelude::*;

const PADDLE_WIDTH: f32 = 200.0;
const PADDLE_HEIGHT: f32 = 20.0;
pub const WALL_WIDTH: f32 = 30.0;

pub fn game_object_spawner(
    windows: Res<Windows>,
    mut commands: Commands,
    app_state: ResMut<State<AppState>>
) {
    let window = windows.get_primary().unwrap();
    spawn_paddle(&window, &mut commands);
    spawn_walls(&window, &mut commands);
    spawn_ball(&window, &mut commands);
}

/// Spawn the ball.
fn spawn_ball(window: &Window, mut commands: &mut Commands) {
    let ball = Ball::default();
    let y_pos = window_bottom(&window) + WALL_WIDTH + PADDLE_HEIGHT - ball.radius;
    commands.spawn_bundle(create_sprite_bundle(0.0, y_pos, 10, ball.radius, ball.radius, Color::GREEN))
            .insert(Ball::default());
}

/// Spawn player sprite.
fn spawn_paddle(window: &Window, mut commands: &mut Commands) {
    let y_pos = window_bottom(&window) + WALL_WIDTH;
    commands.spawn_bundle(create_sprite_bundle(0.0, y_pos, 1, PADDLE_WIDTH, PADDLE_HEIGHT, Color::WHITE))
            .insert(Paddle::default())
            .insert(Collider::Paddle);
}

/// Spawn the walls around the play field.
fn spawn_walls(window: &Window, mut commands: &mut Commands) {
    // top wall
    let y_pos = window_top(&window) - (WALL_WIDTH / 2.0);
    let color = Color::rgb(0.5, 0.5, 0.5);
    commands.spawn_bundle(create_sprite_bundle(0.0, y_pos, 1, window.width(), WALL_WIDTH, color))
            .insert(Wall)
            .insert(Collider::Wall);

    // left wall
    let mut x_pos = window_left(&window) + (WALL_WIDTH / 2.0);
    commands.spawn_bundle(create_sprite_bundle(x_pos, 0.0, 1, WALL_WIDTH, window.height(), color))
            .insert(Wall)
            .insert(Collider::Wall);
    // right wall
    x_pos = window_right(&window) - (WALL_WIDTH / 2.0);
    commands.spawn_bundle(create_sprite_bundle(x_pos, 0.0, 1, WALL_WIDTH, window.height(), color))
            .insert(Wall)
            .insert(Collider::Wall);
}

fn create_sprite_bundle(x: f32, y: f32, z: u8, width: f32, height: f32, color: Color) -> SpriteBundle  {
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, f32::from(z)),
            ..Default::default()

        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}
