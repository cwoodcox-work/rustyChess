use std::collections::HashMap;
use crate::pieces::Piece;

pub struct Move {
    pub piece: Piece,
    pub square: String,
    pub capture: bool,
    pub castle: bool,
    pub check: bool,
}

pub fn translate_input(input: String) -> Move {
    let mut coordinate_x_translate_table: HashMap<&str,&str> = HashMap::from([
        ("a","1"),
        ("b","2"),
        ("c","3"),
        ("d","4"),
        ("e","5"),
        ("f","6"),
        ("g","7"),
        ("h","8"),
    ]);

}
