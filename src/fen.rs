use std::sync::Arc;

use crate::position_state::PositionState;
use crate::Color;
use crate::File;
use crate::Piece;
use crate::PieceType;
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

        let mut king_square = [Square::A1; 2];
        let mut sq;
        for i in 0..8 {
            for j in 0..8 {
                sq = Square::new(File::new(i), Rank::new(j));
                if pieces[sq].is_type(PieceType::KING) {
                    king_square[pieces[sq].color().to_usize()] = sq;
                }
            }
        }

        Ok(Self {
            pieces,
            king_square,
            side_to_move: active_color,
            ply,
            state,
        })
    }

    /// Returns the fen representation of the current position.
    pub fn to_fen(&self) -> String {
        let mut res = String::new();
        let mut empty_squares = 0;

        for i in 0..8 {
            if i > 0 {
                res.push('/');
            }
            for j in 0..8 {
                let sq = Square::new(File::new(j), Rank::new(7 - i));
                if self.pieces[sq] == Piece::EMPTY {
                    empty_squares += 1u8;
                } else {
                    if empty_squares != 0 {
                        debug_assert!(empty_squares <= 8);
                        res.push((b'0' + empty_squares) as char);
                        empty_squares = 0;
                    }
                    res.push(self.pieces[sq].to_char());
                }
            }
            if empty_squares != 0 {
                debug_assert!(empty_squares <= 8);
                res.push((b'0' + empty_squares) as char);
                empty_squares = 0;
            }
        }

        let fullmove_number = (self.ply + 1) / 2;
        format!(
            "{} {} {} {} {} {}",
            res,
            self.side_to_move.to_char(),
            self.state.castling_rights,
            self.state.ep_square,
            self.state.halfmove_clock,
            fullmove_number
        )
    }
}

fn parse_pieces(s: &str) -> Result<[Piece; 120], ParseFenError<'_>> {
    let mut chars = s.chars();
    let mut pieces = [Piece::OFF_BOARD; 120];

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
                    pieces[Square::new(File::new(file), Rank::new(rank))] = Piece::EMPTY;
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
                pieces[Square::new(File::new(file), Rank::new(rank))] = piece;
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
    let mut white_king_side = false;
    let mut white_queen_side = false;
    let mut black_king_side = false;
    let mut black_queen_side = false;

    if s != "-" {
        for c in s.chars() {
            match c {
                'K' => white_king_side = true,
                'Q' => white_queen_side = true,
                'k' => black_king_side = true,
                'q' => black_queen_side = true,

                _ => return Err(ParseFenError::InvalidCastlingRights(s)),
            }
        }
    }

    Ok(CastlingRights::new(
        white_king_side,
        white_queen_side,
        black_king_side,
        black_queen_side,
    ))
}

fn parse_en_passant_square(s: &str) -> Result<Square, ParseFenError<'_>> {
    if s == "-" {
        return Ok(Square::NO_SQ);
    }
    Ok(Square::from_algebraic_notation(s)?)
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
    use crate::utils;
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
        Color::WHITE,
        Square::NO_SQ,
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
        Color::BLACK,
        Square::E3,
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
        Color::WHITE,
        Square::C6,
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
        Color::BLACK,
        Square::NO_SQ,
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
        Color::WHITE,
        Square::NO_SQ,
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
        Color::WHITE,
        Square::NO_SQ,
        CastlingRights::new(false, false, false, false),
        0,
        1
        ; "no castling rights"
    )]
    fn test_from_fen(
        fen: &str,
        pieces: [&str; 12],
        side_to_move: Color,
        en_passant_square: Square,
        castling_rights: CastlingRights,
        halfmove_clock: u16,
        ply: u16,
    ) {
        let bytes: Vec<&[u8]> = pieces.iter().map(|s| s.as_bytes()).collect();
        let mut piece_array = [Piece::OFF_BOARD; 120];
        for i in 0..12 {
            for j in 0..10 {
                piece_array[10 * i + j] = match bytes[i][j] {
                    b'-' => Piece::OFF_BOARD,
                    b' ' => Piece::EMPTY,
                    other => Piece::from_char(other as char).expect("valid piece"),
                }
            }
        }
        let mut king_square = [Square::A1; 2];
        let mut sq;
        for i in 0..8 {
            for j in 0..8 {
                sq = Square::new(File::new(i), Rank::new(j));
                if piece_array[sq].is_type(PieceType::KING) {
                    king_square[piece_array[sq].color().to_usize()] = sq;
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
            king_square,
            side_to_move,
            ply,
            state,
        };

        pretty_assertions::assert_eq!(Position::from_fen(fen).expect("valid position"), expected);
    }

    #[test_case(utils::fen::STARTING_POSITION; "starting position")]
    #[test_case(utils::fen::KIWIPETE; "kiwipete")]
    fn test_to_fen(fen: &str) {
        let pos = Position::from_fen(fen).unwrap();
        pretty_assertions::assert_eq!(pos.to_fen(), fen);
    }
}
