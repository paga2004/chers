use std::sync::Arc;

use crate::position::BoardState;
use crate::position_state::PositionState;
use crate::Color;
use crate::File;
use crate::Piece;
use crate::Position;
use crate::Rank;
use crate::Square;
use crate::{castling_rights::CastlingRights, error::ParseFenError};

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
    pub fn from_fen(fen: &str) -> Result<Self, ParseFenError<'_>> {
        let mut fields = fen.split_whitespace();

        let mut next_field = || fields.next().ok_or(ParseFenError::TooShort);

        let pieces = parse_pieces(next_field()?)?;
        let active_color = parse_color(next_field()?)?;
        let castling_rights = parse_castling_rights(next_field()?)?;
        let en_passant_square = parse_en_passant_square(next_field()?)?;
        let halfmove_clock = parse_halfmove_clock(next_field()?)?;
        let fullmove_number = parse_fullmove_number(next_field()?)?;

        let ply = fullmove_number * 2 - active_color.map(1, 0);

        let state = Arc::new(PositionState::new(
            castling_rights,
            en_passant_square,
            halfmove_clock,
        ));
        Ok(Self {
            pieces,
            side_to_move: active_color,
            ply,
            state,
        })
    }

    /// Returns the fen representation of the current position.
    pub fn to_fen(&self) -> String {
        todo!();
    }
}

// TODO: Rewrite this function
fn parse_pieces(s: &str) -> Result<[BoardState; 120], ParseFenError<'_>> {
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
                if file > 7 {
                    return Err(ParseFenError::WrongNumberOfFiles);
                }
                for _ in 0..c.to_digit(10).unwrap() {
                    pieces[Square::new(File::new(file), Rank::new(rank))] = BoardState::Empty;
                    file += 1;
                }
                continue;
            }
            '9' => return Err(ParseFenError::WrongNumberOfFiles),
            c => {
                if file > 7 {
                    return Err(ParseFenError::WrongNumberOfFiles);
                }
                let piece = Piece::from_char(c).ok_or(ParseFenError::InvalidPiece(c))?;
                pieces[Square::new(File::new(file), Rank::new(rank))] = BoardState::Piece(piece);
                file += 1;
            }
        }
    }

    Ok(pieces)
}

fn parse_color(s: &str) -> Result<Color, ParseFenError<'_>> {
    let c = s.chars().next().ok_or(ParseFenError::TooShort)?;
    Color::from_char(c).ok_or(ParseFenError::InvalidColor(c))
}

fn parse_castling_rights(s: &str) -> Result<CastlingRights, ParseFenError<'_>> {
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

                _ => return Err(ParseFenError::InvalidCastlingRights(s)),
            }
        }
    }

    Ok(castling_rights)
}

fn parse_en_passant_square(s: &str) -> Result<Option<Square>, ParseFenError<'_>> {
    if s == "-" {
        return Ok(None);
    }
    Ok(Some(Square::from_algebraic_notation(s)?))
}

fn parse_halfmove_clock(s: &str) -> Result<u16, ParseFenError<'_>> {
    s.parse()
        .map_err(|_| ParseFenError::InvalidHalfmoveClock(s))
}

