
use chess::GameState;
use chess::Coord;
use std::io;

fn main() {
    let mut my_game = GameState::init();

    while my_game.in_progress {
        println!("{}", &my_game.board);
        println!("{} to move:", &my_game.turn);
        let mut proposed_move = String::new();

        io::stdin()
            .read_line(&mut proposed_move)
            .expect("Failed to read line");

        let mut proposed_move = proposed_move.trim().split_whitespace();

        let first_pos = proposed_move.next();

        let first_coord = match Coord::init_from_string(first_pos) {
            Err(string) => {
                println!("{string}");
                continue;
            }
            Ok(coord) => coord,
        };

        let second_pos = proposed_move.next();

        let second_coord = match Coord::init_from_string(second_pos) {
            Err(string) => {
                println!("{string}");
                continue;
            }
            Ok(coord) => coord,
        };

        match my_game.move_piece(first_coord, second_coord) {
            Err(string) => {
                println!("{string}");
                continue;
            }
            _ => (),
        };
    }
}

// TODO: Figure out how unit testing works in rust, implement custom boards and puzzles
