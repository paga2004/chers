use criterion::criterion_main;

mod make_move;
mod perft;
mod search;

criterion_main!(
    perft::perft_benches,
    make_move::make_move_benches,
    search::search_benches
);
