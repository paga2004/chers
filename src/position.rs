use crate::piece::Piece;
use std::fmt;

pub struct Position {
    pieces: [Piece; 64],
}

impl Position {
    pub fn new() -> Self {
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
