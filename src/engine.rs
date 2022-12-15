use crate::board::{
    Board,
    Move,
    Coord,
    // Result,
};
use crate::pieces::{
    PieceColor,
    PieceKind, Piece,
};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

const WHITE_KING_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, 00, 00, 00, 00, 20, 20],
    [20, 30, 10, 00, 00, 10, 30, 20],
];

const BLACK_KING_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [20, 30, 10, 00, 00, 10, 30, 20],
    [20, 20, 00, 00, 00, 00, 20, 20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
];

const WHITE_QUEEN_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-20, -10, -10, -05, -05, -10, -10, -20],
    [-10, 00, 00, 00, 00, 00, 00, -10],
    [-10, 00, 05, 05, 05, 05, 00, -10],
    [-5, 00, 05, 05, 05, 05, 00, -5],
    [00, 00, 05, 05, 05, 05, 00, -5],
    [-10, 05, 05, 05, 05, 05, 00, -10],
    [-10, 00, 05, 00, 00, 00, 00, -10],
    [-10, -00, -10, -05, -05, -05, -10, -20],
];
const BLACK_QUEEN_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-10, -00, -10, -05, -05, -05, -10, -20],
    [-10, 00, 05, 00, 00, 00, 00, -10],
    [-10, 05, 05, 05, 05, 05, 00, -10],
    [00, 00, 05, 05, 05, 05, 00, -5],
    [-05, 00, 05, 05, 05, 05, 00, -5],
    [-10, 00, 05, 05, 05, 05, 00, -10],
    [-10, 00, 00, 00, 00, 00, 00, -10],
    [-20, -10, -10, -05, -05, -10, -10, -20],
];

const WHITE_ROOK_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [00, 00, 00, 00, 00, 00, 00, 00],
    [05, 10, 10, 10, 10, 10, 10, 5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [00, 00, 00, 05, 05, 00, 00, 00],
];

const BLACK_ROOK_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [00, 00, 00, 05, 05, 00, 00, 00],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [-05, 00, 00, 00, 00, 00, 00, -5],
    [05, 10, 10, 10, 10, 10, 10, 5],
    [00, 00, 00, 00, 00, 00, 00, 00],
];

const WHITE_BISHOP_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 00, 00, 00, 00, 00, 00, -10],
    [-10, 00, 05, 10, 10, 05, 00, -10],
    [-10, 05, 05, 10, 10, 05, 05, -10],
    [-10, 00, 10, 10, 10, 10, 00, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 05, 00, 00, 00, 00, 05, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];

const BLACK_BISHOP_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 05, 00, 00, 00, 00, 05, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 00, 10, 10, 10, 10, 00, -10],
    [-10, 05, 05, 10, 10, 05, 05, -10],
    [-10, 00, 05, 10, 10, 05, 00, -10],
    [-10, 00, 00, 00, 00, 00, 00, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];

const WHITE_KNIGHT_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 00, 00, 00, 00, -20, -40],
    [-30, 00, 10, 15, 15, 10, 00, -30],
    [-30, 05, 15, 20, 20, 15, 05, -30],
    [-30, 00, 15, 20, 20, 15, 00, -30],
    [-30, 05, 10, 15, 15, 10, 05, -30],
    [-40, -20, 00, 05, 05, 00, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

const BLACK_KNIGHT_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 00, 05, 05, 00, -20, -40],
    [-30, 05, 10, 15, 15, 10, 05, -30],
    [-30, 00, 15, 20, 20, 15, 00, -30],
    [-30, 05, 15, 20, 20, 15, 05, -30],
    [-30, 00, 10, 15, 15, 10, 00, -30],
    [-40, -20, 00, 00, 00, 00, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

const WHITE_PAWN_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [00, 00, 00, 00, 00, 00, 00, 00],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [05, 05, 10, 25, 25, 10, 05, 5],
    [00, 00, 00, 20, 20, 00, 00, 00],
    [05, -05, -10, 00, 00, -10, -05, 5],
    [05, 15, -10, -20, -20, 10, 15, 5],
    [00, 00, 00, 00, 00, 00, 00, 00],
];

