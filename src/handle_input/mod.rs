use core::fmt;
use std::collections::HashMap;
use std::io;
use crate::board_state::Square;
use crate::pieces::Kind;
use crate::pieces::Color;
use crate::pieces::movement::find_potential_moves;
use crate::board_state::Board;

pub struct Move {
    pub capture: (bool,u32),
    pub square: Square,
    pub kind: Kind,
}
#[derive(Debug)]
pub enum MoveError {
    AmbiguousMove,
    OccupiedSameColor,
    NoPieceToMove,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveError::AmbiguousMove => write!(f, "Need to specify the original location of the piece being moved."),
            MoveError::NoPieceToMove => write!(f, "There is no possible piece to move to the square provided."),
            MoveError::OccupiedSameColor => write!(f, "The square given is already occupied by your own piece."),
        }
    }
}

pub fn take_input() -> String {
    println!("Enter your move in chess notation. Example: Nf3");
    let mut user_move = String::new();
    io::stdin()
        .read_line(&mut user_move)
        .expect("Failed to read line");

    return user_move.trim().to_string();

}

pub fn move_handler(board: &mut Board, input: String)  -> Result<(), MoveError> {
    let new_move = translate_input(input,board);
    if new_move.capture == (false,1) {
        return Err(MoveError::OccupiedSameColor);
    }
    let potential_moves = find_potential_moves(&new_move,&board);
    if potential_moves.len() > 1 {
        return Err(MoveError::AmbiguousMove);
    }
    else if potential_moves.len() == 1 {
        let points = new_move.capture;
        let og_square = Square {x:potential_moves[0].x.clone(),y:potential_moves[0].y.clone()};
        let piece_moving = match board.grid.remove(&og_square).unwrap() {
            Some(i) => i,
            None => panic!("this should never happen"),
        };
        board.grid.insert(og_square,None);
        board.grid.insert(new_move.square,Some(piece_moving));
        board.turn = match board.turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
        if points.0 {
            match board.score.get_mut(&board.turn) {
                Some(i) => *i -= points.1,
                None => panic!("Shouldn't happen"),
            };
        }
        board.move_count += 1;
        return Ok(());
    }  
    else {
        return Err(MoveError::NoPieceToMove);
    }

}

fn translate_input(input: String,board: &Board)  -> Move{
    let mut the_move = Vec::new();
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
        the_move.push(char);
    }
    if the_move.len() == 2{
        the_move.insert(0,'P');
    }
    let kind = match the_move[0] {
        'N' => Kind::Knight,
        'K' => Kind::King,
        'Q' => Kind::Queen,
        'R' => Kind::Rook,
        'B' => Kind::Bishop,
        'P' => Kind::Pawn,
         _  => panic!(),
    };
    let x = match coordinate_x_translate_table.get(&the_move[1].to_lowercase().to_string().chars().next().unwrap()) {
        Some(i) => i,
        None => panic!("incorrect format"),
    };
        
    let new_square = Square {
        x:x.to_string(),
        y:the_move[2].to_string(),
    };  
    let capture:(bool,u32) = match board.grid.get(&new_square).unwrap() {
        Some(i) => if i.color==board.turn {
            (false,1)
        }
        else {
            (true,i.kind.points())
        },

        None => (false,0), 
    };
    return Move {
        capture: capture,
        square: new_square,
        kind: kind,
    };
     
    
}
