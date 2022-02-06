use bevy::sprite::collide_aabb::Collision;
use crate::prelude::*;

pub struct BallCollisionInfo {
    /** Information about the type the ball has collided with. */
    pub collider: Collider,
    /** Information about the direction of the collision. */
    pub direction: Collision,
    /** Identification of the brick. */
    pub brick_id: u32,
}

impl BallCollisionInfo {
    pub fn new(collider: Collider, direction: Collision) -> Self {
        Self {
            collider,
            direction,
            brick_id: 0
        }
    }

    pub fn new_brick(brick_id: u32, direction: Collision) -> Self {
        Self {
            collider: Collider::Brick,
            direction,
            brick_id
        }
    }
}
