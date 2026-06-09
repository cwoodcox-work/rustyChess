pub mod movement;
use crate::board_state::Square;

#[derive(Debug,Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
    pub square: Square,
    pub moved: bool,
}

impl Piece {
    pub fn symbol(&self) -> &str {
        if self.color == Color::White {
            match self.kind {
            Kind::Bishop => "\u{265D}",
            Kind::Knight => "\u{265E}",
            Kind::Pawn => "\u{265F}",
            Kind::Rook => "\u{265C}",
            Kind::Queen => "\u{265B}",
            Kind::King => "\u{265A}",
            _ => "0",
            }
    }
        else {
            match self.kind {
            Kind::Bishop => "\u{2657}",
            Kind::Knight => "\u{2658}",
            Kind::Pawn => "\u{2659}",
            Kind::Rook => "\u{2656}",
            Kind::Queen => "\u{2655}",
            Kind::King => "\u{2654}",
            _ => "0",
            }
        }
    }
}




#[derive(Eq, Hash, PartialEq,Debug,Copy,Clone)]
pub enum Kind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
    Castle,
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
            Kind::Castle => 0,
        }
    }
}

#[derive(Debug,PartialEq,Clone,Eq,Hash,Copy)]
pub enum Color {
    White,
    Black,
}

