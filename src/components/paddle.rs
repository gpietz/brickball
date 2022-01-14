use crate::prelude::*;

pub enum PaddleMovingDirection {
    None, Left, Right
}

#[derive(Component)]
pub struct Paddle {
    pub moving_direction : PaddleMovingDirection
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            moving_direction : PaddleMovingDirection::None
        }
    }
}
