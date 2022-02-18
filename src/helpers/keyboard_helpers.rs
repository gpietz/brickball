use bevy::prelude::*;

pub struct KeyboardHelpers;

impl KeyboardHelpers {
    pub fn is_shift_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift)
    }

    pub fn is_control_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
        keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl)
    }
}
