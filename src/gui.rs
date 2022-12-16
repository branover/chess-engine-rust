use iced::{button, container, Container, Align, Length, HorizontalAlignment, VerticalAlignment, Background, Button, Row, Column, Element, Sandbox, Settings, Text};
use rand::{thread_rng, seq::SliceRandom};


use lazy_static::lazy_static;

use std::sync::Mutex;
use crate::board::{
    Board,
    Move,
    Coord,
};
use crate::pieces::{
    Piece,
    PieceColor,
    PieceKind
};
use crate::engine::{
    make_best_move
};


pub fn run(get_cpu_move: fn(&Board) -> Move, starting_board: Board) -> iced::Result {
    {
        let mut x = GET_CPU_MOVE.lock().unwrap();
        *x = get_cpu_move;
        let mut x = STARTING_BOARD.lock().unwrap();
        *x = starting_board;
    };
    
    ChessBoard::run(Settings {
        window: iced::window::Settings {
            size: (
                (SQUARE_SIZE * 8) as u32,
                (SQUARE_SIZE * 8) as u32
            ),
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

lazy_static! {
    static ref GET_CPU_MOVE: Mutex<fn(&Board) -> Move> = Mutex::new(best_move);
    static ref STARTING_BOARD: Mutex<Board> = Mutex::new(Board::default());
}

const SQUARE_SIZE: u16 = 48;
// pub const AI_DEPTH: i32 = if cfg!(debug_assertions) {2} else {4};
pub const AI_DEPTH: u8 = 5;


pub fn get_symbol(piece: &Piece) -> impl ToString {
	// match piece.kind {
    //     PieceKind::Pawn => "♟︎",
    //     PieceKind::Knight => "♞",
    //     PieceKind::Bishop => "♝",
    //     PieceKind::Rook => "♜",
    //     PieceKind::Queen => "♛",
    //     PieceKind::King => "♚",
	// }
    match piece.kind {
        PieceKind::Pawn => "P",
        PieceKind::Knight => "N",
        PieceKind::Bishop => "B",
        PieceKind::Rook => "R",
        PieceKind::Queen => "Q",
        PieceKind::King => "K",
	}
}

pub fn best_move(board: &Board) -> Move {
    make_best_move(AI_DEPTH, board).unwrap()
}

pub fn random_move(board: &Board) -> Move {
    let moves = board.list_all_valid_moves();

    let mut rng = thread_rng();
    *moves.choose(&mut rng).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameResult {
    /// The game is not finished, and the game is still in play.
    Continuing,
    /// One player, the victor, checkmated the other.
    /// This stores the color of the winner.
    Victory(PieceColor),
    /// The game is drawn. This can be a result of the current player
    /// having no legal moves and not being in check, or because
    /// both players have insufficient material on the board.
    ///
    /// Insufficient material consists of:
    /// 1. The player only has a king
    /// 2. The player only has a king and a knight
    /// 3. The player only has a king and two knights
    /// 4. The player only has a king and a bishop
    /// 5. The player only has a king and two bishops
    ///
    /// In a regular game of chess, threefold repetition also triggers
    /// a stalemate, but this engine does not have builtin support for
    /// threefold repetition detection yet.
    Stalemate,
    /// An illegal move was made. This can include many things,
    /// such as moving a piece through another piece, attempting
    /// to capture an allied piece, moving non-orthogonally or
    /// non-diagonally, or non-knight-like according the rules
    /// governing the movement of the piece. Additionally,
    /// moves that put the player in check, (for example, moving a pinned piece),
    /// are also illegal.
    IllegalMove(Move),
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectSquare(Coord),
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    }
}

const SELECTED_DARK_SQUARE: iced::Color = rgb!(170,162,58);
const SELECTED_LIGHT_SQUARE: iced::Color = rgb!(205,210,106);

const LIGHT_SQUARE: iced::Color = rgb!(240,217,181);
const DARK_SQUARE: iced::Color = rgb!(181,136,99);


#[derive(Default, Clone, Copy)]
struct ChessSquare { row: usize, col: usize, piece_color: PieceColor, is_selected: bool }

impl From<(Coord, PieceColor, bool)> for ChessSquare {
    fn from(pos_color: (Coord, PieceColor, bool)) -> Self {
        let (pos, color, is_selected) = pos_color;
        Self::new(pos.x, pos.y, color, is_selected)
    }
}

impl ChessSquare {
    fn new(row: usize, col: usize, piece_color: PieceColor, is_selected: bool) -> Self {
        Self { row, col, piece_color, is_selected }
    }

    fn get_bg_color(&self, is_selected: bool) -> iced::Color {
        if (self.row * 9 + self.col) % 2 == 1 {
            if is_selected {
                SELECTED_LIGHT_SQUARE
            } else {
                LIGHT_SQUARE
            }
        } else if is_selected {
            SELECTED_DARK_SQUARE
        } else {
            DARK_SQUARE
        }

    }

    fn get_text_color(&self) -> iced::Color {
        if self.piece_color == PieceColor::White {
            iced::Color::WHITE
        } else {
            iced::Color::BLACK
        }
    }
}


impl button::StyleSheet for ChessSquare {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(self.is_selected))),
            border_color: self.get_bg_color(self.is_selected),
            text_color: self.get_text_color(),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(true))),
            border_color: self.get_bg_color(true),
            text_color: self.get_text_color(),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }
}

