//! This module contains the board representation.
//!
//! [Little-Endian Rank-File Mapping][LERF] is used everywhere.
//!
//! [LERF]: https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian%20Rank-File%20Mapping

use crate::fen;
use crate::Color;
use crate::Move;
use crate::Piece;
use crate::PieceType;
use crate::Rank;
use crate::Square;
use std::fmt;

pub(crate) fn calculate_index(file: usize, rank: usize) -> usize {
    file + 8 * rank
}

/// Represents a chess position.
///
/// This is the heart of the crate. Most of its functionality can be accessed achieved with this
/// struct.
#[allow(missing_copy_implementations)] // copying a position is expensive and should be avoided
#[derive(PartialEq)]
pub struct Position {
    /// All the pieces on the board
    pub(crate) pieces: [Option<Piece>; 64],
    pub(crate) color_to_move: Color,
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
        if let Some(p) = self.pieces[m.from as usize] {
            self.color_to_move = !self.color_to_move;
            // white castling
            if m.from == Square::E1 && p.piece_type == PieceType::King && p.color == Color::White {
                // queenside
                if m.to == Square::C1 {
                    self.pieces[Square::D1 as usize] = self.pieces[Square::A1 as usize];
                    self.pieces[Square::C1 as usize] = Some(p);
                    self.pieces[Square::E1 as usize] = None;
                    self.pieces[Square::A1 as usize] = None;
                    return;
                }
                // kingside
                if m.to == Square::G1 {
                    self.pieces[Square::F1 as usize] = self.pieces[Square::A1 as usize];
                    self.pieces[Square::G1 as usize] = Some(p);
                    self.pieces[Square::E1 as usize] = None;
                    self.pieces[Square::H1 as usize] = None;
                    return;
                }
            }
            // black castling
            if m.from == Square::E8 && p.piece_type == PieceType::King && p.color == Color::Black {
                // queenside
                if m.to == Square::C8 {
                    self.pieces[Square::D8 as usize] = self.pieces[Square::A8 as usize];
                    self.pieces[Square::C8 as usize] = Some(p);
                    self.pieces[Square::E8 as usize] = None;
                    self.pieces[Square::A8 as usize] = None;
                    return;
                }
                // kingside
                if m.to == Square::G8 {
                    self.pieces[Square::F8 as usize] = self.pieces[Square::A8 as usize];
                    self.pieces[Square::G8 as usize] = Some(p);
                    self.pieces[Square::E8 as usize] = None;
                    self.pieces[Square::H8 as usize] = None;
                    return;
                }
            }

            // en passent
            if p.piece_type == PieceType::Pawn
                && m.from.file() != m.to.file()
                && self.pieces[m.to as usize] == None
            {
                let capture_field = if p.color == Color::White {
                    Square::new(m.to.file(), Rank::new(m.to.rank() as u8 - 1))
                } else {
                    Square::new(m.to.file(), Rank::new(m.to.rank() as u8 + 1))
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
        writeln!(f)?;
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

    /// Creates a function to test `Position::make_move`.
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
        () => {};
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
        test_positon_make_move_kingside_castling_white(
            "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4",
            "e1g1",
            "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4",
        );
        test_positon_make_move_queenside_castling_white(
            "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/R3KBNR w KQkq - 5 6",
            "e1c1",
            "r2qkb1r/ppp1pppp/2n5/3p1b2/3PnB2/2NQP3/PPP2PPP/2KR1BNR b kq - 6 6",
        );
        test_positon_make_move_kingside_castling_black(
            "rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 4",
            "e8g8",
            "rnbq1rk1/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R w KQ - 1 5",
        );
        test_positon_make_move_queenside_castling_black(
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
        let expected = r"
White to move

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
