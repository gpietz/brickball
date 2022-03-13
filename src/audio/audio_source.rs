use crate::prelude::*;
use bevy_kira_audio::AudioSource as BevyAudioSource;

pub struct AudioSource {
    pub name: String,
    pub handle: Handle<BevyAudioSource>,
    pub loaded: bool
}
