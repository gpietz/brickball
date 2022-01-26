use std::any::Any;
use crate::prelude::*;

pub const SFX_IMPACT1: &str = "assets/sounds/sfx_sounds_impact1.wav";

#[derive(Component)]
pub struct GameAssets {
}

impl GameAssets {
    pub fn new(mut asset_server: &Res<AssetServer>) -> Self {
        Self {
        }
    }
}
