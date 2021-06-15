use criterion::criterion_main;

mod make_move;
mod perft_benches;

criterion_main!(perft_benches::perft_benches, make_move::make_move_benches);
