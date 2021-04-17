use crate::position::BoardState;
use crate::Color;
use crate::PieceType;
use crate::Position;
use crate::Square;

const KNIGHT_OFFSETS: [i8; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
const BISHOP_OFFSETS: [i8; 4] = [-11, -9, 9, 11];
const ROOK_OFFSETS: [i8; 4] = [-10, -1, 1, 10];
const KING_OFFSETS: [i8; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

const WHITE_PAWN_OFFSETS: [i8; 2] = [-9, -11];
const BLACK_PAWN_OFFSETS: [i8; 2] = [9, 11];

impl Position {
    /// Returns wether a given `Square` is attacked by any piece of a given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::{Position, Square, Color};
    /// let mut position = Position::new();
    ///
    /// assert!(position.is_attacked(Square::E6, Color::Black));
    /// assert!(position.is_attacked(Square::E3, Color::White));
    /// assert!(!position.is_attacked(Square::E3, Color::Black));
    ///
    ///
    /// ```
    pub fn is_attacked(&self, square: Square, attacker: Color) -> bool {
        let index = square as usize;

        // pawns
        for offset in &attacker.fold(WHITE_PAWN_OFFSETS, BLACK_PAWN_OFFSETS) {
            if let BoardState::Piece(p) = self.pieces[(index as i8 + offset) as usize] {
                if p.is_type(PieceType::Pawn) && p.is_color(attacker) {
                    return true;
                }
            }
        }

        // knights
        for offset in &KNIGHT_OFFSETS {
            if let BoardState::Piece(p) = self.pieces[(index as i8 + offset) as usize] {
                if p.is_type(PieceType::Knight) && p.is_color(attacker) {
                    return true;
                }
            }
        }

        // bishops and queens
        for offset in &BISHOP_OFFSETS {
            let mut target_square = (index as i8 + offset) as usize;
            let mut state = self.pieces[target_square];
            while state != BoardState::OffBoard {
                if let BoardState::Piece(p) = state {
                    if (p.is_type(PieceType::Bishop) || p.is_type(PieceType::Queen))
                        && p.is_color(attacker)
                    {
                        return true;
                    }
                    break;
                }
                target_square = (target_square as i8 + offset) as usize;
                state = self.pieces[target_square];
            }
        }

        // rooks and queens
        for offset in &ROOK_OFFSETS {
            let mut target_square = (index as i8 + offset) as usize;
            let mut state = self.pieces[target_square];
            while state != BoardState::OffBoard {
                if let BoardState::Piece(p) = state {
                    if (p.is_type(PieceType::Rook) || p.is_type(PieceType::Queen))
                        && p.is_color(attacker)
                    {
                        return true;
                    }
                    break;
                }
                target_square = (target_square as i8 + offset) as usize;
                state = self.pieces[target_square];
            }
        }

        // king
        for offset in &KING_OFFSETS {
            if let BoardState::Piece(p) = self.pieces[(index as i8 + offset) as usize] {
                if p.is_type(PieceType::King) && p.is_color(attacker) {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::File;
    use crate::Rank;

    use Color::*;
    use Square::*;

    /// Creates a function to test `Position::is_attacked`.
    ///
    /// Curly braces are necessary for rustfmt to work, which is nice because it can automatically
    /// wrap long lines.
    macro_rules! test_position_is_attacked {
        ({ $($name:ident($fen:expr, $color:expr, $attacked_squares:expr $(,)?);)+ }) => {
            $(
                #[test]
                fn $name() {
                   let position = Position::from_fen($fen).expect("valid position");
                    let attacked_squares = $attacked_squares;
                    for i in 0..8 {
                        for j in 0..8 {
                            let square = Square::new(File::new(i), Rank::new(j));
                            print!("{:?} ", square);
                            let expected = attacked_squares.contains(&square);
                            assert_eq!(position.is_attacked(square, $color), expected);
                        }
                    }
                }
            )*
        };
        () => {};
    }

    test_position_is_attacked!({
        test_is_attacked_starting_position_white(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            White,
            [
                A3, B3, C3, D3, E3, F3, G3, H3, A2, B2, C2, D2, E2, F2, G2, H2, B1, C1, D1, E1, F1,
                G1,
            ],
        );
        test_is_attacked_starting_position_black(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            Black,
            [
                A6, B6, C6, D6, E6, F6, G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, B8, C8, D8, E8, F8,
                G8,
            ],
        );
        test_is_attacked_kiwipete_white(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
            White,
            [
                B1, C1, D1, E1, F1, G1, A2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3, H3, A4,
                C4, E4, F4, G4, B5, D5, F5, A6, C6, E6, F6, G5, H5, G6, H6, D7, F7,
            ],
        );
        test_is_attacked_kiwipete_black(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
            Black,
            [
                E2, G2, A3, C3, D3, H3, A4, B4, C4, E4, G4, H4, B5, C5, D5, F5, H5, B6, C6, D6, E6,
                F6, G6, H6, A7, B7, D7, E7, F7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
            ],
        );
    });
}
