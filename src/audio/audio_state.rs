use crate::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use bevy_kira_audio::AudioChannel;

const UNKNOWN_AUDIO_CHANNEL : &str = "Unknown audio channel!";

pub struct AudioState {
    pub audio_loaded: bool,
    pub sources: Vec<AudioSource>,
    pub channels: HashMap<AudioChannel, ChannelAudioState>
}

impl AudioState {
    /** Changes the loop flag for a specific channel. */
    pub fn set_channel_loop_started(&mut self, channel: AudioChannel, loop_started: bool) {
        for audio_channel in self.channels.iter_mut() {
            if *audio_channel.0 == channel {
                audio_channel.1.loop_started = loop_started;
                return;
            }
            panic!("{}", UNKNOWN_AUDIO_CHANNEL);
        }
    }

    pub fn is_channel_loop_started(&self, channel: AudioChannel) -> bool {
        for audio_channel in self.channels.iter() {
            if *audio_channel.0 == channel {
                return audio_channel.1.loop_started;
            }
        }
        panic!("{}", UNKNOWN_AUDIO_CHANNEL);
    }
}
