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
    Empty,
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
            Empty => write!(f, "      "),
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
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
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

    fn is_legal_move(self, first_coord: (usize, usize), second_coord: (usize, usize)) -> bool {
        // What makes a legal move?
        // Ideally, we would like to have a set of legal moves to consider.
        // Where should we store it? When should we compute the set of legal moves?
        let first_piece = self.grid[first_coord.0][first_coord.1];
        
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

    fn move_piece(
        self,
        first_coord: (usize, usize),
        second_coord: (usize, usize),
    ) -> Result<(), String> {
        // If the move is legal, make the move. Return an Option
        if !self.board.is_legal_move(first_coord, second_coord) {
            return Err("Illegal move; enter a legal move.".to_string());
        }
        Ok(())
    }
}

fn convert_coord(pos: Option<&str>) -> Option<(usize, usize)> {
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
                let y = 8 as usize - y.to_digit(10).unwrap() as usize;
                let x = x as usize - 'a' as usize;
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

        my_game.move_piece(first_coord, second_coord);

        println!("{:#?}", first_coord);
        println!("{:#?}", second_coord);
    }
}
