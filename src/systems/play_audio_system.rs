use crate::{prelude::*, AudioState, EventReader, PlaySoundEvent, ResMut};
use bevy_kira_audio::{Audio, AudioChannel};

pub fn play_audio_system(
    audio: Res<Audio>,
    audio_state: ResMut<AudioState>,
    mut ev_reader: EventReader<PlaySoundEvent>
) {
    for ev in ev_reader.iter() {
        let sound_name = &ev.0;


        for source in audio_state.sources.iter() {
            if source.name == *sound_name {

                let channel = get_free_audio_channel(&audio_state);
                if channel.is_some() {
                    let ch = channel.unwrap();
                    audio.play_in_channel(source.handle.clone(), &ch);

                    println!("Playing .... {} (loaded: {})", sound_name.clone(), source.loaded);
                }
                return;
            }
        }
    }
}

fn get_free_audio_channel(audio_state: &ResMut<AudioState>) -> Option<AudioChannel> {
    for channel in audio_state.channels.iter() {
        let ch = channel.0.clone();
        return Some(ch);
    }
    None
}
