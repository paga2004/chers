use crate::error::FenParsingError;
use crate::position::calculate_index;
use crate::Color;
use crate::Piece;
use crate::Position;

pub(crate) const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Position {
    /// Creates a Position from a [FEN] string or returns an error if the fen is invalid.
    ///
    /// At the moment it ignores the active color, castling rights, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Position;
    ///
    /// let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    /// assert_eq!(position, Position::new());
    /// ```
    ///
    /// [FEN]: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn from_fen(fen: &str) -> Result<Self, FenParsingError> {
        let mut fields = fen.split(' ');

        let mut next_field = || fields.next().ok_or(FenParsingError::TooShort);

        let pieces = parse_pieces(next_field()?)?;
        let color_to_move = parse_color(next_field()?)?;

        Ok(Self {
            pieces,
            color_to_move,
        })
    }

    /// Returns the fen representation of the current position.
    pub fn to_fen(&self) -> String {
        todo!();
    }
}

fn parse_pieces(s: &str) -> Result<[Option<Piece>; 64], FenParsingError> {
    let mut chars = s.chars();
    let mut pieces = [None; 64];

    let mut rank = 7;
    let mut file = 0;
    while !(rank == 0 && file == 8) {
        match chars.next().ok_or(FenParsingError::TooShort)? {
            '/' => {
                if file != 8 {
                    return Err(FenParsingError::WrongNumberOfFiles);
                }
                file = 0;
                rank -= 1;
                continue;
            }
            c @ '0'..='9' => {
                file += c.to_digit(10).unwrap() as usize;
                continue;
            }
            c => {
                let piece = Piece::from_char(c).ok_or(FenParsingError::InvalidPiece(c))?;
                let index = calculate_index(file, rank);
                pieces[index] = Some(piece);
                file += 1;
            }
        }
    }

    Ok(pieces)
}

fn parse_color(s: &str) -> Result<Color, FenParsingError> {
    let c = s.chars().next().ok_or(FenParsingError::TooShort)?;
    Color::from_char(c).ok_or(FenParsingError::InvalidColor(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PieceType;

    /// Returns Option<Piece> from a char. This makes it easier to create Pieces in the following tests.
    fn p(symbol: char) -> Option<Piece> {
        let piece_type = match symbol.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            other => panic!("Invalid piece: {}", other),
        };
        let color = if symbol.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        Some(Piece { piece_type, color })
    }

    #[test]
    fn test_from_fen_empty_input() {
        let fen = "";
        assert_eq!(Position::from_fen(fen), Err(FenParsingError::TooShort));
    }

    #[test]
    fn test_from_fen_not_enough_files() {
        let fen = "rnbqkbnr/pppppppp/7/7/7/7/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(FenParsingError::WrongNumberOfFiles)
        );
    }

    #[test]
    fn test_from_fen_too_many_files() {
        let fen = "rnbqkbnr/pppppppp/9/9/9/9/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(FenParsingError::WrongNumberOfFiles)
        );
    }

    #[test]
    fn test_from_fen_invalid_piece() {
        let fen = "rnbqk?nr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(FenParsingError::InvalidPiece('?'))
        );
    }

    #[test]
    fn test_from_fen_invalid_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(FenParsingError::InvalidColor('x'))
        );
    }

    #[test]
    fn test_from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        #[rustfmt::skip]
        let pieces = [
            p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'),
            p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'),
            p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'),
        ];
        let expected = Position {
            pieces,
            color_to_move: Color::White,
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_e4() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";

        #[rustfmt::skip]
        let pieces = [
            p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'),
            p('P'), p('P'), p('P'), p('P'), None, p('P'), p('P'), p('P'),
            None, None, None, None, None, None, None, None,
            None, None, None, None, p('P'), None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'),
            p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'),
        ];
        let expected = Position {
            pieces,
            color_to_move: Color::Black,
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_e4_c5() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";

        #[rustfmt::skip]
        let pieces = [
            p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'),
            p('P'), p('P'), p('P'), p('P'), None, p('P'), p('P'), p('P'),
            None, None, None, None, None, None, None, None,
            None, None, None, None, p('P'), None, None, None,
            None, None, p('p'), None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            p('p'), p('p'), None, p('p'), p('p'), p('p'), p('p'), p('p'),
            p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'),
        ];
        let expected = Position {
            pieces,
            color_to_move: Color::White,
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_e4_c5_nf3() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

        #[rustfmt::skip]
        let pieces = [
            p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), None, p('R'),
            p('P'), p('P'), p('P'), p('P'), None, p('P'), p('P'), p('P'),
            None, None, None, None, None, p('N'), None, None,
            None, None, None, None, p('P'), None, None, None,
            None, None, p('p'), None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            p('p'), p('p'), None, p('p'), p('p'), p('p'), p('p'), p('p'),
            p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'),
        ];
        let expected = Position {
            pieces,
            color_to_move: Color::Black,
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }
}
