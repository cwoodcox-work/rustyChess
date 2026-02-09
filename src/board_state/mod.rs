use std::collections::HashMap;
use crate::pieces::Piece;
use crate::pieces::Kind;
use crate::pieces::Color;
use crate::handle_input::move_handler;

#[derive(Eq, Hash, PartialEq,Debug,Clone)]
pub struct Square {
    pub x: String,
    pub y: String,
}
pub struct Board {
    pub grid: HashMap<Square,Option<Piece>>,
    pub turn: Color,
    pub score: HashMap<Color,u32>,
}

impl Board {
    pub fn update_board(&mut self, player_move: String) {
        move_handler(self,player_move);
    }
    
    pub fn clear_board(&mut self) {
        let grid: &mut HashMap<Square, Option<Piece>> = &mut self.grid;
        let mut x = 1;
        let mut y = 1;
        for _i in 0..64 {
            let square = Square {
                x:x.to_string(),
                y:y.to_string(),
            };
            grid.insert(square, None);
            x = x + 1;
            if x > 8 {
                x = 1;
                y = y + 1;
            }
        }
    }

    pub fn initialize_board() -> Self {
        let grid: HashMap<Square,Option<Piece>> = HashMap::new();
        let turn: Color = Color::White;
        let score: HashMap<Color,u32> = HashMap::from([
            (Color::White,39),
            (Color::Black,39),
        ]);
        let mut board = Self {
            grid: grid,
            turn: turn,
            score: score,
        };
        board.clear_board();
        create_initial_pieces(&mut board.grid);
        return board 
               
    }
}



fn create_initial_pieces (grid: &mut HashMap<Square,Option<Piece>>) {
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
            let y: String = item.1.to_string();
            let x: String = item.0.to_string();
            if item.1 == 8 || item.1 == 7 {
                color = Color::Black;
            }

            let square: Square = Square {
                x:x.clone(),
                y:y.clone(),
            };
            let piece: Piece = Piece {
                kind: key.clone(),
                color: color,
                square: square,
            };
            grid.insert(Square { x:x, y:y },Some(piece));
        }
    }
}

  
