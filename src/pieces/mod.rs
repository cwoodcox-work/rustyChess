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

impl Kind {
    pub fn points(&self) -> u32 {
        match self {
            Kind::Bishop => 3,
            Kind::Knight => 3,
            Kind::Pawn => 1,
            Kind::Rook => 5,
            Kind::Queen => 9,
            Kind::King => 10,
        }
    }
}
#[derive(Debug,PartialEq,Clone,Eq,Hash)]
pub enum Color {
    White,
    Black,
}

