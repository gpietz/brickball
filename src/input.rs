use crate::prelude::*;
use crate::helpers::is_shift_pressed;

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
    mut game_settings: ResMut<GameSettings>
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
                ev_writer.send(GameCommandEvent(GameCommand::ShowCoordinates));
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
            } else if is_center_ball_hotkey_pressed(&keyboard_input) && game_state.direct_ball_movement {
                ev_writer.send(GameCommandEvent(GameCommand::CenterBall));
            } else if keyboard_input.just_pressed(KeyCode::R) {
                game_state.paddle_owns_ball = true;
            }
        }
        _ => {}
    }

    if keyboard_input.just_pressed(KeyCode::S) && is_control_pressed(&keyboard_input) {
        game_settings.toggle_sound_enabled();
    } else if keyboard_input.just_pressed(KeyCode::M) && is_control_pressed(&keyboard_input) {
        game_settings.toggle_music_enabled();
    }
}

enum LevelSelection {
    Next, Previous
}

fn get_level_selection(keyboard_input: &Res<Input<KeyCode>>) -> Option<LevelSelection> {
    if !keyboard_input.pressed(KeyCode::LShift) && !keyboard_input.pressed(KeyCode::RShift) {
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

fn is_center_ball_hotkey_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
    is_shift_pressed(keyboard_input) && keyboard_input.just_pressed(KeyCode::C)
}

fn is_control_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
    keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl)
}
