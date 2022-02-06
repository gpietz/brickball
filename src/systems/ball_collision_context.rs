use crate::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

pub struct BallCollisionContext {
    ball_position: Vec3,
    ball_size: Vec2,
}

impl BallCollisionContext {
    pub fn create_for_ball(ball: &Ball, ball_transform: &Transform) -> Self {
        Self {
            ball_position: ball_transform.translation,
            ball_size: Vec2::new(ball.radius, ball.radius)
        }
    }

    pub fn update_ball_position(&mut self, ball_transform: &Vec3) {
        self.ball_position.x = ball_transform.x;
        self.ball_position.y = ball_transform.y;
    }

    pub fn check_collision(&self, transform: &Transform, sprite: &Sprite) -> Option<Collision> {
        collide(
            self.ball_position,
            self.ball_size,
            transform.translation,
            sprite.custom_size.unwrap()
        )
    }
}
