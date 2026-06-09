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
    pub prev_move: Option<(Piece,Square,bool)>,
    pub piece_registry: HashMap<(Kind,Color),HashSet<Square>>,
    pub check:Option<Color>,
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
        let prev_move = None;

        let count: u32 = 0;
        let check = None;
        let mut board = Self {
            grid,
            turn,
            score,
            move_count: count,
            prev_move,
            piece_registry: registry,
            check,
        };
        board.clear_board();
        create_initial_pieces(&mut board.grid, &mut board.piece_registry);
        board 
               
    }

    pub fn print_board(&self) {
        for y in (0..=8).rev() {
            let mut row = String::new();
            row.push_str(&y.to_string()[..]);
            for x in 1..=8 {
                if y != 0i32 {
                    let square = Square {
                        x:x.to_string(),
                        y:y.to_string(),
                    };
                    let mut piece_string: &str = " "; 
                    if let Some(i) = self.grid.get(&square).unwrap() {
                        piece_string = i.symbol();
                    };
                    row.push_str(piece_string);
                }
                else {
                    row.push_str(&x.to_string()[..])
                }
            }
            println!("{row}");
        }
    }
    pub fn generate_fen(&self) -> String {
        let mut result = String::new();
        for y in (1..=8).rev() {
            let mut empty_squares: u32 = 0;
            for x in 1..=8 {
                let square = Square {
                    x:x.to_string(),
                    y:y.to_string(),
                };
                if let Some(i) = self.grid.get(&square).unwrap() {
                    if empty_squares > 0u32 {
                        result.push_str(&empty_squares.to_string()[..]);
                        empty_squares = 0;
                    }
                    result.push_str(
                    if i.color == Color::White { 
                        match i.kind {
                        Kind::Rook => "R",
                        Kind::Bishop => "B",
                        Kind::King => "K",
                        Kind::Queen => "Q",
                        Kind::Knight => "N",
                        Kind::Pawn => "P",
                        _ => "0",
                        }
                    }else {
                        match i.kind {
                        Kind::Rook => "r",
                        Kind::Bishop => "b",
                        Kind::King => "k",
                        Kind::Queen => "q",
                        Kind::Knight => "n",
                        Kind::Pawn => "p",
                        _ => "0",
                        }
                    });
                } else {
                    empty_squares += 1;
                    if x == 8i32 {
                        result.push_str(&empty_squares.to_string()[..]);
                    }
                }
            }
            if y != 1i32 {
                result.push('/');
            }
        }
        result
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
                kind: *key,
                color,
                square: square.clone(),
                moved: false,
            };
            let kind = key;
            let reg_key = (*kind,color);
            if let Some(i) = registry.get_mut(&reg_key) {
                i.insert(square);
            }
            else {
                registry.entry(reg_key).or_insert(HashSet::from([square]));
            }
            grid.insert(Square { x, y },Some(piece));
        }
    }
}

  
