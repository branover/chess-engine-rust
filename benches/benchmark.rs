use criterion::{criterion_group, criterion_main, Criterion};
use chess_engine::board::Board;

fn test_all_possible_moves() {
    let board = Board::from_fen("r3kb1r/pppqppp1/2n1bn2/3p2Pp/4P3/P2B1N1P/1PPP1P2/RNBQK2R w KQkq h6 0 1").unwrap();
    board.list_all_valid_moves();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all_possible_moves", |b| b.iter(|| test_all_possible_moves()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);