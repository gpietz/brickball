mod setup;
mod spawners;
mod resources;
mod systems;
mod components;

mod prelude {
    pub const TIME_STEP: f32 = 1. / 60.;

    pub use bevy::prelude::*;
    pub use crate::resources::*;
    pub use crate::spawners::*;
    pub use crate::components::*;
}

use crate::prelude::*;
use crate::systems::player_movement;
use crate::systems::ball_movement;
use setup::*;

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
        .insert_resource(BallCalculations::default())
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_startup_player",
            SystemStage::single(player_spawn.system())
        )
        .add_startup_stage(
            "game_startup_ball",
            SystemStage::single(ball_spawn.system())
        )
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(player_movement.system())
        .add_system(ball_movement.system())
        .run();
}
