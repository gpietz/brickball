use std::thread::current;
use crate::levels::MAX_LEVELS;

pub struct GameState {
    /// Flag indicating game is in pause state.
    pub pause: bool,

    /// Flag indicating ball can be controlled with keys.
    pub direct_ball_movement: bool,

    /// Actual game level. \
    /// First one is the actual level, second for the brick spawning system.
    pub current_level: [u8; 2],
}

impl GameState {
    pub fn update_level(&mut self, new_level: u8) {
        self.current_level[0] = new_level;
        self.current_level[1] = new_level;
    }

    pub fn activate_level(&mut self, new_level: u8) {
        if new_level <= MAX_LEVELS {
            self.current_level[0] = new_level;
        }
    }

    pub fn activate_next_level(&mut self) {
        if self.current_level[0] < MAX_LEVELS {
            self.current_level[0] += 1;
        }
    }

    pub fn activate_previous_level(&mut self) {
        if self.current_level[0] > 1 {
            self.current_level[0] -= 1;
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pause: false,
            direct_ball_movement: false,
            current_level: [0, 0],
        }
    }
}
