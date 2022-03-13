use crate::prelude::*;

// Textures
pub const MAIN_MENU_GFX: &str = "graphics/main_menu01.png";
pub const BALL_GFX: &str = "graphics/ball01.png";
pub const PADDLE_GFX: &str = "graphics/paddle01.png";
pub const WALL_HORIZONTAL_GFX: &str = "graphics/wall_horizontal_01.png";
// Fonts
pub const DEBUG_FONT: &str = "fonts/FiraMono-Medium.ttf";

#[derive(Component)]
pub struct GameAssets {
    pub main_menu_gfx: Handle<Image>,
    pub ball_gfx: Handle<Image>,
    pub paddle_gfx: Handle<Image>,
    pub wall_horizontal: Handle<Image>,
    pub debug_font: Handle<Font>,

}

impl GameAssets {
    pub fn new(mut asset_server: &Res<AssetServer>) -> Self {
        Self {
            // load textures
            main_menu_gfx: asset_server.load(MAIN_MENU_GFX).into(),
            ball_gfx: asset_server.load(BALL_GFX).into(),
            paddle_gfx: asset_server.load(PADDLE_GFX).into(),
            wall_horizontal: asset_server.load(WALL_HORIZONTAL_GFX).into(),
            // load fonts
            debug_font: asset_server.load(DEBUG_FONT).into(),
        }
    }
}
