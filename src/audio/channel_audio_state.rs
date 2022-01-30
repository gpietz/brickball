use bevy::utils::tracing::{Event, Id, Metadata, Subscriber};
use bevy::utils::tracing::span::{Attributes, Record};
use crate::prelude::*;

pub struct ChannelAudioState {
    pub stopped: bool,
    pub paused: bool,
    pub loop_started: bool,
    pub volume: f32
}

impl Default for ChannelAudioState {
    fn default() -> Self {
        ChannelAudioState {
            volume: 1.0,
            stopped: true,
            loop_started: false,
            paused: false
        }
    }
}
