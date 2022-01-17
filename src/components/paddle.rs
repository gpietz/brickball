use crate::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub velocity: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            velocity: 0.0,
        }
    }
}
