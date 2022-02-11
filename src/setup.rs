use std::fs::File;
use crate::prelude::*;
use bevy::prelude::*;
use winapi::um::*;
use std::path::Path;

const WINDOW_TITLE : &str = "Bevy Test";

pub fn setup(asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut game_state: ResMut<GameState>) {

    load_settings(&mut commands);

    // center/position window (using the winapi crate!);
    // the window title will be set here cause it is beeing ignored in the window descriptor.
    let mut window = windows.get_primary_mut().unwrap();
    window.set_title(WINDOW_TITLE.to_string());
    window.set_resizable(false);
    center_window(&mut window);

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // load sounds and graphics
    commands.insert_resource(GameAssets::new(&asset_server));

    center_window(&mut window);
}

fn center_window(window: &mut Window) {
    let (desktop_width, desktop_height) = get_desktop_dimensions();
    let x_pos = ((desktop_width as f32 / 2f32) - (window.width() / 2f32)) as i32;
    let y_pos = ((desktop_height as f32 / 2f32) - (window.width() / 2f32)) as i32;
    window.set_position(IVec2::new(x_pos, y_pos));
}

fn get_desktop_dimensions() -> (i32, i32) {
    unsafe {
        let desktop_width = winuser::GetSystemMetrics(winuser::SM_CXSCREEN);
        let desktop_height = winuser::GetSystemMetrics(winuser::SM_CYSCREEN);
        (desktop_width, desktop_height)
    }
}

fn load_settings(mut commands: &mut Commands) {
    if Path::new(GAMESETTINGS_FILENAME).exists() {
        let file = File::open(GAMESETTINGS_FILENAME);
        if file.is_ok() {
            let reader_result = ::serde_json::from_reader(file.unwrap());
            if reader_result.is_ok() {
                let game_settings: GameSettings = reader_result.unwrap();
                commands.insert_resource(game_settings);
                println!("Loaded game settings from {}", GAMESETTINGS_FILENAME);
                return;
            }
        }
    }
    commands.insert_resource(GameSettings::default());
}
