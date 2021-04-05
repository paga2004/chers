use crate::piece::{Color, Piece, PieceType};
use crate::position::{calculate_index, Position};

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
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut sectors = fen.split(' ');

        let mut pieces = [None; 64];
        let mut piece_chars = sectors.next().ok_or("FEN too short")?.chars();
        let mut rank = 7;
        let mut file = 0;
        while rank != 0 || file != 8 {
            let c = piece_chars.next().ok_or("FEN too short")?;

            if c == '/' {
                if file != 8 {
                    return Err("Wrong number of files".to_string());
                }
                file = 0;
                rank -= 1;
                continue;
            }
            if let Some(number) = c.to_digit(10) {
                file += number as usize;
                continue;
            }
            let piece_type = match c.to_ascii_lowercase() {
                'p' => PieceType::Pawn,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'r' => PieceType::Rook,
                'q' => PieceType::Queen,
                'k' => PieceType::King,
                other => {
                    return Err(format!("Invalid piece: {}", other));
                }
            };
            let color = if c.is_uppercase() {
                Color::White
            } else {
                Color::Black
            };
            let index = calculate_index(file, rank);
            pieces[index] = Some(Piece { piece_type, color });
            file += 1;
        }
        let color_to_move = match sectors.next().ok_or("FEN too short")? {
            "w" => Color::White,
            "b" => Color::Black,
            other => return Err(format!("Invalid color: {}", other)),
        };

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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(Position::from_fen(fen), Err("FEN too short".to_string()));
    }

    #[test]
    fn test_from_fen_wrong_number_of_files() {
        let fen = "rnbqkbnr/pppppppp/7/7/7/7/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err("Wrong number of files".to_string())
        );
    }

    #[test]
    fn test_from_fen_empty_unexpected_character() {
        let fen = "rnbqk?nr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(Position::from_fen(fen), Err("Invalid piece: ?".to_string()));
    }

    #[test]
    fn test_from_fen_invalid_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert_eq!(Position::from_fen(fen), Err("Invalid color: x".to_string()));
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
