use bevy::prelude::*;
use winapi::um::*;
use crate::spawners::*;
use crate::resources::*;
use crate::WindowSize;

const WINDOW_TITLE : &str = "Bevy Test";

pub fn setup(mut commands: Commands,
             mut windows: ResMut<Windows>,
             mut materials: ResMut<Assets<ColorMaterial>>) {

    // center/position window (using the winapi crate!);
    // the window title will be set here cause it is beeing ignored in the window descriptor.
    let window = windows.get_primary_mut().unwrap();
    window.set_title(WINDOW_TITLE.to_string());
    window.set_resizable(false);
    center_window(window);

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // create main resources
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(1., 1., 1.).into()),
        ball_material: materials.add(Color::rgb(0., 1., 0.).into()),
    });
    commands.insert_resource(WindowSize {
        width: window.width(),
        height: window.height()
    });
}

fn center_window(window: &mut Window) {
    let (desktop_width, desktop_height) = get_desktop_dimensions();
    let window_width = window.width();
    let window_height = window.height();
    let xpos = ((desktop_width as f32 / 2f32) - (window_width / 2f32)) as i32;
    let ypos = ((desktop_height as f32 / 2f32) - (window_height / 2f32)) as i32;
    window.set_position(IVec2::new(xpos, ypos));
}

fn get_desktop_dimensions() -> (i32, i32) {
    unsafe
    {
        let desktop_width = winuser::GetSystemMetrics(winuser::SM_CXSCREEN);
        let desktop_height = winuser::GetSystemMetrics(winuser::SM_CYSCREEN);
        (desktop_width, desktop_height)
    }
}
