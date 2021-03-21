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
        for i in 0..8 {
            for j in 0..8 {
                match self.pieces[calculate_index(j, 7 - i)] {
                    Some(piece) => write!(f, "{}", piece)?,
                    None => write!(f, "_")?,
                }
                if j != 7 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
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
    fn test_display() {
        let expected = "r n b q k b n r\n\
                        p p p p p p p p\n\
                        _ _ _ _ _ _ _ _\n\
                        _ _ _ _ _ _ _ _\n\
                        _ _ _ _ _ _ _ _\n\
                        _ _ _ _ _ _ _ _\n\
                        P P P P P P P P\n\
                        R N B Q K B N R\n";
        assert_eq!(format!("{}", Position::new()), expected);
    }
}
