pub mod board_state;
pub mod pieces;
pub mod handle_input;

use crate::board_state::Board;
use crate::handle_input::take_input;

pub fn start() {
    let mut board: Board = Board::initialize_board();
    loop {
        let input = take_input();
        match board.update_board(input) {
            Ok(_) => board.print_board(),
            Err(e) => print!("{e}"),
        }
        board.show_score();
    }
    
}


