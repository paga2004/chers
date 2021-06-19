use chers::{perft, utils, Position};
use criterion::{black_box, criterion_group, Criterion};

fn perft_starting_postion(c: &mut Criterion) {
    let pos = Position::new();

    c.bench_function("perft 3 starting position", |b| {
        b.iter(|| perft(&pos, black_box(3)))
    });
}

fn perft_kiwipete(c: &mut Criterion) {
    let pos = Position::from_fen(utils::fen::KIWIPETE).unwrap();

    c.bench_function("perft 3 kiwipete", |b| b.iter(|| perft(&pos, black_box(3))));
}

criterion_group!(
    name = perft_benches;
    config = Criterion::default().sample_size(50);
    targets = perft_starting_postion, perft_kiwipete
);
