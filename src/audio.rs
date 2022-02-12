
mod audio_source;
mod audio_state;
mod channel_audio_state;

use crate::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use std::collections::HashMap;

pub use audio_source::*;
pub use audio_state::*;
pub use channel_audio_state::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(audio_setup)
           .add_system(play_music_system)
           .add_system(play_sound_system);
    }
}

fn audio_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let mut audio_sources = HashMap::new();
    audio_sources.insert("impact1", "sounds/sfx_sounds_impact1.wav");
    audio_sources.insert("impact2", "sounds/sfx_sounds_impact3.wav");
    audio_sources.insert("impact3", "sounds/sfx_sounds_impact6.wav");
    audio_sources.insert("impact_wall", "sounds/sfx_sounds_impact11.wav");
    audio_sources.insert("music_intro", "music/intro.ogg");
    audio_sources.insert("music_loop", "music/loop.ogg");

    let mut audio_state = AudioState::default();
    audio_state.create_channel("first");
    audio_state.create_channel("second");
    audio_state.create_channel("third");
    for (name, filename) in audio_sources.iter() {
        audio_state.add_source(AudioSource {
            name: name.to_string(),
            loaded: false,
            handle: asset_server.load(&filename.to_string())
        })
    }
    commands.insert_resource(audio_state);
}

fn play_music_system(
    app_state: Res<State<AppState>>,
    game_settings: Res<GameSettings>,
    mut audio_state: ResMut<AudioState>,
    audio: ResMut<Audio>
) {
    if app_state.current().clone() == AppState::IntroScreen {
        return;
    }

    let mut music_channel = audio_state.get_looped_audio_channel();
    if !game_settings.music_enabled && music_channel.is_ok() {
        let channel = music_channel.unwrap();
        audio.stop_channel(&channel);
        audio_state.set_channel_looped(channel, false);
        println!("Stopped music.")
    } else if game_settings.music_enabled && music_channel.is_err() {
        let audio_source = audio_state.get_audio_source("music_loop");
        if audio_source.is_some() {
            let free_channel = audio_state.get_free_audio_channel();
            if free_channel.is_some() {
                let channel = free_channel.unwrap();
                if audio_state.is_channel_known(&channel) {
                    println!("Playing music.");
                    audio.play_looped_in_channel(audio_source.unwrap().handle.clone(), &channel);
                    audio_state.set_channel_looped(channel, true);
                }
            }
        }
    }
}

fn play_sound_system(
    game_settings: Res<GameSettings>,
    mut ev_reader_play_sound: EventReader<PlaySoundEvent>,
    audio_state: Res<AudioState>,
    audio: Res<Audio>,
) {
    if !game_settings.sound_enabled {
        return;
    }

    for ev in ev_reader_play_sound.iter().filter(|ev| !ev.looped) {
        // find audio source
        let audio_source = audio_state.get_audio_source(&ev.name);
        if audio_source.is_none() {
            eprintln!("Audio source not found: {}", ev.name);
            return;
        }

        // find free channel
        let free_channel = audio_state.get_free_audio_channel();
        if free_channel.is_none() {
            eprintln!("No free audio channel found; unable to play sound: {}", ev.name);
            return;
        }

        audio.play_in_channel(audio_source.unwrap().handle.clone(), &free_channel.unwrap());
    }
}
