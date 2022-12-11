use std::fmt;
use colored::*;

// const PAWN: u8 = 1;
// const KNIGHT: u8 = 2;
// const BISHOP: u8 = 3;
// const ROOK: u8 = 4;
// const QUEEN: u8 = 5;
// const KING: u8 = 6;
// const BLACK: u8 = 0;
// const WHITE: u8 = 10;
// const W_PAWN: u8 = PAWN + WHITE;
// const W_KNIGHT: u8 = KNIGHT + WHITE;
// const W_BISHOP: u8 = BISHOP + WHITE;
// const W_ROOK: u8 = ROOK + WHITE;
// const W_QUEEN: u8 = QUEEN + WHITE;
// const W_KING: u8 = KING + WHITE;


#[derive(Debug)]
pub enum BoardError {
    ParseError(String),
    MoveError(String),
}
type Result<T> = std::result::Result<T, BoardError>;

impl std::error::Error for BoardError {}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            BoardError::ParseError(desc) => write!(f, "Error parsing FEN: {}", desc),
            BoardError::MoveError(desc) => write!(f, "Error making move: {}", desc),
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn from_notation(notation: &str) -> Result<Self> {
        // Translate a chess notation like 'a4' to a tuple of coordinates (0, 3)
        let mut chars = notation.chars();
        let x = chars.next().ok_or(BoardError::ParseError("Invalid notation".to_string()))?;
        let y = chars.next().ok_or(BoardError::ParseError("Invalid notation".to_string()))?;
        let x = 8 - (x as u8 - 97);
        let y = 7 - (y as u8 - 49);
        if x > 7 || y > 7 {
            return Err(BoardError::ParseError("Invalid notation".to_string()));
        }
        Ok(Coord { x: x as usize, y: y as usize })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
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
    color: PieceColor,
    piece: PieceKind,
}

impl Piece {
    pub const fn pawn(color: PieceColor) -> Self {
        Self { piece: Pawn, color }
    }

    pub const fn knight(color: PieceColor) -> Self {
        Self { piece: Knight, color }
    }

    pub const fn bishop(color: PieceColor) -> Self {
        Self { piece: Bishop, color } 
    }

    pub const fn rook(color: PieceColor) -> Self {
        Self { piece: Rook, color }
    }

    pub const fn queen(color: PieceColor) -> Self {
        Self { piece: Queen, color }
    }

    pub const fn king(color: PieceColor) -> Self {
        Self { piece: King, color }
    }

    fn fancy_char(self) -> &'static str {
        match self.piece {
            Pawn => "♙",
            Knight => "♞",
            Bishop => "♝",
            Rook => "♜",
            Queen => "♛",
            King => "♚",
        }
    }
    
}

#[derive(PartialEq)]
pub struct Board {
    pub board: [[Square; 8]; 8],
    pub turn: PieceColor,
    pub white_king: (u8, u8),
    pub black_king: (u8, u8),
    pub white_castle: (bool, bool),
    pub black_castle: (bool, bool),
    pub en_passant: Option<(u8, u8)>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            board: [[Square::Empty; 8]; 8],
            turn: PieceColor::White,
            white_king: (4, 0),
            black_king: (4, 7),
            white_castle: (true, true),
            black_castle: (true, true),
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
        }
        if castle.contains('Q') {
            board.white_castle.1 = true;
        }
        if castle.contains('k') {
            board.black_castle.0 = true;
        }
        if castle.contains('q') {
            board.black_castle.1 = true;
        }

        let en_passant = fen.next().ok_or(BoardError::ParseError("Invalid length of FEN".to_string()))?;
        if en_passant != "-" {
            let mut chars = en_passant.chars();
            let x = chars.next().ok_or(BoardError::ParseError("Invalid en passant row".to_string()))?;
            let y = chars.next().ok_or(BoardError::ParseError("Invalid en passant column".to_string()))?;
            board.en_passant = Some((x as u8 - 97, y as u8 - 49));
            if board.en_passant.unwrap().0 > 7 || board.en_passant.unwrap().0 > 7 {
                return Err(BoardError::ParseError("Invalid en passant square".to_string()));
            }
        }

        let halfmove_clock = fen.next().ok_or(BoardError::ParseError("Missing halfmove clock".to_string()))?;
        board.halfmove_clock = halfmove_clock.parse::<u8>().map_err(|_| BoardError::ParseError("Invalid halfmove clock".to_string()))?;

        let fullmove_number = fen.next().ok_or(BoardError::ParseError("Missing fullmove clock".to_string()))?;
        board.fullmove_number = fullmove_number.parse::<u8>().map_err(|_| BoardError::ParseError("Invalid fullmove clock".to_string()))?;

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

                print!("{}", cell);
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
            Ok(())
        } else {
            Err(BoardError::MoveError("No piece to move".to_string()))
        }
    }

    pub fn do_move(&mut self, notation_from: &str, notation_to: &str) -> Result<()> {
        let from = Coord::from_notation(notation_from)?;
        let to = Coord::from_notation(notation_to)?;
        if !self.is_valid_move(from, to) {
            return Err(BoardError::MoveError("Invalid move".to_string()));
        }
        self.move_piece(from, to)
    }

    pub fn is_valid_move(&self, from: Coord, to: Coord) -> bool {
        true
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_builder() {
        let fen_board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let default_board: Board = Board::default();

        fen_board.pretty_print_board();
        default_board.pretty_print_board();

        assert!(default_board == fen_board)
    }
}