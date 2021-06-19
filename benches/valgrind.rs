use chers::{perft, utils, Position};
use iai::black_box;

fn perft_starting_position_3() -> u64 {
    let pos = Position::new();
    perft(&pos, black_box(3))
}

fn perft_kiwipete_3() -> u64 {
    let pos = Position::from_fen(utils::fen::KIWIPETE).unwrap();
    perft(&pos, black_box(3))
}

iai::main!(perft_starting_position_3, perft_kiwipete_3);
