use crate::position::BoardState;
use crate::Color;
use crate::File;
use crate::Piece;
use crate::Position;
use crate::Rank;
use crate::Square;
use crate::{castling_rights::CastlingRights, error::ParseFenError};

pub(crate) const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Position {
    /// Creates a Position from a [FEN] string or returns an error if the fen is invalid.
    ///
    /// At the moment it ignores halfmove clock and fullmove number.
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
    pub fn from_fen(fen: &str) -> Result<Self, ParseFenError> {
        let mut fields = fen.split(' ');

        let mut next_field = || fields.next().ok_or(ParseFenError::TooShort);

        let pieces = parse_pieces(next_field()?)?;
        let active_color = parse_color(next_field()?)?;
        let castling_rights = parse_castling_rights(next_field()?)?;
        let en_passant_square = parse_en_passant_square(next_field()?)?;

        Ok(Self {
            pieces,
            side_to_move: active_color,
            castling_rights,
            en_passant_square,
        })
    }

    /// Returns the fen representation of the current position.
    pub fn to_fen(&self) -> String {
        todo!();
    }
}

fn parse_pieces(s: &str) -> Result<[BoardState; 120], ParseFenError> {
    let mut chars = s.chars();
    let mut pieces = [BoardState::OffBoard; 120];

    let mut rank = 7;
    let mut file = 0;
    while !(rank == 0 && file == 8) {
        match chars.next().ok_or(ParseFenError::TooShort)? {
            '/' => {
                if file != 8 {
                    return Err(ParseFenError::WrongNumberOfFiles);
                }
                file = 0;
                rank -= 1;
                continue;
            }
            c @ '0'..='8' => {
                for _ in 0..c.to_digit(10).unwrap() {
                    pieces[Square::new(File::new(file), Rank::new(rank))] = BoardState::Empty;
                    file += 1;
                }
                continue;
            }
            '9' => return Err(ParseFenError::WrongNumberOfFiles),
            c => {
                let piece = Piece::from_char(c).ok_or(ParseFenError::InvalidPiece(c))?;
                pieces[Square::new(File::new(file), Rank::new(rank))] = BoardState::Piece(piece);
                file += 1;
            }
        }
    }

    Ok(pieces)
}

fn parse_color(s: &str) -> Result<Color, ParseFenError> {
    let c = s.chars().next().ok_or(ParseFenError::TooShort)?;
    Color::from_char(c).ok_or(ParseFenError::InvalidColor(c))
}

fn parse_castling_rights(s: &str) -> Result<CastlingRights, ParseFenError> {
    let mut castling_rights = CastlingRights {
        white_king_side: false,
        white_queen_side: false,
        black_king_side: false,
        black_queen_side: false,
    };

    if s != "-" {
        for c in s.chars() {
            match c {
                'K' => castling_rights.white_king_side = true,
                'Q' => castling_rights.white_queen_side = true,
                'k' => castling_rights.black_king_side = true,
                'q' => castling_rights.black_queen_side = true,

                other => return Err(ParseFenError::InvalidCastlingRights(other)),
            }
        }
    }

    Ok(castling_rights)
}

