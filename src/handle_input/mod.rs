use std::collections::HashMap;
use std::io;
use crate::board_state::Square;
use crate::pieces::Piece;
use crate::pieces::Kind;
use crate::pieces::Color;
use crate::pieces::movement::find_potential_moves;
use crate::board_state::Board;

pub struct Move {
    pub piece: Piece,
    pub square: Square,
    pub capture: bool,
    pub castle: bool,
    pub check: bool,
}

pub fn take_input() -> String {
    println!("Enter your move in chess notation. Example: Nf3");
    let mut user_move = String::new();
    io::stdin()
        .read_line(&mut user_move)
        .expect("Failed to read line");

    return user_move.trim().to_string();

}

pub fn move_handler(board: &mut Board, input: String)  {
    let new_move = translate_input(input,&board.turn);
    let potential_moves = find_potential_moves(&new_move.piece);
    let mut from_piece = Vec::new();
    for square in potential_moves.iter() {
        let pos_piece = match board.grid.get(square).unwrap() {
            Some(i) =>(i.square.clone(),i.kind.clone(),i.color.clone()),
            None => continue,
        };
        if pos_piece.1 == new_move.piece.kind && pos_piece.2 == new_move.piece.color {
            from_piece.push(pos_piece)
        }
    }
    
    if from_piece.len() == 1 {
        let new_location = Square {
        x:from_piece[0].0.x.clone(),
        y:from_piece[0].0.y.clone(),
        };
        board.grid.insert(new_location,None);
        board.grid.insert(new_move.square,Some(new_move.piece));
    }   

}

fn translate_input(input: String,turn: &Color)  -> Move{
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
    let kind = match the_move[0] {
        'N' => Kind::Knight,
        'K' => Kind::King,
        'Q' => Kind::Queen,
        'R' => Kind::Rook,
        'B' => Kind::Bishop,
        _ => panic!(),
    };
    let color = match turn {
        Color::White => Color::White,
        Color::Black => Color::Black,
    };
    let x = match coordinate_x_translate_table.get(&the_move[1].to_lowercase().to_string().chars().next().unwrap()) {
        Some(i) => i,
        None => panic!(),
    };
        
    let new_square = Square {
        x:x.to_string(),
        y:the_move[2].to_string(),
    };  
    return Move {
        piece: Piece {kind: kind, color: color, square: new_square},
        square:Square {x:x.to_string(),y:the_move[2].to_string()},
        capture: false,
        castle: false,
        check: false,
    };
     
    
}
