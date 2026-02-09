use crate::board_state::Board;
use crate::board_state::Square;
use crate::handle_input::Move;
use crate::pieces::Kind;

pub fn find_potential_moves(new_move: &Move,board: &Board) -> Vec<Square> {
    let potential_moves = match new_move.kind {
        Kind::Rook => horizontal(false,&new_move.square, false,board),
        Kind::Bishop => diagonal(false,&new_move.square, false,board),
        Kind::King => combine_movement(true,&new_move.square,false,board),
        Kind::Knight => lshape(&new_move.square,board),
        Kind::Pawn => combine_movement(true,&new_move.square,true,board),
        Kind::Queen => combine_movement(false,&new_move.square,false,board),
    };
    return potential_moves;

}

fn horizontal(limit: bool,location: &Square,pawn: bool,board: &Board) -> Vec<Square> {
    let x: u32 = location.x.clone().parse().expect("Failed to parse integer");
    let y: u32 = location.y.clone().parse().expect("Failed to parse integer");
    let mut upx: u32 = x+1;
    let mut upy: u32 = y+1;
    let mut downx = x-1;
    let mut downy = y-1;
    let mut moves: Vec<Square> = Vec::new();
    let mut right = upx <= 8;
    let mut left = downx >=1;
    let mut down = downy >= 1;
    let mut up = upy <= 8;
    let mut all_moves = right || left || down || up;
    let mut pawn_count = 1;
    while all_moves {
        if right && !pawn {
            let mut blocked = false;
            let potential_square = Square {x:upx.to_string(),y:y.to_string()};
            let occupied = match board.grid.get(&potential_square).unwrap() {
                Some(i) => Some(i.color == board.turn),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    moves.push(potential_square);
                }
                else {
                    blocked = true;
                },
                None => upx += 1,
            };          
            right = upx <=8;
            if blocked {
                right = false;
            }
        }
        if left && !pawn {
            let mut blocked = false;
            let potential_square = Square {x:downx.to_string(),y:y.to_string()};
            let occupied = match board.grid.get(&potential_square).unwrap() {
                Some(i) => Some(i.color == board.turn),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    moves.push(potential_square);
                }
                else {
                    blocked = true;
                },
                None => downx -= 1,
            };            
            left = downx <=8;
            if blocked {
                left = false;
            } 
        }
        if up && !pawn {
            let mut blocked = false;
            let potential_square = Square {x:x.to_string(),y:upy.to_string()};
            let occupied = match board.grid.get(&potential_square).unwrap() {
                Some(i) => Some(i.color == board.turn),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    moves.push(potential_square);
                }
                else {
                    blocked = true;
                },
                None => upy += 1,
            };          
            up = upy <=8;
            if blocked {
                up = false;
            }
        }
        if down {
            let mut blocked = false;
            let potential_square = Square {x:x.to_string(),y:downy.to_string()};
            let occupied = match board.grid.get(&potential_square).unwrap() {
                Some(i) => Some(i.color == board.turn),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    moves.push(potential_square);
                }
                else {
                    blocked = true;
                },
                None => downy -= 1,
            };            
            down = downy <=8;
            if blocked {
                down = false;
            }
        }
        all_moves = right || left || down || up;
        if limit == true {
            if !pawn {
                return moves;
            }
            else {
                if pawn_count < 2 {
                    pawn_count += 1;
                    continue;
                }
                else {
                    return moves;
                }
            }
        }
    }    
    return moves;
}

fn diagonal(limit: bool, location: &Square, pawn: bool,board: &Board) -> Vec<Square> {
    let mut moves: Vec<Square> = Vec::new();
    let x: u32 = location.x.clone().parse().expect("Failed to parse integer");
    let y: u32 = location.y.clone().parse().expect("Failed to parse integer");
    let mut upx = x+1;
    let mut upy = y+1;
    let mut downx = x-1;
    let mut downy = y-1;
    let mut right = upx <= 8;
    let mut left = downx >=1;
    let mut down = downy >= 1;
    let mut up = upy <=8; 
    //need blocked variables for each diagonal path since I cant just use right,left, etc.. since that could block other potential paths as those variables are reused. 
    let mut blockedur = false;
    let mut blockedul = false;
    let mut blockeddr = false;
    let mut blockeddl = false;
    let mut all_moves = right || left || down || up;
    while all_moves {
        if up && !pawn {
            if right && !blockedur {
                let potential_square = Square {x:upx.to_string(),y:upy.to_string()};
                let occupied = match board.grid.get(&potential_square).unwrap() {
                    Some(i) => Some(i.color == board.turn),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockedur = true;
                        moves.push(potential_square);
                    }
                    else {
                        blockedur = true;
                    },
                    None => blockedur = false,
                };           
            }
            if left && !blockedul {
                let potential_square = Square {x:downx.to_string(),y:upy.to_string()};
                let occupied = match board.grid.get(&potential_square).unwrap() {
                    Some(i) => Some(i.color == board.turn),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockedul = true;
                        moves.push(potential_square);
                    }
                    else {
                        blockedul = true;
                    },
                    None => blockedul = false,
                };           
            }
        }
        if down {
            if right && !blockeddr {
                let potential_square = Square {x:upx.to_string(),y:downy.to_string()};
                let occupied = match board.grid.get(&potential_square).unwrap() {
                    Some(i) => Some(i.color == board.turn),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockeddr = true;
                        moves.push(potential_square);
                    }
                    else {
                        blockeddr = true;
                    },
                    None => blockeddr = false,
                };   
            }
            if left && !blockeddl {
                let potential_square = Square {x:downx.to_string(),y:downy.to_string()};
                let occupied = match board.grid.get(&potential_square).unwrap() {
                    Some(i) => Some(i.color == board.turn),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockeddl = true;
                        moves.push(potential_square);
                    }
                    else {
                        blockeddl = true;
                    },
                    None => blockeddl = false,
                };   
            }
        }
        if limit || pawn {
            return moves;
        }
        upy += 1;
        downy-=1;
        upx+=1;
        downx-=1;
        left = downx >=1;
        right = upx <= 8;
        down = downy >= 1;
        up = upy <=8;
        all_moves = right || left || down || up;

    }
    return moves;
}

fn combine_movement(limit: bool, location: &Square, pawn: bool,board: &Board) -> Vec<Square> {
    let mut diagonal = diagonal(limit,&location,pawn,board);
    let mut horizontal = horizontal(limit,&location,pawn,board);
    diagonal.append(&mut horizontal);
    return diagonal;
}

fn lshape(location: &Square,board: &Board) -> Vec<Square> {
    let mut moves: Vec<Square> = Vec::new();
    let x: u32 = location.x.clone().parse().expect("Failed to parse integer");
    let y: u32 = location.y.clone().parse().expect("Failed to parse integer");
    let coordinates: [(u32,u32);8] = [(x-1,y+2),(x+1,y+2),(x+2,y+1),(x+2,y-1),(x+1,y-2),(x-1,y-2),(x-2,y+1),(x-2,y-1)];
    for point in coordinates.iter() {
        let potential_square = Square {x:point.0.to_string(),y:point.1.to_string()};
        let mut blocked = false;
        let mut dif_color = false;
        match board.grid.get(&potential_square).unwrap() {
            Some(i) => if i.color == board.turn {
                blocked = true;
                }
            else {
                blocked = true;
                dif_color = true;
            }
            None => blocked = false,
        };
        if blocked && !dif_color {
            moves.push(potential_square);
        }
    }
    return moves;
}