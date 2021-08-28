use std::fmt;
use std::sync::Arc;

use crate::utils;
use crate::BitMove;
use crate::Color;
use crate::File;
use crate::ParsedMove;
use crate::Piece;
use crate::PieceType;
use crate::PositionState;
use crate::Rank;
use crate::Square;

pub(crate) const WHITE_PAWN_OFFSET: i8 = 10;
pub(crate) const BLACK_PAWN_OFFSET: i8 = -10;
pub(crate) const WHITE_PAWN_CAPTURE_OFFSETS: [i8; 2] = [9, 11];
pub(crate) const BLACK_PAWN_CAPTURE_OFFSETS: [i8; 2] = [-9, -11];
pub(crate) const KNIGHT_OFFSETS: [i8; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
pub(crate) const BISHOP_OFFSETS: [i8; 4] = [-11, -9, 9, 11];
pub(crate) const ROOK_OFFSETS: [i8; 4] = [-10, -1, 1, 10];
pub(crate) const KING_OFFSETS: [i8; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

// TODO: remove this
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum BoardState {
    OffBoard,
    Empty,
    Piece(Piece),
}

/// A chess position.
#[derive(Clone)]
pub struct Position {
    pub(crate) pieces: [BoardState; 120],
    pub(crate) king_square: [Square; 2],
    pub(crate) side_to_move: Color,
    pub(crate) ply: u16,

    pub(crate) state: Arc<PositionState>,
}

impl Position {
    /// Creates a new position that represents the starting position.
    pub fn new() -> Self {
        Self::from_fen(utils::fen::STARTING_POSITION).unwrap()
    }

    /// Makes a move on the current position.
    ///
    /// If the move is illegal `false` will be returned and the position is left unchanged.
    /// Otherwise `true` will be returned.
    pub fn make_move(&mut self, m: ParsedMove) -> bool {
        // TODO: better to pass by refrence?
        let legal_moves = self.generate_legal_moves();
        if let Some(bit_move) = legal_moves.iter().find(|bm| *bm == &m) {
            self.make_bit_move(bit_move);
            true
        } else {
            false
        }
    }

    /// Makes a move on the current position.
    ///
    /// # Saftey
    ///
    /// This should only be called if the move is legal. For a safer function see
    /// [`Position::make_move`], which takes a [`ParsedMove`] instead.
    pub fn make_bit_move(&mut self, m: &BitMove) {
        let state = &self.state;
        if let BoardState::Piece(p) = self.pieces[m.origin()] {
            self.side_to_move = !self.side_to_move;
            self.ply += 1;
            let halfmove_clock = if m.is_capture() || p.is_type(PieceType::Pawn) {
                0
            } else {
                state.halfmove_clock + 1
            };
            let mut castling_rights = state.castling_rights;
            let ep_square = if m.is_double_push() {
                Some(Square::new(
                    m.target().file(),
                    p.color.map(Rank::Third, Rank::Sixth),
                ))
            } else {
                None
            };

            // en passent
            let capture_field = if m.is_en_passant() {
                if p.color == Color::White {
                    Square::new(m.target().file(), m.target().rank() - 1)
                } else {
                    Square::new(m.target().file(), m.target().rank() + 1)
                }
            } else {
                m.target()
            };

            // TODO: simplify
            let captured_piece = if m.is_capture() {
                if let BoardState::Piece(p) = self.pieces[capture_field] {
                    Some(p)
                } else {
                    None
                }
            } else {
                None
            };

            // promotion
            let piece = if m.is_promotion() {
                Piece::new(m.promotion_piece(), p.color)
            } else {
                p
            };

            // castling rights
            // TODO: Use a castling mask for this
            let mut remove_castling_rights = |sq| match sq {
                Square::A1 => {
                    castling_rights.white_queen_side = false;
                }
                Square::E1 => {
                    castling_rights.white_queen_side = false;
                    castling_rights.white_king_side = false;
                }
                Square::H1 => {
                    castling_rights.white_king_side = false;
                }

                Square::A8 => {
                    castling_rights.black_queen_side = false;
                }
                Square::E8 => {
                    castling_rights.black_queen_side = false;
                    castling_rights.black_king_side = false;
                }
                Square::H8 => {
                    castling_rights.black_king_side = false;
                }

                _ => {}
            };
            remove_castling_rights(m.origin());
            remove_castling_rights(m.target());

            self.state = Arc::new(PositionState {
                castling_rights,
                ep_square,
                halfmove_clock,
                prev_move: Some(*m),
                captured_piece,
                prev_state: Some(state.clone()),
            });

            if m.origin() == self.king_square[!self.side_to_move as usize] {
                self.king_square[!self.side_to_move as usize] = m.target();
            }
            // white castling
            match p.color {
                Color::White => {
                    if m.is_king_side_castle() {
                        self.pieces[Square::F1] = self.pieces[Square::H1];
                        self.pieces[Square::G1] = BoardState::Piece(p);
                        self.pieces[Square::E1] = BoardState::Empty;
                        self.pieces[Square::H1] = BoardState::Empty;
                        return;
                    }
                    if m.is_queen_side_castle() {
                        self.pieces[Square::D1] = self.pieces[Square::A1];
                        self.pieces[Square::C1] = BoardState::Piece(p);
                        self.pieces[Square::E1] = BoardState::Empty;
                        self.pieces[Square::A1] = BoardState::Empty;
                        return;
                    }
                }
                Color::Black => {
                    if m.is_king_side_castle() {
                        self.pieces[Square::F8] = self.pieces[Square::H8];
                        self.pieces[Square::G8] = BoardState::Piece(p);
                        self.pieces[Square::E8] = BoardState::Empty;
                        self.pieces[Square::H8] = BoardState::Empty;
                        return;
                    }
                    if m.is_queen_side_castle() {
                        self.pieces[Square::D8] = self.pieces[Square::A8];
                        self.pieces[Square::C8] = BoardState::Piece(p);
                        self.pieces[Square::E8] = BoardState::Empty;
                        self.pieces[Square::A8] = BoardState::Empty;
                        return;
                    }
                }
            }

            // normal move
            self.pieces[capture_field] = BoardState::Empty;
            self.pieces[m.target()] = BoardState::Piece(piece);
            self.pieces[m.origin()] = BoardState::Empty;
        }
    }

    /// Undoes the last played move.
    ///
    /// # Panics
    ///
    /// Panics if no move has been played yet.
    pub fn undo_move(&mut self) {
        self.side_to_move = !self.side_to_move;
        self.ply -= 1;
        let m = self.state.prev_move.unwrap();
        if let BoardState::Piece(p) = self.pieces[m.target()] {
            let capture_field = if m.is_en_passant() {
                if self.side_to_move == Color::White {
                    Square::new(m.target().file(), m.target().rank() - 1)
                } else {
                    Square::new(m.target().file(), m.target().rank() + 1)
                }
            } else {
                m.target()
            };

            let piece = if m.is_promotion() {
                Piece::new(PieceType::Pawn, p.color)
            } else {
                p
            };
            let captured_piece = match self.state.captured_piece {
                Some(p) => BoardState::Piece(p),
                None => BoardState::Empty,
            };
            if m.target() == self.king_square[self.side_to_move as usize] {
                self.king_square[self.side_to_move as usize] = m.origin();
            }

            self.state = self.state.prev_state.as_ref().unwrap().clone();

            // castling
            match p.color {
                Color::White => {
                    if m.is_king_side_castle() {
                        self.pieces[Square::H1] = self.pieces[Square::F1];
                        self.pieces[Square::E1] = BoardState::Piece(p);
                        self.pieces[Square::F1] = BoardState::Empty;
                        self.pieces[Square::G1] = BoardState::Empty;
                        return;
                    }
                    if m.is_queen_side_castle() {
                        self.pieces[Square::A1] = self.pieces[Square::D1];
                        self.pieces[Square::E1] = BoardState::Piece(p);
                        self.pieces[Square::C1] = BoardState::Empty;
                        self.pieces[Square::D1] = BoardState::Empty;
                        return;
                    }
                }
                Color::Black => {
                    if m.is_king_side_castle() {
                        self.pieces[Square::H8] = self.pieces[Square::F8];
                        self.pieces[Square::E8] = BoardState::Piece(p);
                        self.pieces[Square::F8] = BoardState::Empty;
                        self.pieces[Square::G8] = BoardState::Empty;
                        return;
                    }
                    if m.is_queen_side_castle() {
                        self.pieces[Square::A8] = self.pieces[Square::D8];
                        self.pieces[Square::E8] = BoardState::Piece(p);
                        self.pieces[Square::C8] = BoardState::Empty;
                        self.pieces[Square::D8] = BoardState::Empty;
                        return;
                    }
                }
            }
            self.pieces[m.target()] = BoardState::Empty;
            self.pieces[m.origin()] = BoardState::Piece(piece);
            self.pieces[capture_field] = captured_piece;
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        let state = &self.state;
        let other_state = &other.state;

        self.pieces == other.pieces
            && self.side_to_move == other.side_to_move
            && self.ply == other.ply
            && state == other_state
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = &self.state;
        // print flags
        writeln!(f)?;
        writeln!(f, "Active color: {}", self.side_to_move)?;
        writeln!(f, "Castling rights: {}", state.castling_rights)?;
        write!(f, "En passant: ")?;
        if let Some(s) = state.ep_square {
            writeln!(f, "{}", s)?;
        } else {
            writeln!(f, "-")?;
        }
        writeln!(f, "Halfmove clock: {}", state.halfmove_clock)?;
        writeln!(f, "Ply: {}", self.ply)?;
        writeln!(f)?;

        // print board
        writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐")?;
        for i in (0..8).rev() {
            let rank = Rank::new(i);
            write!(f, "{} │", i + 1)?;
            for j in 0..8 {
                let file = File::new(j);
                write!(f, " ")?;
                if let BoardState::Piece(piece) = self.pieces[Square::new(file, rank)] {
                    write!(f, "{}", piece)?;
                } else {
                    write!(f, " ")?;
                }
                write!(f, " │")?;
            }
            if i > 0 {
                writeln!(f, "\n  ├───┼───┼───┼───┼───┼───┼───┼───┤")?;
            } else {
                writeln!(f, "\n  └───┴───┴───┴───┴───┴───┴───┴───┘")?;
            }
        }

        // print letters
        write!(f, " ")?;
        for i in 0..8 {
            write!(f, "   {}", (97u8 + i) as char)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    use test_case::test_case;

    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "e2e4", "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"; "e2e4")]
    #[test_case("rnbqkbnr/pppppppp/8/8/4p3/8/pppp1ppp/rnbqkbnr b kqkq e3 0 1", "c7c5", "rnbqkbnr/pp1ppppp/8/2p5/4p3/8/pppp1ppp/rnbqkbnr w kqkq c6 0 2"; "c7c5")]
    #[test_case("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2", "e4d5", "rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2"; "capture")]
    #[test_case("rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3", "e5d6", "rnbqkbnr/1pp1pppp/p2P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3"; "en passant white")]
    #[test_case("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4", "e1g1", "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4"; "kingside castling white")]
    #[test_case("r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/R3KBNR w KQkq - 5 6", "e1c1", "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/2KR1BNR b kq - 6 6"; "queenside castling white")]
    #[test_case("rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 4", "e8g8", "rnbq1rk1/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R w KQ - 1 5"; "kingside castling black")]
    #[test_case("r3kbnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R b KQkq - 6 5", "e8c8", "2kr1bnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R w KQ - 7 6"; "queenside castling black")]
    #[test_case("8/8/2k5/4K3/8/8/4p3/8 b - - 0 90", "e2e1Q", "8/8/2k5/4K3/8/8/8/4q3 w - - 0 91"; "promotion black")]
    #[test_case("5b2/6P1/2k5/4K3/3p4/3B4/8/8 w - - 3 92", "g7f8Q", "5Q2/8/2k5/4K3/3p4/3B4/8/8 b - - 0 92"; "promotion with capture")]
    #[test_case("8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8N", "5N2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85"; "promtotion to knight")]
    #[test_case("8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8B", "5B2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85"; "promotion to bishop")]
    #[test_case("8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8R", "5R2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85"; "promotion to rook")]
    // There was a bug in these positions on commit 31459f2b8cee5d4ab8fd1d3152d1ca432b7df125.
    #[test_case( "r3k2r/p1ppqNb1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 2", "f7h8", "r3k2N/p1ppq1b1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 2"; "bug 2.4")]
    #[test_case("r3k2r/2ppqNb1/1n2pnp1/pb1P4/1p2P3/2N2Q1p/PPPBBPPP/1R2K2R w Kkq - 0 3", "e1g1", "r3k2r/2ppqNb1/1n2pnp1/pb1P4/1p2P3/2N2Q1p/PPPBBPPP/1R3RK1 b kq - 1 3"; "bug 3.3")]
    fn test_position_make_move(pos: &str, m: &str, expected: &str) {
        let mut pos = Position::from_fen(pos).expect("valid position");
        let m = ParsedMove::from_coordinate_notation(m).expect("valid move");
        let expected = Position::from_fen(expected).expect("valid position");

        assert!(pos.make_move(m));
        pretty_assertions::assert_eq!(pos, expected);
    }

    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "e2e4"; "e2e4")]
    #[test_case("rnbqkbnr/pppppppp/8/8/4p3/8/pppp1ppp/rnbqkbnr b kqkq e3 0 1", "c7c5"; "c7c5")]
    #[test_case("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2", "e4d5"; "capture")]
    #[test_case("rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3", "e5d6"; "en passant white")]
    #[test_case("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4", "e1g1"; "kingside castling white")]
    #[test_case("r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/R3KBNR w KQkq - 5 6", "e1c1"; "queenside castling white")]
    #[test_case("rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 4", "e8g8"; "kingside castling black")]
    #[test_case("r3kbnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R b KQkq - 6 5", "e8c8"; "queenside castling black")]
    #[test_case("8/8/2k5/4K3/8/8/4p3/8 b - - 0 90", "e2e1Q"; "promotion black")]
    #[test_case("5b2/6P1/2k5/4K3/3p4/3B4/8/8 w - - 3 92", "g7f8Q"; "promotion with capture")]
    #[test_case("8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8N"; "promtotion to knight")]
    #[test_case("8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8B"; "promotion to bishop")]
    #[test_case("8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8R"; "promotion to rook")]
    fn test_position_undo_move(pos: &str, m: &str) {
        let expected = Position::from_fen(pos).unwrap();
        let mut pos = expected.clone();
        let m = ParsedMove::from_coordinate_notation(m).unwrap();

        assert!(pos.make_move(m));
        pos.undo_move();
        pretty_assertions::assert_eq!(pos, expected);
    }

    #[test]
    fn test_position_display() {
        let expected = r"
Active color: white
Castling rights: KQkq
En passant: -
Halfmove clock: 0
Ply: 1

  ┌───┬───┬───┬───┬───┬───┬───┬───┐
8 │ r │ n │ b │ q │ k │ b │ n │ r │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
7 │ p │ p │ p │ p │ p │ p │ p │ p │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
6 │   │   │   │   │   │   │   │   │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
5 │   │   │   │   │   │   │   │   │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
4 │   │   │   │   │   │   │   │   │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
3 │   │   │   │   │   │   │   │   │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
2 │ P │ P │ P │ P │ P │ P │ P │ P │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
1 │ R │ N │ B │ Q │ K │ B │ N │ R │
  └───┴───┴───┴───┴───┴───┴───┴───┘
    a   b   c   d   e   f   g   h";
        assert_eq!(format!("{}", Position::new()), expected);
    }
}
