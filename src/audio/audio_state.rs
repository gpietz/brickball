use crate::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use bevy_kira_audio::AudioChannel;
use crate::audio::AudioSource;

pub struct AudioState {
    pub audio_loaded: bool,
    pub sources: Vec<AudioSource>,
    pub channels: HashMap<AudioChannel, ChannelAudioState>
}
