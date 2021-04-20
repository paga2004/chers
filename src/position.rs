use crate::castling_rights::CastlingRights;
use crate::fen;
use crate::Color;
use crate::File;
use crate::Move;
use crate::Piece;
use crate::PieceType;
use crate::Rank;
use crate::Square;
use std::fmt;

pub(crate) const WHITE_PAWN_OFFSET: i8 = 10;
pub(crate) const BLACK_PAWN_OFFSET: i8 = -10;
pub(crate) const WHITE_PAWN_CAPTURE_OFFSETS: [i8; 2] = [9, 11];
pub(crate) const BLACK_PAWN_CAPTURE_OFFSETS: [i8; 2] = [-9, -11];
pub(crate) const KNIGHT_OFFSETS: [i8; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
pub(crate) const BISHOP_OFFSETS: [i8; 4] = [-11, -9, 9, 11];
pub(crate) const ROOK_OFFSETS: [i8; 4] = [-10, -1, 1, 10];
pub(crate) const KING_OFFSETS: [i8; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum BoardState {
    OffBoard,
    Empty,
    Piece(Piece),
}

/// Represents a chess position.
#[allow(missing_copy_implementations)] // copying a position is expensive and should be avoided
#[derive(PartialEq, Clone)]
pub struct Position {
    pub(crate) pieces: [BoardState; 120],
    pub(crate) side_to_move: Color,
    pub(crate) castling_rights: CastlingRights,
    pub(crate) en_passant_square: Option<Square>,
}

impl Position {
    /// Returns the starting position.
    pub fn new() -> Self {
        Self::from_fen(fen::STARTING_FEN).unwrap()
    }

    /// Makes a move on the current position.
    ///
    /// This function does not check whether the move is legal.
    pub fn make_move(&mut self, m: &Move) {
        if let BoardState::Piece(p) = self.pieces[m.from] {
            self.side_to_move = !self.side_to_move;

            // en passant square
            self.en_passant_square = None;
            if p.is_type(PieceType::Pawn) {
                let (starting_rank, en_passant_rank, double_push_rank) = p.color.map(
                    (Rank::Second, Rank::Third, Rank::Fourth),
                    (Rank::Seventh, Rank::Sixth, Rank::Fifth),
                );

                // TODO: store this information in the move
                if m.from.rank() == starting_rank && m.to.rank() == double_push_rank {
                    self.en_passant_square = Some(Square::new(m.to.file(), en_passant_rank));
                }
            }

            // castling rights
            match m.from {
                Square::A1 => {
                    self.castling_rights.white_queen_side = false;
                }
                Square::E1 => {
                    self.castling_rights.white_queen_side = false;
                    self.castling_rights.white_king_side = false;
                }
                Square::H1 => {
                    self.castling_rights.white_king_side = false;
                }

                Square::A8 => {
                    self.castling_rights.black_queen_side = false;
                }
                Square::E8 => {
                    self.castling_rights.black_queen_side = false;
                    self.castling_rights.black_king_side = false;
                }
                Square::H8 => {
                    self.castling_rights.black_king_side = false;
                }

                _ => {}
            }

            // white castling
            if m.from == Square::E1 && p.piece_type == PieceType::King && p.color == Color::White {
                // queenside
                if m.to == Square::C1 {
                    self.pieces[Square::D1] = self.pieces[Square::A1];
                    self.pieces[Square::C1] = BoardState::Piece(p);
                    self.pieces[Square::E1] = BoardState::Empty;
                    self.pieces[Square::A1] = BoardState::Empty;
                    return;
                }
                // kingside
                if m.to == Square::G1 {
                    self.pieces[Square::F1] = self.pieces[Square::A1];
                    self.pieces[Square::G1] = BoardState::Piece(p);
                    self.pieces[Square::E1] = BoardState::Empty;
                    self.pieces[Square::H1] = BoardState::Empty;
                    return;
                }
            }
            // black castling
            if m.from == Square::E8 && p.piece_type == PieceType::King && p.color == Color::Black {
                // queenside
                if m.to == Square::C8 {
                    self.pieces[Square::D8] = self.pieces[Square::A8];
                    self.pieces[Square::C8] = BoardState::Piece(p);
                    self.pieces[Square::E8] = BoardState::Empty;
                    self.pieces[Square::A8] = BoardState::Empty;
                    return;
                }
                // kingside
                if m.to == Square::G8 {
                    self.pieces[Square::F8] = self.pieces[Square::A8];
                    self.pieces[Square::G8] = BoardState::Piece(p);
                    self.pieces[Square::E8] = BoardState::Empty;
                    self.pieces[Square::H8] = BoardState::Empty;
                    return;
                }
            }

            // en passent
            if p.piece_type == PieceType::Pawn
                && m.from.file() != m.to.file()
                && self.pieces[m.to] == BoardState::Empty
            {
                let capture_field = if p.color == Color::White {
                    Square::new(m.to.file(), m.to.rank() - 1)
                } else {
                    Square::new(m.to.file(), m.to.rank() + 1)
                };
                dbg!(capture_field);
                self.pieces[capture_field] = BoardState::Empty;
            }

            // promotion
            let piece = if let Some(promotion_piece) = m.promotion_piece {
                Piece::new(promotion_piece, p.color)
            } else {
                p
            };

            // normal move
            self.pieces[m.to] = BoardState::Piece(piece);
            self.pieces[m.from] = BoardState::Empty;
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print flags
        writeln!(f)?;
        writeln!(f, "Active color: {}", self.side_to_move)?;
        writeln!(f, "Castling rights: {}", self.castling_rights)?;
        write!(f, "En passant: ")?;
        if let Some(s) = self.en_passant_square {
            writeln!(f, "{}", s)?;
        } else {
            writeln!(f, "-")?;
        }
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

    #[test_case( "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "e2e4", "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1" ; "e2e4")]
    #[test_case( "rnbqkbnr/pppppppp/8/8/4p3/8/pppp1ppp/rnbqkbnr b kqkq e3 0 1", "c7c5", "rnbqkbnr/pp1ppppp/8/2p5/4p3/8/pppp1ppp/rnbqkbnr w kqkq c6 0 2" ; "c7c5")]
    #[test_case( "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2", "e4d5", "rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2" ; "capture")]
    #[test_case( "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3", "e5d6", "rnbqkbnr/1pp1pppp/p2P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3" ; "en passant white")]
    #[test_case( "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4", "e1g1", "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4" ; "kingside castling white")]
    #[test_case( "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/R3KBNR w KQkq - 5 6", "e1c1", "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/2KR1BNR b kq - 6 6" ; "queenside castling white")]
    #[test_case( "rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 4", "e8g8", "rnbq1rk1/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R w KQ - 1 5" ; "kingside castling black")]
    #[test_case( "r3kbnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R b KQkq - 6 5", "e8c8", "2kr1bnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R w KQ - 7 6" ; "queenside castling black")]
    #[test_case( "8/8/2k5/4K3/8/8/4p3/8 b - - 0 90", "e2e1Q", "8/8/2k5/4K3/8/8/8/4q3 w - - 0 91" ; "promotion black")]
    #[test_case( "5b2/6P1/2k5/4K3/3p4/3B4/8/8 w - - 3 92", "g7f8Q", "5Q2/8/2k5/4K3/3p4/3B4/8/8 b - - 0 92" ; "promotion with capture")]
    #[test_case( "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8N", "5N2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85" ; "promtotion to knight")]
    #[test_case( "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8B", "5B2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85" ; "promotion to bishop")]
    #[test_case( "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85", "f7f8R", "5R2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85" ; "promotion to rook")]
    fn test_position_make_move(pos: &str, m: &str, expected: &str) {
        let mut pos = Position::from_fen(pos).expect("valid position");
        let m = Move::from_coordinate_notation(m).expect("valid move");
        let expected = Position::from_fen(expected).expect("valid position");

        pos.make_move(&m);
        pretty_assertions::assert_eq!(pos, expected);
    }

    #[test]
    fn test_position_display() {
        let expected = r"
Active color: white
Castling rights: KQkq
En passant: -

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
