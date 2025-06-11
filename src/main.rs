use std::fmt;
use std::fs;
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
    R,
    N,
    B,
    K,
    Q,
    P,
}
use Piece::*;

/*
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
*/

struct Tile {
    color: Option<Color>,
    piece: Option<Piece>,
}

impl Tile {
    fn init(color: Option<Color>, piece: Option<Piece>) -> Tile {
        Tile {
            color: color,
            piece: piece,
        }
    }

    fn init_from_string(tile_str: &str) -> Result<Tile, String> {
        println!("{tile_str}");

        // Check if string is correct length
        if (tile_str.len() == 1 || tile_str.len() == 4) {
            return Err(String::from(
                "invalid tile format; incorrect number of chars",
            ));
        }

        // First char definitely exists at this point
        let first_char = tile_str.chars().nth(0).unwrap();

        // Handle empty squares first
        if first_char == '-' {
            return Ok(Tile {
                color: None,
                piece: None,
            });
        }

        // Check if longer string is correctly formatted
        if !(tile_str.chars().nth(1).unwrap() == '(' && tile_str.chars().nth(3).unwrap() == ')') {
            return Err(String::from("invalid tile format; expected parenthesis"));
        }

        // Third char definitely exists and is relevant at this point
        let third_char = tile_str.chars().nth(2).unwrap();

        let piece: Piece = match first_char {
            'r' => R,
            'n' => N,
            'b' => B,
            'q' => Q,
            'k' => K,
            'p' => P,
            _ => return Err(String::from("did not enter a valid piece")),
        };

        let color: Color = match third_char {
            'b' => Blk,
            'w' => Wht,
            _ => return Err(String::from("did not enter a valid color")),
        };

        Ok(Tile {
            color: Some(color),
            piece: Some(piece),
        })
    }
}

struct Board {
    grid: Option<[[Tile; 8]; 8]>,
}

// TODO: work on this!
impl Board {
    fn init_from_io(file_path: &str) -> Result<Board, String> {
        // Initialize a board from io input.
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        return Board::init_from_string(&contents);
    }

    fn init_from_string(config_str: &str) -> Result<Board, String> {
        // Initialize a board from string input.
        let tile_strs: Vec<&str> = config_str.split(",").collect();

        println!("{:#?}", tile_strs.len());

        if tile_strs.len() != 64 {
            return Err(String::from("Board format is invalid, length mismatch"));
        }

        let mut tile_vec: Vec<Vec<Tile>> = vec![];
        let mut tile_ind: usize = 0;

        for i in 0..8 as usize {
            tile_vec.push(vec![]);
            for j in 0..8 as usize {
                tile_vec[i][j] = match Tile::init_from_string(tile_strs[tile_ind].trim()) {
                    Ok(tile) => tile,
                    Err(error_str) => return Err(error_str),
                };
                tile_ind += 1
            }
        }

        for tile_str in tile_strs {
            let tile = match Tile::init_from_string(tile_str.trim()) {
                Ok(tile) => tile,
                Err(error_str) => return Err(error_str),
            };
        }

        Ok(Board { grid: None })
    }
}

/*
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
*/

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
            board: Board::init_from_io("boards/standard").unwrap(),
        }
    }
    /*
        fn move_piece(
            self,
            first_coord: (usize, usize),
            second_coord: (usize, usize),
        ) -> Result<(), String> {
            // If the move is legal, make the move. Return an Option
            if !self.is_legal_move(first_coord, second_coord) {
                return Err("Illegal move; enter a legal move.".to_string());
            }
            Ok(())
        }

        fn get_legal_moves(self, coord: (usize, usize)) -> Vec<(usize, usize)> {
            // Given piece, return vector of legal moves
            let piece = self.board.grid[coord.0][coord.1];
            let mut legal_moves = Vec::new();

            match piece {
                Empty => Vec::new(),
                R(c) => {
                    let mut curr_loc = coord;
                    let hi = (curr_loc.0 + 1)..8;
                    // Check up direction
                    for i in (curr_loc.0 + 1)..8 {
                        match self.board.grid[i][curr_loc.1] {
                            Empty => legal_moves.push((i, curr_loc.1)),
                            peepee => {
                                if color == c {
                                    break
                                } else {
                                    legal_moves.push((i, curr_loc.1));
                                    break
                                }
                            }
                        };
                    }
                    // Check down direction
                    for i in [(curr_loc.0 - 1)..=0] {
                        match self.board.grid[i][curr_loc.1] {
                            Empty => legal_moves.append((i, curr_loc.1)),
                            curr_piece(color) => {
                                if color == c {
                                    break
                                } else {
                                    legal_moves.append((i, curr_loc.1));
                                    break
                                }
                            }
                        };
                    }
                    // Check left direction
                    for j in [(curr_loc.1 - 1)..-1] {
                        match self.board.grid[curr_loc.0][j] {
                            Empty => legal_moves.append((curr_loc.0, j)),
                            curr_piece(color) => {
                                if color == c {
                                    break
                                } else {
                                    legal_moves.append((curr_loc.0, j));
                                    break
                                }
                            }
                        };
                    }
                    // Check right direction
                    for j in [(curr_loc.1 + 1)..8] {
                        match self.board.grid[curr_loc.0][j] {
                            Empty => legal_moves.append((curr_loc.0, j)),
                            curr_piece(color) => {
                                if color == c {
                                    break
                                } else {
                                    legal_moves.append((curr_loc.0, j));
                                    break
                                }
                            }
                        };
                    }
                },
            }
        }

        fn is_legal_move(self, first_coord: (usize, usize), second_coord: (usize, usize)) -> bool {
            // What makes a legal move?
            // Ideally, we would like to have a set of legal moves to consider.
            // Where should we store it? When should we compute the set of legal moves?
            let first_piece = self.board.grid[first_coord.0][first_coord.1];

            let first_coord = (first_coord.0 as i32, first_coord.1 as i32);
            let second_coord = (second_coord.0 as i32, second_coord.1 as i32);

            match first_piece {
                Empty => false,
                R(c) => is_legal_move_rook() // first_coord.0 == second_coord.0 || first_coord.1 == second_coord.1,
                N(c) => {
                    abs(first_coord.0 - second_coord.0) == 2 && abs(first_coord.1 - second_coord.1) == 1 ||
                    abs(first_coord.1 - second_coord.1) == 2 && abs(first_coord.0 - second_coord.0) == 1
                },
                B(c) => abs(first_coord.0 - second_coord.0) == abs(first_coord.1 - second_coord.1),
                K(c) => abs(first_coord.0 - second_coord.0) <= 1 && abs(first_coord.1 - second_coord.1) <= 1,
                Q(c) => {
                    (first_coord.0 == second_coord.0 || first_coord.1 == second_coord.1) ||
                    abs(first_coord.0 - second_coord.0) == abs(first_coord.1 - second_coord.1)
                },
                P(c) => {
                    // TODO: fix this simple approximation
                    if c == Wht {
                        first_coord.0 - second_coord.0 == 1 && first_coord.1 == second_coord.1
                    } else {
                        first_coord.0 - second_coord.0 == -1 && first_coord.1 == second_coord.1
                    }
                }
            }

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
    */
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

        /*
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
        */
    }
}
