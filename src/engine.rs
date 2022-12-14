use crate::board::{
    Board,
    Move,
    Coord,
    // Result,
};
use crate::pieces::{
    PieceColor,
    PieceKind,
};
use std::thread;

fn piece_score(piece: PieceKind) -> i32 {
    match piece {
        PieceKind::Pawn => 100,
        PieceKind::Knight => 300,
        PieceKind::Bishop => 300,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 2000,
    }
}

fn calculate_material(board: &Board, color: PieceColor) -> i32 {
    let mut score = 0;
    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board.piece_at(Coord{x, y}) {
                if piece.color == color {
                    score += piece_score(piece.kind);
                } else {
                    score -= piece_score(piece.kind);
                }
            }
        }
    }
    score
}

pub fn make_best_move(depth: u8, board: Board) -> Option<Move> {
    // Call minimax algorithm
    let possible_moves = board.list_all_valid_moves();
    if possible_moves.len() == 0 {
        return None
    }

    let mut best_move = Move{from: Coord{x: 0, y: 0}, to: Coord{x: 0, y: 0}, promote: None};
    let mut best_score = i32::MIN;
 
    let handles = possible_moves
        .into_iter()
        .map(|m|  {
            let getting_move_for = board.turn;
            let mut board = board;
            match board.do_move_from_coord(m.from, m.to) {
                Ok(_) => {
                    let handle = thread::spawn(move || {
                        let score = minimax(board, depth, i32::MIN, i32::MAX, false, getting_move_for);
                        // println!("Move: {:?}, Score: {}", m, score);
                        (m, score)
                    });
                    Ok(handle)
                },
                Err(e) => {
                    Err(e)
                }
            }            
        }).filter_map(|result| result.ok())
        .collect::<Vec<_>>();

    handles.into_iter().for_each(|handle| {
        let (m, score) = handle.join().unwrap();
        if score > best_score {
            best_score = score;
            best_move = m;
        }
    });

    // println!("Max score: {}, Move: {:?}", best_score, best_move);
    Some(best_move)        
}


fn minimax(
    board: Board,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    is_maximizing: bool,
    getting_move_for: PieceColor,
) -> i32 {

    // println!("Depth: {}, Alpha: {}, Beta: {}, Is maximizing: {}", depth, alpha, beta, is_maximizing);
    if depth == 0 {
        return eval_position(&board, getting_move_for);
    }

    let legal_moves = board.list_all_valid_moves();
    let mut best_move_value;

    if is_maximizing {
        best_move_value = i32::MIN;

        for m in &legal_moves {
            let mut board = board;
            let child_board_value = match board.do_move_from_coord(m.from, m.to) {
                Ok(_) => minimax(board, depth - 1, alpha, beta, !is_maximizing, getting_move_for),
                Err(_) => {
                    continue;
                }
            };

            if child_board_value > best_move_value {
                best_move_value = child_board_value;
            }

            if best_move_value > alpha {
                alpha = best_move_value
            }

            if beta <= alpha {
                return best_move_value;
            }
        }
    } else {
        best_move_value = i32::MAX;

        for m in &legal_moves {
            let mut board = board;
            let child_board_value = match board.do_move_from_coord(m.from, m.to) {
                Ok(_) => minimax(board, depth - 1, alpha, beta, !is_maximizing, getting_move_for),
                Err(_) => {
                    continue;
                }
            };
            if child_board_value < best_move_value {
                best_move_value = child_board_value;
            }

            if best_move_value < beta {
                beta = best_move_value
            }

            if beta <= alpha {
                return best_move_value;
            }
        }
    }

    best_move_value
}

#[inline]
fn eval_position(board: &Board, color: PieceColor) -> i32 {
    match color {
        PieceColor::White => {
            if board.in_checkmate.1 {
                return i32::MAX
            }
            if board.in_stalemate.1 {
                return 0
            }
        },
        PieceColor::Black => {
            if board.in_checkmate.0 {
                return i32::MAX
            }
            if board.in_stalemate.0 {
                return 0
            }
        }
    };
    calculate_material(board, color) + calc_attack_defend_score(board, color)
}

pub fn calc_attack_defend_score(board: &Board, color: PieceColor) -> i32 {
    let mut score = 0;

    for y in 0..8 {
        for x in 0..8 {
            let coord = Coord { x, y };
            if let Some(piece) = board.piece_at(coord) {
                let this_piece_moves = piece.list_possible_moves(coord);
                for m in this_piece_moves {
                    if board.can_attack_square(coord, m) {
                        if let Some(piece) = board.piece_at(m) {
                            if piece.color == color {
                                // Piece is defended
                                score += piece_score(piece.kind) / 10;
                            } else {
                                // Piece is attacked
                                score += piece_score(piece.kind) / 5;
                            }
                        } else {
                            // Empty square is attacked
                            score += 10;
                        }
                    } 
                }
                
            }
        }
    }
    score
}