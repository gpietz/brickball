use bevy::prelude::*;

pub fn window_top(window: &Window) -> f32 {
   window.height() / 2.0
}

pub fn window_bottom(window: &Window) -> f32 {
    -(window.height() / 2.0)
}

pub fn window_left(window: &Window) -> f32 {
    -(window.width() / 2.0)
}

pub fn window_right(window: &Window) -> f32 {
    window.width() / 2.0
}

pub fn transform_x(window: &Window, value: f32) -> f32 { value - (window.width() / 2.) }

pub fn transform_y(window: &Window, value: f32) -> f32 { value + (window.height() / 2.) }
