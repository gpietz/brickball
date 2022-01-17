use std::vec;

#[derive(PartialEq)]
pub enum GameCommand {
    ShowCoordinates,
    RemoveBricks,
    CenterBall,
}

pub struct GameCommandStack {
    commands: Vec<GameCommand>,
}

impl Default for GameCommandStack {
    fn default() -> Self {
        Self {
            commands: Vec::new()
        }
    }
}

impl GameCommandStack {
    pub fn add(&mut self, command: GameCommand) -> bool {
        if !self.commands.contains(&command) {
            self.commands.push(command);
            return true;
        }
        false
    }

    pub fn contains(&mut self, command: GameCommand, auto_remove: bool) -> bool {
        false //let index = self.commands.iter().position(|&c| c == command).unwrap();
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
