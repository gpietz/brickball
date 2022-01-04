pub struct GameState {
    pub pause: bool,
    pub direct_ball_movement: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pause: false,
            direct_ball_movement: false
        }
    }
}
