use crate::piece::PieceType;
use crate::square::Square;

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
    /// use chers::Move;
    ///
    /// let m1 = Move::from_smith_notation("e2e4");
    /// let m2 = Move::from_smith_notation("e2e9");
    ///
    /// assert!(m1.is_some());
    /// assert!(m2.is_none());
    /// ```
    ///
    /// [Smith Notation]: https://web.archive.org/web/20160117212352/https://www.chessclub.com/chessviewer/smith.html
    pub fn from_smith_notation(m: &str) -> Option<Self> {
        let mut chars = m.chars();

        let mut c = chars.next()?;
        if c < 'a' || c > 'h' {
            return None;
        }
        let from_file = c as usize - 'a' as usize;

        c = chars.next()?;
        if c < '1' || c > '8' {
            return None;
        }
        let from_rank = c.to_digit(10)? as usize - 1;

        c = chars.next()?;
        if c < 'a' || c > 'h' {
            return None;
        }
        let to_file = c as usize - 'a' as usize;

        c = chars.next()?;
        if c < '1' || c > '8' {
            return None;
        }
        let to_rank = c.to_digit(10)? as usize - 1;

        let mut promotion_piece = None;
        if let Some(c) = chars.next() {
            let promotion_information;
            if "pnbrqkEcC".contains(c) {
                // capture indicator
                promotion_information = chars.next();
            } else {
                promotion_information = Some(c);
            }
            if let Some(c) = promotion_information {
                promotion_piece = match c {
                    'N' => Some(PieceType::Knight),
                    'B' => Some(PieceType::Bishop),
                    'R' => Some(PieceType::Rook),
                    'Q' => Some(PieceType::Queen),
                    _ => {
                        return None;
                    }
                };
            }
        }

        // too long
        if chars.next().is_some() {
            return None;
        }

        Some(Self {
            from: Square::new(from_file, from_rank),
            to: Square::new(to_file, to_rank),
            promotion_piece,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_smith_notation_invalid() {
        assert_eq!(Move::from_smith_notation(""), None);
        assert_eq!(Move::from_smith_notation("e2"), None);
        assert_eq!(Move::from_smith_notation("e2e9"), None);
        assert_eq!(Move::from_smith_notation("e0e2"), None);
        assert_eq!(Move::from_smith_notation("A1e2"), None);
        assert_eq!(Move::from_smith_notation("e1A2"), None);
        assert_eq!(Move::from_smith_notation("f7f8P"), None);
        assert_eq!(Move::from_smith_notation("f7f8px"), None);
        assert_eq!(Move::from_smith_notation("f7f8x"), None);
        assert_eq!(Move::from_smith_notation("f7f8xQ"), None);
        assert_eq!(Move::from_smith_notation("f7e8Qx"), None);
        assert_eq!(Move::from_smith_notation("f7e8rQ "), None);
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
                    assert_eq!(Move::from_smith_notation($move), Some(expected));
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
