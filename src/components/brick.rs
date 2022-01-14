use crate::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub hits_required: u8,
}

impl Default for Brick {
    fn default() -> Self {
        Self {
            hits_required: 1
        }
    }
}
