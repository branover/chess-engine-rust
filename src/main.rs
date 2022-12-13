// use std::io::Write;
use chess_engine::{
    board::Board,
    // pieces::PieceColor,
    gui::{run, best_move},
    // engine::make_best_move
};


// fn main() {
//     let mut b = Board::default();
//     // let mut b = board::Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").unwrap();

//     loop {
//         b.pretty_print_board();
//         let to_move = match b.turn {
//             PieceColor::White => "White",
//             PieceColor::Black => "Black",
//         };
//         println!("{} to move. Enter move or q to quit", to_move);
//         print!(">> ");
//         std::io::stdout().flush().unwrap();
//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();
//         let input = input.trim();
//         if input == "q" {
//             break;
//         }
//         if let Ok((from, to)) = b.parse_move(input) {
//             match b.do_move(&from, &to) {
//                 Ok(_) => {},
//                 Err(e) => {
//                     println!("Invalid move: {}", e);
//                     continue;
//                 }
//             }
//             if b.get_checkmate() {
//                 println!("Checkmate! Winner is {:?}", b.turn.opposite());
//                 break;
//             } else if b.get_stalemate() {
//                 println!("Stalemate!");
//                 break;
//             }
//         } else {
//             println!("Invalid move: valid format is e2e4");
//             continue;
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut b = Board::default();

    // run(random_move, b)?;
    run(best_move, b)?;
    Ok(())
}