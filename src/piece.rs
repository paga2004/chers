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

    #[inline]
    pub(crate) fn from_u8(n: u8) -> Self {
        Self(n)
    }

    #[inline]
    pub(crate) const fn to_u8(self) -> u8 {
        self.0 as u8
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

    /// No piece
    pub const EMPTY: Self = Self(0);
    /// White pawn
    pub const W_PAWN: Self = Self(PieceType::PAWN_W.to_u8());
    pub(crate) const OFF_BOARD: Self = Self(PieceType::PAWN_B.to_u8());
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
    pub const B_PAWN: Self = Self(PieceType::PAWN_B.to_u8() + Self::BLACK_COLOR_CODE);
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
    /// assert_eq!(Piece::W_PAWN.piece_type(), PieceType::PAWN_W);
    /// assert_eq!(Piece::W_KING.piece_type(), PieceType::KING);
    ///
    /// assert_eq!(Piece::B_PAWN.piece_type(), PieceType::PAWN_B);
    /// assert_eq!(Piece::B_KING.piece_type(), PieceType::KING);
    /// ```
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
    /// assert_eq!(Piece::new(PieceType::PAWN_W, Color::WHITE), Piece::W_PAWN);
    /// assert_eq!(Piece::new(PieceType::KING, Color::WHITE), Piece::W_KING);
    ///
    /// assert_eq!(Piece::new(PieceType::PAWN_B, Color::BLACK), Piece::B_PAWN);
    /// assert_eq!(Piece::new(PieceType::KING, Color::BLACK), Piece::B_KING);
    /// ```

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

    /// Returns true if the color of `self` matches `color`.
    pub fn is_color(self, color: Color) -> bool {
        self.color() == color
    }

    /// Returns true if the piece type of `self` matches `piece_type`.
    pub fn is_type(self, piece_type: PieceType) -> bool {
        self.piece_type() == piece_type
    }

    pub(crate) fn is_piece(self) -> bool {
        self == Self::W_PAWN || self.0 > Self::OFF_BOARD.0
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.piece_type().to_char();
        if self.is_color(Color::WHITE) {
            write!(f, "{}", symbol.to_ascii_uppercase())
        } else {
            write!(f, "{}", symbol)
        }
    }
}
