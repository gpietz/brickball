mod setup;
mod spawners;
mod systems;
mod materials;
mod components;
mod levels;
mod brick_materials;
mod rectangle;

mod prelude {
    pub const TIME_STEP: f32 = 1. / 60.;
    pub const PADDLE_SPEED: f32 = 500.;

    pub use bevy::prelude::*;
    pub use crate::spawners::*;
    pub use crate::components::*;
    pub use crate::materials::*;
    pub use crate::components::*;
    pub use crate::brick_materials::*;
}

use crate::prelude::*;
use crate::systems::player_movement::*;
use crate::systems::ball_movement::*;
use crate::systems::text_update::*;
use crate::systems::bricks_spawn::*;
use crate::setup::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    IntroScreen,
    Main
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.00)))
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(GameState::default())
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_startup_walls",
            SystemStage::single(walls_spawn.system())
        )
        .add_startup_stage(
            "game_startup_player",
            SystemStage::single(paddle_spawn.system())
        )
        .add_startup_stage(
            "game_startup_ball",
            SystemStage::single(ball_spawn.system())
        )
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(bricks_spawn.system())
        .add_system(player_movement.system())
        .add_system(ball_movement.system())
        .add_system(text_update.system())
        .run();
}