fn parse_fullmove_number(s: &str) -> Result<u16, ParseFenError<'_>> {
    s.parse()
        .map_err(|_| ParseFenError::InvalidFullmoveNumber(s))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use ParseFenError::*;

    #[test_case("", TooShort; "too short")]
    #[test_case("rnbqkbnr/pppppppp/7/7/7/7/PPPPPPPP/RNBQKBNR w KQkq - 0 1", WrongNumberOfFiles; "not enough files")]
    #[test_case("rnbqkbnr/p7p/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", WrongNumberOfFiles; "too many files")]
    #[test_case("rnbqkbnr/pppppppp/9/9/9/9/PPPPPPPP/RNBQKBNR w KQkq - 0 1", WrongNumberOfFiles; "digit 9 in first sector")]
    #[test_case("rnbqk?nr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", InvalidPiece('?'); "invalid piece")]
    #[test_case("k7/8/8/8/8/8/8/k7 x KQkq - 0 1", InvalidColor('x'); "invalid color")]
    #[test_case("k7/8/8/8/8/8/8/k7 w Kx - 0 1", InvalidCastlingRights("Kx"); "invalid castling rights x")]
    #[test_case("k7/8/8/8/8/8/8/k7 w KQkqx - 0 1", InvalidCastlingRights("KQkqx"); "invalid castling rights trailing character")]
    fn test_from_fen_invalid(fen: &str, err: ParseFenError<'_>) {
        pretty_assertions::assert_eq!(Position::from_fen(fen), Err(err));
    }

    #[test_case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        [
            "----------",
            "----------",
            "-RNBQKBNR-",
            "-PPPPPPPP-",
            "-        -",
            "-        -",
            "-        -",
            "-        -",
            "-pppppppp-",
            "-rnbqkbnr-",
            "----------",
            "----------",
        ],
        Color::White,
        None,
        CastlingRights::default(),
        0,
        1
        ; "starting position"
    )]
    #[test_case(
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        [
            "----------",
            "----------",
            "-RNBQKBNR-",
            "-PPPP PPP-",
            "-        -",
            "-    P   -",
            "-        -",
            "-        -",
            "-pppppppp-",
            "-rnbqkbnr-",
            "----------",
            "----------",
        ],
        Color::Black,
        Some(Square::E3),
        CastlingRights::default(),
        0,
        2
        ; "e4"
    )]
    #[test_case(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        [
            "----------",
            "----------",
            "-RNBQKBNR-",
            "-PPPP PPP-",
            "-        -",
            "-    P   -",
            "-  p     -",
            "-        -",
            "-pp ppppp-",
            "-rnbqkbnr-",
            "----------",
            "----------",
        ],
        Color::White,
        Some(Square::C6),
        CastlingRights::default(),
        0,
        3
        ; "e4 c5"
    )]
    #[test_case(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        [
           "----------",
           "----------",
           "-RNBQKB R-",
           "-PPPP PPP-",
           "-     N  -",
           "-    P   -",
           "-  p     -",
           "-        -",
           "-pp ppppp-",
           "-rnbqkbnr-",
           "----------",
           "----------",
        ],
        Color::Black,
        None,
        CastlingRights::default(),
        1,
        4
        ; "e4 c5 nf3"
    )]
    #[test_case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 1",
        [
           "----------",
           "----------",
           "-RNBQKBNR-",
           "-PPPPPPPP-",
           "-        -",
           "-        -",
           "-        -",
           "-        -",
           "-pppppppp-",
           "-rnbqkbnr-",
           "----------",
           "----------",
        ],
        Color::White,
        None,
        CastlingRights::new(true, false, false, true),
        0,
        1
        ; "castling rights"
    )]
    #[test_case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        [
           "----------",
           "----------",
           "-RNBQKBNR-",
           "-PPPPPPPP-",
           "-        -",
           "-        -",
           "-        -",
           "-        -",
           "-pppppppp-",
           "-rnbqkbnr-",
           "----------",
           "----------",
        ],
        Color::White,
        None,
        CastlingRights::new(false, false, false, false),
        0,
        1
        ; "no castling rights"
    )]
    fn test_from_fen(
        fen: &str,
        pieces: [&str; 12],
        side_to_move: Color,
        en_passant_square: Option<Square>,
        castling_rights: CastlingRights,
        halfmove_clock: u16,
        ply: u16,
    ) {
        let bytes: Vec<&[u8]> = pieces.iter().map(|s| s.as_bytes()).collect();
        let mut piece_array = [BoardState::OffBoard; 120];
        for i in 0..12 {
            for j in 0..10 {
                piece_array[10 * i + j] = match bytes[i][j] {
                    b'-' => BoardState::OffBoard,
                    b' ' => BoardState::Empty,
                    other => {
                        BoardState::Piece(Piece::from_char(other as char).expect("valid piece"))
                    }
                }
            }
        }
        let state = Arc::new(PositionState::new(
            castling_rights,
            en_passant_square,
            halfmove_clock,
        ));
        let expected = Position {
            pieces: piece_array,
            side_to_move,
            ply,
            state,
        };

        pretty_assertions::assert_eq!(Position::from_fen(fen).expect("valid position"), expected);
    }
}
