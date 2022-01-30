use crate::prelude::*;
use std::collections::HashMap;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use crate::audio::AudioSource;

pub fn audio_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let mut channels = HashMap::new();
    init_channel(&mut channels, "first");
    init_channel(&mut channels, "second");
    init_channel(&mut channels, "third");

    let mut sources = Vec::new();
    sources.push(create_audio_source(&asset_server, "impact1", "sounds/sfx_sounds_impact1.wav"));
    sources.push(create_audio_source(&asset_server, "impact2", "sounds/sfx_sounds_impact3.wav"));
    sources.push(create_audio_source(&asset_server, "impact3", "sounds/sfx_sounds_impact6.wav"));
    sources.push(create_audio_source(&asset_server, "impact_wall", "sounds/sfx_sounds_impact11.wav"));
    sources.push(create_audio_source(&asset_server, "music_intro", "music/intro.ogg"));
    sources.push(create_audio_source(&asset_server, "music_loop", "music/loop.ogg"));

    let audio_state = AudioState {
        audio_loaded: false,
        channels,
        sources
    };

    commands.insert_resource(audio_state);
}

fn init_channel(channels: &mut HashMap<AudioChannel, ChannelAudioState>, name: &str) {
    channels.insert(
        AudioChannel::new(name.to_owned()),
        ChannelAudioState::default()
    );
}

fn create_audio_source(asset_server: &ResMut<AssetServer>, name: &str, filename: &str)
    -> AudioSource {
    AudioSource {
        name: name.to_string(),
        loaded: false,
        handle: asset_server.load(filename),
    }
}
