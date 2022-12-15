use criterion::{criterion_group, criterion_main, Criterion};
use chess_engine::board::Board;
use chess_engine::engine::make_best_move;

fn test_all_possible_moves() {
    let board = Board::from_fen("r3kb1r/pppqppp1/2n1bn2/3p2Pp/4P3/P2B1N1P/1PPP1P2/RNBQK2R w KQkq h6 0 1").unwrap();
    board.list_all_valid_moves();
}

pub fn queen_capture() {
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
    }
}

pub fn engine(depth: u8, moves: u8) {
    let mut board = Board::default();
    for _ in 0..moves {
        let mv = make_best_move(depth, &board).unwrap();
        board.do_move_from_coord(mv.from, mv.to).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all_possible_moves", |b| b.iter(|| test_all_possible_moves()));
    c.bench_function("queen_capture", |b| b.iter(|| queen_capture()));
}

fn engine_benchmark(c: &mut Criterion) {
    c.bench_function("engine_d2_5mv", |b| b.iter(|| engine(2, 5)));
    // c.bench_function("engine_d3_5mv", |b| b.iter(|| engine(3, 5)));

}

criterion_group!(benches, criterion_benchmark, engine_benchmark);
criterion_main!(benches);