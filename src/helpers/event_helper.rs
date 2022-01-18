use crate::{EventReader, GameCommand, GameCommandEvent};

pub fn has_game_command(ev_reader: &mut EventReader<GameCommandEvent>, command: GameCommand) -> bool {
    for ev in ev_reader.iter() {
        if ev.0 == command {
            return true;
        }
    }
    false
}
