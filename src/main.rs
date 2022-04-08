mod game;

use game::{Game, GameResponse};
use std::io;
fn main() {
    let mut g = Game::new();
    let stdin = io::stdin();

    loop {
        println!("Enter command:");
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                match g.parse_command() {
                    GameResponse::OK => {
                        println!("Command received");
                    }
                    GameResponse::EXIT => {
                        println!("Exiting game");
                        break;
                    }
                    GameResponse::UNKNOWN => {
                        println!("Unknown command");
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
            }
        }
    }
}