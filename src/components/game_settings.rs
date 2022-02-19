use serde_derive::{Deserialize, Serialize};
use std::fs::File;

pub const GAMESETTINGS_FILENAME: &str = "brickball.json";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GameSettings {
    pub sound_enabled: bool,
    pub music_enabled: bool,
    pub mouse_enabled: bool,
    pub fps_display_enabled: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            music_enabled: true,
            mouse_enabled: false,
            fps_display_enabled: false,
        }
    }
}

impl GameSettings {
    pub fn toggle_sound_enabled(&mut self) {
        self.sound_enabled = !self.sound_enabled;
        self.save();
        println!("{}", if self.sound_enabled {
            "Sounds enabled."
        } else {
            "Sounds disabled."
        });

    }

    pub fn toggle_music_enabled(&mut self) {
        self.music_enabled = !self.music_enabled;
        self.save();
        println!("{}", if self.music_enabled {
            "Music playback enabled."
        } else {
            "Music playback disabled."
        });
    }

    pub fn toggle_fps_display_enabled(&mut self) {
        self.fps_display_enabled = !self.fps_display_enabled;
        self.save();
        println!("{}", if self.fps_display_enabled {
            "FPS display enabled."
        } else {
            "FPS display disabled."
        });
    }

    pub fn save(&self) -> bool {
        let mut file = File::create(GAMESETTINGS_FILENAME);
        if file.is_err() {
            return false;
        }
        ::serde_json::to_writer(file.unwrap(), &self).is_ok()
    }
}
