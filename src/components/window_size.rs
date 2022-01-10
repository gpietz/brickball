pub struct WindowSize {
    pub width: f32,
    pub height: f32
}

impl WindowSize {
    pub fn get_top_boundary(&self) -> f32 {
        self.height / 2.
    }

    pub fn get_bottom_boundary(&self) -> f32 {
        -(self.height / 2.)
    }

    pub fn get_left_boundary(&self) -> f32 {
        -(self.width / 2.)
    }

    pub fn get_right_boundary(&self) -> f32 {
        self.width / 2.
    }

    pub fn transform_x(&self, value: f32) -> f32 { value - (self.width / 2.) }

    pub fn transform_y(&self, value: f32) -> f32 { value + (self.height / 2.) }
}
