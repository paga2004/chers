use chers::{perft, Position};
use criterion::{black_box, criterion_group, Criterion};

fn perft_starting_postion(c: &mut Criterion) {
    let pos = Position::new();

    c.bench_function("perft 3 starting position", |b| {
        b.iter(|| perft(&pos, black_box(3)))
    });
}

fn perft_kiwipete(c: &mut Criterion) {
    let pos =
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();

    c.bench_function("perft 3 kiwipete", |b| b.iter(|| perft(&pos, black_box(3))));
}

criterion_group!(
    name = perft_benches;
    config = Criterion::default().sample_size(50);
    targets = perft_starting_postion, perft_kiwipete
);
