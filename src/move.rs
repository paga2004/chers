use crate::error::ParseMoveError;
use crate::PieceType;
use crate::Square;
use std::fmt;

/// Represents a chess move.
///
/// The move can either be a normal move, a capture, castling, or a promotion.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) promotion_piece: Option<PieceType>,
}

impl Move {
    /// Creates a new `Move`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::{Move, Square, PieceType};
    ///
    /// let e2e4 = Move::new(Square::E2, Square::E4, None);
    /// let promotion = Move::new(Square::F7, Square::F8, Some(PieceType::Queen));
    /// ```
    pub fn new(from: Square, to: Square, promotion_piece: Option<PieceType>) -> Self {
        Self {
            from,
            to,
            promotion_piece,
        }
    }

    /// Creates a new `Move` from pure algebraic coordinate notation.
    ///
    /// The UCI communication protocol uses exactly this notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Move;
    ///
    /// let m1 = Move::from_coordinate_notation("e2e4");
    /// let m2  = Move::from_coordinate_notation("e1g1"); // white short castling
    /// let m3  = Move::from_coordinate_notation("e7e8q"); // promotion
    ///
    /// let m4  = Move::from_coordinate_notation("e4"); // invalid
    /// let m5  = Move::from_coordinate_notation("Bxe5"); // invalid
    ///
    /// assert!(m1.is_ok());
    /// assert!(m2.is_ok());
    /// assert!(m3.is_ok());
    /// assert!(m4.is_err());
    /// assert!(m5.is_err());
    /// ```
    pub fn from_coordinate_notation(s: &str) -> Result<Self, ParseMoveError> {
        let from = s.get(..2).ok_or(ParseMoveError::TooShort)?;
        let to = s.get(2..4).ok_or(ParseMoveError::TooShort)?;

        let promotion_piece = s
            .chars()
            .nth(4)
            .map(|c| PieceType::from_char(c).ok_or(ParseMoveError::InvalidPromotionPiece(c)))
            .map_or(Ok(None), |r| r.map(Some))?;

        Ok(Self {
            from: Square::from_algebraic_notation(from)?,
            to: Square::from_algebraic_notation(to)?,
            promotion_piece,
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)?;
        if let Some(p) = self.promotion_piece {
            write!(f, "{}", p.to_char().to_ascii_uppercase())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::error::ParseSquareError;

    #[test]
    fn from_coordinate_notation_too_short() {
        assert_eq!(
            Move::from_coordinate_notation(""),
            Err(ParseMoveError::TooShort)
        );
        assert_eq!(
            Move::from_coordinate_notation("e"),
            Err(ParseMoveError::TooShort)
        );
        assert_eq!(
            Move::from_coordinate_notation("e2"),
            Err(ParseMoveError::TooShort)
        );
        assert_eq!(
            Move::from_coordinate_notation("e2e"),
            Err(ParseMoveError::TooShort)
        );
    }

    #[test]
    fn from_coordinate_notation_invalid_square() {
        assert_eq!(
            Move::from_coordinate_notation("x1e2"),
            Err(ParseMoveError::InvalidSquare(
                ParseSquareError::InvalidFile('x')
            ))
        );
        assert_eq!(
            Move::from_coordinate_notation("e1x2"),
            Err(ParseMoveError::InvalidSquare(
                ParseSquareError::InvalidFile('x')
            ))
        );
    }

    #[test]
    fn from_coordinate_notation_invalid_rank() {
        assert_eq!(
            Move::from_coordinate_notation("e2e9"),
            Err(ParseMoveError::InvalidSquare(
                ParseSquareError::InvalidRank('9')
            ))
        );
        assert_eq!(
            Move::from_coordinate_notation("e0e2"),
            Err(ParseMoveError::InvalidSquare(
                ParseSquareError::InvalidRank('0')
            ))
        );
    }

    #[test]
    fn from_coordinate_notation_invalid_promotion_piece() {
        assert_eq!(
            Move::from_coordinate_notation("f7f8X"),
            Err(ParseMoveError::InvalidPromotionPiece('X'))
        );
    }

    /// Creates a function to test `Move::from_coordinate_notation`.
    ///
    /// Curly braces are necessary for rustfmt to work, which is nice because it can automatically
    /// wrap long lines.
    macro_rules! test_move_from_coordinate_notation {
        ({ $($name:ident($move:expr, $from:expr, $to:expr, $promotion_piece:expr $(,)?);)+ }) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    let expected = Move::new($from, $to, $promotion_piece);
                    assert_eq!(Move::from_coordinate_notation($move), Ok(expected));
                }
            )*
        };
        () => {};
    }

    test_move_from_coordinate_notation!({
        test_move_from_coordinate_notation_e2e4("e2e4", Square::E2, Square::E4, None);
        test_move_from_coordinate_notation_e4g5("e4g5", Square::E4, Square::G5, None);
        test_move_from_coordinate_notation_f7f8Q(
            "f7f8Q",
            Square::F7,
            Square::F8,
            Some(PieceType::Queen),
        );
        test_move_from_coordinate_notation_f7f8R(
            "f7f8R",
            Square::F7,
            Square::F8,
            Some(PieceType::Rook),
        );
        test_move_from_coordinate_notation_f7f8B(
            "f7f8B",
            Square::F7,
            Square::F8,
            Some(PieceType::Bishop),
        );
        test_move_from_coordinate_notation_f7f8N(
            "f7f8N",
            Square::F7,
            Square::F8,
            Some(PieceType::Knight),
        );
    });
}
