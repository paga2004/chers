use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

#[cfg(test)]
type Decoder = BufReader<zstd::Decoder<'static, BufReader<File>>>;

#[derive(Debug)]
pub struct FenIterator<R>(BufReader<R>);

impl<R: std::io::Read> FenIterator<R> {
    pub fn new(reader: R) -> Self {
        Self(BufReader::new(reader))
    }
}

impl FenIterator<File> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = File::open(path)?;
        Ok(Self::new(f))
    }
}

#[cfg(test)]
impl FenIterator<Decoder> {
    pub fn from_zstd_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let d = zstd::Decoder::new(f)?;
        Ok(Self::new(BufReader::new(d)))
    }
}

impl<R: std::io::Read> Iterator for FenIterator<R> {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = String::new();
        match self.0.read_line(&mut s) {
            Ok(0) => None,
            Ok(_) => Some(Ok(s)),
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
pub fn random_fens() -> FenIterator<Decoder> {
    FenIterator::from_zstd_file("/data/archives/datasets/chess/sorted.fen.zst").unwrap()
}

pub static RANDOM_FENS: [&str; 85] = [
    "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
    "rnbqkbnr/ppp1pppp/8/3p4/8/5N2/PPPPPPPP/RNBQKB1R w KQkq d6 0 2",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq d3 0 2",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 3",
    "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq c3 0 3",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 4",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 1 4",
    "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP4/5N2/PP1NPPPP/R1BQKB1R w KQkq - 2 5",
    "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP4/4PN2/PP1N1PPP/R1BQKB1R b KQkq - 0 5",
    "rnbqkb1r/pp3ppp/4pn2/2pp4/2PP4/4PN2/PP1N1PPP/R1BQKB1R w KQkq - 0 6",
    "rnbqkb1r/pp3ppp/4pn2/2pp4/2PP4/1P2PN2/P2N1PPP/R1BQKB1R b KQkq - 0 6",
    "r1bqkb1r/pp3ppp/2n1pn2/2pp4/2PP4/1P2PN2/P2N1PPP/R1BQKB1R w KQkq - 1 7",
    "r1bqkb1r/pp3ppp/2n1pn2/2pp4/2PP4/1P2PN2/PB1N1PPP/R2QKB1R b KQkq - 2 7",
    "r1bqkb1r/pp3ppp/2n1pn2/3p4/2Pp4/1P2PN2/PB1N1PPP/R2QKB1R w KQkq - 0 8",
    "r1bqkb1r/pp3ppp/2n1pn2/3p4/2PP4/1P3N2/PB1N1PPP/R2QKB1R b KQkq - 0 8",
    "r1bqk2r/pp2bppp/2n1pn2/3p4/2PP4/1P3N2/PB1N1PPP/R2QKB1R w KQkq - 1 9",
    "r1bqk2r/pp2bppp/2n1pn2/3p4/2PP4/1P3N2/PB1N1PPP/2RQKB1R b Kkq - 2 9",
    "r1bq1rk1/pp2bppp/2n1pn2/3p4/2PP4/1P3N2/PB1N1PPP/2RQKB1R w K - 3 10",
    "r1bq1rk1/pp2bppp/2n1pn2/3p4/2PP4/1P1B1N2/PB1N1PPP/2RQK2R b K - 4 10",
    "r2q1rk1/pp1bbppp/2n1pn2/3p4/2PP4/1P1B1N2/PB1N1PPP/2RQK2R w K - 5 11",
    "r2q1rk1/pp1bbppp/2n1pn2/3p4/2PP4/1P1B1N2/PB1N1PPP/2RQ1RK1 b - - 6 11",
    "r2q1rk1/pp1bbppp/2n1p3/3p3n/2PP4/1P1B1N2/PB1N1PPP/2RQ1RK1 w - - 7 12",
    "r2q1rk1/pp1bbppp/2n1p3/3p3n/2PP4/1P1B1N2/PB1N1PPP/2RQR1K1 b - - 8 12",
    "r2q1rk1/pp1bbppp/2n1p3/3p4/2PP1n2/1P1B1N2/PB1N1PPP/2RQR1K1 w - - 9 13",
    "r2q1rk1/pp1bbppp/2n1p3/3p4/2PP1n2/1P3N2/PB1N1PPP/1BRQR1K1 b - - 10 13",
    "r2q1rk1/pp1b1ppp/2nbp3/3p4/2PP1n2/1P3N2/PB1N1PPP/1BRQR1K1 w - - 11 14",
    "r2q1rk1/pp1b1ppp/2nbp3/3p4/2PP1n2/1P3NP1/PB1N1P1P/1BRQR1K1 b - - 0 14",
    "r2q1rk1/pp1b1ppp/2nbp1n1/3p4/2PP4/1P3NP1/PB1N1P1P/1BRQR1K1 w - - 1 15",
    "r2q1rk1/pp1b1ppp/2nbp1n1/3pN3/2PP4/1P4P1/PB1N1P1P/1BRQR1K1 b - - 2 15",
    "2rq1rk1/pp1b1ppp/2nbp1n1/3pN3/2PP4/1P4P1/PB1N1P1P/1BRQR1K1 w - - 3 16",
    "2rq1rk1/pp1N1ppp/2nbp1n1/3p4/2PP4/1P4P1/PB1N1P1P/1BRQR1K1 b - - 0 16",
    "2r2rk1/pp1q1ppp/2nbp1n1/3p4/2PP4/1P4P1/PB1N1P1P/1BRQR1K1 w - - 0 17",
    "2r2rk1/pp1q1ppp/2nbp1n1/3p4/2PP4/1P3NP1/PB3P1P/1BRQR1K1 b - - 1 17",
    "2r2rk1/pp1q1ppp/2n1p1n1/3p4/1bPP4/1P3NP1/PB3P1P/1BRQR1K1 w - - 2 18",
    "2r2rk1/pp1q1ppp/2n1p1n1/3p4/1bPP4/1P2RNP1/PB3P1P/1BRQ2K1 b - - 3 18",
    "2rr2k1/pp1q1ppp/2n1p1n1/3p4/1bPP4/1P2RNP1/PB3P1P/1BRQ2K1 w - - 4 19",
    "2rr2k1/pp1q1ppp/2n1p1n1/3p4/1bPP3P/1P2RNP1/PB3P2/1BRQ2K1 b - h3 0 19",
    "2rr2k1/pp1qnppp/2n1p3/3p4/1bPP3P/1P2RNP1/PB3P2/1BRQ2K1 w - - 1 20",
    "2rr2k1/pp1qnppp/2n1p3/3p4/1bPP3P/PP2RNP1/1B3P2/1BRQ2K1 b - - 0 20",
    "2rr2k1/pp1qnppp/2n1p3/b2p4/2PP3P/PP2RNP1/1B3P2/1BRQ2K1 w - - 1 21",
    "2rr2k1/pp1qnppp/2n1p3/b2p4/1PPP3P/P3RNP1/1B3P2/1BRQ2K1 b - - 0 21",
    "2rr2k1/ppbqnppp/2n1p3/3p4/1PPP3P/P3RNP1/1B3P2/1BRQ2K1 w - - 1 22",
    "2rr2k1/ppbqnppp/2n1p3/2Pp4/1P1P3P/P3RNP1/1B3P2/1BRQ2K1 b - - 0 22",
    "2r1r1k1/ppbqnppp/2n1p3/2Pp4/1P1P3P/P3RNP1/1B3P2/1BRQ2K1 w - - 1 23",
    "2r1r1k1/ppbqnppp/2n1p3/2Pp4/1P1P3P/P2QRNP1/1B3P2/1BR3K1 b - - 2 23",
    "2r1r1k1/ppbqnp1p/2n1p1p1/2Pp4/1P1P3P/P2QRNP1/1B3P2/1BR3K1 w - - 0 24",
    "2r1r1k1/ppbqnp1p/2n1p1p1/2Pp4/1P1P3P/P2Q1NP1/1B2RP2/1BR3K1 b - - 1 24",
    "2r1r1k1/ppbq1p1p/2n1p1p1/2Pp1n2/1P1P3P/P2Q1NP1/1B2RP2/1BR3K1 w - - 2 25",
    "2r1r1k1/ppbq1p1p/2n1p1p1/2Pp1n2/1P1P3P/P1BQ1NP1/4RP2/1BR3K1 b - - 3 25",
    "2r1r1k1/ppbq1p2/2n1p1p1/2Pp1n1p/1P1P3P/P1BQ1NP1/4RP2/1BR3K1 w - h6 0 26",
    "2r1r1k1/ppbq1p2/2n1p1p1/1PPp1n1p/3P3P/P1BQ1NP1/4RP2/1BR3K1 b - - 0 26",
    "2r1r1k1/ppbqnp2/4p1p1/1PPp1n1p/3P3P/P1BQ1NP1/4RP2/1BR3K1 w - - 1 27",
    "2r1r1k1/ppbqnp2/4p1p1/1PPp1n1p/3P3P/P2Q1NP1/3BRP2/1BR3K1 b - - 2 27",
    "2r1r3/ppbqnpk1/4p1p1/1PPp1n1p/3P3P/P2Q1NP1/3BRP2/1BR3K1 w - - 3 28",
    "2r1r3/ppbqnpk1/4p1p1/1PPp1n1p/P2P3P/3Q1NP1/3BRP2/1BR3K1 b - - 0 28",
    "r3r3/ppbqnpk1/4p1p1/1PPp1n1p/P2P3P/3Q1NP1/3BRP2/1BR3K1 w - - 1 29",
    "r3r3/ppbqnpk1/4p1p1/PPPp1n1p/3P3P/3Q1NP1/3BRP2/1BR3K1 b - - 0 29",
    "r3r3/1pbqnpk1/p3p1p1/PPPp1n1p/3P3P/3Q1NP1/3BRP2/1BR3K1 w - - 0 30",
    "r3r3/1pbqnpk1/pP2p1p1/P1Pp1n1p/3P3P/3Q1NP1/3BRP2/1BR3K1 b - - 0 30",
    "rb2r3/1p1qnpk1/pP2p1p1/P1Pp1n1p/3P3P/3Q1NP1/3BRP2/1BR3K1 w - - 1 31",
    "rb2r3/1p1qnpk1/pP2p1p1/P1Pp1n1p/3P3P/3Q1NP1/2BBRP2/2R3K1 b - - 2 31",
    "rb2r3/1p1q1pk1/pPn1p1p1/P1Pp1n1p/3P3P/3Q1NP1/2BBRP2/2R3K1 w - - 3 32",
    "rb2r3/1p1q1pk1/pPn1p1p1/P1Pp1n1p/B2P3P/3Q1NP1/3BRP2/2R3K1 b - - 4 32",
    "rb6/1p1qrpk1/pPn1p1p1/P1Pp1n1p/B2P3P/3Q1NP1/3BRP2/2R3K1 w - - 5 33",
    "rb6/1p1qrpk1/pPn1p1p1/P1Pp1n1p/B2P3P/2BQ1NP1/4RP2/2R3K1 b - - 6 33",
    "rb6/1p1qrpk1/pP2p1p1/P1Ppnn1p/B2P3P/2BQ1NP1/4RP2/2R3K1 w - - 7 34",
    "rb6/1p1qrpk1/pP2p1p1/P1PpPn1p/B6P/2BQ1NP1/4RP2/2R3K1 b - - 0 34",
    "rb6/1p2rpk1/pP2p1p1/P1PpPn1p/q6P/2BQ1NP1/4RP2/2R3K1 w - - 0 35",
    "rb6/1p2rpk1/pP2p1p1/P1PpPn1p/q2N3P/2BQ2P1/4RP2/2R3K1 b - - 1 35",
    "rb6/1p2rpk1/pP2p1p1/P1PpP2p/q2n3P/2BQ2P1/4RP2/2R3K1 w - - 0 36",
    "rb6/1p2rpk1/pP2p1p1/P1PpP2p/q2Q3P/2B3P1/4RP2/2R3K1 b - - 0 36",
    "rb6/1p1qrpk1/pP2p1p1/P1PpP2p/3Q3P/2B3P1/4RP2/2R3K1 w - - 1 37",
    "rb6/1p1qrpk1/pP2p1p1/P1PpP2p/3Q3P/6P1/3BRP2/2R3K1 b - - 2 37",
    "rb2r3/1p1q1pk1/pP2p1p1/P1PpP2p/3Q3P/6P1/3BRP2/2R3K1 w - - 3 38",
    "rb2r3/1p1q1pk1/pP2p1p1/P1PpP1Bp/3Q3P/6P1/4RP2/2R3K1 b - - 4 38",
    "rbr5/1p1q1pk1/pP2p1p1/P1PpP1Bp/3Q3P/6P1/4RP2/2R3K1 w - - 5 39",
    "rbr5/1p1q1pk1/pP2pBp1/P1PpP2p/3Q3P/6P1/4RP2/2R3K1 b - - 6 39",
    "rbr5/1p1q1p1k/pP2pBp1/P1PpP2p/3Q3P/6P1/4RP2/2R3K1 w - - 7 40",
    "rbr5/1p1q1p1k/pPP1pBp1/P2pP2p/3Q3P/6P1/4RP2/2R3K1 b - - 0 40",
    "rbr5/3q1p1k/pPp1pBp1/P2pP2p/3Q3P/6P1/4RP2/2R3K1 w - - 0 41",
    "rbr5/3q1p1k/pPp1pBp1/P1QpP2p/7P/6P1/4RP2/2R3K1 b - - 1 41",
    "rbr5/3q1p2/pPp1pBpk/P1QpP2p/7P/6P1/4RP2/2R3K1 w - - 2 42",
    "rbr5/3q1p2/pPp1pBpk/P1QpP2p/7P/6P1/1R3P2/2R3K1 b - - 3 42",
    "rbr5/1q3p2/pPp1pBpk/P1QpP2p/7P/6P1/1R3P2/2R3K1 w - - 4 43",
    "rbr5/1q3p2/pPp1pBpk/P1QpP2p/1R5P/6P1/5P2/2R3K1 b - - 5 43",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Position;

    #[test]
    fn random_fens_test() {
        let iter = random_fens();
        let mut count = 0;
        iter.for_each(|s| {
            let fen = s.unwrap();
            let pos = Position::from_fen(&fen);
            assert!(pos.is_ok(), "{}", fen);
            count += 1;
        });
    }
}
