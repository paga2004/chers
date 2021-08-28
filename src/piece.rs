use crate::Color;
use std::fmt;

/// The type of a piece. Pawns are distinct by color because they have diffrent move-directions.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PieceType(u8);

impl PieceType {
    pub(crate) const NIL: Self = Self(0);
    /// White pawn
    pub const PAWN_W: Self = Self(1);
    /// Black pawn
    pub const PAWN_B: Self = Self(2);
    /// Knight
    pub const KNIGHT: Self = Self(3);
    /// Bishop
    pub const BISHOP: Self = Self(4);
    /// Rook
    pub const ROOK: Self = Self(5);
    /// Queen
    pub const QUEEN: Self = Self(6);
    /// King
    pub const KING: Self = Self(7);
    /// Creates a `PieceType` from its english letter or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::PieceType;
    ///
    /// assert_eq!(PieceType::from_char('K'), Some(PieceType::KING));
    /// assert_eq!(PieceType::from_char('N'), Some(PieceType::KNIGHT));
    /// assert_eq!(PieceType::from_char('n'), Some(PieceType::KNIGHT));
    /// assert_eq!(PieceType::from_char('X'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'P' => Some(Self::PAWN_W),
            'p' => Some(Self::PAWN_B),
            'n' | 'N' => Some(Self::KNIGHT),
            'b' | 'B' => Some(Self::BISHOP),
            'r' | 'R' => Some(Self::ROOK),
            'q' | 'Q' => Some(Self::QUEEN),
            'k' | 'K' => Some(Self::KING),
            _ => None,
        }
    }

    /// Returns the english lowercase letter corresponding to the `PieceType`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::PieceType;
    ///
    /// assert_eq!(PieceType::PAWN_W.to_char(), 'p');
    /// assert_eq!(PieceType::KNIGHT.to_char(), 'n');
    /// assert_eq!(PieceType::PAWN_W.to_char(), 'p');
    /// assert_eq!(PieceType::KNIGHT.to_char(), 'n');
    /// assert_eq!(PieceType::KING.to_char(), 'k');
    /// ```
    pub fn to_char(self) -> char {
        match self {
            Self::PAWN_W | Self::PAWN_B => 'p',
            Self::KNIGHT => 'n',
            Self::BISHOP => 'b',
            Self::ROOK => 'r',
            Self::QUEEN => 'q',
            Self::KING => 'k',
            Self::NIL => ' ',
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// A piece.
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
    /// assert_eq!(Piece::from_char('K'), Some(Piece::new(PieceType::KING, Color::WHITE)));
    /// assert_eq!(Piece::from_char('N'), Some(Piece::new(PieceType::KNIGHT, Color::WHITE)));
    /// assert_eq!(Piece::from_char('n'), Some(Piece::new(PieceType::KNIGHT, Color::BLACK)));
    /// assert_eq!(Piece::from_char('N'), Some(Piece::new(PieceType::KNIGHT, Color::WHITE)));
    /// assert_eq!(Piece::from_char('n'), Some(Piece::new(PieceType::KNIGHT, Color::BLACK)));
    ///
    /// assert_eq!(Piece::from_char('x'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        let piece_type = PieceType::from_char(c)?;
        let color = if c.is_ascii_uppercase() {
            Color::WHITE
        } else {
            Color::BLACK
        };
        Some(Piece::new(piece_type, color))
    }

    /// Returns true if the color of `self` matches `color`.
    pub fn is_color(self, color: Color) -> bool {
        self.color == color
    }

    /// Returns true if the piece type of `self` matches `piece_type`.
    pub fn is_type(self, piece_type: PieceType) -> bool {
        self.piece_type == piece_type
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.piece_type.to_char();
        if self.color == Color::WHITE {
            write!(f, "{}", symbol.to_ascii_uppercase())
        } else {
            write!(f, "{}", symbol)
        }
    }
}
