use crate::prelude::*;
use crate::helpers::KeyboardHelpers;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input);
    }
}

fn keyboard_input(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_writer: EventWriter<GameCommandEvent>,
    mut game_state: ResMut<GameState>,
    mut game_settings: ResMut<GameSettings>,
    ball_query: Query<&Transform, With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
) {
    match app_state.current() {
        AppState::Game => {
            // Releases the ball if it's stuck to the paddle
            if game_state.paddle_owns_ball && keyboard_input.just_pressed(KeyCode::Space) {
                game_state.paddle_owns_ball = false;
            }

            // Switch to next or previous level (Shift & +/-)
            let level_selection = get_level_selection(&keyboard_input);
            match level_selection {
                Some(LevelSelection::Next) => {
                    game_state.activate_next_level();
                },
                Some(LevelSelection::Previous) => {
                    game_state.activate_previous_level();
                },
                _ => {}
            }

            if keyboard_input.just_pressed(KeyCode::F1) {
                game_state.toggle_direct_ball_movement();
            } else if keyboard_input.just_pressed(KeyCode::F2) {
                print_ball_paddle_coordinates(&ball_query, &paddle_query);
            } else if keyboard_input.just_pressed(KeyCode::F3) {
                game_state.toggle_test_circle();
            } else if keyboard_input.just_pressed(KeyCode::F4) {
                ev_writer.send(GameCommandEvent(GameCommand::RemoveBricks));
                println!("All bricks removed.");
            } else if keyboard_input.just_pressed(KeyCode::F5) {
                game_state.current_level[1] = 0;
                println!("All bricks have been reset.")
            } else if keyboard_input.just_pressed(KeyCode::F6) {
                game_state.toggle_show_ball_coords();
            } else if keyboard_input.just_pressed(KeyCode::F7) {
                game_state.toggle_show_ball_collision_field();
            } else if keyboard_input.just_pressed(KeyCode::R) {
                game_state.paddle_owns_ball = true;
            }

            // Shift+C = Center ball (when direct ball movement is activated)
            if KeyboardHelpers::is_shift_pressed(&keyboard_input) {
                if keyboard_input.just_pressed(KeyCode::C) && game_state.direct_ball_movement {
                    ev_writer.send(GameCommandEvent(GameCommand::CenterBall));
                }
            }
        }
        _ => {}
    }

    // ---------------------------------------------------------------------------------------------
    // Control modifier pressed
    // ---------------------------------------------------------------------------------------------
    if KeyboardHelpers::is_control_pressed(&keyboard_input) {
        if keyboard_input.just_pressed(KeyCode::S)  {
            game_settings.toggle_sound_enabled();
        } else if keyboard_input.just_pressed(KeyCode::M) {
            game_settings.toggle_music_enabled();
        } else if keyboard_input.just_pressed(KeyCode::F) {
            game_settings.toggle_fps_display_enabled();
        }
    }
}

// Prints ball and paddle coordinates on the console.
fn print_ball_paddle_coordinates(
    ball_query: &Query<&Transform, With<Ball>>,
    paddle_query: &Query<&Transform, (With<Paddle>, Without<Ball>)>
) {
    let ball_transform = ball_query.single();
    let paddle_transform = paddle_query.single();
    println!("Ball: {}, {} | Paddle: {}, {}",
        ball_transform.translation.x,
        ball_transform.translation.y,
        paddle_transform.translation.x,
        paddle_transform.translation.y
    );
}

enum LevelSelection {
    Next, Previous
}

fn get_level_selection(keyboard_input: &Res<Input<KeyCode>>) -> Option<LevelSelection> {
    if !KeyboardHelpers::is_shift_pressed(keyboard_input) {
        return None;
    }

    if keyboard_input.just_pressed(KeyCode::Plus) ||
        keyboard_input.just_pressed(KeyCode::NumpadAdd) {
        return Some(LevelSelection::Next);
    }

    if keyboard_input.just_pressed(KeyCode::Minus) ||
        keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
        return Some(LevelSelection::Previous);
    }

    return None;
}
