use std::collections::HashMap;
use crate::pieces::Piece;
use crate::pieces::Kind;
use crate::pieces::Color;
use crate::handle_input::translate_input;


pub struct Board {
    pub grid: HashMap<String,Option<Piece>>,
    pub turn: Color
}

pub fn initialize_board() -> Board {
    let mut grid: HashMap<String,Option<Piece>> = HashMap::new();
    let turn = Color::White;
    let mut row = 3;
    let mut column = 1;
    for i in 0..64 {
        if i <32 {
            
            create_starting_pieces(&mut grid);        
        }
        else {
            let square = row.to_string() + &column.to_string()[..];
            grid.insert(square,None);
            column += 1;
            if column > 8 {
                column = 1;
                row +=1;
            }
        }
    } 
   
              
    Board {
        grid: grid,
        turn: turn,
    }
}

fn create_starting_pieces (grid: &mut HashMap<String,Option<Piece>>) {
    let mut coordinates: HashMap<Kind, Vec<(i32,i32)>> = HashMap::new();
    coordinates.insert(Kind::Rook, vec![(1,1),(1,8),(8,1),(8,8)]);
    coordinates.insert(Kind::Knight, vec![(2,1),(2,8),(7,1),(7,8)]);
    coordinates.insert(Kind::Bishop, vec![(3,1),(3,8),(6,1),(6,8)]);
    coordinates.insert(Kind::Queen, vec![(4,1),(4,8)]);
    coordinates.insert(Kind::King, vec![(5,1),(5,8)]);
    coordinates.insert(Kind::Pawn, vec![(1,2),(2,2),(3,2),(4,2),(5,2),(6,2),(7,2),(8,2),(1,7),(2,7),(3,7),(4,7),(5,7),(6,7),(7,7),(8,7)]);
    for (key,val) in coordinates.iter() {
        for item in val {
            let mut color = Color::White;
            let row = item.1.to_string();
            let column = item.0.to_string();
            if item.1 == 8 || item.1 == 7 {
                color = Color::Black;
            }
            let piece = Piece {
                kind: key.clone(),
                color: color,
            };

            let square: String = column + &row[..];
            grid.insert(square.clone(),Some(piece));
        }
    }
}

pub fn update_board(player_move: String,previous_board: Board) -> Board {
    let player_move = translate_input(player_move);
    return previous_board;
}
  
