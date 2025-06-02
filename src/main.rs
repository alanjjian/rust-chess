use std::fmt;

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

fn main() {
    let my_bussy = Board::init();
    println!("{my_bussy}");
}
