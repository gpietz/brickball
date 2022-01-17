use bevy::prelude::*;

pub fn is_shift_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
    keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift)
}
