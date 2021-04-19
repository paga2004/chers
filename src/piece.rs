use crate::Color;
use std::fmt;
use std::ops::Index;

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

impl PieceType {
    /// Creates a `PieceType` from its english letter or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::PieceType;
    ///
    /// assert_eq!(PieceType::from_char('K'), Some(PieceType::King));
    /// assert_eq!(PieceType::from_char('N'), Some(PieceType::Knight));
    /// assert_eq!(PieceType::from_char('n'), Some(PieceType::Knight));
    ///
    /// assert_eq!(PieceType::from_char('X'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'p' | 'P' => Some(Self::Pawn),
            'n' | 'N' => Some(Self::Knight),
            'b' | 'B' => Some(Self::Bishop),
            'r' | 'R' => Some(Self::Rook),
            'q' | 'Q' => Some(Self::Queen),
            'k' | 'K' => Some(Self::King),
            _ => None,
        }
    }

    /// Returns the english lowercase letter corresponding to the `PieceType`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::PieceType;
    ///
    /// assert_eq!(PieceType::Pawn.to_char(), 'p');
    /// assert_eq!(PieceType::Knight.to_char(), 'n');
    /// assert_eq!(PieceType::King.to_char(), 'k');
    /// ```
    pub fn to_char(&self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::Knight => 'n',
            Self::Bishop => 'b',
            Self::Rook => 'r',
            Self::Queen => 'q',
            Self::King => 'k',
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl<T> Index<PieceType> for [T; 6] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        &self[index as usize]
    }
}
/// Represents a piece.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}

impl Piece {
    /// Creates a new `Piece` from a `PieceType` and a `Color`.
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    /// Creates a `Piece` from its english letter or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::{Piece, PieceType, Color};
    ///
    /// assert_eq!(Piece::from_char('K'), Some(Piece::new(PieceType::King, Color::White)));
    /// assert_eq!(Piece::from_char('N'), Some(Piece::new(PieceType::Knight, Color::White)));
    /// assert_eq!(Piece::from_char('n'), Some(Piece::new(PieceType::Knight, Color::Black)));
    ///
    /// assert_eq!(Piece::from_char('x'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        let piece_type = PieceType::from_char(c)?;
        let color = if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        Some(Piece::new(piece_type, color))
    }

    /// Returns true if the color of `self` matches `color`
    pub fn is_color(self, color: Color) -> bool {
        self.color == color
    }

    /// Returns true if the piece type of `self` matches `piece_type`
    pub fn is_type(self, piece_type: PieceType) -> bool {
        self.piece_type == piece_type
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.piece_type.to_char();
        if self.color == Color::White {
            write!(f, "{}", symbol.to_ascii_uppercase())
        } else {
            write!(f, "{}", symbol)
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_piece_type_index() {
        let a = [1, 2, 3, 4, 5, 6];
        assert_eq!(a[PieceType::Pawn], 1);
        assert_eq!(a[PieceType::Knight], 2);
        assert_eq!(a[PieceType::Bishop], 3);
        assert_eq!(a[PieceType::Rook], 4);
        assert_eq!(a[PieceType::Queen], 5);
        assert_eq!(a[PieceType::King], 6);
    }
}
