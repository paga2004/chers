use chers::{utils, BitMove, Position};
use criterion::{black_box, criterion_group, BatchSize, Criterion};

fn make_move_100(c: &mut Criterion) {
    let data: Vec<(Position, BitMove)> = utils::fen::RANDOM_FENS
        .iter()
        .map(|fen| Position::from_fen(fen).unwrap())
        .flat_map(|pos| {
            pos.generate_legal_moves()
                .into_iter()
                .map(move |m| (pos.clone(), m))
        })
        .collect();

    c.bench_function("make move 100", |b| {
        b.iter_batched(
            || data.clone(),
            |data| {
                for (mut pos, m) in data {
                    pos.make_bit_move(&m);
                    black_box(pos);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    name = make_move_benches;
    config = Criterion::default();
    targets = make_move_100
);
