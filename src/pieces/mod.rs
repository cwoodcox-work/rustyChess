pub mod movement;
use crate::board_state::Square;

#[derive(Debug,Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
    pub square: Square,
}

#[derive(Eq, Hash, PartialEq,Clone,Debug)]
pub enum Kind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}
#[derive(Debug,PartialEq,Clone)]
pub enum Color {
    White,
    Black,
}

