use crate::piece::Piece;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Position {
    pieces: [Piece; 64],
}

impl Position {
    /// Creates a new Position
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    /// Creates a Position from a [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) string.
    /// At the moment it ignores the active color, castling rights, etc.
    ///
    /// # Panics
    ///
    /// Panics if the fen is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Position;
    ///
    /// let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// assert_eq!(position, Position::new());
    /// ```
    pub fn from_fen(fen: &str) -> Self {
        let mut pieces = [Piece::None; 64];
        let mut i = 0;
        let mut chars = fen.chars();
        while i < 64 {
            let c = chars.next().expect("Invalid FEN: too short");
            if let Some(number) = c.to_digit(10) {
                i += number as usize;
                continue;
            }
            if c == '/' {
                assert!(i % 8 == 0, "Invalid FEN: wrong number of files");
                continue;
            }
            let piece = match c {
                'P' => Piece::WhitePawn,
                'N' => Piece::WhiteKnight,
                'B' => Piece::WhiteBishop,
                'R' => Piece::WhiteRook,
                'Q' => Piece::WhiteQueen,
                'K' => Piece::WhiteKing,
                'p' => Piece::BlackPawn,
                'n' => Piece::BlackKnight,
                'b' => Piece::BlackBishop,
                'r' => Piece::BlackRook,
                'q' => Piece::BlackQueen,
                'k' => Piece::BlackKing,
                other => panic!("Invalid FEN: unexpected character {}", other),
            };
            pieces[i] = piece;
            i += 1;
        }
        Self { pieces }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..64 {
            write!(f, "{}", self.pieces[i])?;
            if i % 8 == 7 {
                write!(f, "\n")?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        use Piece::*;

        #[rustfmt::skip]
        let pieces = [
            BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook,
            BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn,
            WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook,
        ];
        let expected = Position { pieces };
        assert_eq!(Position::new(), expected);
    }

    #[test]
    #[should_panic(expected = "Invalid FEN: too short")]
    fn test_from_fen_empty_input() {
        let fen = "";
        let _ = Position::from_fen(fen);
    }

    #[test]
    #[should_panic(expected = "Invalid FEN: wrong number of files")]
    fn test_from_fen_wrong_number_of_files() {
        let fen = "rnbqkbnr/pppppppp/7/7/7/7/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let _ = Position::from_fen(fen);
    }

    #[test]
    #[should_panic(expected = "Invalid FEN: unexpected character ?")]
    fn test_from_fen_empty_unexpected_character() {
        let fen = "rnbqk?nr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let _ = Position::from_fen(fen);
    }

    #[test]
    fn test_from_fen_starting_position() {
        use Piece::*;

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        #[rustfmt::skip]
        let pieces = [
            BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook,
            BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn, WhitePawn,
            WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook,
        ];
        let expected = Position { pieces };

        assert_eq!(Position::from_fen(fen), expected);
    }

    #[test]
    fn test_from_fen_e4() {
        use Piece::*;

        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";

        #[rustfmt::skip]
        let pieces = [
            BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook,
            BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, WhitePawn, None, None, None,
            None, None, None, None, None, None, None, None,
            WhitePawn, WhitePawn, WhitePawn, WhitePawn, None, WhitePawn, WhitePawn, WhitePawn,
            WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook,
        ];
        let expected = Position { pieces };

        assert_eq!(Position::from_fen(fen), expected);
    }

    #[test]
    fn test_from_fen_e4_c5() {
        use Piece::*;

        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";

        #[rustfmt::skip]
        let pieces = [
            BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook,
            BlackPawn, BlackPawn, None, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn,
            None, None, None, None, None, None, None, None,
            None, None, BlackPawn, None, None, None, None, None,
            None, None, None, None, WhitePawn, None, None, None,
            None, None, None, None, None, None, None, None,
            WhitePawn, WhitePawn, WhitePawn, WhitePawn, None, WhitePawn, WhitePawn, WhitePawn,
            WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, WhiteKnight, WhiteRook,
        ];
        let expected = Position { pieces };

        assert_eq!(Position::from_fen(fen), expected);
    }

    #[test]
    fn test_from_fen_e4_c5_nf3() {
        use Piece::*;

        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

        #[rustfmt::skip]
        let pieces = [
            BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackBishop, BlackKnight, BlackRook,
            BlackPawn, BlackPawn, None, BlackPawn, BlackPawn, BlackPawn, BlackPawn, BlackPawn,
            None, None, None, None, None, None, None, None,
            None, None, BlackPawn, None, None, None, None, None,
            None, None, None, None, WhitePawn, None, None, None,
            None, None, None, None, None, WhiteKnight, None, None,
            WhitePawn, WhitePawn, WhitePawn, WhitePawn, None, WhitePawn, WhitePawn, WhitePawn,
            WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhiteBishop, None, WhiteRook,
        ];
        let expected = Position { pieces };

        assert_eq!(Position::from_fen(fen), expected);
    }

    #[test]
    fn test_from_fen_all_none() {
        let fen = "8/8/8/8/8/8/8/8";
        assert_eq!(
            Position::from_fen(fen),
            Position {
                pieces: [Piece::None; 64]
            }
        );
    }

    #[test]
    fn test_display() {
        let expected = "r n b q k b n r\n\
                        p p p p p p p p\n\
                        _ _ _ _ _ _ _ _\n\
                        _ _ _ _ _ _ _ _\n\
                        _ _ _ _ _ _ _ _\n\
                        _ _ _ _ _ _ _ _\n\
                        P P P P P P P P\n\
                        R N B Q K B N R\n";
        assert_eq!(format!("{}", Position::new()), expected);
    }
}
