use std::convert::identity;
use crate::prelude::*;

pub fn main_menu_system(
    mut query: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>

) {
    let query_result = query.get_single_mut();
    if query_result.is_err() {
            // Main menu graphics not yet present, so we've to spawn it.
            commands.spawn_bundle(SpriteBundle {
                texture: game_assets.main_menu_gfx.clone(),
                ..Default::default()
            })
            .insert(MainMenu);
    }

    // Let's start the game;
    // removes the main menu graphics by despawning.
    if keyboard_input.just_pressed(KeyCode::Space) {
        let entity = query_result.unwrap();
        commands.entity(entity).despawn();
        keyboard_input.clear_just_pressed(KeyCode::Space);
        app_state.set(AppState::Game).unwrap();
    }
}
