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
use crate::systems::text_update_system::*;
use crate::systems::brick_spawning_system::*;
use crate::systems::test_circle_system::*;
use crate::systems::ball_collision_system::*;
use crate::systems::play_audio_system::*;
use crate::systems::check_audio_loading_system::*;
use crate::setup::*;
use crate::audio_setup::*;
use bevy::app::Events;
use bevy::ecs::system::Remove;
use bevy::window::WindowResized;
use bevy::input::system::exit_on_esc_system;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_kira_audio::AudioPlugin;

use crate::systems::keyboard_input_system::keyboard_input_system;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    IntroScreen,
    Main
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(AudioPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.00)))
        .insert_resource(WindowDescriptor {
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(GameState::default())
        .insert_resource(Levels::default())
        .insert_resource(GameAssets { })
        .add_event::<GameCommandEvent>()
        .add_event::<PlaySoundEvent>()
        .add_startup_system(setup.system())
        .add_startup_system(game_object_spawner)
        .add_startup_system(audio_setup)
        .add_system(check_audio_loading_system)
        .add_system(brick_spawning_system)
        .add_system(paddle_movement_system)
        .add_system(ball_collision_system)
        .add_system(ball_movement_system)
        .add_system(test_circle_system)
        .add_system(keyboard_input_system)
        .add_system(text_update_system)
        .add_system(play_audio_system)
        .add_system(exit_on_esc_system)
        .run();
}

// NEW CODE FOR WINDOW SIZE DETECTION !!!!!
// fn resize_notificator(windows: ResMut<Windows>, mut window_size: ResMut<WindowSize>) {
//     let windows = windows.get_primary().unwrap();
//     window_size.width = windows.width();
//     window_size.height = windows.height();
//     println!("Window size now: {}x{}", windows.width(), windows.height());
// }
