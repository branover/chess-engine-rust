use criterion::{criterion_group, criterion_main, Criterion};
use chess_engine::board::Board;

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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all_possible_moves", |b| b.iter(|| test_all_possible_moves()));
    c.bench_function("queen_capture", |b| b.iter(|| queen_capture()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);