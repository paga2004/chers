use criterion::criterion_main;

mod perft_benches;

criterion_main!(perft_benches::perft_benches);
