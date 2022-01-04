use crate::prelude::*;

pub struct Ball {
    pub velocity: Vec2,
    pub radius: f32,
    pub sticking_on_paddle: bool
}

impl Ball {
    pub fn change_speed(&mut self, value: f32) -> bool {
        let new_velocity = self.velocity.x + value;
        if new_velocity < 1. || new_velocity > 8. {
            return false;
        }
        self.velocity.x = new_velocity;
        self.velocity.y = new_velocity;
        println!("Ball speed now: {}", self.velocity.y);
        true
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(5., 5.),
            radius: 20.0,
            sticking_on_paddle: true
        }
    }
}
