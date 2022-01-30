use crate::prelude::*;

pub struct PlaySoundEvent {
    pub name: String,
    pub looped: bool,
}

impl PlaySoundEvent {
    pub fn normal(asset_name: &str) -> Self {
        Self {
            name: asset_name.to_string(),
            looped: false
        }
    }

    pub fn looped(asset_name: &str) -> Self {
        Self {
            name: asset_name.to_string(),
            looped: true
        }
    }
}
