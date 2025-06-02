use std::char;
use std::fmt;
use std::io;

enum Color {
    // #idoseecoloractually
    Blk,
    Wht,
}
use Color::*;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Blk => write!(f, "Blk"),
            Wht => write!(f, "Wht"),
        }
    }
}

enum Piece {
    R(Color),
    N(Color),
    B(Color),
    K(Color),
    Q(Color),
    P(Color),
    E,
}
use Piece::*;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R(color) => write!(f, "R({color})"),
            N(color) => write!(f, "N({color})"),
            B(color) => write!(f, "B({color})"),
            K(color) => write!(f, "K({color})"),
            Q(color) => write!(f, "Q({color})"),
            P(color) => write!(f, "P({color})"),
            E => write!(f, "      "),
        }
    }
}

struct Board {
    grid: [[Piece; 8]; 8],
}

impl Board {
    fn init() -> Board {
        let grid = [
            [
                R(Blk),
                N(Blk),
                B(Blk),
                Q(Blk),
                K(Blk),
                B(Blk),
                N(Blk),
                R(Blk),
            ],
            [
                P(Blk),
                P(Blk),
                P(Blk),
                P(Blk),
                P(Blk),
                P(Blk),
                P(Blk),
                P(Blk),
            ],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [
                P(Wht),
                P(Wht),
                P(Wht),
                P(Wht),
                P(Wht),
                P(Wht),
                P(Wht),
                P(Wht),
            ],
            [
                R(Wht),
                N(Wht),
                B(Wht),
                Q(Wht),
                K(Wht),
                B(Wht),
                N(Wht),
                R(Wht),
            ],
        ];
        Board { grid: grid }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "-------------------------------------------------------------------------\n"
        )?;
        for rank in &self.grid {
            write!(f, "| ")?;
            for piece in rank {
                write!(f, "{piece} | ")?;
            }
            write!(
                f,
                "\n-------------------------------------------------------------------------\n"
            )?;
        }
        Ok(())
    }
}

struct GameState {
    in_progress: bool,
    turn: Color,
    board: Board,
}

impl GameState {
    fn init() -> GameState {
        GameState {
            in_progress: true,
            turn: Wht,
            board: Board::init(),
        }
    }
}

fn convert_coord(pos: Option<&str>) -> Option<(u32, u32)> {
    // format is correct
    match pos {
        None => None,
        Some(coord) => {
            if coord.len() != 2 {
                return None;
            }
            let mut coord_iter = coord.chars();
            let x = coord_iter.next().unwrap();
            let y = coord_iter.next().unwrap();
            if ('a'..='h').contains(&x) && ('1'..='8').contains(&y) {
                let y = 8 - y.to_digit(10).unwrap();
                let x = x as u32 - 'a' as u32;
                return Some((y, x));
            } else {
                return None;
            }
        }
    }
}

fn main() {
    let my_game = GameState::init();
    while my_game.in_progress {
        println!("{} to move:", my_game.turn);
        let mut proposed_move = String::new();

        io::stdin()
            .read_line(&mut proposed_move)
            .expect("Failed to read line");

        let mut proposed_move = proposed_move.trim().split_whitespace();

        let first_pos = proposed_move.next();

        let first_coord = match convert_coord(first_pos) {
            None => {
                println!("Invalid format: Use chess coordinates to describe position (i.e a6)");
                continue;
            }
            Some(coord) => coord,
        };

        let second_pos = proposed_move.next();

        let second_coord = match convert_coord(second_pos) {
            None => {
                println!("Invalid format: Use chess coordinates to describe position (i.e a6)");
                continue;
            }
            Some(coord) => coord,
        };

        println!("{:#?}", first_coord);
        println!("{:#?}", second_coord);
    }
}
