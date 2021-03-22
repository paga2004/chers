use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White")?,
            Color::Black => write!(f, "Black")?,
        }
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
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
