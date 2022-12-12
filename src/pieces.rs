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
        // Check that coords are from (0,0) to (7,7)
        if from.x > 7 || from.y > 7 || to.x > 7 || to.y > 7 {
            return false;
        }
        from != to && 
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
        if self.is_valid_pawn_capture(from, to) {
            return true;
        }

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

    fn is_valid_pawn_capture(&self, from: Coord, to: Coord) -> bool {
        if (from.x as i32 - to.x as i32).abs() != 1 {
            return false;
        }

        match self.color {
            PieceColor::Black => to.y == from.y + 1,
            PieceColor::White => to.y == from.y - 1,
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

    pub fn list_possible_moves(&self, from: Coord) -> Vec<Coord> {
        match self.kind {
            Pawn => self.list_possible_pawn_moves(from),
            Knight => self.list_possible_knight_moves(from),
            Bishop => self.list_possible_bishop_moves(from),
            Rook => self.list_possible_rook_moves(from),
            Queen => self.list_possible_queen_moves(from),
            King => self.list_possible_king_moves(from),
        }
    }

    fn list_possible_pawn_moves(&self, from: Coord) -> Vec<Coord> {
        let mut moves = Vec::new();
        let mut to_x: i32 = from.x as i32;
        let mut to_y: i32 = from.y as i32;
        match self.color {
            PieceColor::Black => {
                to_y += 1;
                if to_y < 8 {
                    moves.push(Coord { x: to_x as usize, y: to_y as usize });
                }
                if from.y == 1 {
                    to_y += 1;
                    if to_y < 8 {
                        moves.push(Coord { x: to_x as usize, y: to_y as usize });
                    }
                }
                // Add pawn captures
                to_x -= 1;
                to_y = from.y as i32 + 1;
                if to_x >= 0 && to_y < 8 {
                    moves.push(Coord { x: to_x as usize, y: to_y as usize });
                }
                to_x += 2;
                if to_x < 8 && to_y < 8 {
                    moves.push(Coord { x: to_x as usize, y: to_y as usize });
                }
            }
            PieceColor::White => {
                to_y -= 1;
                if to_y >= 0 {
                    moves.push(Coord { x: to_x as usize, y: to_y as usize });
                }
                if from.y == 6 {
                    to_y -= 1;
                    if to_y >= 0 {
                        moves.push(Coord { x: to_x as usize, y: to_y as usize });
                    }
                }
                // Add pawn captures
                to_x -= 1;
                to_y = from.y as i32 - 1;
                if to_x >= 0 && to_y >= 0 {
                    moves.push(Coord { x: to_x as usize, y: to_y as usize });
                }
                to_x += 2;
                if to_x < 8 && to_y >= 0 {
                    moves.push(Coord { x: to_x as usize, y: to_y as usize });
                }
            }
        }
        moves

    }

    fn list_possible_knight_moves(&self, from: Coord) -> Vec<Coord> {
        let mut moves = Vec::new();
        let mut to_x: i32 = from.x as i32 + 2;
        let mut to_y: i32 = from.y as i32 + 1;
        if to_x < 8 && to_y < 8 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 - 2;
        to_y = from.y as i32 - 1;
        if to_x >= 0 && to_y >= 0 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 + 2;
        to_y = from.y as i32 - 1;
        if to_x < 8 && to_y >= 0 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 - 2;
        to_y = from.y as i32 + 1;
        if to_x >= 0 && to_y < 8 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 + 1;
        to_y = from.y as i32 + 2;
        if to_x < 8 && to_y < 8 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 - 1;
        to_y = from.y as i32 - 2;
        if to_x >= 0 && to_y >= 0 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 + 1;
        to_y = from.y as i32 - 2;
        if to_x < 8 && to_y >= 0 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        to_x = from.x as i32 - 1;
        to_y = from.y as i32 + 2;
        if to_x >= 0 && to_y < 8 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
        }
        moves
    }

    fn list_possible_bishop_moves(&self, from: Coord) -> Vec<Coord> {
        let mut moves = Vec::new();
        let mut to_x: i32 = from.x as i32 + 1;
        let mut to_y: i32 = from.y as i32 + 1;
        while to_x < 8 && to_y < 8 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
            to_x += 1;
            to_y += 1;
        }
        to_x = from.x as i32 - 1;
        to_y = from.y as i32 - 1;
        while to_x >= 0 && to_y >= 0 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
            to_x -= 1;
            to_y -= 1;
        }
        to_x = from.x as i32 + 1;
        to_y = from.y as i32 - 1;
        while to_x < 8 && to_y >= 0 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
            to_x += 1;
            to_y -= 1;
        }
        to_x = from.x as i32 - 1;
        to_y = from.y as i32 + 1;
        while to_x >= 0 && to_y < 8 {
            moves.push(Coord { x: to_x as usize, y: to_y as usize });
            to_x -= 1;
            to_y += 1;
        }
        moves
    }

    fn list_possible_rook_moves(&self, from: Coord) -> Vec<Coord> {
        let mut moves = Vec::new();
        let mut to_x: i32 = from.x as i32 + 1;
        while to_x < 8 {
            moves.push(Coord { x: to_x as usize, y: from.y });
            to_x += 1;
        }
        to_x = from.x as i32 - 1;
        while to_x >= 0 {
            moves.push(Coord { x: to_x as usize, y: from.y });
            to_x -= 1;
        }
        let mut to_y: i32 = from.y as i32 + 1;
        while to_y < 8 {
            moves.push(Coord { x: from.x, y: to_y as usize });
            to_y += 1;
        }
        to_y = from.y as i32 - 1;
        while to_y >= 0 {
            moves.push(Coord { x: from.x, y: to_y as usize });
            to_y -= 1;
        }
        moves
    }

    fn list_possible_queen_moves(&self, from: Coord) -> Vec<Coord> {
        let mut moves = Vec::new();
        moves.append(&mut self.list_possible_bishop_moves(from));
        moves.append(&mut self.list_possible_rook_moves(from));
        moves
    }

    fn list_possible_king_moves(&self, from: Coord) -> Vec<Coord> {
        let mut moves = Vec::new();

        for x in [from.x as i32, from.x as i32 +1, from.x as i32 -1].iter() {
            for y in [from.y as i32 , from.y as i32 +1, from.y as i32 -1].iter() {
                if *x < 8 && *y < 8 && *x >= 0 && *y >= 0 && (*x != from.x as i32 || *y != from.y as i32) {
                    moves.push(Coord { x: *x as usize, y: *y as usize });
                }
            }
        }
        moves
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn invalid_piece_move() {
        let mut board = Board::default();
        let moves = [
            ("e2", "e2")
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn pawn_move() {
        let mut board = Board::default();
        let moves = [
            ("e2", "e4"),
            ("d7", "d5"),
            ("e4", "e5"),
            ("h7", "h6"),
            ("a2", "a4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn pawn_move_invalid() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();

        let moves = [
            ("e4", "e6"),
            ("e4", "e3"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn knight_move() {
        let mut board = Board::default();
        let moves = [
            ("g1", "f3"),
            ("b8", "c6"),
            ("b1", "c3"),
            ("g8", "f6"),
            ("f3", "g5"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn knight_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("f3", "g6"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn bishop_move() {
        let mut board = Board::default();

        // Get pawn out of the way
        board.do_move("d2", "d4").unwrap();
        board.do_move("e7", "e5").unwrap();

        let moves = [
            ("c1", "f4"),
            ("a7", "a6"),
            ("f4", "g5"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn bishop_move_invalid() {
        let mut board = Board::default();

        // Get pawn out of the way
        board.do_move("d2", "d4").unwrap();

        let moves = [
            ("c1", "f5"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn rook_move() {
        let mut board = Board::default();
        // Get pawn out of the way
        board.do_move("a2", "a4").unwrap();
        board.do_move("h7", "h5").unwrap();

        let moves = [
            ("a1", "a3"),
            ("h5", "h4"),
            ("a3", "h3"),
            ("a7", "a6"),
            ("h3", "d3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn rook_move_invalid() {
        let mut board = Board::default();

        let moves = [
            ("a1", "b2"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn queen_move() {
        let mut board = Board::default();
        // Get pawn out of the way
        board.do_move("d2", "d4").unwrap();
        board.do_move("e7", "e5").unwrap();

        let moves = [
            ("d1", "d3"),
            ("a7", "a6"),
            ("d3", "g6"),
            ("a6", "a5"),
            ("g6", "g3"),
            ("a5", "a4"),
            ("g3", "a3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn queen_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("d1", "e3"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn king_move() {
        let mut board = Board::default();

        // Get pawn out of the way
        board.do_move("e2", "e4").unwrap();


        let moves = [
            ("e7", "e6"),
            ("e1", "e2"),
            ("e8", "e7"),
            ("e2", "e3"),
            ("e7", "d6"),
            ("e3", "e2"),
            ("d6", "c6"),
            ("e2", "f3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn king_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("e1", "e4"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());        
        }
    }

    #[test]
    fn queen_capture() {
        let mut board = Board::default();
        let moves = [
            ("d2", "d4"),
            ("e7", "e5"),
            ("d1", "d3"),
            ("e5", "d4"),
            ("d3", "d4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
            board.pretty_print_board();
        }
    }

    #[test]
    fn move_blocked() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("e7", "e5").unwrap();

        // Should be blocked by pawn
        let result = board.do_move("d1", "d4");
        assert!(result.is_err());
    }

    #[test]
    fn move_blocked_knight() {
        let mut board = Board::default();
        // Should not be blocked
        let result = board.do_move("b1", "c3");
        assert!(result.is_ok());

        board.do_move("e7", "e5").unwrap();

        // Should be blocked by queen
        let result = board.do_move("c3", "d1");
        assert!(result.is_err());

        // Should not be blocked
        let result = board.do_move("c3", "b1");
        assert!(result.is_ok());
    }

    #[test]
    fn pawn_cant_capture_in_front() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("e7", "e5").unwrap();

        // Should not be able to capture in front
        let result = board.do_move("e4", "e5");
        assert!(result.is_err());
    }

    #[test]
    fn pawn_can_capture_diagonal() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("d7", "d5").unwrap();

        // Should be able to capture diagonal
        let result = board.do_move("e4", "d5");
        assert!(result.is_ok());
    }

    #[test]
    fn same_color_cant_move_twice() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("e7", "e5").unwrap();

        // Black should not be able to move twice
        let result = board.do_move("e8", "e7");
        assert!(result.is_err());
    }

}
