pub mod board_state;
pub mod pieces;
pub mod handle_input;

use crate::board_state::Board;
use crate::handle_input::take_input;
use crate::board_state::Square;

pub fn start() {
    let mut board: Board = Board::initialize_board();
    let grid = &board.grid;
    println!("{grid:?}");
    let input = take_input();
    board.update_board(input);
    let square = Square {
        x:"6".to_string(),
        y:"3".to_string(),
    };
    let variable = match board.grid.get(&square).unwrap() {
        Some(i) => true,
        None => true,
    };
    if variable {
        println!("it worked");
    }
    else {
        println!("it didnt work");
    }
}


