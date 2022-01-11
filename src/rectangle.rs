use bevy::prelude::Sprite;
use crate::Transform;

pub struct Rectangle {
    pub upper_left: [f32; 2],
    pub lower_right: [f32; 2]
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            upper_left: [0., 0.],
            lower_right: [0., 0.],
        }
    }
}

impl Rectangle {
    pub fn create_from(rect: &Rectangle) -> Self {
        Self {
            upper_left: [rect.upper_left[0], rect.upper_left[1]],
            lower_right: [rect.lower_right[0], rect.lower_right[1]]
        }
    }

    pub fn create_from_values(transform: &Transform, width: f32, height: f32) -> Self {
        let x1 = transform.translation.x - (width / 2.);
        let y1 = transform.translation.y + (height / 2.);
        let x2 = transform.translation.x + (width / 2.);
        let y2 = transform.translation.y - (height / 2.);
        Self {
            upper_left: [x1, y1],
            lower_right: [x2, y2]
        }
    }

    pub fn create_from_sprite(transform: &Transform, sprite: &Sprite) -> Self {
        let x1 = transform.translation.x - (sprite.size.x / 2.);
        let y1 = transform.translation.y + (sprite.size.y / 2.);
        let x2 = transform.translation.x + (sprite.size.x / 2.);
        let y2 = transform.translation.y - (sprite.size.y / 2.);
        Self {
            upper_left: [x1, y1],
            lower_right: [x2, y2]
        }
    }

    pub fn intersects_with(&self, rect: &Rectangle) -> bool {
        rect.is_inside([self.upper_left[0], self.upper_left[1]])
            || rect.is_inside([self.lower_right[0], self.upper_left[1]])
            || rect.is_inside([self.lower_right[0], self.lower_right[1]])
            || rect.is_inside([self.upper_left[0], self.lower_right[1]])
    }

    fn is_inside(&self, point: [f32; 2]) -> bool {
        self.upper_left[1] >= point[1]
            && self.lower_right[1] <= point[1]
            && self.upper_left[0] <= point[0]
            && self.lower_right[0] >= point[0]
    }

    pub fn transform(&mut self, x: f32, y: f32) {
        self.upper_left[0] += x;
        self.upper_left[1] += y;
        self.lower_right[0] += x;
        self.lower_right[1] += y;
    }

    pub fn copy_from(&mut self, rect: &Rectangle) {
        self.upper_left[0] = rect.upper_left[0];
        self.upper_left[1] = rect.upper_left[1];
        self.lower_right[0] = rect.lower_right[1];
        self.lower_right[1] = rect.lower_right[1];
    }
}
