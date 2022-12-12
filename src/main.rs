#![feature(test)]

mod board;
mod pieces;
mod test;
use std::io::Write;

fn main() {
    let mut b = board::Board::default();
    // let mut b = board::Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").unwrap();

    while true {
        b.pretty_print_board();
        let to_move = match b.turn {
            pieces::PieceColor::White => "White",
            pieces::PieceColor::Black => "Black",
        };
        println!("{} to move. Enter move or q to quit", to_move);
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "q" {
            break;
        }
        if let Ok((from, to)) = b.parse_move(input) {
            match b.do_move(&from, &to) {
                Ok(_) => {},
                Err(e) => {
                    println!("Invalid move: {}", e);
                    continue;
                }
            }
        } else {
            println!("Invalid move: valid format is e2e4");
            continue;
        }
    }
}

