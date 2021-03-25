//! This module contains the board representation.
//!
//! Little-Endian Rank-File Mapping is used everywhere.
//! See [here](https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian%20Rank-File%20Mapping)

use crate::fen;
use crate::piece::{Color, Piece, PieceType};
use crate::r#move::Move;
use std::fmt;

pub(crate) fn calculate_index(file: usize, rank: usize) -> usize {
    file + 8 * rank
}

/// Represents each field of the board.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Field {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Field {
    /// Creates a `Field` from file and rank.
    ///
    /// # Panics
    ///
    /// Panics if `file` or `rank` are > 7
    pub fn new(file: usize, rank: usize) -> Self {
        assert!(file <= 7);
        assert!(rank <= 7);
        Self::from_index(calculate_index(file, rank))
    }
    fn from_index(index: usize) -> Self {
        use Field::*;
        #[rustfmt::skip]
        return [
             A1, B1, C1, D1, E1, F1, G1, H1,
             A2, B2, C2, D2, E2, F2, G2, H2,
             A3, B3, C3, D3, E3, F3, G3, H3,
             A4, B4, C4, D4, E4, F4, G4, H4,
             A5, B5, C5, D5, E5, F5, G5, H5,
             A6, B6, C6, D6, E6, F6, G6, H6,
             A7, B7, C7, D7, E7, F7, G7, H7,
             A8, B8, C8, D8, E8, F8, G8, H8
        ][index];
    }

    pub fn file(self) -> usize {
        self as usize % 8
    }

    pub fn rank(self) -> usize {
        self as usize / 8
    }
}

#[derive(PartialEq)]
pub struct Position {
    /// All the pieces on the board
    pub(crate) pieces: [Option<Piece>; 64],
    pub(crate) color_to_move: Color,
}

impl Position {
    /// Creates a new Position
    pub fn new() -> Self {
        Self::from_fen(fen::STARTING_FEN).unwrap()
    }

