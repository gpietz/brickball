use crate::prelude::*;

#[derive(Component, PartialEq)]
pub enum Collider {
    Paddle,
    Brick,
    Wall
}
