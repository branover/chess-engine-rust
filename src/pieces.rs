use crate::board::Coord;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum PieceColor {
    #[default]
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

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::White => write!(f, "White"),
            PieceColor::Black => write!(f, "Black"),
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
            PieceColor::Black => {
                if from.y == 7 {
                    return false;
                }
                to.y == from.y + 1
            },
            PieceColor::White => {
                if from.y == 0 {
                    return false;
                }
                to.y == from.y - 1
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
        // Non-castle move
        ((from.x as i32 - to.x as i32).abs() <= 1 && (from.y as i32 - to.y as i32).abs() <= 1) ||
        // Castle move
        (from.x == 4 && from.y == 0 && to.x == 6 && to.y == 0) || (from.x == 4 && from.y == 7 && to.x == 6 && to.y == 7) ||
        (from.x == 4 && from.y == 0 && to.x == 2 && to.y == 0) || (from.x == 4 && from.y == 7 && to.x == 2 && to.y == 7)
    }

    #[inline]
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
        let mut moves = Vec::with_capacity(4);
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
        let mut moves = Vec::with_capacity(8);
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
        // Possible castle moves
        if from == (Coord { x: 4, y: 0 }) {
            moves.push(Coord { x: 6, y: 0 });
            moves.push(Coord { x: 2, y: 0 });
        } else if from == (Coord { x: 4, y: 7 }) {
            moves.push(Coord { x: 6, y: 7 });
            moves.push(Coord { x: 2, y: 7 });
        }
        moves
    }
    
}
