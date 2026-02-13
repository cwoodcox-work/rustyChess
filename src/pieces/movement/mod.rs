use crate::board_state::Board;
use crate::board_state::Square;
use crate::handle_input::Move;
use crate::handle_input::MoveError;
use crate::pieces::Kind;

pub fn find_potential_moves(new_move: &Move,board: &Board) -> Result<Vec<Square>,MoveError> {
    let potential_moves = match new_move.kind {
        Kind::Rook => horizontal(false,&new_move, false,board),
        Kind::Bishop => diagonal(false,&new_move, false,board),
        Kind::King => combine_movement(true,&new_move,false,board),
        Kind::Knight => lshape(&new_move,board),
        Kind::Pawn => combine_movement(true,&new_move,true,board),
        Kind::Queen => combine_movement(false,&new_move,false,board),
    };
    match potential_moves {
        Ok(i) => return Ok(i),
        Err(m) => return Err(m),
    };
}

fn horizontal(limit: bool,new_move: &Move,pawn: bool,board: &Board) -> Result<Vec<Square>,MoveError> {
    let x: u32 = new_move.square.x.clone().parse().expect("Failed to parse integer");
    let y: u32 = new_move.square.y.clone().parse().expect("Failed to parse integer");
    let piece_list = match board.piece_registry.get(&(new_move.kind.clone(),board.turn.clone())) {
        Some(i) => i,
        None => return Err(MoveError::NoPieceToMove),
    };
    let mut pawn_count = 1;
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
    while all_moves {
        if right && !pawn {
            let mut blocked = false;
            let potential_square = Square {x:upx.to_string(),y:y.to_string()};
            let occupied = match board.grid.get(&potential_square).unwrap() {
                Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    for sq in piece_list {
                        if *sq == potential_square {
                            moves.push(potential_square);
                            break;
                        }
                        else {
                            continue;
                        }
                    }
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
                Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    for sq in piece_list {
                        if *sq == potential_square {
                            moves.push(potential_square);
                            break;
                        }
                    }
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
        if up  {
            let mut blocked = false;
            let potential_square = Square {x:x.to_string(),y:upy.to_string()};
            let occupied = match board.grid.get(&potential_square).unwrap() {
                Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    for sq in piece_list {
                        if *sq == potential_square {
                            moves.push(potential_square);
                            break;
                        }
                        else {
                            continue;
                        }
                    }
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
                Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                None => None,
            };
            match occupied {
                Some(i) => if i {
                    blocked = true;
                    for sq in piece_list {
                        if *sq == potential_square {
                            moves.push(potential_square);
                            break;
                        }
                        else {
                            continue;
                        }
                    }
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
                return Ok(moves);
            }
            else {
                if pawn_count < 2 && moves.is_empty() {
                    pawn_count += 1;
                    continue;
                }
                else {
                    return Ok(moves);
                }
            }
        }
    }  
    return Ok(moves);
}

fn diagonal(limit: bool, new_move: &Move, pawn: bool,board: &Board) -> Result<Vec<Square>,MoveError> {
    let mut moves: Vec<Square> = Vec::new();
    let x: i32 = new_move.square.x.clone().parse().expect("Failed to parse integer");
    let y: i32 = new_move.square.y.clone().parse().expect("Failed to parse integer");
    let piece_list = match board.piece_registry.get(&(new_move.kind.clone(),board.turn.clone())) {
        Some(i) => i,
        None => return Err(MoveError::NoPieceToMove),
    };
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
        if up {
            if right && !blockedur {
                let potential_square = Square {x:upx.to_string(),y:upy.to_string()};
                let occupied = match board.grid.get(&potential_square).unwrap() {
                    Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockedur = true;
                        for sq in piece_list {
                            if *sq == potential_square {
                                moves.push(potential_square);
                                break;
                            }
                            else {
                                continue;
                            }
                        }
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
                    Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockedul = true;
                        for sq in piece_list {
                            if *sq == potential_square {
                                moves.push(potential_square);
                                break;
                            }
                            else {
                                continue;
                            }
                        }
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
                    Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockeddr = true;
                        for sq in piece_list {
                            if *sq == potential_square {
                                moves.push(potential_square);
                                break;
                            }
                            else {
                                continue;
                            }
                        }
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
                    Some(i) => Some(i.color == board.turn && i.kind == new_move.kind),
                    None => None,
                };
                match occupied {
                    Some(i) => if i {
                        blockeddl = true;
                        for sq in piece_list {
                            if *sq == potential_square {
                                moves.push(potential_square);
                                break;
                            }
                            else {
                                continue;
                            }
                        }
                    }
                    else {
                        blockeddl = true;
                    },
                    None => blockeddl = false,
                };   
            }
        }
        if limit || pawn {
            return Ok(moves);
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
    return Ok(moves);
}

fn combine_movement(limit: bool, new_move: &Move, pawn: bool,board: &Board) -> Result<Vec<Square>,MoveError> {
    let mut horizontal = match horizontal(limit,&new_move,pawn,board) {
        Ok(list) => list,
        Err(m) => return Err(m),
    };
    let mut diagonal = match diagonal(limit,&new_move,pawn,board) {
        Ok(list) => list,
        Err(m) => return Err(m),
    };
    if !pawn {
        diagonal.append(&mut horizontal);
        return Ok(diagonal);
    }
    else if new_move.capture.0 {
        return Ok(diagonal);
    }
    else {
        return Ok(horizontal);
    }
}

fn lshape(new_move: &Move,board: &Board) -> Result<Vec<Square>,MoveError> {
    let mut moves: Vec<Square> = Vec::new();
    let piece_list = match board.piece_registry.get(&(new_move.kind.clone(),board.turn.clone())) {
        Some(i) => i,
        None => return Err(MoveError::NoPieceToMove),
    };
    let x: i32 = new_move.square.x.clone().parse().expect("Failed to parse integer");
    let y: i32 = new_move.square.y.clone().parse().expect("Failed to parse integer");
    let coordinates: [(i32,i32);8] = [(x-1,y+2),(x+1,y+2),(x+2,y+1),(x+2,y-1),(x+1,y-2),(x-1,y-2),(x-2,y+1),(x-2,y-1)];
    for point in coordinates.iter() {
        if point.0 >= 1 && point.1 >= 1 && point.0 <= 8 && point.1 <= 8 {
            let potential_square = Square {x:point.0.to_string(),y:point.1.to_string()};
            let mut blocked = false;
            let mut dif_color = false;
            match board.grid.get(&potential_square).unwrap() {
                Some(i) => if i.color == board.turn && i.kind == new_move.kind {
                    blocked = true;
                    }
                else {
                    blocked = true;
                    dif_color = true;
                }
                None => blocked = false,
            };
            if blocked && !dif_color {
                for sq in piece_list {
                    if *sq == potential_square {
                        moves.push(potential_square);
                        break;
                    }
                    else {
                        continue;
                    }
                }
            }
        
            }
            else {
                continue;
            }
    }
    return Ok(moves);
}