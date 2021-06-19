use crate::position::BoardState;
use crate::position::{
    BISHOP_OFFSETS, BLACK_PAWN_CAPTURE_OFFSETS, KING_OFFSETS, KNIGHT_OFFSETS, ROOK_OFFSETS,
    WHITE_PAWN_CAPTURE_OFFSETS,
};
use crate::Color;
use crate::File;
use crate::PieceType;
use crate::Position;
use crate::Rank;
use crate::Square;

impl Position {
    /// Returns wether a given `Square` is attacked by any piece of a given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::{Position, Square, Color};
    ///
    /// let mut position = Position::new();
    ///
    /// assert!(position.is_attacked(Square::E6, Color::Black));
    /// assert!(position.is_attacked(Square::E3, Color::White));
    /// assert!(!position.is_attacked(Square::E3, Color::Black));
    /// ```
    pub fn is_attacked(&self, square: Square, attacker: Color) -> bool {
        let index = square as usize;

        // pawns
        for offset in &attacker.map(BLACK_PAWN_CAPTURE_OFFSETS, WHITE_PAWN_CAPTURE_OFFSETS) {
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
            let mut target = (index as i8 + offset) as usize;
            let mut state = self.pieces[target];
            while state != BoardState::OffBoard {
                if let BoardState::Piece(p) = state {
                    if (p.is_type(PieceType::Bishop) || p.is_type(PieceType::Queen))
                        && p.is_color(attacker)
                    {
                        return true;
                    }
                    break;
                }
                target = (target as i8 + offset) as usize;
                state = self.pieces[target];
            }
        }

        // rooks and queens
        for offset in &ROOK_OFFSETS {
            let mut target = (index as i8 + offset) as usize;
            let mut state = self.pieces[target];
            while state != BoardState::OffBoard {
                if let BoardState::Piece(p) = state {
                    if (p.is_type(PieceType::Rook) || p.is_type(PieceType::Queen))
                        && p.is_color(attacker)
                    {
                        return true;
                    }
                    break;
                }
                target = (target as i8 + offset) as usize;
                state = self.pieces[target];
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

        false
    }

    /// Returns wether the side to move is in check.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::Position;
    /// let pos1 = Position::new();
    /// let pos2 = Position::from_fen("rnbqkbnr/ppp1pppp/8/1B1p4/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2").unwrap();
    ///
    /// assert!(!pos1.is_check());
    /// assert!(pos2.is_check());
    /// ```
    pub fn is_check(&self) -> bool {
        self.in_check(self.side_to_move)
    }

    /// Returns wether the given side is in check.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::{Position, Color};
    /// let pos = Position::from_fen("rnbqkbnr/ppp1pppp/8/1B1p4/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2").unwrap();
    ///
    /// assert!(!pos.in_check(Color::White));
    /// assert!(pos.in_check(Color::Black));
    /// ```
    pub fn in_check(&self, side: Color) -> bool {
        // FIXME: This loop really slow.
        let mut sq = Square::A1;
        'outer: for i in 0..8 {
            for j in 0..8 {
                sq = Square::new(File::new(i), Rank::new(j));
                if let BoardState::Piece(p) = self.pieces[sq] {
                    if p.is_type(PieceType::King) && p.is_color(side) {
                        break 'outer;
                    }
                }
            }
        }
        self.is_attacked(sq, !side)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::File;
    use crate::Rank;

    use Color::*;
    use Square::*;

    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", White, &[ A3, B3, C3, D3, E3, F3, G3, H3, A2, B2, C2, D2, E2, F2, G2, H2, B1, C1, D1, E1, F1, G1, ]; "starting position white")]
    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", Black, &[ A6, B6, C6, D6, E6, F6, G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, B8, C8, D8, E8, F8, G8, ]; "starting position black")]
    #[test_case("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", White, &[ B1, C1, D1, E1, F1, G1, A2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3, H3, A4, C4, E4, F4, G4, B5, D5, F5, A6, C6, E6, F6, G5, H5, G6, H6, D7, F7, ]; "kiwipete white")]
    #[test_case("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", Black, &[ E2, G2, A3, C3, D3, H3, A4, B4, C4, E4, G4, H4, B5, C5, D5, F5, H5, B6, C6, D6, E6, F6, G6, H6, A7, B7, D7, E7, F7, H7, A8, B8, C8, D8, E8, F8, G8, H8, ]; "kiwipete black")]
    fn test_position_is_attacked(fen: &str, color: Color, expected_squares: &[Square]) {
        let position = Position::from_fen(fen).expect("valid position");
        for i in 0..8 {
            for j in 0..8 {
                let square = Square::new(File::new(i), Rank::new(j));
                let expected = expected_squares.contains(&square);
                pretty_assertions::assert_eq!(
                    position.is_attacked(square, color),
                    expected,
                    "Failed at {:?}",
                    square
                );
            }
        }
    }
}
