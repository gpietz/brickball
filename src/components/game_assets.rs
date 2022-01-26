use crate::prelude::*;

pub const MAIN_MENU_GFX: &str = "graphics/main_menu01.png";

#[derive(Component)]
pub struct GameAssets {
    pub main_menu_gfx: Handle<Image>
}

impl GameAssets {
    pub fn new(mut asset_server: &Res<AssetServer>) -> Self {
        Self {
            main_menu_gfx: asset_server.load(MAIN_MENU_GFX).into()
        }
    }
}
