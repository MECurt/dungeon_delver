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

    pub fn parse_command(&mut self) -> GameResponse {
        self.turn_count+=1;
        if self.turn_count < 3 {
            GameResponse::OK
        } else {
            GameResponse::EXIT
        }
    }
}