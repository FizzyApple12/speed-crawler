pub enum InputBindings {
    PlayerMoveUp,
    PlayerMoveDown,
    PlayerMoveLeft,
    PlayerMoveRight,

    GamePause,
}

impl From<InputBindings> for &str {
    fn from(binding: InputBindings) -> Self {
        match binding {
            InputBindings::PlayerMoveUp => "player_move_up",
            InputBindings::PlayerMoveDown => "player_move_down",
            InputBindings::PlayerMoveLeft => "player_move_left",
            InputBindings::PlayerMoveRight => "player_move_right",

            InputBindings::GamePause => "game_pause",
        }
    }
}
