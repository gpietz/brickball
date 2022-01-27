use crate::helpers::is_shift_pressed;
use crate::prelude::*;

pub fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>,
    mut ev_writer: EventWriter<GameCommandEvent>,
    mut game_state: ResMut<GameState>
) {
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
        _ => { }
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
    } else if is_center_ball_hotkey_pressed(&keyboard_input) && game_state.direct_ball_movement {
        ev_writer.send(GameCommandEvent(GameCommand::CenterBall));
    } else if keyboard_input.just_pressed(KeyCode::R) {
        game_state.paddle_owns_ball = true;
    }
}

enum LevelSelection {
    Next, Previous
}

fn get_level_selection(keyboard_input : &Res<Input<KeyCode>>) -> Option<LevelSelection> {
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
