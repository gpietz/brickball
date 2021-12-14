use bevy::ecs::entity::Entities;
use bevy::ecs::query::ReadOnlyFetch;
use crate::prelude::*;

//==== Ball ========================================================================================

pub struct Ball {
    pub sticking_on_paddle: bool,
}

impl Ball {
    pub fn collision_with_paddle(&mut self) {
    }

    pub fn collision_with_wall(&mut self) {
    }

    pub fn collision_with_brick(&mut self) {
    }
}

//==== PaddleMovingDirection =======================================================================

pub enum PaddleMovingDirection {
    None, Left, Right
}

//==== Paddle ======================================================================================

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

//==== WindowSize ==================================================================================

pub struct WindowSize {
    pub width: f32,
    pub height: f32
}

//==== Position ====================================================================================

pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
        }
    }
}

//==== MoveSpeed ===================================================================================

pub struct MoveSpeed(pub f32);

impl MoveSpeed {
    pub fn new(speed: f32) -> Self { Self(speed) }
}

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(500.)
    }
}
