//! This module contains the board representation.
//!
//! Little-Endian Rank-File Mapping is used everywhere.
//! See [here](https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian%20Rank-File%20Mapping)

use crate::fen;
use crate::piece::{Color, Piece};
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
    /// Panics if `file` or `rank` are < 0 or > 7
    pub fn new(file: usize, rank: usize) -> Self {
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
    fn test_field_from_index() {
        for i in 0..64 {
            let f = Field::from_index(i);
            assert_eq!(i, f as usize);
        }
    }

    #[test]
    fn test_display() {
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
