use chers::{utils, Position};
use criterion::{black_box, criterion_group, Criterion};

fn search_kiwipete(c: &mut Criterion) {
    let mut pos = Position::from_fen(utils::fen::KIWIPETE).unwrap();

    c.bench_function("search 2 kiwipete", |b| b.iter(|| pos.search(black_box(2))));
}

criterion_group!(
    name = search_benches;
    config = Criterion::default().sample_size(10);
    targets = search_kiwipete
);
