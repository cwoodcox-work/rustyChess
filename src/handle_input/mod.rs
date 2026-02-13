use core::fmt;
use std::collections::HashMap;
use std::io;
use crate::board_state::Square;
use crate::pieces::Piece;
use crate::pieces::Kind;
use crate::pieces::Color;
use crate::pieces::movement::find_potential_moves;
use crate::board_state::Board;

#[derive(Clone)]
pub struct Move {
    //boolean represents capture move or not. u32 is score if capture. it is 1 if blocked by same color piece. 0 if open square.
    pub capture: (bool,u32),
    pub square: Square,
    pub kind: Kind,
    //old_mov holds the potential coordinates of where the piece is coming from. coordinates are 0 if unknown
    pub old_mov: (char,char),
}
#[derive(Debug)]
pub enum MoveError {
    AmbiguousMove,
    OccupiedSameColor,
    NoPieceToMove,
    WrongFormat,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveError::AmbiguousMove => write!(f, "Need to specify the original location of the piece being moved. "),
            MoveError::NoPieceToMove => write!(f, "There is no possible piece to move to the square provided. "),
            MoveError::OccupiedSameColor => write!(f, "The square given is already occupied by your own piece. "),
            MoveError::WrongFormat => write!(f, "Whatever you entered, doesn't seem to be in the correct format. "),
        }
    }
}

pub fn take_input() -> String {
    println!("Enter your move in chess Algebraic notation. Example: Nf3");
    let mut user_move = String::new();
    io::stdin()
        .read_line(&mut user_move)
        .expect("Failed to read line");

    return user_move.trim().to_string();

}

pub fn move_handler(board: &mut Board, input: String)  -> Result<(), MoveError> {
    let new_move = match translate_input(input,board) {
        Ok(mov) => mov,
        Err(error) => return Err(error),
    };

    let potential_moves = match find_potential_moves(&new_move,&board) {
        Ok(list) => list,
        Err(m) => return Err(m),
    };
    
    let mut loo = false;
    let mut fin_move = Square{x:"0".to_string(), y:"0".to_string()};
    if potential_moves.len() > 1 && new_move.old_mov.0 != '0' {
        loo = true;
        for sq in &potential_moves {
            if sq.x == new_move.old_mov.0.to_string() && sq.y == new_move.old_mov.0.to_string() {
                fin_move = sq.clone();
            }
            else if sq.x == new_move.old_mov.0.to_string() {
                fin_move = sq.clone();
            }
        }
        if fin_move.x == "0".to_string() { 
            return Err(MoveError::AmbiguousMove);
        }
    }       

    if potential_moves.len() == 1 || loo {
        let points = new_move.capture;
        let mut og_square = Square {x:potential_moves[0].x.clone(),y:potential_moves[0].y.clone()};
        if loo {
            og_square = fin_move;
        } 
        {
            let piece_list = match board.piece_registry.get_mut(&(new_move.kind.clone(),board.turn.clone())) {
                Some(i) => i,
                None => return Err(MoveError::NoPieceToMove),
            };
            piece_list.remove(&og_square);
            piece_list.insert(new_move.square.clone());
        }
        let old_kind = match board.grid.get(&og_square).unwrap() {
                Some(i) => (i.kind.clone(),i.color.clone()),
                None => panic!("this should never happend since we know this is a capture"),
        };       
        board.piece_registry.get_mut(&old_kind).unwrap().remove(&new_move.square);
        if points.0 {
            match board.score.get_mut(&board.turn) {
                Some(i) => *i -= points.1,
                None => panic!("Shouldn't happen"),
            };
        }
        let piece_moving = match board.grid.get(&og_square).unwrap() {
            Some(i) => i.clone(),
            None => panic!("this should never happen"),
        };
        board.grid.insert(og_square,None);
        board.grid.insert(new_move.square,Some(piece_moving));
        board.turn = match board.turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
        board.move_count += 1;
        return Ok(());
    }  
    else {
        return Err(MoveError::NoPieceToMove);
    }

}

fn translate_input(input: String,board: &Board)  -> Result<Move,MoveError> {
    let mut kind: char = ' ';
    let mut uppercase_count = 0;
    let mut coordinates: Vec<(char,char)> = Vec::new();
    let mut old_sq: (char,char) = ('0','0');
    let mut new_sq: (char, char) = ('0','0');
    let coordinate_x_translate_table: HashMap<char,char> = HashMap::from([
        ('a','1'),
        ('b','2'),
        ('c','3'),
        ('d','4'),
        ('e','5'),
        ('f','6'),
        ('g','7'),
        ('h','8'),]);
    for char in input.chars() {
        if char.is_lowercase() {
            let y = match input.find(char) {
                Some(i) => if input.chars().nth(i+1).unwrap().is_ascii_digit() {
                    input.chars().nth(i+1).unwrap()
                }
                else {
                    '0'
                },
                None => return Err(MoveError::WrongFormat),
            };
            let x = match coordinate_x_translate_table.get(&char) {
                Some(i) => i,
                None => return Err(MoveError::WrongFormat),
            };
            coordinates.push((*x, y));
        }
        else if char.is_uppercase() {
            uppercase_count += 1;
            kind = char;
        }
    };
    if coordinates.len() > 1 {
        old_sq = coordinates[0].clone();
        new_sq = coordinates[1].clone();
    }
    else {
        new_sq = coordinates[0].clone();
    }
    if uppercase_count == 0 {
        kind = 'P';
        if old_sq == ('0','0') {
            old_sq = (new_sq.0,'0');
        }
    };
    let kind = match kind {
        'N' => Kind::Knight,
        'K' => Kind::King,
        'Q' => Kind::Queen,
        'R' => Kind::Rook,
        'B' => Kind::Bishop,
        'P' => Kind::Pawn,
         _  => return Err(MoveError::WrongFormat),
    }; 
    let new_square = Square {
        x:new_sq.0.to_string(),
        y:new_sq.1.to_string(),
    }; 
    let capture:(bool,u32) = match board.grid.get(&new_square).unwrap() {
        Some(i) => if i.color==board.turn {
            return Err(MoveError::OccupiedSameColor)
        }
        else {
            (true,i.kind.points())
        },

        None => (false,0), 
    };
    return Ok(Move {
        capture: capture,
        square: new_square,
        kind: kind,
        old_mov: old_sq,
    })
     
    
}
