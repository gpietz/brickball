use bevy_kira_audio::AudioChannel;
use crate::prelude::*;

pub struct AudioSource {
    pub name: String,
    pub handle: Handle<bevy_kira_audio::AudioSource>,
    pub loaded: bool
}
