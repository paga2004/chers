use chers::{perft, Position};
use iai::black_box;

fn perft_starting_position_3() -> u64 {
    let pos = Position::new();
    perft(&pos, black_box(3))
}

fn perft_kiwipete_3() -> u64 {
    let pos =
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    perft(&pos, black_box(3))
}

iai::main!(perft_starting_position_3, perft_kiwipete_3);
