// use std::io::Write;
use chess_engine::{
    board::Board,
    // pieces::PieceColor,
    gui::{run, best_move},
};


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let b = Board::default();
//     // run(random_move, b)?;
//     run(best_move, b)?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut b = Board::default();
    run(best_move, b)?;
    Ok(())
}