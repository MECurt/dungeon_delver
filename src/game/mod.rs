mod tile;

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
    LOOK,
    WAIT,
    EXIT,
    UNKNOWN,
}
pub struct Game {
    turn_count: u32,
    floor: Vec<Vec<tile::Tile>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn_count: 0,
            floor: tile::floor(5, 5),
        }
    }

    pub fn parse_command(&mut self, input: &String) -> GameResponse {
        match Game::command_map(input) {
            GameCommand::EXIT => GameResponse::EXIT,
            GameCommand::LOOK => {
                self.display_floor_at(5, (1, 1));
                GameResponse::OK
            },
            GameCommand::UNKNOWN => GameResponse::UNKNOWN,
            _ => {
                self.turn_count+=1;
                GameResponse::OK
            }
        }
    }

    fn command_map(input: &String) -> GameCommand {
        use GameCommand::*;
        match input.trim() {
            "north" => NORTH,
            "south" => SOUTH,
            "east" => EAST,
            "west" => WEST,

            "up" => NORTH,
            "down" => SOUTH,
            "right" => EAST,
            "left" => WEST,

            "look" => LOOK,
            "wait" => WAIT,

            "exit" => EXIT,
            "quit" => EXIT,

            _ => UNKNOWN,
        }
    }

    fn display_floor_at(&self, radius: u32, id: (i32, i32)) {
        let (centre_x, centre_y) = id;
        for y in centre_y - radius as i32 .. centre_y + radius as i32 + 1 {
            for x in centre_x - radius as i32 .. centre_x + radius as i32 + 1 {
                if x >= 0 && y >= 0 && x < self.floor.len() as i32 && y < self.floor[0].len() as i32 {
                    print!("{}", self.floor[x as usize][y as usize].show());
                } else {
                    print!("{}", '#');
                }
            }
            println!("");
        }
    }
}