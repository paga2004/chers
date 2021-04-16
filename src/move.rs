use crate::error::MoveParsingError;
use crate::File;
use crate::PieceType;
use crate::Rank;
use crate::Square;

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

    /// Creates a new `Move` from [Smith Notation].
    ///
    /// Returns an `None` if the string is not a valid move. However, it doesn't check if the move
    /// is legal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::error::MoveParsingError;
    /// use chers::Move;
    ///
    /// let m1 = Move::from_smith_notation("e2e4");
    /// let m2 = Move::from_smith_notation("e2e9");
    ///
    /// assert!(m1.is_ok());
    /// assert_eq!(m2, Err(MoveParsingError::InvalidRank('9')));
    /// ```
    ///
    /// [Smith Notation]: https://web.archive.org/web/20160117212352/https://www.chessclub.com/chessviewer/smith.html
    pub fn from_smith_notation(m: &str) -> Result<Self, MoveParsingError> {
        let mut chars = m.chars();
        let mut c;
        let mut next_char = || chars.next().ok_or(MoveParsingError::TooShort);

        c = next_char()?;
        let from_file = File::from_char(c).ok_or(MoveParsingError::InvalidFile(c))?;

        c = next_char()?;
        let from_rank = Rank::from_char(c).ok_or(MoveParsingError::InvalidRank(c))?;

        c = next_char()?;
        let to_file = File::from_char(c).ok_or(MoveParsingError::InvalidFile(c))?;

        c = next_char()?;
        let to_rank = Rank::from_char(c).ok_or(MoveParsingError::InvalidRank(c))?;

        let promotion_info = match chars.next() {
            Some(c) if c.is_ascii_lowercase() => {
                let _ = PieceType::from_char(c)
                    .ok_or(MoveParsingError::InvalidCaptureIndicatior(c))
                    .map(|_| ())?;

                chars.next()
            }
            other => other,
        };

        let promotion_piece = if let Some(c) = promotion_info {
            Some(PieceType::from_char(c).ok_or(MoveParsingError::InvalidPromotionPiece(c))?)
        } else {
            None
        };

        Ok(Self {
            from: Square::new(from_file, from_rank),
            to: Square::new(to_file, to_rank),
            promotion_piece,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_smith_notation_too_short() {
        assert_eq!(
            Move::from_smith_notation(""),
            Err(MoveParsingError::TooShort)
        );
        assert_eq!(
            Move::from_smith_notation("e2"),
            Err(MoveParsingError::TooShort)
        );
    }

    #[test]
    fn from_smith_notation_invalid_file() {
        assert_eq!(
            Move::from_smith_notation("x1e2"),
            Err(MoveParsingError::InvalidFile('x'))
        );
        assert_eq!(
            Move::from_smith_notation("e1x2"),
            Err(MoveParsingError::InvalidFile('x'))
        );
    }

    #[test]
    fn from_smith_notation_invalid_rank() {
        assert_eq!(
            Move::from_smith_notation("e2e9"),
            Err(MoveParsingError::InvalidRank('9'))
        );
        assert_eq!(
            Move::from_smith_notation("e0e2"),
            Err(MoveParsingError::InvalidRank('0'))
        );
    }

    #[test]
    fn from_smith_notation_invalid_capture_indicator() {
        assert_eq!(
            Move::from_smith_notation("f7f8x"),
            Err(MoveParsingError::InvalidCaptureIndicatior('x'))
        );
        assert_eq!(
            Move::from_smith_notation("f7f8xQ"),
            Err(MoveParsingError::InvalidCaptureIndicatior('x'))
        );
    }

    #[test]
    fn from_smith_notation_invalid_promotion_piece() {
        assert_eq!(
            Move::from_smith_notation("f7f8pX"),
            Err(MoveParsingError::InvalidPromotionPiece('X'))
        );
        assert_eq!(
            Move::from_smith_notation("f7f8X"),
            Err(MoveParsingError::InvalidPromotionPiece('X'))
        );
    }

    /// Creates a function to test `Move::from_smith_notation`.
    ///
    /// Curly braces are necessary for rustfmt to work, which is nice because it can automatically
    /// wrap long lines.
    macro_rules! test_move_from_smith_notation {
        ({ $($name:ident($move:expr, $from:expr, $to:expr, $promotion_piece:expr $(,)?);)+ }) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    let expected = Move::new($from, $to, $promotion_piece);
                    assert_eq!(Move::from_smith_notation($move), Ok(expected));
                }
            )*
        };
        () => {};
    }

    test_move_from_smith_notation!({
        test_move_from_smith_notation_e2e4("e2e4", Square::E2, Square::E4, None);
        test_move_from_smith_notation_e4g5p("e4g5p", Square::E4, Square::G5, None);
        test_move_from_smith_notation_f7f8Q(
            "f7f8Q",
            Square::F7,
            Square::F8,
            Some(PieceType::Queen),
        );
        test_move_from_smith_notation_f7f8nQ(
            "f7f8nQ",
            Square::F7,
            Square::F8,
            Some(PieceType::Queen),
        );
        test_move_from_smith_notation_f7f8R("f7f8R", Square::F7, Square::F8, Some(PieceType::Rook));
        test_move_from_smith_notation_f7f8B(
            "f7f8B",
            Square::F7,
            Square::F8,
            Some(PieceType::Bishop),
        );
        test_move_from_smith_notation_f7f8N(
            "f7f8N",
            Square::F7,
            Square::F8,
            Some(PieceType::Knight),
        );
    });
}
