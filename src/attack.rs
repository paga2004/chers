use crate::position::{
    BISHOP_OFFSETS, BLACK_PAWN_CAPTURE_OFFSETS, KING_OFFSETS, KNIGHT_OFFSETS, ROOK_OFFSETS,
    WHITE_PAWN_CAPTURE_OFFSETS,
};
use crate::Color;
use crate::Piece;
use crate::Position;
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
    /// assert!(position.is_attacked(Square::E6, Color::BLACK));
    /// assert!(position.is_attacked(Square::E3, Color::WHITE));
    /// assert!(!position.is_attacked(Square::E3, Color::BLACK));
    /// ```
    pub fn is_attacked(&self, square: Square, attacker: Color) -> bool {
        let index = square.to_usize();

        // pawns
        for offset in &attacker.map(BLACK_PAWN_CAPTURE_OFFSETS, WHITE_PAWN_CAPTURE_OFFSETS) {
            if self.pieces[(index as i8 + offset) as usize]
                == attacker.map(Piece::W_PAWN, Piece::B_PAWN)
            {
                return true;
            }
        }

        // knights
        for offset in &KNIGHT_OFFSETS {
            if self.pieces[(index as i8 + offset) as usize]
                == attacker.map(Piece::W_KNIGHT, Piece::B_KNIGHT)
            {
                return true;
            }
        }

        // bishops and queens
        for offset in &BISHOP_OFFSETS {
            let mut target = (index as i8 + offset) as usize;
            let mut piece = self.pieces[target];
            while piece != Piece::OFF_BOARD {
                if piece != Piece::EMPTY {
                    if piece == attacker.map(Piece::W_BISHOP, Piece::B_BISHOP)
                        || piece == attacker.map(Piece::W_QUEEN, Piece::B_QUEEN)
                    {
                        return true;
                    }
                    break;
                }
                target = (target as i8 + offset) as usize;
                piece = self.pieces[target];
            }
        }

        // rooks and queens
        for offset in &ROOK_OFFSETS {
            let mut target = (index as i8 + offset) as usize;
            let mut piece = self.pieces[target];
            while piece != Piece::OFF_BOARD {
                if piece != Piece::EMPTY {
                    if piece == attacker.map(Piece::W_ROOK, Piece::B_ROOK)
                        || piece == attacker.map(Piece::W_QUEEN, Piece::B_QUEEN)
                    {
                        return true;
                    }
                    break;
                }
                target = (target as i8 + offset) as usize;
                piece = self.pieces[target];
            }
        }

        // king
        for offset in &KING_OFFSETS {
            if self.pieces[(index as i8 + offset) as usize]
                == attacker.map(Piece::W_KING, Piece::B_KING)
            {
                return true;
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
    #[inline]
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
    /// assert!(!pos.in_check(Color::WHITE));
    /// assert!(pos.in_check(Color::BLACK));
    /// ```
    #[inline]
    pub fn in_check(&self, side: Color) -> bool {
        self.is_attacked(self.king_square[side], !side)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::File;
    use crate::Rank;

    use crate::utils;

    #[test_case(utils::fen::STARTING_POSITION, Color::WHITE, &[Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3, Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1]; "starting position white")]
    #[test_case(utils::fen::STARTING_POSITION, Color::BLACK, &[Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6, Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8]; "starting position black")]
    #[test_case(utils::fen::KIWIPETE, Color::WHITE, &[Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::A2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2, Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3, Square::A4, Square::C4, Square::E4, Square::F4, Square::G4, Square::B5, Square::D5, Square::F5, Square::A6, Square::C6, Square::E6, Square::F6, Square::G5, Square::H5, Square::G6, Square::H6, Square::D7, Square::F7]; "kiwipete white")]
    #[test_case(utils::fen::KIWIPETE, Color::BLACK, &[Square::E2, Square::G2, Square::A3, Square::C3, Square::D3, Square::H3, Square::A4, Square::B4, Square::C4, Square::E4, Square::G4, Square::H4, Square::B5, Square::C5, Square::D5, Square::F5, Square::H5, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6, Square::A7, Square::B7, Square::D7, Square::E7, Square::F7, Square::H7, Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8]; "kiwipete black")]
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
