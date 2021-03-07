use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    None,
}

impl Piece {
    fn is_white(self) -> bool {
        (self as u8) <= 5
    }
    fn is_black(self) -> bool {
        self != Self::None && (self as u8) >= 6
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Piece::WhitePawn => write!(f, "P")?,
            Piece::WhiteKnight => write!(f, "N")?,
            Piece::WhiteBishop => write!(f, "B")?,
            Piece::WhiteRook => write!(f, "R")?,
            Piece::WhiteQueen => write!(f, "Q")?,
            Piece::WhiteKing => write!(f, "K")?,
            Piece::BlackPawn => write!(f, "p")?,
            Piece::BlackKnight => write!(f, "n")?,
            Piece::BlackBishop => write!(f, "b")?,
            Piece::BlackRook => write!(f, "r")?,
            Piece::BlackQueen => write!(f, "q")?,
            Piece::BlackKing => write!(f, "k")?,
            Piece::None => write!(f, "_")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_white() {
        assert!(Piece::WhitePawn.is_white());
        assert!(Piece::WhiteKnight.is_white());
        assert!(Piece::WhiteBishop.is_white());
        assert!(Piece::WhiteRook.is_white());
        assert!(Piece::WhiteQueen.is_white());
        assert!(Piece::WhiteKing.is_white());

        assert!(!Piece::BlackKing.is_white());
        assert!(!Piece::BlackPawn.is_white());
        assert!(!Piece::BlackKnight.is_white());
        assert!(!Piece::BlackBishop.is_white());
        assert!(!Piece::BlackRook.is_white());
        assert!(!Piece::BlackQueen.is_white());
        assert!(!Piece::BlackKing.is_white());

        assert!(!Piece::None.is_white())
    }

    #[test]
    fn test_is_black() {
        assert!(Piece::BlackPawn.is_black());
        assert!(Piece::BlackKnight.is_black());
        assert!(Piece::BlackBishop.is_black());
        assert!(Piece::BlackRook.is_black());
        assert!(Piece::BlackQueen.is_black());
        assert!(Piece::BlackKing.is_black());

        assert!(!Piece::WhiteKing.is_black());
        assert!(!Piece::WhitePawn.is_black());
        assert!(!Piece::WhiteKnight.is_black());
        assert!(!Piece::WhiteBishop.is_black());
        assert!(!Piece::WhiteRook.is_black());
        assert!(!Piece::WhiteQueen.is_black());
        assert!(!Piece::WhiteKing.is_black());

        assert!(!Piece::None.is_black())
    }
}
