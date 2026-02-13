use std::collections::HashMap;
use std::collections::HashSet;
use crate::handle_input::MoveError;
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
    pub move_count: u32,
    pub piece_registry: HashMap<(Kind,Color),HashSet<Square>>,
}

impl Board {
    pub fn update_board(&mut self, player_move: String) -> Result<(), MoveError> {
        match move_handler(self,player_move) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        };
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
        let registry: HashMap<(Kind,Color),HashSet<Square>> = HashMap::new();
        let turn: Color = Color::White;
        let score: HashMap<Color,u32> = HashMap::from([
            (Color::White,39),
            (Color::Black,39),
        ]);
        let count: u32 = 0;
        let mut board = Self {
            grid: grid,
            turn: turn,
            score: score,
            move_count: count,
            piece_registry: registry,
        };
        board.clear_board();
        create_initial_pieces(&mut board.grid, &mut board.piece_registry);
        return board 
               
    }

    pub fn print_board(&self) {
        for y in (1..=8).rev() {
            let mut row = String::new();
            for x in 1..=8 {
                let square = Square {
                    x:x.to_string(),
                    y:y.to_string(),
                };
                let mut piece_string: &str = " ";
                if let Some(i) = self.grid.get(&square).unwrap() {
                    let piece = i;
                    piece_string = match piece.kind {
                    Kind::Rook => "R",
                    Kind::Knight => "N",
                    Kind::Bishop => "B",
                    Kind::Queen => "Q",
                    Kind::King => "K",
                    Kind::Pawn => "P",
                    };

                }
                row.push_str(piece_string);
            }
            println!("{row}");
        }
    }

    pub fn show_score(&self) {
        let white = self.score.get(&Color::White).unwrap();
        let black = self.score.get(&Color::Black).unwrap();
        println!("The score is: ");
        println!("White: {white} Black: {black} ");
    }
}



fn create_initial_pieces (grid: &mut HashMap<Square,Option<Piece>>,registry: &mut HashMap<(Kind,Color),HashSet<Square>>) {
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
                color: color.clone(),
                square: Some(square.clone()),
            };
            let kind = key.clone();
            let reg_key = (kind,color.clone());
            if let Some(i) = registry.get_mut(&reg_key) {
                i.insert(square);
            }
            else {
                registry.entry(reg_key).or_insert(HashSet::from([square]));
            }
            grid.insert(Square { x:x, y:y },Some(piece));
        }
    }
}

  
