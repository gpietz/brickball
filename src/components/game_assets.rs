use crate::prelude::*;

pub const MAIN_MENU_GFX: &str = "graphics/main_menu01.png";
pub const DEBUG_FONT: & str = "fonts/FiraMono-Medium.ttf";

#[derive(Component)]
pub struct GameAssets {
    pub main_menu_gfx: Handle<Image>,
    pub debug_font: Handle<Font>
}

impl GameAssets {
    pub fn new(mut asset_server: &Res<AssetServer>) -> Self {
        Self {
            main_menu_gfx: asset_server.load(MAIN_MENU_GFX).into(),
            debug_font: asset_server.load(DEBUG_FONT).into(),
        }
    }
}
