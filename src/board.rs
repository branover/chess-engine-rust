use crate::pieces::{
    Piece,
    PieceColor,
    PieceKind,
};

use std::{fmt, cmp::Ordering};
use colored::*;
// use std::collections::HashSet;
use rustc_hash::FxHashSet;

#[derive(Debug)]
pub enum BoardError {
    ParseError(String),
    MoveError(String),
}
pub type Result<T> = std::result::Result<T, BoardError>;

impl std::error::Error for BoardError {}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoardError::ParseError(desc) => write!(f, "Error parsing input: {desc}"),
            BoardError::MoveError(desc) => write!(f, "Error making move: {desc}"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

impl Square {
    fn fancy_char(self) -> &'static str {
        match self {
            Square::Occupied(piece) => piece.fancy_char(),
            _ => " ",
        }
    }  
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn from_notation(notation: &str) -> Result<Self> {
        // Translate a chess notation like 'a4' to a tuple of coordinates (0, 3)
        let mut chars = notation.chars();
        let x = chars.next().ok_or(BoardError::ParseError("Invalid notation".to_string()))?;
        let y = chars.next().ok_or(BoardError::ParseError("Invalid notation".to_string()))?;
        if x < 'a' || x > 'h' || y < '1' || y > '8' {
            return Err(BoardError::ParseError("Invalid notation".to_string()));
        }

        let x = x as u8 - 97;
        let y = 7 - (y as u8 - 49);
        Ok(Coord { x: x as usize, y: y as usize })
    }

    pub fn to_notation(self) -> String {
        let x = (self.x as u8 + 97) as char;
        let y = (7 - self.y as u8 + 49) as char;
        format!("{x}{y}")
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Move {
    pub from: Coord,
    pub to: Coord,
    pub promote: Option<PieceKind>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Board {
    pub board: [[Square; 8]; 8],
    pub turn: PieceColor,
    pub white_king: Coord,
    pub black_king: Coord,
    pub white_castle: (bool, bool),
    pub black_castle: (bool, bool),
    pub in_check: (bool, bool),
    pub in_checkmate: (bool, bool),
    pub in_stalemate: (bool, bool),
    pub en_passant: Option<Coord>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            board: [[Square::Empty; 8]; 8],
            turn: PieceColor::White,
            white_king: Coord {x: 4, y: 7},
            black_king: Coord {x: 4, y: 0},
            white_castle: (true, true),
            black_castle: (true, true),
            in_check: (false, false),
            in_checkmate: (false, false),
            in_stalemate: (false, false),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn default() -> Board {
        let mut board = Board::empty();
        board.board = [
            [Square::Occupied(Piece::rook(PieceColor::Black)), Square::Occupied(Piece::knight(PieceColor::Black)), Square::Occupied(Piece::bishop(PieceColor::Black)), Square::Occupied(Piece::queen(PieceColor::Black)), Square::Occupied(Piece::king(PieceColor::Black)), Square::Occupied(Piece::bishop(PieceColor::Black)), Square::Occupied(Piece::knight(PieceColor::Black)), Square::Occupied(Piece::rook(PieceColor::Black))],
            [Square::Occupied(Piece::pawn(PieceColor::Black)); 8],
            [Square::Empty; 8],
            [Square::Empty; 8],
            [Square::Empty; 8],
            [Square::Empty; 8],
            [Square::Occupied(Piece::pawn(PieceColor::White)); 8],
            [Square::Occupied(Piece::rook(PieceColor::White)), Square::Occupied(Piece::knight(PieceColor::White)), Square::Occupied(Piece::bishop(PieceColor::White)), Square::Occupied(Piece::queen(PieceColor::White)), Square::Occupied(Piece::king(PieceColor::White)), Square::Occupied(Piece::bishop(PieceColor::White)), Square::Occupied(Piece::knight(PieceColor::White)), Square::Occupied(Piece::rook(PieceColor::White))],
        ];
        board
    }

    pub fn from_fen(fen: &str) -> Result<Board> {
        let mut board = Board::empty();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut fen = fen.split_whitespace();

        let lines = fen.next().ok_or(BoardError::ParseError("Lines not provided".to_string()))?;
        for c in lines.chars() {
            if x > 8 || y > 7 {
                return Err(BoardError::ParseError("Board too big".to_string()));
            }
            match c {
                'p' => board.board[y][x] = Square::Occupied(Piece::pawn(PieceColor::Black)),
                'n' => board.board[y][x] = Square::Occupied(Piece::knight(PieceColor::Black)),
                'b' => board.board[y][x] = Square::Occupied(Piece::bishop(PieceColor::Black)),
                'r' => board.board[y][x] = Square::Occupied(Piece::rook(PieceColor::Black)),
                'q' => board.board[y][x] = Square::Occupied(Piece::queen(PieceColor::Black)),
                'k' => board.board[y][x] = Square::Occupied(Piece::king(PieceColor::Black)),
                'P' => board.board[y][x] = Square::Occupied(Piece::pawn(PieceColor::White)),
                'N' => board.board[y][x] = Square::Occupied(Piece::knight(PieceColor::White)),
                'B' => board.board[y][x] = Square::Occupied(Piece::bishop(PieceColor::White)),
                'R' => board.board[y][x] = Square::Occupied(Piece::rook(PieceColor::White)),
                'Q' => board.board[y][x] = Square::Occupied(Piece::queen(PieceColor::White)),
                'K' => board.board[y][x] = Square::Occupied(Piece::king(PieceColor::White)),
                '/' => {
                    x = 0;
                    y += 1;
                    continue;
                }
                '0'..='8' => {
                    board.board[y][x] = Square::Empty;
                    x += c.to_digit(10).unwrap() as usize - 1;
                }                
                _ => return Err(BoardError::ParseError("Invalid character in lines".to_string())),
            }
            x += 1
        }
        
        let turn = fen.next().ok_or(BoardError::ParseError("Invalid length of FEN".to_string()))?;
        match turn {
            "w" => board.turn = PieceColor::White,
            "b" => board.turn = PieceColor::Black,
            _ => return Err(BoardError::ParseError("Invalid turn character".to_string())),
        }

        let castle = fen.next().ok_or(BoardError::ParseError("Invalid length of FEN".to_string()))?;
        if castle.contains('K') {
            board.white_castle.0 = true;
        } else {
            board.white_castle.0 = false;
        }
        if castle.contains('Q') {
            board.white_castle.1 = true;
        } else {
            board.white_castle.1 = false;
        }
        if castle.contains('k') {
            board.black_castle.0 = true;
        } else {
            board.black_castle.0 = false;
        }
        if castle.contains('q') {
            board.black_castle.1 = true;
        } else {
            board.black_castle.1 = false;
        }

        let en_passant = fen.next().ok_or(BoardError::ParseError("Invalid length of FEN".to_string()))?;
        if en_passant != "-" {
            board.en_passant = Some(Coord::from_notation(en_passant)?);
        }

        let halfmove_clock = fen.next().ok_or(BoardError::ParseError("Missing halfmove clock".to_string()))?;
        board.halfmove_clock = halfmove_clock.parse::<u8>().map_err(|_| BoardError::ParseError("Invalid halfmove clock".to_string()))?;

        let fullmove_number = fen.next().ok_or(BoardError::ParseError("Missing fullmove clock".to_string()))?;
        board.fullmove_number = fullmove_number.parse::<u8>().map_err(|_| BoardError::ParseError("Invalid fullmove clock".to_string()))?;

        board.set_check();
        board.check_end_conditions();
        Ok(board)
    }

    pub fn pretty_print_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                let square = self.board[i][j];
                let cell = format!("{} ", square.fancy_char());
                let cell = match square {
                    Square::Occupied(Piece { color: PieceColor::White, .. }) => cell.white(),
                    Square::Occupied(Piece { color: PieceColor::Black, .. }) => cell.black(),
                    _ => cell.white(),
                };

                let cell = if (i + j) % 2 != 0 {
                    cell.on_truecolor(158, 93, 30)
                } else {
                    cell.on_truecolor(205, 170, 125)
                };

                print!("{cell}");
            }
            println!(" {}", 8 - i);
        }
        println!("a b c d e f g h");

    }

    pub fn move_piece(&mut self, from: Coord, to: Coord) -> Result<()> {
        let piece = self.board[from.y][from.x];
        if let Square::Occupied(piece) = piece {
            self.board[from.y][from.x] = Square::Empty;
            self.board[to.y][to.x] = Square::Occupied(piece);

            // Clear en passant square
            self.en_passant = None;

            match piece.kind {
                PieceKind::Pawn => {
                    // Set en passant square
                    if (from.y as i8 - to.y as i8).abs() == 2 {
                        self.en_passant = Some(Coord{x: from.x, y: (from.y + to.y) / 2});
                    } else if piece.color == PieceColor::White && to.y == 0 {
                        self.board[to.y][to.x] = Square::Occupied(Piece::queen(PieceColor::White));
                    } else if piece.color == PieceColor::Black && to.y == 7 {
                        self.board[to.y][to.x] = Square::Occupied(Piece::queen(PieceColor::Black));
                    }
                },
                PieceKind::King => {
                    // Remove ability to castle based on moved piece
                    if piece.color == PieceColor::White {
                        self.white_castle = (false, false);
                        self.white_king = to;
                    } else {
                        self.black_castle = (false, false);
                        self.black_king = to;
                    }
                },
                PieceKind::Rook => {
                    // Remove ability to castle based on moved piece
                    if piece.color == PieceColor::White {
                        if from.x == 0 {
                            self.white_castle.1 = false;
                        } else if from.x == 7 {
                            self.white_castle.0 = false;
                        }
                    } else if from.x == 0 {
                        self.black_castle.1 = false;
                    } else if from.x == 7 {
                        self.black_castle.0 = false;
                    }
                },
                _ => {}
            }
            Ok(())
        } else {
            Err(BoardError::MoveError("No piece to move".to_string()))
        }        
    }

    pub fn do_move(&mut self, from: &str, to: &str) -> Result<()> {
        self.do_move_from_coord(Coord::from_notation(from)?, Coord::from_notation(to)?)
    }

    pub fn do_move_from_coord(&mut self, from: Coord, to: Coord) -> Result<()> {
        if self.is_castle(from, to) {
            self.do_castle(from, to)?;
        } else if self.is_en_passant(from, to) {
            self.do_en_passant(from, to)?;
        } else {
            if !self.is_valid_move(from, to) {
                return Err(BoardError::MoveError(format!("Invalid move from {} to {}", from.to_notation(), to.to_notation())));
            }
            self.move_piece(from, to)?; 
        }
        self.end_turn();
        Ok(())
    }

    #[inline]
    fn end_turn(&mut self) {
        self.turn = self.turn.opposite();
        self.set_check();
        self.check_end_conditions();
    }

    pub fn parse_move(&self, notation: &str) -> Result<(String, String)> {
        let mut chars = notation.chars();
        let from = chars.by_ref().take(2).collect::<String>();
        let to = chars.by_ref().take(2).collect::<String>();
        // let promotion = chars.next().map(|c| PieceKind::from_char(c));
        Ok((from, to))
    }

    #[inline]
    pub fn is_valid_move(&self, from: Coord, to: Coord) -> bool {
        let piece = match self.piece_at(from) {
            Some(piece) => piece,
            _ => return false,
        };

        // Stops pieces from moving if it's not their turn
        if piece.color != self.turn {
            return false;
        }

        // Must be moving to a square that the piece can attack (with exceptions)
        if !self.can_attack_square(from, to) {
            return false
        }

        // If the player is in check, they must remove check
        if self.get_check() && !self.removes_check(from, to) {
            return false;
        }

        // If the player is not in check, they must not move into check
        if self.would_be_in_check(from, to) {
            return false;
        }

        true
    }

    #[inline]
    fn can_attack_square(&self, from: Coord, to: Coord) -> bool {
        let piece = match self.piece_at(from) {
            Some(piece) => piece,
            _ => return false,
        };

        // Stops pieces from moving to the same square or to invalid squares for that type of piece
        if !piece.is_valid_piece_move(from, to) {
            return false;
        }

        // Stops pieces from capturing the same color and pawns from moving diagonally without capturing
        if let Some(piece_at) = self.piece_at(to) {
            if piece_at.color == piece.color {
                return false;
            }
            if piece.kind == PieceKind::Pawn && (to.x == from.x) {
                return false;
            }
        } else if piece.kind == PieceKind::Pawn && (to.x != from.x) {
            if let Some(en_passant) = self.en_passant {
                if en_passant != to {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Stops pieces from moving through other pieces (except knights)
        if piece.kind != PieceKind::Knight && self.move_blocked(from, to) {
            return false;
        }

        true
    }

    fn is_castle(&self, from: Coord, to: Coord) -> bool {
        if let Some(piece) = self.piece_at(from) {
            if piece.kind == PieceKind::King && (from.x == 4) && (to.x == 6 || to.x == 2) {
                return true;
            }
        }
        false
    }

    fn do_castle(&mut self, from: Coord, to: Coord) -> Result<()> {
        if self.get_check() {
            return Err(BoardError::MoveError("Cannot castle while in check".to_string()));
        }

        let king = self.piece_at(from).unwrap();
        if king.kind != PieceKind::King || king.color != self.turn {
            return Err(BoardError::MoveError("Invalid castle".to_string()));
        }
        let rook_square = if to.x == from.x + 2 {
            self.board[from.y][7]
        } else {
            self.board[from.y][0]
        };


        let rook = if let Square::Occupied(rook) = rook_square {
            if rook.kind != PieceKind::Rook {
                return Err(BoardError::MoveError("Invalid castle".to_string()));
            }
            rook
        } else {
            return Err(BoardError::MoveError("Invalid castle".to_string()));
        };

        if king.color != rook.color {
            return Err(BoardError::MoveError("Invalid castle".to_string()));
        }

        if to.x == 6 {
            // Kingside castle
            if !match king.color {
                PieceColor::White => self.white_castle.0,
                PieceColor::Black => self.black_castle.0,
            } {
                return Err(BoardError::MoveError("Can't kingside castle".to_string()));
            }
            if self.is_valid_move(from, to) && self.is_valid_move(Coord {x: 7, y: from.y}, Coord { x: 5, y: from.y }) {
                self.move_piece(from, to)?;
                self.move_piece(Coord { x: 7, y: from.y }, Coord { x: 5, y: from.y })?;
            } else {
                return Err(BoardError::MoveError("Invalid castle".to_string()));
            }

        } else if to.x == 2 {
            // Queenside castle
            if !match king.color {
                PieceColor::White => self.white_castle.1,
                PieceColor::Black => self.black_castle.1,
            } {
                return Err(BoardError::MoveError("Can't queenside castle".to_string()));
            }
            if self.is_valid_move(from, to) && self.is_valid_move(Coord {x: 0, y: from.y}, Coord { x: 3, y: from.y }) {
                self.move_piece(from, to)?;
                self.move_piece(Coord { x: 0, y: from.y }, Coord { x: 3, y: from.y })?;
            } else {
                return Err(BoardError::MoveError("Invalid castle".to_string()));
            }
        }

        Ok(())
    }

    fn is_en_passant(&self, from: Coord, to: Coord) -> bool {
        if let Some(piece) = self.piece_at(from) {
            if piece.kind == PieceKind::Pawn && (to.x != from.x) {
                if let Some(en_passant) = self.en_passant {
                    return en_passant == to;
                }
            }
        }
        false
    }

    fn do_en_passant(&mut self, from: Coord, to: Coord) -> Result<()> {
        let piece = self.piece_at(from).unwrap();
        if piece.kind != PieceKind::Pawn || piece.color != self.turn {
            return Err(BoardError::MoveError("Invalid en passant".to_string()));
        }

        if let Some(en_passant) = self.en_passant {
            if en_passant == to && self.is_valid_move(from, to) {
                self.move_piece(from, to)?;
                // Remove the captured pawn in front of the en passant square
                let captured_pawn_square = match piece.color {
                    PieceColor::White => Coord{x: to.x, y: to.y + 1},
                    PieceColor::Black => Coord{x: to.x, y: to.y - 1},
                };
                self.piece_at(captured_pawn_square).ok_or(BoardError::MoveError("Invalid en passant".to_string()))?;
                
                self.board[captured_pawn_square.y][captured_pawn_square.x] = Square::Empty;
                return Ok(());
            }
        }

        Err(BoardError::MoveError("Invalid en passant".to_string()))
    }

    #[inline]
    pub fn piece_at(&self, coord: Coord) -> Option<Piece> {
        match self.board[coord.y][coord.x] {
            Square::Occupied(piece) => Some(piece),
            _ => None,
        }
    }

    pub fn move_blocked(&self, from: Coord, to: Coord) -> bool {
        let mut x: i32 = from.x as i32;
        let mut y: i32 = from.y as i32;

        let x_dir = match from.x.cmp(&to.x) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        let y_dir = match from.y.cmp(&to.y) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        while x != to.x as i32 || y != to.y as i32 {
            x += x_dir;
            y += y_dir;
            if self.board[y as usize][x as usize] != Square::Empty && (x != to.x as i32 || y != to.y as i32) {
                return true;
            }
        }

        false
    }

    pub fn list_all_valid_moves(&self) -> Vec<Move> {
        self.list_all_valid_moves_color(self.turn)
    }

    pub fn list_all_valid_moves_color(&self, color: PieceColor) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                let coord = Coord { x, y };
                if let Some(piece) = self.piece_at(coord) {
                    if piece.color == color {
                        let this_piece_moves = piece.list_possible_moves(coord);
                        this_piece_moves.iter().for_each(|m| {
                            if self.is_valid_move(coord, *m) {
                                moves.push(Move { from: coord, to: *m, promote: None});
                            }
                        });
                    }
                }
            }
        }
        moves
    }

    pub fn has_valid_moves(&self) -> bool {
        self.has_valid_moves_color(self.turn)
    }

    pub fn has_valid_moves_color(&self, color: PieceColor) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let coord = Coord { x, y };
                if let Some(piece) = self.piece_at(coord) {
                    if piece.color == color {
                        let this_piece_moves = piece.list_possible_moves(coord);
                        for m in this_piece_moves {
                            if self.is_valid_move(coord, m) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    #[inline]
    pub fn list_all_attacked_squares(&self) -> FxHashSet<Coord> {
        self.list_all_attacked_squares_color(self.turn)
    }

    pub fn list_all_attacked_squares_color(&self, color: PieceColor) -> FxHashSet<Coord> {
        let mut attacked_squares: FxHashSet<Coord> = FxHashSet::default();

        for y in 0..8 {
            for x in 0..8 {
                let coord = Coord { x, y };
                if let Some(piece) = self.piece_at(coord) {
                    if piece.color == color {
                        let this_piece_moves = piece.list_possible_moves(coord);
                        for m in this_piece_moves {
                            if self.can_attack_square(coord, m) {
                                attacked_squares.insert(m);
                            } 
                        }
                    }
                }
            }
        }
        attacked_squares
    }

    fn would_be_in_check(&self, from: Coord, to: Coord) -> bool {
        let mut board = *self;
        board.move_piece(from, to).unwrap();
        board.turn = board.turn.opposite();
        board.is_in_check(self.turn)
    }

    fn removes_check(&self, from: Coord, to: Coord) -> bool {
        !self.would_be_in_check(from, to)
    }

    fn is_in_check(&self, color: PieceColor) -> bool {
        let king_coord = match color {
            PieceColor::White => self.white_king,
            PieceColor::Black => self.black_king,
        };

        for y in 0..8 {
            for x in 0..8 {
                let coord = Coord { x, y };
                if let Some(piece) = self.piece_at(coord) {
                    if piece.color != color && self.can_attack_square(coord, king_coord) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn set_check(&mut self) {
        self.in_check = match self.turn {
            PieceColor::White => (self.is_in_check(PieceColor::White), false),
            PieceColor::Black => (false, self.is_in_check(PieceColor::Black)),
        };
    }

    pub fn get_check(&self) -> bool {
        match self.turn {
            PieceColor::White => self.in_check.0,
            PieceColor::Black => self.in_check.1,
        }
    }

    pub fn get_checkmate(&self) -> bool {
        match self.turn {
            PieceColor::White => self.in_checkmate.0,
            PieceColor::Black => self.in_checkmate.1,
        }
    }

    pub fn get_stalemate(&self) -> bool {
        match self.turn {
            PieceColor::White => self.in_stalemate.0,
            PieceColor::Black => self.in_stalemate.1,
        }
    }

    pub fn check_end_conditions(&mut self) {
        if !self.has_valid_moves_color(self.turn) {
            match self.turn {
                PieceColor::White => {
                    if self.in_check.0 {
                        self.in_checkmate.0 = true;
                    } else {
                        self.in_stalemate.0 = true;
                    }
                }
                PieceColor::Black => {
                    if self.in_check.1 {
                        self.in_checkmate.1 = true;
                    } else {
                        self.in_stalemate.1 = true;
                    }
                }
            }
        }
    }

}
