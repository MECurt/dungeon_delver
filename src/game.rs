pub enum GameResponse {
    OK,
    EXIT,
    UNKNOWN,
}

enum GameCommand {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    WAIT,
    EXIT,
}
pub struct Game {
    turn_count: u64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn_count: 0,
        }
    }

    pub fn parse_command(&mut self, input: &String) -> GameResponse {
        match Game::command_map(input) {
            Some(command) => {
                match command {
                    GameCommand::EXIT => GameResponse::EXIT,
                    _ => {
                        self.turn_count+=1;
                        GameResponse::OK
                    }
                }
            },
            None => GameResponse::UNKNOWN,
        }
    }

    fn command_map(input: &String) -> Option<GameCommand> {
        use GameCommand::*;
        match input.trim() {
            "north" => Some(NORTH),
            "south" => Some(SOUTH),
            "east" => Some(EAST),
            "west" => Some(WEST),

            "up" => Some(NORTH),
            "down" => Some(SOUTH),
            "right" => Some(EAST),
            "left" => Some(WEST),

            "wait" => Some(WAIT),

            "exit" => Some(EXIT),
            "quit" => Some(EXIT),

            _ => None,
        }
    }
}