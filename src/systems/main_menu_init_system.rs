use crate::prelude::*;

pub fn main_menu_init_system(
    mut ev_writer: EventWriter<PlaySoundEvent>,
) {
    ev_writer.send(PlaySoundEvent::looped("music_loop"));
}
