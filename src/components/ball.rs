use bevy::ecs::query::ReadOnlyFetch;
use crate::prelude::*;

pub struct Ball {
    pub velocity: Vec2,
    pub radius: f32,
    pub sticking_on_paddle: bool
}

impl Ball {
    pub fn change_speed(&mut self, value: f32) {
        self.velocity.x = calculate_new_speed(self.velocity.x, value);
        self.velocity.y = calculate_new_speed(self.velocity.y, value);
        println!("Ball speed now: {}, {}", self.velocity.x, self.velocity.y);
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

fn calculate_new_speed(current_speed: f32, value: f32) -> f32 {
    let new_speed = f32::abs(current_speed) + value;
    if new_speed < 1. || new_speed > 8. {
        return current_speed;
    }
    let new_speed = if current_speed < 0. { -new_speed } else { new_speed };
    new_speed
}
