use bevy::asset::LoadState;
use crate::prelude::*;

pub fn check_audio_loading_system(
    mut audio_state: ResMut<AudioState>,
    asset_server: ResMut<AssetServer>
) {
    // TODO FixMe
    // if audio_state.audio_loaded {
    //     return;
    // }
    //
    // let mut loaded_count : u32 = 0;
    // for audio_source in audio_state.sources.iter_mut() {
    //     if LoadState::Loaded == asset_server.get_load_state(&audio_source.handle) {
    //         audio_source.loaded = true;
    //         loaded_count += 1;
    //     }
    // }
    //
    // if loaded_count == audio_state.sources.len() as u32 {
    //     audio_state.audio_loaded = true;
    // }
}
