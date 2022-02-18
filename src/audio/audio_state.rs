use crate::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use bevy_kira_audio::{Audio, AudioChannel};

const UNKNOWN_AUDIO_CHANNEL : &str = "Unknown audio channel!";

pub struct AudioState {
    sources: Vec<AudioSource>,
    channels: HashMap<AudioChannel, ChannelAudioState>
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            sources: Vec::new(),
            channels: HashMap::new(),
        }
    }
}

impl AudioState {
    pub fn create_channel(&mut self, name: &str) {
        let audio_channel = AudioChannel::new(name.to_owned());
        self.channels.insert(audio_channel, ChannelAudioState::default());
    }

    pub fn add_channel(&mut self, channel: AudioChannel) {
        self.channels.insert(channel, ChannelAudioState::default());
    }

    pub fn add_source(&mut self, source: AudioSource) {
        self.sources.push(source);
    }

    /** Changes the loop flag for a specific channel. */
    pub fn set_channel_looped(&mut self, channel: AudioChannel, looped: bool) -> bool {
        let mut audio_channel = self.channels.iter_mut().find(|ch| *ch.0 == channel);
        if audio_channel.is_some() {
            audio_channel.unwrap().1.loop_started = looped;
            return true;
        }
        false
    }

    pub fn is_channel_loop_started(&self, channel: AudioChannel) -> bool {
        for (audio_channel, channel_state) in self.channels.iter() {
            if *audio_channel == channel {
                return channel_state.loop_started;
            }
        }
        panic!("{}", UNKNOWN_AUDIO_CHANNEL);
    }

    pub fn is_channel_known(&self, channel: &AudioChannel) -> bool {
        self.channels.contains_key(channel)
    }

    pub fn get_free_audio_channel(&self) -> Option<AudioChannel> {
        for (audio_channel, channel_state) in self.channels.iter() {
            if !channel_state.loop_started {
                let ch = audio_channel.clone();
                return Some(ch);
            }
        }
        None
    }

    pub fn get_looped_audio_channel(&self) -> Result<AudioChannel, &str> {
        let mut result = Result::Err("No looped audio channel found!");
        for channel in self.channels.iter() {
            let channel_state = channel.1;
            if channel_state.loop_started {
                if result.is_err() {
                    result = Result::Ok(channel.0.clone());
                    break;
                }
                panic!("More a one looped audio channel found!");
            }
        }
        result
    }

    pub fn get_audio_source(&self, name: &str) -> Option<&AudioSource> {
        self.sources.iter().find(|a| a.name == name)
    }

    /** Returns a flag indicating whether all audio sources have been loaded. */
    pub fn all_loaded(&self) -> bool {
        self.sources.iter().filter(|&s| !s.loaded).count() == 0
    }
}