    /// Make a move on the current position.
    ///
    /// This function does not check whether the move is legal.
    pub fn make_move(&mut self, m: &Move) {
        if let Some(p) = self.pieces[m.from as usize] {
            self.color_to_move = !self.color_to_move;
            // white castling
            if m.from == Field::E1 && p.piece_type == PieceType::King && p.color == Color::White {
                // long
                if m.to == Field::C1 {
                    self.pieces[Field::D1 as usize] = self.pieces[Field::A1 as usize];
                    self.pieces[Field::C1 as usize] = Some(p);
                    self.pieces[Field::E1 as usize] = None;
                    self.pieces[Field::A1 as usize] = None;
                    return;
                }
                // short
                if m.to == Field::G1 {
                    self.pieces[Field::F1 as usize] = self.pieces[Field::A1 as usize];
                    self.pieces[Field::G1 as usize] = Some(p);
                    self.pieces[Field::E1 as usize] = None;
                    self.pieces[Field::H1 as usize] = None;
                    return;
                }
            }
            // black castling
            if m.from == Field::E8 && p.piece_type == PieceType::King && p.color == Color::Black {
                // long
                if m.to == Field::C8 {
                    self.pieces[Field::D8 as usize] = self.pieces[Field::A8 as usize];
                    self.pieces[Field::C8 as usize] = Some(p);
                    self.pieces[Field::E8 as usize] = None;
                    self.pieces[Field::A8 as usize] = None;
                    return;
                }
                // short
                if m.to == Field::G8 {
                    self.pieces[Field::F8 as usize] = self.pieces[Field::A8 as usize];
                    self.pieces[Field::G8 as usize] = Some(p);
                    self.pieces[Field::E8 as usize] = None;
                    self.pieces[Field::H8 as usize] = None;
                    return;
                }
            }

            // en passent
            if p.piece_type == PieceType::Pawn
                && m.from.file() != m.to.file()
                && self.pieces[m.to as usize] == None
            {
                let capture_field = if p.color == Color::White {
                    Field::new(m.to.file(), m.to.rank() - 1)
                } else {
                    Field::new(m.to.file(), m.to.rank() + 1)
                };
                self.pieces[capture_field as usize] = None;
            }

            // promotion
            let piece = if let Some(promotion_piece) = m.promotion_piece {
                Piece::new(promotion_piece, p.color)
            } else {
                p
            };

            // normal move
            self.pieces[m.to as usize] = Some(piece);
            self.pieces[m.from as usize] = None;
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print flags
        writeln!(f, "{} to move", self.color_to_move)?;
        writeln!(f)?;

        // print board
        writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐")?;
        for rank in (0..8).rev() {
            write!(f, "{} │", rank + 1)?;
            for file in 0..8 {
                write!(f, " ")?;
                if let Some(piece) = self.pieces[calculate_index(file, rank)] {
                    write!(f, "{}", piece)?;
                } else {
                    write!(f, " ")?;
                }
                write!(f, " │")?;
            }
            if rank > 0 {
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
    use super::*;

    #[test]
    fn test_field_new() {
        use Field::*;
        assert_eq!(Field::new(0, 0), A1);
        assert_eq!(Field::new(0, 7), A8);
        assert_eq!(Field::new(7, 0), H1);
        assert_eq!(Field::new(7, 7), H8);
    }

    #[test]
    #[should_panic]
    fn test_field_new_out_of_bounds_field() {
        let _ = Field::new(8, 0);
    }

    #[test]
    #[should_panic]
    fn test_field_new_out_of_bounds_rank() {
        let _ = Field::new(0, 8);
    }

    #[test]
    fn test_field_from_index() {
        for i in 0..64 {
            let f = Field::from_index(i);
            assert_eq!(i, f as usize);
        }
    }

    #[test]
    fn test_field_file() {
        use Field::*;
        assert_eq!(A1.file(), 0);
        assert_eq!(A2.file(), 0);
        assert_eq!(A8.file(), 0);
        assert_eq!(B1.file(), 1);
        assert_eq!(B2.file(), 1);
        assert_eq!(B8.file(), 1);
        assert_eq!(H1.file(), 7);
        assert_eq!(H2.file(), 7);
        assert_eq!(H8.file(), 7);
    }

    #[test]
    fn test_field_rank() {
        use Field::*;
        assert_eq!(A1.rank(), 0);
        assert_eq!(B1.rank(), 0);
        assert_eq!(H1.rank(), 0);
        assert_eq!(A2.rank(), 1);
        assert_eq!(B2.rank(), 1);
        assert_eq!(H2.rank(), 1);
        assert_eq!(A8.rank(), 7);
        assert_eq!(B8.rank(), 7);
        assert_eq!(H8.rank(), 7);
    }

    /// Generates a function to test `Position::make_move`.
    ///
    /// Curly braces are necessary for rustfmt to work, which is nice because it can automatically
    /// wrap long lines.
    macro_rules! test_position_make_move {
        ({ $($name:ident($position:expr, $move:expr, $expected:expr $(,)?);)+ }) => {
            $(
                #[test]
                fn $name() {
                    let mut pos = Position::from_fen($position).unwrap();
                    let m = Move::from_smith_notation($move).unwrap();
                    let expected = Position::from_fen($expected).unwrap();

                    pos.make_move(&m);
                    assert_eq!(pos, expected);
                }
            )*
        };
    }

    test_position_make_move!({
        test_position_make_move_e2e4(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "e2e4",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        );
        test_position_make_move_c7c5(
            "rnbqkbnr/pppppppp/8/8/4p3/8/pppp1ppp/rnbqkbnr b kqkq e3 0 1",
            "c7c5",
            "rnbqkbnr/pp1ppppp/8/2p5/4p3/8/pppp1ppp/rnbqkbnr w kqkq c6 0 2",
        );

        // capture
        test_position_make_move_capture(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
            "e4d5",
            "rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2",
        );
        test_positon_make_move_en_passant_white(
            "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            "e5d6",
            "rnbqkbnr/1pp1pppp/p2P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
        );
        test_positon_make_move_en_passant_black(
            "rnbqkbnr/pppp1ppp/8/8/P3pP2/8/1PPPP1PP/RNBQKBNR b KQkq f3 0 3",
            "e4f3",
            "rnbqkbnr/pppp1ppp/8/8/P7/5p2/1PPPP1PP/RNBQKBNR w KQkq - 0 4",
        );

        // castling
        test_positon_make_move_short_castling_white(
            "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4",
            "e1g1",
            "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4",
        );
        test_positon_make_move_long_castling_white(
            "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/R3KBNR w KQkq - 5 6",
            "e1c1",
            "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/2KR1BNR b kq - 6 6",
        );
        test_positon_make_move_short_castling_black(
            "rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 4",
            "e8g8",
            "rnbq1rk1/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R w KQ - 1 5",
        );
        test_positon_make_move_long_castling_black(
            "r3kbnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R b KQkq - 6 5",
            "e8c8",
            "2kr1bnr/pppqpppp/2n1b3/3pN3/2PP4/2N5/PP2PPPP/R1BQKB1R w KQ - 7 6",
        );

        // promotion
        test_position_make_move_promotion_white(
            "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85",
            "f7f8Q",
            "5Q2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85",
        );
        test_position_make_move_promotion_black(
            "8/8/2k5/4K3/8/8/4p3/8 b - - 0 90",
            "e2e1Q",
            "8/8/2k5/4K3/8/8/8/4q3 w - - 0 91",
        );
        test_position_make_move_promotion_capture(
            "5b2/6P1/2k5/4K3/3p4/3B4/8/8 w - - 3 92",
            "g7f8Q",
            "5Q2/8/2k5/4K3/3p4/3B4/8/8 b - - 0 92",
        );
        test_position_make_move_promotion_knight(
            "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85",
            "f7f8N",
            "5N2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85",
        );
        test_position_make_move_promotion_bishop(
            "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85",
            "f7f8B",
            "5B2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85",
        );
        test_position_make_move_promotion_rook(
            "8/5P1P/2k5/4b1P1/3p4/3B1K2/8/8 w - - 1 85",
            "f7f8R",
            "5R2/7P/2k5/4b1P1/3p4/3B1K2/8/8 b - - 0 85",
        );
    });

    #[test]
    fn test_position_make_move_promotion() {
        let mut position = Position::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let m = Move::from_smith_notation("a7a8Q").unwrap();
        let expected = Position::from_fen("Q3k3/8/8/8/8/8/8/4K3 b - - 0 1").unwrap();

        position.make_move(&m);
        assert_eq!(position, expected);
    }

    #[test]
    fn test_position_display() {
        let expected = "\
White to move\n\n  \

  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n\
8 │ r │ n │ b │ q │ k │ b │ n │ r │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
7 │ p │ p │ p │ p │ p │ p │ p │ p │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
6 │   │   │   │   │   │   │   │   │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
5 │   │   │   │   │   │   │   │   │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
4 │   │   │   │   │   │   │   │   │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
3 │   │   │   │   │   │   │   │   │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
2 │ P │ P │ P │ P │ P │ P │ P │ P │\n  \
  ├───┼───┼───┼───┼───┼───┼───┼───┤\n\
1 │ R │ N │ B │ Q │ K │ B │ N │ R │\n  \
  └───┴───┴───┴───┴───┴───┴───┴───┘\n    \
    a   b   c   d   e   f   g   h";
        assert_eq!(format!("{}", Position::new()), expected);
    }
}