struct ChessBoardStyle;

impl container::StyleSheet for ChessBoardStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: iced::Color::BLACK,
            border_width: 10.0,
            border_radius: 0.0,
            ..container::Style::default()
        }
    }
}

#[derive(Clone)]
pub struct ChessBoard {
    get_cpu_move: fn(&Board) -> Move,
    starting_board: Board,
    result: GameResult,
    from_square: Option<Coord>,
    board: Board,
    squares: [button::State; 64],
}

impl Default for ChessBoard {
    fn default() -> Self {
        let x = GET_CPU_MOVE.lock().unwrap();
        let get_cpu_move = *x;
        let x = STARTING_BOARD.lock().unwrap();
        let starting_board = *x;
        let board = *x;
        Self {
            get_cpu_move,
            starting_board,
            result: GameResult::Continuing,
            from_square: None,
            board,
            squares: [button::State::default(); 64]
        }
    }
}

impl Sandbox for ChessBoard {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        match self.result {
            GameResult::Victory(color) => format!("{color} wins"),
            GameResult::Stalemate => "Stalemate".to_string(),
            GameResult::IllegalMove(m) => format!("Illegal move by {}, '({},{})'", self.board.turn, m.from.to_notation(), m.to.to_notation()),
            _ => String::from("Chess")
        }
    }

    fn update(&mut self, message: Message) {
        match self.result {
            GameResult::Victory(_) | GameResult::Stalemate => {
                self.board = self.starting_board;
                self.result = GameResult::Continuing;
            },
            _ => {
                match (self.from_square, message) {
                    (None, Message::SelectSquare(pos)) => {
                        self.from_square = Some(pos);
                    }
                    (Some(from), Message::SelectSquare(to)) if from != to => {
                        let m = Move { from, to, promote: None };
                        
                        self.from_square = None;
                        self.result = match self.board.do_move_from_coord(m) {
                            Ok(_) => {
                                if self.board.get_checkmate() {
                                    GameResult::Victory(self.board.turn)
                                } else if self.board.get_stalemate() {
                                    GameResult::Stalemate
                                } else {
                                    GameResult::Continuing
                                }
                            },
                            Err(_) => GameResult::IllegalMove(m)
                        };
                        match self.result {
                            GameResult::Continuing => {
                                let cpu_move = (self.get_cpu_move)(&self.board);
                                self.result = match self.board.do_move_from_coord(cpu_move) {
                                    Ok(_) => {
                                        if self.board.get_checkmate() {
                                            GameResult::Victory(self.board.turn)
                                        } else if self.board.get_stalemate() {
                                            GameResult::Stalemate
                                        } else {
                                            GameResult::Continuing
                                        }
                                    },
                                    Err(_) => GameResult::IllegalMove(cpu_move)
                                };
                            },
                            GameResult::Victory(_) | GameResult::Stalemate => {
                                self.board = self.starting_board;
                            }
                            _ => {}
                        }
                    },
                    (Some(_), Message::SelectSquare(to)) => {
                        self.from_square = Some(to);
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut result = Column::new().spacing(0).align_items(Align::Center);
        let mut row = Row::new().spacing(0).align_items(Align::Center);
        let mut i = 0;

        for button in &mut self.squares {        
            let c = i / 8;
            let r = i % 8;

            let pos = Coord { y: c, x: r };

            let (text, color) = if let Some(piece) = self.board.piece_at(pos) {
                (get_symbol(&piece).to_string(), piece.color)
            } else {
                (String::from(" "), PieceColor::White)
            };
            
            row = row.push(Button::new(button,
                    Text::new(text)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .size(SQUARE_SIZE)
                )
                .width(Length::Units(SQUARE_SIZE))
                .height(Length::Units(SQUARE_SIZE))
                .on_press(Message::SelectSquare(pos))
                .style(ChessSquare::from((pos, color, self.from_square == Some(pos))))
            );
            i += 1;

            if i % 8 == 0 {
                result = result.push(row);
                row = Row::new().spacing(0).align_items(Align::Center);
            }
        }
        
        Container::new(result)
            .style(ChessBoardStyle)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .padding(1)
            .into()
    }
}