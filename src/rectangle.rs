use crate::prelude::*;
use winapi::um::wingdi::Rectangle;

pub struct Rectangle {
    pub upper_left: [f32; 2],
    pub lower_right: [f32; 2],
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            upper_left: [0.0, 0.0],
            lower_right: [0.0, 0.0],
        }
    }
}

impl Rectangle {
    pub fn create_from_xy(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            upper_left: [x, y],
            lower_right: [x + width, y + height],
        }
    }

    pub fn create_from(rect: &Rectangle) -> Self {
        Self {
            upper_left: [rect.upper_left[0], rect.upper_left[1]],
            lower_right: [rect.lower_right[0], rect.lower_right[1]],
        }
    }

    pub fn create_from_values(transform: &Transform, width: f32, height: f32) -> Self {
        let x1 = transform.translation.x - (width / 2.0);
        let y1 = transform.translation.y + (height / 2.0);
        let x2 = transform.translation.x + (width / 2.0);
        let y2 = transform.translation.y - (height / 2.0);
        Self {
            upper_left: [x1, y1],
            lower_right: [x2, y2],
        }
    }

    pub fn create_from_sprite(transform: &Transform, sprite: &Sprite) -> Self {
        if sprite.custom_size.is_none() {
            panic!("The sprite object has no custom size!");
        }
        let size = sprite.custom_size.unwrap();
        let x1 = transform.translation.x - (size.x / 2.0);
        let y1 = transform.translation.y + (size.y / 2.0);
        let x2 = transform.translation.x + (size.x / 2.0);
        let y2 = transform.translation.y - (size.x / 2.0);
        Self {
            upper_left: [x1, y1],
            lower_right: [x2, y2],
        }
    }

    pub fn create_from_window(window: &Window) -> Self {
        Self {
            upper_left: [window_left(&window), window_top(&window)],
            lower_right: [window_right(&window), window_bottom(&window)],
        }
    }

    pub fn intersects_with(&self, rect: &Rectangle) -> bool {
        rect.is_inside(Vec2::new(self.upper_left[0], self.upper_left[1]))
            || rect.is_inside(Vec2::new(self.lower_right[0], self.upper_left[1]))
            || rect.is_inside(Vec2::new(self.lower_right[0], self.lower_right[1]))
            || rect.is_inside(Vec2::new(self.upper_left[0], self.lower_right[1]))
    }

    pub fn is_inside(&self, point: Vec2) -> bool {
        Self::is_between(point.y, self.upper_left[1], self.lower_right[1])
            && Self::is_between(point.x, self.upper_left[0], self.lower_right[0])
    }

    pub fn is_inside_ref(&self, x: &f32, y: &f32) -> bool {
        let x_pos = *x;
        let y_pos = *y;
        Self::is_between(y_pos, self.upper_left[1], self.lower_right[1])
            && Self::is_between(x_pos, self.upper_left[0], self.lower_right[0])
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

    pub fn print_values(&self) {
        println!(
            "Rectangle -> upper_left: {},{}  lower_right: {},{}",
            self.upper_left[0], self.upper_left[1], self.lower_right[0], self.lower_right[1]
        );
    }

    fn is_between(value: f32, from: f32, to: f32) -> bool {
        value >= from && value <= to
    }
}
