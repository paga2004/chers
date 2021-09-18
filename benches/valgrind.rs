use chers::{perft, utils, BitMove, Position};
use iai::black_box;

fn perft_starting_position_3() -> u64 {
    let mut pos = Position::new();
    perft(&mut pos, black_box(3))
}

fn perft_kiwipete_3() -> u64 {
    let mut pos = Position::from_fen(utils::fen::KIWIPETE).unwrap();
    perft(&mut pos, black_box(3))
}

fn search_kiwipete_3() -> BitMove {
    let mut pos = Position::from_fen(utils::fen::KIWIPETE).unwrap();
    pos.search(black_box(3))
}

iai::main!(
    perft_starting_position_3,
    perft_kiwipete_3,
    search_kiwipete_3
);
