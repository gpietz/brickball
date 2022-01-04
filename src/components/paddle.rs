use crate::prelude::*;

pub enum PaddleMovingDirection {
    None, Left, Right
}

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
