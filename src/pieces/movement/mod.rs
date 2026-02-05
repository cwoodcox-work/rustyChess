use crate::board_state::Square;
use crate::pieces::Piece;
use crate::pieces::Kind;

pub fn find_potential_moves(piece: &Piece) -> Vec<Square> {
    let potential_moves = match piece.kind {
        Kind::Rook => horizontal(false,&piece.square, false),
        Kind::Bishop => diagonal(false,&piece.square, false),
        Kind::King => combine_movement(true,&piece.square,false),
        Kind::Knight => lshape(&piece.square),
        Kind::Pawn => combine_movement(true,&piece.square,true),
        Kind::Queen => combine_movement(false,&piece.square,false),
    };
    return potential_moves;

}

fn horizontal(limit: bool,location: &Square,pawn: bool) -> Vec<Square> {
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
    let mut up = upy <=8;
    let mut all_moves = right || left || down || up;
    let mut pawn_count =1;
    while all_moves {
        if right && !pawn {
            moves.push(Square {x:upx.to_string(),y:y.to_string()});
            upx += 1;
            right = upx <=8;
        }
        if left && !pawn {
            moves.push(Square { x: downx.to_string(), y: y.to_string() });
            downx -= 1;
            left = downx >=1;
        }
        if up {
            moves.push(Square { x: x.to_string(), y: upy.to_string() });
            upy += 1;
            up = upy <=8;
        }
        if down && !pawn {
            moves.push(Square { x: x.to_string(), y: downy.to_string()});
            downy -= 1;
            down = downy >= 1;
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

fn diagonal(limit: bool, location: &Square, pawn: bool) -> Vec<Square> {
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
    let mut all_moves = right || left || down || up;
    while all_moves {
        if up {
            if right {
                moves.push(Square {x: upx.to_string(), y: upy.to_string()});
            }
            if left {
                moves.push(Square {x: downx.to_string(), y: upy.to_string()});
            }
        }
        if down && !pawn {
            if right {
                moves.push(Square {x: upx.to_string(), y: downy.to_string()});
            }
            if left {
                moves.push(Square {x: downx.to_string(), y: downy.to_string()});
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

fn combine_movement(limit: bool, location: &Square, pawn: bool) -> Vec<Square> {
    let mut diagonal = diagonal(limit,&location,pawn);
    let mut horizontal = horizontal(limit,&location,pawn);
    diagonal.append(&mut horizontal);
    return diagonal;
}

fn lshape(location: &Square) -> Vec<Square> {
    let mut moves: Vec<Square> = Vec::new();
    let x: u32 = location.x.clone().parse().expect("Failed to parse integer");
    let y: u32 = location.y.clone().parse().expect("Failed to parse integer");
    let coordinates: [(u32,u32);8] = [(x-1,y+2),(x+1,y+2),(x+2,y+1),(x+2,y-1),(x+1,y-2),(x-1,y-2),(x-2,y+1),(x-2,y-1)];
    for point in coordinates.iter() {
        let in_bounds = point.1 >= 1 && point.1 <= 8 && point.0 >= 1 && point.0 <= 8 ;
        if in_bounds {
            moves.push(Square {x:point.0.to_string(),y:point.1.to_string()});
        }
    }
    return moves;
}