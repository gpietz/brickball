use crate::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub id: u32,
    pub hits_required: u8,
}

impl Brick {
    pub fn new(id: u32, hits_required: u8) -> Self {
        Self {
            id,
            hits_required
        }
    }
}
