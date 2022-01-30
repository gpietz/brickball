use crate::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

pub fn play_audio_system(
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>,
    mut ev_reader_play_sound: EventReader<PlaySoundEvent>,
) {
    let mut looped_channel = Option::None;

    for ev in ev_reader_play_sound.iter() {
        let sound_name = &ev.name;
        for source in audio_state.sources.iter() {
            if source.name == *sound_name {
                let channel = get_free_audio_channel(&audio_state);
                if channel.is_some() {
                    let ch = channel.unwrap();
                    if !ev.looped {
                        audio.play_in_channel(source.handle.clone(), &ch);
                    } else {
                        looped_channel = Option::Some(ch.clone());
                        audio.play_looped_in_channel(source.handle.clone(), &ch);
                    }
                }
                break;
            }
        }
    }

    if looped_channel.is_some() {
        audio_state.set_channel_loop_started(looped_channel.unwrap(), true);
    }
}

fn get_free_audio_channel(audio_state: &ResMut<AudioState>) -> Option<AudioChannel> {
    for channel in audio_state.channels.iter() {
        let channel_state = channel.1;
        if !channel_state.loop_started {
            let ch = channel.0.clone();
            return Some(ch);
        }
    }
    None
}
