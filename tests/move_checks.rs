use chess::Color::*;
use chess::Coord;
use chess::GameState;

#[test]
fn test_illegal_rook_move() -> Result<(), String> {
    let mut game_state = GameState::init(&Wht, "boards/rook_test");
    let first_coord = Coord::init_from_string(Some("a1")).unwrap();
    let second_coord = Coord::init_from_string(Some("e4")).unwrap();
    match game_state.move_piece(first_coord, second_coord) {
        Err(_yikes) => Ok(()),
        Ok(_pog) => Err("Should error".to_string()),
    }
}
