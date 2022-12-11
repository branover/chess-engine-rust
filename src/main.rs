mod board;
mod pieces;

fn main() {
    // let b = board::Board::default();
    let mut b = board::Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").unwrap();
    // println!("{}", b);
    b.pretty_print_board();

    b.do_move("e1", "e4").unwrap();
    b.pretty_print_board();

    b.do_move("e4", "e1").unwrap();
    b.pretty_print_board()
}