fn parse_en_passant_square(s: &str) -> Result<Option<Square>, ParseFenError> {
    if s == "-" {
        return Ok(None);
    }
    Ok(Some(Square::from_algebraic_notation(s)?))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::PieceType;

    /// Returns Option<Piece> from a char.
    ///
    /// This makes it easier to create Pieces in the following tests.
    fn p(symbol: char) -> BoardState {
        match symbol {
            ' ' => BoardState::Empty,
            '-' => BoardState::OffBoard,
            other => {
                let piece_type = match other.to_ascii_lowercase() {
                    'p' => PieceType::Pawn,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    other => panic!("invalid piece: {}", other),
                };
                let color = if symbol.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                BoardState::Piece(Piece { piece_type, color })
            }
        }
    }

    #[test]
    fn test_from_fen_empty_input() {
        let fen = "";
        assert_eq!(Position::from_fen(fen), Err(ParseFenError::TooShort));
    }

    #[test]
    fn test_from_fen_not_enough_files() {
        let fen = "rnbqkbnr/pppppppp/7/7/7/7/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(ParseFenError::WrongNumberOfFiles)
        );
    }

    #[test]
    fn test_from_fen_too_many_files() {
        let fen = "rnbqkbnr/pppppppp/9/9/9/9/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(ParseFenError::WrongNumberOfFiles)
        );
    }

    #[test]
    fn test_from_fen_invalid_piece() {
        let fen = "rnbqk?nr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(ParseFenError::InvalidPiece('?'))
        );
    }

    #[test]
    fn test_from_fen_invalid_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert_eq!(
            Position::from_fen(fen),
            Err(ParseFenError::InvalidColor('x'))
        );
    }

    #[test]
    fn test_from_fen_invalid_castling_rights() {
        let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1";
        assert_eq!(
            Position::from_fen(fen1),
            Err(ParseFenError::InvalidCastlingRights('x'))
        );
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w x - 0 1";
        assert_eq!(
            Position::from_fen(fen2),
            Err(ParseFenError::InvalidCastlingRights('x'))
        );
    }

    #[test]
    fn test_from_fen_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        #[rustfmt::skip]
        let pieces = [
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'), p('-'),
            p('-'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('-'),
            p('-'), p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
        ];
        let expected = Position {
            pieces,
            side_to_move: Color::White,
            ..Default::default()
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_e4() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";

        #[rustfmt::skip]
        let pieces = [
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'), p('-'),
            p('-'), p('P'), p('P'), p('P'), p('P'), p(' '), p('P'), p('P'), p('P'), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p('P'), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('-'),
            p('-'), p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
        ];
        let expected = Position {
            pieces,
            side_to_move: Color::Black,
            en_passant_square: Some(Square::E3),
            ..Default::default()
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_e4_c5() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";

        #[rustfmt::skip]
        let pieces = [
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'), p('-'),
            p('-'), p('P'), p('P'), p('P'), p('P'), p(' '), p('P'), p('P'), p('P'), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p('P'), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p('p'), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p('p'), p('p'), p(' '), p('p'), p('p'), p('p'), p('p'), p('p'), p('-'),
            p('-'), p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
        ];
        let expected = Position {
            pieces,
            side_to_move: Color::White,
            en_passant_square: Some(Square::C6),
            ..Default::default()
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_e4_c5_nf3() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

        #[rustfmt::skip]
        let pieces = [
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p(' '), p('R'), p('-'),
            p('-'), p('P'), p('P'), p('P'), p('P'), p(' '), p('P'), p('P'), p('P'), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p('N'), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p('P'), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p('p'), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p('p'), p('p'), p(' '), p('p'), p('p'), p('p'), p('p'), p('p'), p('-'),
            p('-'), p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
        ];
        let expected = Position {
            pieces,
            side_to_move: Color::Black,
            ..Default::default()
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 1";

        #[rustfmt::skip]
        let pieces = [
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'), p('-'),
            p('-'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('-'),
            p('-'), p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
        ];
        let expected = Position {
            pieces,
            side_to_move: Color::White,
            castling_rights: CastlingRights::new(true, false, false, true),
            ..Default::default()
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }

    #[test]
    fn test_from_fen_no_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";

        #[rustfmt::skip]
        let pieces = [
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('R'), p('N'), p('B'), p('Q'), p('K'), p('B'), p('N'), p('R'), p('-'),
            p('-'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('P'), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p(' '), p('-'),
            p('-'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('p'), p('-'),
            p('-'), p('r'), p('n'), p('b'), p('q'), p('k'), p('b'), p('n'), p('r'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
            p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'), p('-'),
        ];
        let expected = Position {
            pieces,
            side_to_move: Color::White,
            castling_rights: CastlingRights::new(false, false, false, false),
            ..Default::default()
        };

        assert_eq!(Position::from_fen(fen), Ok(expected));
    }
}
