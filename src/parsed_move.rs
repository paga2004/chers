use crate::error::ParseMoveError;
use crate::{PieceType, Square};

use std::fmt;

/// A chess move.
///
/// This type can be created by parsing multiple diffrent formats. For a more efficient
/// representation of a chess move that is used internally by this engine see
/// [`BitMove`](crate::BitMove).
///
/// The move can either be a normal move, a capture, castling, or a promotion.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParsedMove {
    pub(crate) origin: Square,
    pub(crate) target: Square,
    pub(crate) promotion_piece: Option<PieceType>,
}

impl ParsedMove {
    /// Creates a new `ParsedMove`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::{ParsedMove, Square, PieceType};
    ///
    /// let e2e4 = ParsedMove::new(Square::E2, Square::E4, None);
    /// let promotion = ParsedMove::new(Square::F7, Square::F8, Some(PieceType::QUEEN));
    /// ```
    pub fn new(origin: Square, target: Square, promotion_piece: Option<PieceType>) -> Self {
        Self {
            origin,
            target,
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
    /// use chers::ParsedMove;
    ///
    /// let m1 = ParsedMove::from_coordinate_notation("e2e4");
    /// let m2 = ParsedMove::from_coordinate_notation("e1g1"); // white short castling
    /// let m3 = ParsedMove::from_coordinate_notation("e7e8q"); // promotion
    ///
    /// let m4  = ParsedMove::from_coordinate_notation("e4"); // invalid
    /// let m5  = ParsedMove::from_coordinate_notation("Bxe5"); // invalid
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
            .map(|c| {
                PieceType::from_char(c)
                    .ok_or(ParseMoveError::InvalidPromotionPiece(c))
                    .and_then(|p| match p {
                        PieceType::KING | PieceType::PAWN => {
                            Err(ParseMoveError::InvalidPromotionPiece(c))
                        }
                        other => Ok(other),
                    })
            })
            .map_or(Ok(None), |r| r.map(Some))?;

        Ok(Self {
            origin: Square::from_algebraic_notation(from)?,
            target: Square::from_algebraic_notation(to)?,
            promotion_piece,
        })
    }
}

impl fmt::Display for ParsedMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.origin, self.target)?;
        if let Some(p) = self.promotion_piece {
            write!(f, "{}", p.to_char())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::error::ParseSquareError;

    #[test_case(""; "empty string")] // error without the explicit name
    #[test_case("e")]
    #[test_case("e4")]
    #[test_case("e4e")]
    fn from_coordinate_notation_too_short(m: &str) {
        pretty_assertions::assert_eq!(
            ParsedMove::from_coordinate_notation(m),
            Err(ParseMoveError::TooShort)
        );
    }

    #[test_case("x1e2")]
    #[test_case("e1x2")]
    fn from_coordinate_notation_invalid_square(m: &str) {
        pretty_assertions::assert_eq!(
            ParsedMove::from_coordinate_notation(m),
            Err(ParseMoveError::InvalidSquare(
                ParseSquareError::InvalidFile('x')
            ))
        );
    }

    #[test_case("e0e4", '0')]
    #[test_case("e4e0", '0')]
    #[test_case("e9e4", '9')]
    #[test_case("e4e9", '9')]
    fn from_coordinate_notation_invalid_rank(m: &str, c: char) {
        pretty_assertions::assert_eq!(
            ParsedMove::from_coordinate_notation(m),
            Err(ParseMoveError::InvalidSquare(
                ParseSquareError::InvalidRank(c)
            ))
        );
    }

    #[test_case("e7e8x", 'x')]
    #[test_case("e7e8p", 'p')] // promotion to pawn is impossible
    #[test_case("e7e8k", 'k')] // promotion to king is impossible
    fn from_coordinate_notation_invalid_promotion_piece(m: &str, c: char) {
        pretty_assertions::assert_eq!(
            ParsedMove::from_coordinate_notation(m),
            Err(ParseMoveError::InvalidPromotionPiece(c))
        );
    }

    #[test_case("e2e4", Square::E2, Square::E4, None)]
    #[test_case("e4g5", Square::E4, Square::G5, None)]
    #[test_case("f7f8q", Square::F7, Square::F8, Some(PieceType::QUEEN))]
    #[test_case("f7f8r", Square::F7, Square::F8, Some(PieceType::ROOK))]
    #[test_case("f7f8b", Square::F7, Square::F8, Some(PieceType::BISHOP))]
    #[test_case("f7f8n", Square::F7, Square::F8, Some(PieceType::KNIGHT))]
    fn test_move_from_coordinate_notation(
        m: &str,
        from: Square,
        to: Square,
        promotion_piece: Option<PieceType>,
    ) {
        let expected = ParsedMove::new(from, to, promotion_piece);
        pretty_assertions::assert_eq!(ParsedMove::from_coordinate_notation(m), Ok(expected));
    }
}
