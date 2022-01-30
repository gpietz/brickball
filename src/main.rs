extern crate core;

mod audio;
mod audio_setup;
mod components;
mod events;
mod game_object_spawner;
mod helpers;
mod level_color;
mod level_data;
mod levels;
mod rectangle;
mod setup;
mod systems;

mod prelude {
    pub const TIME_STEP: f32 = 1. / 60.;
    pub const PADDLE_SPEED: f32 = 500.;

    pub use crate::audio::*;
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::game_object_spawner::*;
    pub use crate::helpers::*;
    pub use crate::level_color::*;
    pub use crate::level_data::*;
    pub use crate::levels::*;
    pub use crate::rectangle::*;
    pub use bevy::prelude::*;
}

use crate::audio_setup::*;
use crate::prelude::*;
use crate::setup::*;
use crate::systems::ball_collision_field_system::*;
use crate::systems::ball_collision_system::*;
use crate::systems::ball_movement_system::*;
use crate::systems::brick_spawning_system::*;
use crate::systems::check_audio_loading_system::*;
use crate::systems::keyboard_input_system::*;
use crate::systems::main_menu_init_system::*;
use crate::systems::main_menu_system::*;
use crate::systems::paddle_movement_system::*;
use crate::systems::play_audio_system::*;
use crate::systems::show_ball_coords_system::*;
use crate::systems::test_circle_system::*;
use bevy::ecs::schedule::IntoSystemDescriptor;
use bevy::input::system::exit_on_esc_system;
use bevy::window::WindowResized;
use bevy_kira_audio::AudioPlugin;
use bevy_prototype_lyon::plugin::ShapePlugin;

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
        .add_startup_system(audio_setup)
        .add_system(check_audio_loading_system)
        .add_system(exit_on_esc_system)
        // main menu stage
        .add_system_set(SystemSet::on_enter(AppState::MainMenu)
            .with_system(main_menu_init_system))
        .add_system_set(SystemSet::on_update(AppState::MainMenu)
            .with_system(main_menu_system))
        // game stage
        .add_system_set(SystemSet::on_enter(AppState::Game)
            .with_system(game_object_spawner))
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(brick_spawning_system)
                .with_system(paddle_movement_system)
                .with_system(ball_movement_system
                    .chain(ball_collision_system)
                    .chain(ball_collision_field_system),
                )
                .with_system(test_circle_system)
                .with_system(keyboard_input_system)
                .with_system(show_ball_coords_system),
        )
        .add_system(play_audio_system)
        .run();
}

// NEW CODE FOR WINDOW SIZE DETECTION !!!!!
// fn resize_notificator(windows: ResMut<Windows>, mut window_size: ResMut<WindowSize>) {
//     let windows = windows.get_primary().unwrap();
//     window_size.width = windows.width();
//     window_size.height = windows.height();
//     println!("Window size now: {}x{}", windows.width(), windows.height());
// }
