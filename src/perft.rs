use crate::Position;

/// Counts the number of leaf nodes from generating moves to a certain depth.
pub fn perft(pos: &Position, depth: u16) -> u64 {
    match depth {
        0 => 1,
        1 => pos.generate_legal_moves().len() as u64,
        _ => {
            let mut count = 0;

            for m in pos.generate_legal_moves() {
                // TODO: get rid of this clone
                let mut new_pos = pos.clone();
                new_pos.make_bit_move(&m);
                count += perft(&new_pos, depth - 1);
            }
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::fen::STARTING_FEN;

    fn print_perft_results(pos: &Position, depth: u16) -> String {
        let mut result = String::new();
        if depth == 0 {
            return result;
        }
        for m in pos.generate_legal_moves() {
            let mut new_pos = pos.clone();
            new_pos.make_bit_move(&m);
            result.push_str(&format!("{}: {}\n", m, perft(&new_pos, depth - 1)));
        }
        result
    }

    const POS_1: &str = STARTING_FEN;
    const POS_2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"; // kiwipete
    const POS_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const POS_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const POS_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    // data from stockfish (`position fen <fen>\n go perft <depth>`) and https://www.chessprogramming.org/Perft_Results

    #[test_case(POS_1, 0,             1; "starting position 0")]
    #[test_case(POS_1, 1,            20; "starting position 1")]
    #[test_case(POS_1, 2,           400; "starting position 2")]
    #[test_case(POS_1, 3,         8_902; "starting position 3")]
    #[test_case(POS_1, 4,       197_281; "starting position 4")]
    #[test_case(POS_1, 5,     4_865_609; "starting position 5")]
    #[test_case(POS_1, 6,   119_060_324; "inconclusive -starting position 6")]
    #[test_case(POS_2, 1,            48; "kiwipete 1")]
    #[test_case(POS_2, 2,         2_039; "kiwipete 2")]
    #[test_case(POS_2, 3,        97_862; "kiwipete 3")]
    #[test_case(POS_2, 4,     4_085_603; "kiwipete 4")]
    #[test_case(POS_2, 5,   193_690_690; "kiwipete 5")]
    #[test_case(POS_2, 6, 8_031_647_685; "inconclusive - kiwipete 6")]
    #[test_case(POS_3, 1,            14; "position3 1")]
    #[test_case(POS_3, 2,           191; "position3 2")]
    #[test_case(POS_3, 3,         2_812; "position3 3")]
    #[test_case(POS_3, 4,        43_238; "position3 4")]
    #[test_case(POS_3, 5,       674_624; "position3 5")]
    #[test_case(POS_3, 6,    11_030_083; "inconclusive - position3 6")]
    #[test_case(POS_4, 1,             6; "position4 1")]
    #[test_case(POS_4, 2,           264; "position4 2")]
    #[test_case(POS_4, 3,         9_467; "position4 3")]
    #[test_case(POS_4, 4,       422_333; "position4 4")]
    #[test_case(POS_4, 5,    15_833_292; "position4 5")]
    #[test_case(POS_4, 6,   706_045_033; "inconclusive - position4 6")]
    #[test_case(POS_5, 1,            44; "position5 1")]
    #[test_case(POS_5, 2,         1_486; "position5 2")]
    #[test_case(POS_5, 3,        62_379; "position5 3")]
    #[test_case(POS_5, 4,     2_103_487; "position5 4")]
    #[test_case(POS_5, 5,    89_941_194; "position5 5")]
    #[test_case(POS_6, 1,            46; "position6 1")]
    #[test_case(POS_6, 2,         2_079; "position6 2")]
    #[test_case(POS_6, 3,        89_890; "position6 3")]
    #[test_case(POS_6, 4,     3_894_594; "position6 4")]
    #[test_case(POS_6, 5,   164_075_551; "position6 5")]
    #[test_case(POS_6, 6, 6_923_051_137; "inconclusive - position6 6")]
    // There was a bug in the following positions on commit 31459f2b8cee5d4ab8fd1d3152d1ca432b7df125.
    #[test_case("r3k2r/p1ppqNb1/bn2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1",    3,    88_799; "bug 2.1")]
    #[test_case("r3k2r/p1ppqNb1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 2",  2,     2_084; "bug 2.2")]
    #[test_case("r3k2r/p1ppqNb1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/1R2K2R b Kkq - 2 2",  4, 4_056_978; "bug 3")]
    #[test_case("r3k2r/2ppqNb1/1n2pnp1/pb1P4/1p2P3/2N2Q1p/PPPBBPPP/1R2K2R w Kkq - 0 3",   3,    92_182; "bug 3.1")]
    #[test_case("r3k2r/2ppqNb1/1n2pnp1/pb1P4/1p2P3/2N2Q1p/PPPBBPPP/1R3RK1 b kq - 1 3",    2,     2_097; "bug 3.2")]
    #[test_case("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1PPBBPPP/R3K2R b KQkq - 0 1",  4, 4_627_439; "bug 4")]
    #[test_case("r3k2r/p1ppqpb1/1n2pnp1/3PN3/1pb1P3/P1N2Q1p/1PPBBPPP/R3K2R w KQkq - 1 2", 3,   104_588; "bug 4.1")]
    #[test_case("r3k2r/p1ppqpb1/1n2pnp1/3PN3/Ppb1P3/2N2Q1p/1PPBBPPP/R3K2R b KQkq - 0 2",  2,     2_122; "bug 4.2")]
    fn test_perft(fen: &str, depth: u16, expected: u64) {
        let pos = Position::from_fen(fen).expect("valid position");
        let result = perft(&pos, depth);
        if result != expected {
            panic!(
                "Perft did not return the expected results!\
                \nresult {}, expected: {}, difference: {}\
                \n\nDetailed results:\n{}",
                result,
                expected,
                result as i128 - expected as i128,
                print_perft_results(&pos, depth)
            );
        }
    }
}
