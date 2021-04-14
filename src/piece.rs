use crate::Color;
use std::fmt;

/// Represents a piece without considering its color.
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Represents a piece.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}

impl Piece {
    pub(crate) fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        if self.color == Color::White {
            write!(f, "{}", symbol.to_ascii_uppercase())
        } else {
            write!(f, "{}", symbol)
        }
    }
}
