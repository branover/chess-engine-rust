use crate::board::{
    Board,
    Move,
    Coord,
    Result,
};
use crate::pieces::{
    PieceColor,
    PieceKind,
};

fn piece_score(piece: PieceKind) -> i32 {
    match piece {
        PieceKind::Pawn => 100,
        PieceKind::Knight => 300,
        PieceKind::Bishop => 300,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 10000,
    }
}

fn calculate_material(board: &Board) -> i32 {
    let mut score = 0;
    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board.piece_at(Coord{x, y}) {
                if piece.color == PieceColor::White {
                    score += piece_score(piece.kind);
                } else {
                    score -= piece_score(piece.kind);
                }
            }
        }
    }
    score
}

pub fn make_best_move(depth: u8, board: &Board) -> Option<Move> {
    // List all possible moves and simulate them recursively, then take the one that is the best for the current color
    let possible_moves = board.list_all_valid_moves();
    if possible_moves.len() == 0 {
        return None
    }
    let move_evaluations = possible_moves
        .iter()
        .map(|m|  (m, simulate_move(board, m, depth)))
        .filter(|(_, score)| score.is_ok())
        .map(|(m, score)| (m, score.unwrap()))
        .collect::<Vec<_>>();
    if move_evaluations.len() == 0 {
        return None
    }
    match board.turn {
        PieceColor::White => Some(move_evaluations.iter().max_by_key(|(_, score)| score).unwrap().0.clone()),
        PieceColor::Black => Some(move_evaluations.iter().min_by_key(|(_, score)| score).unwrap().0.clone()),
    }
}

fn simulate_move(board: &Board, m: &Move, depth: u8) -> Result<i32> {
    println!("HERE");
    let mut board = board.clone();
    match board.do_move_from_coord(m.from, m.to) {
        // Ok(_) => Ok(eval_position(&board)),
        Ok(_) => {},
        Err(e) => {
            // board.pretty_print_board();
            // println!("Engine tried to make invalid move: ({},{}): {:?}", m.from.to_notation(), m.to.to_notation(), e);
            return Err(e);
        }
    };

    if depth == 0 {
        return Ok(eval_position(&board))
    } else {
        match make_best_move(depth - 1, &board) {
            Some(m) => {
                // simulate_move(&board, &m, depth - 1),
                board.do_move_from_coord(m.from, m.to).unwrap();
                Ok(eval_position(&board))
            }
            None => Ok(eval_position(&board))
        }
    }
}

pub fn eval_position(board: &Board) -> i32 {
    match board.turn {
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
                return i32::MIN
            }
            if board.in_stalemate.0 {
                return 0
            }
        }
    };
    calculate_material(board) + eval_side(board, PieceColor::White) - eval_side(board, PieceColor::Black)
}


fn eval_side(board: &Board, color: PieceColor) -> i32 {
    let mut score = 0;
    let attacked_squares = board.list_all_attacked_squares_color(color);
    attacked_squares.iter().for_each(|coord| {
        if let Some(piece) = board.piece_at(*coord) {
            // Piece is defended
            if piece.color == color {
                score += piece_score(piece.kind) / 10;
            } 
            // Piece is attacked
            else {
                score += piece_score(piece.kind) / 5;
            }
        } else {
            // Empty square is attacked
            score += 10;
        }
    });
    
    score
}