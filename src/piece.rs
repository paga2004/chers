use std::fmt;

use std::ops::Index;

use crate::Color;

/// The type of a piece.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PieceType(u8);

impl PieceType {
    /// Pawn
    pub const PAWN: Self = Self(0);
    /// Knight
    pub const KNIGHT: Self = Self(1);
    /// Bishop
    pub const BISHOP: Self = Self(2);
    /// Rook
    pub const ROOK: Self = Self(3);
    /// Queen
    pub const QUEEN: Self = Self(4);
    /// King
    pub const KING: Self = Self(5);

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
            'p' | 'P' => Some(Self::PAWN),
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
    /// assert_eq!(PieceType::PAWN.to_char(), 'p');
    /// assert_eq!(PieceType::KNIGHT.to_char(), 'n');
    /// assert_eq!(PieceType::PAWN.to_char(), 'p');
    /// assert_eq!(PieceType::KNIGHT.to_char(), 'n');
    /// assert_eq!(PieceType::KING.to_char(), 'k');
    /// ```
    pub fn to_char(self) -> char {
        match self {
            Self::PAWN => 'p',
            Self::KNIGHT => 'n',
            Self::BISHOP => 'b',
            Self::ROOK => 'r',
            Self::QUEEN => 'q',
            Self::KING => 'k',
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn from_u8(n: u8) -> Self {
        Self(n)
    }

    #[inline]
    pub(crate) const fn to_u8(self) -> u8 {
        self.0 as u8
    }
}

impl<T> Index<PieceType> for [T; 6] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// A piece.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece(u8);

impl Piece {
    const COLOR_SHIFT: u8 = 3;
    const BLACK_COLOR_CODE: u8 = 1 << Self::COLOR_SHIFT;

    /// White pawn
    pub const W_PAWN: Self = Self(PieceType::PAWN.to_u8());
    /// White knight
    pub const W_KNIGHT: Self = Self(PieceType::KNIGHT.to_u8());
    /// White bishop
    pub const W_BISHOP: Self = Self(PieceType::BISHOP.to_u8());
    /// White rook
    pub const W_ROOK: Self = Self(PieceType::ROOK.to_u8());
    /// White queen
    pub const W_QUEEN: Self = Self(PieceType::QUEEN.to_u8());
    /// White king
    pub const W_KING: Self = Self(PieceType::KING.to_u8());

    /// Black pawn
    pub const B_PAWN: Self = Self(PieceType::PAWN.to_u8() + Self::BLACK_COLOR_CODE);
    /// Black knight
    pub const B_KNIGHT: Self = Self(PieceType::KNIGHT.to_u8() + Self::BLACK_COLOR_CODE);
    /// Black bishop
    pub const B_BISHOP: Self = Self(PieceType::BISHOP.to_u8() + Self::BLACK_COLOR_CODE);
    /// Black rook
    pub const B_ROOK: Self = Self(PieceType::ROOK.to_u8() + Self::BLACK_COLOR_CODE);
    /// Black queen
    pub const B_QUEEN: Self = Self(PieceType::QUEEN.to_u8() + Self::BLACK_COLOR_CODE);
    /// Black king
    pub const B_KING: Self = Self(PieceType::KING.to_u8() + Self::BLACK_COLOR_CODE);

    /// No piece
    pub const EMPTY: Self = Self(14);
    pub(crate) const OFF_BOARD: Self = Self(15);

    /// Returns the color of the piece
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Piece;
    /// use chers::Color;
    ///
    ///
    /// assert_eq!(Piece::W_PAWN.color(), Color::WHITE);
    /// assert_eq!(Piece::W_KING.color(), Color::WHITE);
    ///
    /// assert_eq!(Piece::B_PAWN.color(), Color::BLACK);
    /// assert_eq!(Piece::B_KING.color(), Color::BLACK);
    /// ```
    #[inline]
    pub fn color(self) -> Color {
        Color::from_bool((self.0 >> 3 & 1) == 1)
    }

    /// Returns the type of the piece
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Piece;
    /// use chers::PieceType;
    ///
    ///
    /// assert_eq!(Piece::W_PAWN.piece_type(), PieceType::PAWN);
    /// assert_eq!(Piece::W_KING.piece_type(), PieceType::KING);
    ///
    /// assert_eq!(Piece::B_PAWN.piece_type(), PieceType::PAWN);
    /// assert_eq!(Piece::B_KING.piece_type(), PieceType::KING);
    /// ```
    #[inline]
    pub fn piece_type(self) -> PieceType {
        PieceType::from_u8(self.0 & 7)
    }

    /// Creates a new `Piece` from a `PieceType` and a `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Piece;
    /// use chers::PieceType;
    /// use chers::Color;
    ///
    ///
    /// assert_eq!(Piece::new(PieceType::PAWN, Color::WHITE), Piece::W_PAWN);
    /// assert_eq!(Piece::new(PieceType::KING, Color::WHITE), Piece::W_KING);
    ///
    /// assert_eq!(Piece::new(PieceType::PAWN, Color::BLACK), Piece::B_PAWN);
    /// assert_eq!(Piece::new(PieceType::KING, Color::BLACK), Piece::B_KING);
    /// ```
    #[inline]
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self((color.to_u8() << Self::COLOR_SHIFT) + piece_type.to_u8())
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

    /// Returns the english lowercase letter corresponding to the `Piece`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Piece;
    ///
    /// assert_eq!(Piece::W_PAWN.to_char(), 'P');
    /// assert_eq!(Piece::B_PAWN.to_char(), 'p');
    /// assert_eq!(Piece::W_KNIGHT.to_char(), 'N');
    /// assert_eq!(Piece::B_KNIGHT.to_char(), 'n');
    /// assert_eq!(Piece::W_KING.to_char(), 'K');
    /// assert_eq!(Piece::B_KING.to_char(), 'k');
    /// ```
    pub fn to_char(self) -> char {
        match self.color() {
            Color::WHITE => self.piece_type().to_char().to_ascii_uppercase(),
            Color::BLACK => self.piece_type().to_char(),
        }
    }

    /// Returns true if the color of `self` matches `color`.
    #[inline]
    pub fn is_color(self, color: Color) -> bool {
        self.color() == color
    }

    /// Returns true if the piece type of `self` matches `piece_type`.
    #[inline]
    pub fn is_type(self, piece_type: PieceType) -> bool {
        self.piece_type() == piece_type
    }

    #[inline]
    pub(crate) fn is_piece(self) -> bool {
        self.0 < Self::EMPTY.0
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_piece() {
            return write!(f, " ");
        }
        let symbol = self.piece_type().to_char();
        if self.is_color(Color::WHITE) {
            write!(f, "{}", symbol.to_ascii_uppercase())
        } else {
            write!(f, "{}", symbol)
        }
    }
}
