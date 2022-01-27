mod setup;
mod game_object_spawner;
mod systems;
mod components;
mod rectangle;
mod levels;
mod level_data;
mod level_color;
mod helpers;
mod events;
mod audio_setup;
mod audio;

mod prelude {
    pub const TIME_STEP: f32 = 1. / 60.;
    pub const PADDLE_SPEED: f32 = 500.;

    pub use bevy::prelude::*;
    pub use crate::game_object_spawner::*;
    pub use crate::components::*;
    pub use crate::helpers::*;
    pub use crate::levels::*;
    pub use crate::level_data::*;
    pub use crate::level_color::*;
    pub use crate::rectangle::*;
    pub use crate::events::*;
    pub use crate::audio::*;
}

use crate::prelude::*;
use crate::systems::paddle_movement_system::*;
use crate::systems::ball_movement_system::*;
use crate::systems::brick_spawning_system::*;
use crate::systems::test_circle_system::*;
use crate::systems::ball_collision_system::*;
use crate::systems::play_audio_system::*;
use crate::systems::check_audio_loading_system::*;
use crate::systems::main_menu_system::*;
use crate::systems::keyboard_input_system::*;
use crate::systems::show_ball_coords_system::*;
use crate::setup::*;
use crate::audio_setup::*;
use bevy::ecs::schedule::IntoSystemDescriptor;
use bevy::window::WindowResized;
use bevy::input::system::exit_on_esc_system;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_kira_audio::AudioPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(AudioPlugin)
        .add_state(AppState::MainMenu)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.00)))
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(GameState::default())
        .insert_resource(Levels::default())
        .add_event::<GameCommandEvent>()
        .add_event::<PlaySoundEvent>()
        .add_startup_system(setup.system())
        //.add_startup_system(game_object_spawner)
        .add_startup_system(audio_setup)
        .add_system(check_audio_loading_system)
        .add_system(exit_on_esc_system)
        // main menu stage
        .add_system_set(create_main_menu_set(main_menu_system))
        // game stage
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game_object_spawner))
        .add_system_set(create_game_set(brick_spawning_system))
        .add_system_set(create_game_set(paddle_movement_system))
        .add_system_set(create_game_set(ball_collision_system))
        .add_system_set(create_game_set(ball_movement_system))
        .add_system_set(create_game_set(test_circle_system))
        .add_system_set(create_game_set(keyboard_input_system))
        .add_system_set(create_game_set(play_audio_system))
        .add_system_set(create_game_set(show_ball_coords_system))
        .run();
}

fn create_main_menu_set<Params>(system: impl IntoSystemDescriptor<Params>) -> SystemSet {
    SystemSet::on_update(AppState::MainMenu).with_system(system)
}

fn create_game_set<Params>(system: impl IntoSystemDescriptor<Params>) -> SystemSet {
    SystemSet::on_update(AppState::Game).with_system(system)
}

// NEW CODE FOR WINDOW SIZE DETECTION !!!!!
// fn resize_notificator(windows: ResMut<Windows>, mut window_size: ResMut<WindowSize>) {
//     let windows = windows.get_primary().unwrap();
//     window_size.width = windows.width();
//     window_size.height = windows.height();
//     println!("Window size now: {}x{}", windows.width(), windows.height());
// }
