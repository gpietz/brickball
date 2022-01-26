use crate::prelude::*;
use bevy::prelude::*;
use winapi::um::*;

const WINDOW_TITLE : &str = "Bevy Test";

pub fn setup(asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut game_state: ResMut<GameState>) {

    //let a : Handle<AudioSource> = asset_server.load("assets/sounds/sfx_sounds_impact1.mp3");

    // center/position window (using the winapi crate!);
    // the window title will be set here cause it is beeing ignored in the window descriptor.
    let mut window = windows.get_primary_mut().unwrap();
    window.set_title(WINDOW_TITLE.to_string());
    window.set_resizable(false);
    center_window(&mut window);

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // create text for position display
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "Pos: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 16.0,
                        color: Color::YELLOW
                    }
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 16.0,
                        color: Color::YELLOW
                    }
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(DebugText);

    // load sounds
    commands.insert_resource(GameAssets::new(&asset_server));

    center_window(&mut window);
}

fn center_window(window: &mut Window) {
    let (desktop_width, desktop_height) = get_desktop_dimensions();
    let window_width = window.width();
    let window_height = window.height();
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
