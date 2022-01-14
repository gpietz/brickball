mod setup;
mod spawners;
mod systems;
mod components;
mod rectangle;
mod levels;
mod level_data;
mod level_color;

mod prelude {
    pub const TIME_STEP: f32 = 1. / 60.;
    pub const PADDLE_SPEED: f32 = 500.;

    pub use bevy::prelude::*;
    pub use crate::spawners::*;
    pub use crate::components::*;
    pub use crate::components::*;
    pub use crate::levels::*;
    pub use crate::level_data::*;
    pub use crate::level_color::*;
}

use crate::prelude::*;
use crate::systems::player_movement::*;
use crate::systems::ball_movement::*;
use crate::systems::text_update::*;
use crate::systems::bricks_spawn::*;
use crate::setup::*;
use bevy::app::Events;
use bevy::window::WindowResized;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    IntroScreen,
    Main
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.00)))
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(GameState::default())
        .insert_resource(Levels::default())
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_startup_walls",
            SystemStage::single(walls_spawn)
        )
        .add_startup_stage(
            "game_startup_player",
            SystemStage::single(paddle_spawn)
        )
        .add_startup_stage(
            "game_startup_ball",
            SystemStage::single(ball_spawn.system())
        )
        // .add_system(resize_notificator.system())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(bricks_spawn)
        .add_system(player_movement)
        .add_system(ball_movement)
        .add_system(text_update)
        .run();
}


// NEW CODE FOR WINDOW SIZE DETECTION !!!!!
// fn resize_notificator(windows: ResMut<Windows>, mut window_size: ResMut<WindowSize>) {
//     let windows = windows.get_primary().unwrap();
//     window_size.width = windows.width();
//     window_size.height = windows.height();
//     println!("Window size now: {}x{}", windows.width(), windows.height());
// }
