use std::default::*;
use std::fmt;
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Color {
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

#[derive(Clone, Copy)]
pub enum Piece {
    R,
    N,
    B,
    K,
    Q,
    P,
}
use Piece::*;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R => write!(f, "R"),
            N => write!(f, "N"),
            B => write!(f, "B"),
            K => write!(f, "K"),
            Q => write!(f, "Q"),
            P => write!(f, "P"),
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    color: Option<Color>,
    piece: Option<Piece>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.color.is_none() && self.piece.is_none() {
            write!(f, "      ")
        } else {
            let piece = self.piece.as_ref().unwrap();
            let color = self.color.as_ref().unwrap();
            write!(f, "{piece}({color})")
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::init(None, None)
    }
}

impl Tile {
    fn init(color: Option<Color>, piece: Option<Piece>) -> Tile {
        Tile {
            color: color,
            piece: piece,
        }
    }

    fn init_from_string(tile_str: &str) -> Result<Tile, String> {
        // Check if string is correct length
        if !(tile_str.len() == 1 || tile_str.len() == 4) {
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

pub struct Board {
    grid: [[Tile; 8]; 8],
}

impl Default for Board {
    fn default() -> Board {
        Board::init()
    }
}

impl Board {
    fn init() -> Board {
        // Initialize a board with a completely empty grid
        let grid: [[Tile; 8]; 8] = Default::default();
        Board { grid: grid }
    }

    fn init_from_io(file_path: &str) -> Result<Board, String> {
        // Initialize a board from io input.
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        return Board::init_from_string(&contents);
    }

    fn init_from_string(config_str: &str) -> Result<Board, String> {
        // Initialize a board from string input.
        let tile_strs: Vec<&str> = config_str.split(",").collect();

        if tile_strs.len() != 64 {
            return Err(String::from("Board format is invalid, length mismatch"));
        }

        let mut board = Board::init();
        let mut ind = 0;

        for i in 0..8 as usize {
            for j in 0..8 as usize {
                let tile = match Tile::init_from_string(tile_strs[ind].trim()) {
                    Ok(tile) => tile,
                    Err(error_str) => return Err(error_str),
                };
                board.grid[i][j] = tile;
                ind += 1;
            }
        }
        Ok(board)
    }

    fn get_tile(&self, coord: Coord) -> &Tile {
        return &self.grid[coord.y][coord.x];
    }

    fn set_tile(&mut self, coord: Coord, piece: Option<Piece>, color: Option<Color>) {
        self.grid[coord.y][coord.x] = Tile {
            piece: piece,
            color: color,
        }
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
            for tile in rank {
                write!(f, "{tile} | ")?;
            }
            write!(
                f,
                "\n-------------------------------------------------------------------------\n"
            )?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Coord {
    // Coordinates read from bottom-left
    // To get index, use
    x: usize,
    y: usize,
}

impl Coord {
    pub fn init_from_string(pos: Option<&str>) -> Result<Coord, String> {
        // format is correct
        match pos {
            None => Err("Couldn't initialize blank coordinates".to_string()),
            Some(coord) => {
                if coord.len() != 2 {
                    return Err("Input is invalid length".to_string());
                }
                let mut coord_iter = coord.chars();
                let x = coord_iter.next().unwrap();
                let y = coord_iter.next().unwrap();
                if ('a'..='h').contains(&x) && ('1'..='8').contains(&y) {
                    let y = 8 as usize - y.to_digit(10).unwrap() as usize;
                    let x = x as usize - 'a' as usize;
                    return Ok(Coord { x: x, y: y });
                } else {
                    return Err("Weird coordinates you got there!".to_string());
                }
            }
        }
    }
}
pub struct GameState {
    pub in_progress: bool,
    pub turn: Color,
    pub board: Board,
}

impl GameState {
    pub fn init(turn: &Color, board_path: &str) -> GameState {
        GameState {
            in_progress: true,
            turn: *turn,
            board: Board::init_from_io(board_path).unwrap(),
        }
    }

    pub fn move_piece(&mut self, first_coord: Coord, second_coord: Coord) -> Result<(), String> {
        // If the move is legal, make the move. Return an Option

        if !self.is_legal_move(first_coord, second_coord) {
            return Err("Illegal move; enter a legal move.".to_string());
        }
        let board = &mut self.board;
        let tile = board.get_tile(first_coord);
        let piece = tile.piece.as_ref().unwrap();
        let color = tile.color.as_ref().unwrap();

        board.set_tile(second_coord, Some(*piece), Some(*color));
        board.set_tile(first_coord, None, None);
        Ok(())
    }

    fn get_legal_moves(&self, coord: Coord) -> Vec<Coord> {
        // Given a coord, return vector of legal moves based on piece located at that coord.
        let tile = self.board.get_tile(coord);

        let piece = match &tile.piece {
            None => return vec![],
            Some(piece) => piece,
        };

        let color = tile.color.as_ref().unwrap();

        let mut legal_moves = Vec::new();

        match piece {
            R => {
                let mut curr_loc = coord;

                // Check up direction
                for i in (curr_loc.y + 1)..8 {
                    curr_loc = Coord {
                        x: curr_loc.x,
                        y: i,
                    };
                    let curr_tile = self.board.get_tile(curr_loc);
                    match self.board.get_tile(curr_loc).piece {
                        None => {
                            legal_moves.push(curr_loc);
                        }
                        Some(_piece) => {
                            if &curr_tile.color.unwrap() != color {
                                legal_moves.push(curr_loc)
                            }
                            break;
                        }
                    };
                }

                // Check down direction
                for i in (curr_loc.y - 1)..=0 {
                    curr_loc = Coord {
                        x: curr_loc.x,
                        y: i,
                    };
                    let curr_tile = self.board.get_tile(curr_loc);
                    match self.board.get_tile(curr_loc).piece {
                        None => {
                            legal_moves.push(curr_loc);
                        }
                        Some(_piece) => {
                            if &curr_tile.color.unwrap() != color {
                                legal_moves.push(curr_loc)
                            }
                            break;
                        }
                    };
                }

                // Check left direction
                for j in (curr_loc.x - 1)..=0 {
                    curr_loc = Coord {
                        x: j,
                        y: curr_loc.y,
                    };
                    let curr_tile = self.board.get_tile(curr_loc);
                    match self.board.get_tile(curr_loc).piece {
                        None => {
                            legal_moves.push(curr_loc);
                        }
                        Some(_piece) => {
                            if &curr_tile.color.unwrap() != color {
                                legal_moves.push(curr_loc)
                            }
                            break;
                        }
                    };
                }

                // Check right direction
                for j in (curr_loc.y + 1)..8 {
                    curr_loc = Coord {
                        x: j,
                        y: curr_loc.y,
                    };
                    let curr_tile = self.board.get_tile(curr_loc);
                    match self.board.get_tile(curr_loc).piece {
                        None => {
                            legal_moves.push(curr_loc);
                        }
                        Some(_piece) => {
                            if &curr_tile.color.unwrap() != color {
                                legal_moves.push(curr_loc)
                            }
                            break;
                        }
                    };
                }
            }
            _ => (),
        };
        legal_moves
    }

    fn is_legal_move(&self, first_coord: Coord, second_coord: Coord) -> bool {
        // What makes a legal move?
        // Ideally, we would like to have a set of legal moves to consider.
        // Where should we store it? When should we compute the set of legal moves?

        self.get_legal_moves(first_coord).contains(&second_coord)
    }
}
