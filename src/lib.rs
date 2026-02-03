pub mod board_state;
pub mod pieces;
pub mod handle_input;

use crate::board_state::initialize_board;
use crate::board_state::Board;

pub fn start() {
    let board: Board = initialize_board();
    let grid = board.grid;
    println!("{grid:?}");
    
}