const BLACK_PAWN_POSITION_WEIGHTS: [[i32; 8]; 8] = [
    [00, 00, 00, 00, 00, 00, 00, 00],
    [05, 15, -10, -20, -20, 10, 15, 5],
    [05, -05, -10, 00, 00, -10, -05, 5],
    [00, 00, 00, 20, 20, 00, 00, 00],
    [05, 05, 10, 25, 25, 10, 05, 5],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [00, 00, 00, 00, 00, 00, 00, 00],
];

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

pub fn get_weighted_value(piece: Piece, position: Coord) -> i32 {
    // println!("piece: {:?}, position: {:?}", piece, position);
    let weights = match piece.kind {
        PieceKind::King => match piece.color{
            PieceColor::White => WHITE_KING_POSITION_WEIGHTS,
            PieceColor::Black => BLACK_KING_POSITION_WEIGHTS,
        },
        PieceKind::Queen => match piece.color {
            PieceColor::White => WHITE_QUEEN_POSITION_WEIGHTS,
            PieceColor::Black => BLACK_QUEEN_POSITION_WEIGHTS,
        },
        PieceKind::Rook => match piece.color {
            PieceColor::White => WHITE_ROOK_POSITION_WEIGHTS,
            PieceColor::Black => BLACK_ROOK_POSITION_WEIGHTS,
        },
        PieceKind::Bishop => match piece.color {
            PieceColor::White => WHITE_BISHOP_POSITION_WEIGHTS,
            PieceColor::Black => BLACK_BISHOP_POSITION_WEIGHTS,
        },
        PieceKind::Knight => match piece.color {
            PieceColor::White => WHITE_KNIGHT_POSITION_WEIGHTS,
            PieceColor::Black => BLACK_KNIGHT_POSITION_WEIGHTS,
        },
        PieceKind::Pawn => match piece.color {
            PieceColor::White => WHITE_PAWN_POSITION_WEIGHTS,
            PieceColor::Black => BLACK_PAWN_POSITION_WEIGHTS,
        },
    };
    let value = weights[(position.y) as usize][position.x as usize]
        + (piece_score(piece.kind));
    // println!("value: {}", value);
    value
}

pub fn make_best_move(depth: u8, board: &Board) -> Option<Move> {
    // Call minimax algorithm
    let possible_moves = board.list_all_valid_moves().clone();
    if possible_moves.is_empty() {
        return None
    }

    let mut best_move = Move{from: Coord{x: 0, y: 0}, to: Coord{x: 0, y: 0}, promote: None};
    let mut best_score = i32::MIN;

    let n_jobs = possible_moves.len();
    let n_workers = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();

    for m in possible_moves {
        let getting_move_for = board.turn;
        let mut board = *board;
        board.do_move_from_coord(m).unwrap();
        let tx = tx.clone();
        pool.execute(move || {
            let score = minimax(&board, depth, i32::MIN, i32::MAX, false, getting_move_for);
            // println!("{:?}: {}", m, score);
            tx.send((m, score)).unwrap();
        });
    }

    for _ in 0..n_jobs {
        let (m, score) = rx.recv().unwrap();
        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }
    // println!("Best move: {:?} with score {}", best_move, best_score);
    Some(best_move)        
}


fn minimax(
    board: &Board,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    is_maximizing: bool,
    getting_move_for: PieceColor,
) -> i32 {

    if depth == 0 {
        return eval_position(&board, getting_move_for);
    }

    let legal_moves = board.list_all_valid_moves();
    let mut best_move_value;

    if is_maximizing {
        best_move_value = i32::MIN;

        for m in legal_moves {
            let mut board = *board;
            board.do_move_from_coord(m).unwrap();
            let child_board_value = minimax(&board, depth - 1, alpha, beta, !is_maximizing, getting_move_for);
            
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

        for m in legal_moves {
            let mut board = *board;
            board.do_move_from_coord(m).unwrap();
            let child_board_value = minimax(&board, depth - 1, alpha, beta, !is_maximizing, getting_move_for);

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
    let mut score = 0;
    for y in 0..8 {
        for x in 0..8 {
            let coord = Coord { x, y };
            if let Some(piece) = board.piece_at(coord) {
                let weighted_value = get_weighted_value(piece, coord);
                score += match piece.color == color {
                    true => weighted_value,
                    _ =>  -weighted_value,
                };
            }
        }
    }
    score
}