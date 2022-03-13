use crate::prelude::*;
use crate::sprite_bundle_factory::*;
use std::cmp;

const PADDLE_WIDTH: f32 = 200.0;
const PADDLE_HEIGHT: f32 = 20.0;
pub const WALL_WIDTH: f32 = 30.0;

pub fn game_object_spawner(
    windows: Res<Windows>,
    mut commands: Commands,
    app_state: ResMut<State<AppState>>,
    game_assets: Res<GameAssets>,
) {
    let window = windows.get_primary().unwrap();
    spawn_paddle(&window, &mut commands, &game_assets);
    spawn_walls(&window, &mut commands, &game_assets);
    spawn_ball(&window, &mut commands, &game_assets);
}

/// Spawn the ball.
fn spawn_ball(window: &Window, mut commands: &mut Commands, game_assets: &Res<GameAssets>) {
    let ball = Ball::default();
    let y_pos = window_bottom(&window) + WALL_WIDTH + PADDLE_HEIGHT - ball.radius;
    let mut sprite_bundle = SpriteBundleFactory::with_texture(0.0, y_pos, 10, ball.radius, ball.radius, &game_assets.ball_gfx);
    commands.spawn_bundle(sprite_bundle)
            .insert(Ball::default());
}

/// Spawn player sprite.
fn spawn_paddle(window: &Window, mut commands: &mut Commands, game_assets: &Res<GameAssets>) {
    let y_pos = window_bottom(&window) + WALL_WIDTH;
    let mut sprite_bundle = SpriteBundleFactory::with_texture(0.0, y_pos, 1, PADDLE_WIDTH, PADDLE_HEIGHT, &game_assets.paddle_gfx);
    commands.spawn_bundle(sprite_bundle)
            .insert(Paddle::default())
            .insert(Collider::Paddle);
}

/// Spawn the walls around the play field.
fn spawn_walls(window: &Window, mut commands: &mut Commands, game_assets: &Res<GameAssets>) {
    const WALL_TOP_WIDTH: f32 = 80.0;

    // top wall
    let mut x_pos = window_left(window) + WALL_TOP_WIDTH;
    let y_pos = window_top(&window) - (WALL_WIDTH / 2.0);
    let mut total_width = 0.0;
    while total_width < window.width() {
        commands.spawn_bundle(SpriteBundleFactory::with_texture(x_pos, y_pos, 1, WALL_TOP_WIDTH, 16.0, &game_assets.wall_horizontal))
            .insert(Wall)
            .insert(Collider::Wall);
        x_pos += WALL_TOP_WIDTH;
        total_width += WALL_TOP_WIDTH;
    }

    let color = Color::rgb(0.5, 0.5, 0.5);

    // left wall
    let mut x_pos = window_left(&window) + (WALL_WIDTH / 2.0);
    commands.spawn_bundle(SpriteBundleFactory::with_color(x_pos, 0.0, 1, WALL_WIDTH, window.height(), color))
            .insert(Wall)
            .insert(Collider::Wall);
    // right wall
    x_pos = window_right(&window) - (WALL_WIDTH / 2.0);
    commands.spawn_bundle(SpriteBundleFactory::with_color(x_pos, 0.0, 1, WALL_WIDTH, window.height(), color))
            .insert(Wall)
            .insert(Collider::Wall);
}
