use crate::board::Coord;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
use PieceKind::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

impl Piece {
    pub const fn pawn(color: PieceColor) -> Self {
        Self { kind: Pawn, color }
    }

    pub const fn knight(color: PieceColor) -> Self {
        Self { kind: Knight, color }
    }

    pub const fn bishop(color: PieceColor) -> Self {
        Self { kind: Bishop, color } 
    }

    pub const fn rook(color: PieceColor) -> Self {
        Self { kind: Rook, color }
    }

    pub const fn queen(color: PieceColor) -> Self {
        Self { kind: Queen, color }
    }

    pub const fn king(color: PieceColor) -> Self {
        Self { kind: King, color }
    }

    pub fn fancy_char(&self) -> &'static str {
        match self.kind {
            Pawn => "♙",
            Knight => "♞",
            Bishop => "♝",
            Rook => "♜",
            Queen => "♛",
            King => "♚",
        }
    }

    pub fn is_valid_piece_move(&self, from: Coord, to: Coord) -> bool {
        match self.kind {
            Pawn => self.is_valid_pawn_move(from, to),
            Knight => self.is_valid_knight_move(from, to),
            Bishop => self.is_valid_bishop_move(from, to),
            Rook => self.is_valid_rook_move(from, to),
            Queen => self.is_valid_queen_move(from, to),
            King => self.is_valid_king_move(from, to),
        }
    }

    fn is_valid_pawn_move(&self, from: Coord, to: Coord) -> bool {
        println!("{:?} {:?}", from, to);
        if from.x != to.x {
            return false;
        }

        match self.color {
            PieceColor::Black => {
                if from.y == 1 {
                    to.y == 3 || to.y == 2
                } else {
                    to.y == from.y + 1
                }
            }
            PieceColor::White => {
                if from.y == 6 {
                    to.y == 4 || to.y == 5
                } else {
                    to.y == from.y - 1
                }
            }
        }
    }

    fn is_valid_knight_move(&self, from: Coord, to: Coord) -> bool {
        // Ensures that the move is only 2 squares in one direction and 1 square in the other
        (
            (from.x as i32 - to.x as i32).abs() == 2 && 
            (from.y as i32 - to.y as i32).abs() == 1
        ) || (
            (from.x as i32 - to.x as i32).abs() == 1 && 
            (from.y as i32 - to.y as i32).abs() == 2
        )
    }

    fn is_valid_bishop_move(&self, from: Coord, to: Coord) -> bool {
        (from.x as i32 - to.x as i32).abs() == (from.y as i32 - to.y as i32).abs()
    }

    fn is_valid_rook_move(&self, from: Coord, to: Coord) -> bool {
        from.x == to.x || from.y == to.y
    }

    fn is_valid_queen_move(&self, from: Coord, to: Coord) -> bool {
        self.is_valid_rook_move(from, to) || self.is_valid_bishop_move(from, to)
    }

    fn is_valid_king_move(&self, from: Coord, to: Coord) -> bool {
        (from.x as i32 - to.x as i32).abs() <= 1 && (from.y as i32 - to.y as i32).abs() <= 1
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn pawn_move() {
        let mut board = Board::default();
        let moves = [
            ("e2", "e4"),
            ("e4", "e5"),
            ("a2", "a4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            let piece = board.piece_at(Coord::from_notation(to).unwrap()).unwrap();
            assert_eq!(piece, Piece{kind: PieceKind::Pawn, color: PieceColor::White});
            board.pretty_print_board();
        }
    }

    #[test]
    #[should_panic]
    fn pawn_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("e2", "e4"),
            ("e4", "e6"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn knight_move() {
        let mut board = Board::default();
        let moves = [
            ("g1", "f3"),
            ("b1", "c3"),
            ("f3", "g5"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            let piece = board.piece_at(Coord::from_notation(to).unwrap()).unwrap();
            assert_eq!(piece, Piece{kind: PieceKind::Knight, color: PieceColor::White});
            board.pretty_print_board();
        }
    }

    #[test]
    #[should_panic]
    fn knight_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("f3", "g6"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn bishop_move() {
        let mut board = Board::default();
        let moves = [
            ("c1", "f4"),
            ("f4", "g5"),
            ("g5", "c1"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            let piece = board.piece_at(Coord::from_notation(to).unwrap()).unwrap();
            assert_eq!(piece, Piece{kind: PieceKind::Bishop, color: PieceColor::White});
            board.pretty_print_board();
        }
    }

    #[test]
    #[should_panic]
    fn bishop_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("c1", "f5"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn rook_move() {
        let mut board = Board::default();
        let moves = [
            ("a1", "a8"),
            ("a8", "h8"),
            ("h8", "h1"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            let piece = board.piece_at(Coord::from_notation(to).unwrap()).unwrap();
            assert_eq!(piece, Piece{kind: PieceKind::Rook, color: PieceColor::White});
            board.pretty_print_board();
        }
    }

    #[test]
    #[should_panic]
    fn rook_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("a1", "b2"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn queen_move() {
        let mut board = Board::default();
        let moves = [
            ("d1", "d8"),
            ("d8", "h4"),
            ("h4", "a4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            let piece = board.piece_at(Coord::from_notation(to).unwrap()).unwrap();
            assert_eq!(piece, Piece{kind: PieceKind::Queen, color: PieceColor::White});
            board.pretty_print_board();
        }
    }

    #[test]
    #[should_panic]
    fn queen_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("d1", "e3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn king_move() {
        let mut board = Board::default();
        let moves = [
            ("e1", "e2"),
            ("e2", "e3"),
            ("e3", "e2"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            let piece = board.piece_at(Coord::from_notation(to).unwrap()).unwrap();
            assert_eq!(piece, Piece{kind: PieceKind::King, color: PieceColor::White});
            board.pretty_print_board();
        }
    }

    #[test]
    #[should_panic]
    fn king_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("e1", "e4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

}
