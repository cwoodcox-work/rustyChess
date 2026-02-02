#[derive(Debug)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
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
#[derive(Debug)]
pub enum Color {
    White,
    Black,
}