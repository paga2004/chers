use crate::position::{
    BISHOP_OFFSETS, BLACK_PAWN_CAPTURE_OFFSETS, BLACK_PAWN_OFFSET, KING_OFFSETS, KNIGHT_OFFSETS,
    ROOK_OFFSETS, WHITE_PAWN_CAPTURE_OFFSETS, WHITE_PAWN_OFFSET,
};
use crate::BitMove;
use crate::Color;
use crate::File;
use crate::MoveList;
use crate::Piece;
use crate::PieceType;
use crate::Position;
use crate::Rank;
use crate::Square;

impl Position {
    // Functions target add moves target the MoveList. They can later be used target assign diffrent scores target
    // the moves for move ordering.
    fn add_quiet(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_quiet(origin, target));
    }

    fn add_double_pawn_push(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_pawn_push(origin, target));
    }

    fn add_capture(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_capture(origin, target));
    }

    fn add_en_passant(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_en_passant(origin, target));
    }

    fn add_promotion_capture(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_promotion_capture(
            origin,
            target,
            PieceType::QUEEN,
        ));
        moves.push(BitMove::new_promotion_capture(
            origin,
            target,
            PieceType::ROOK,
        ));
        moves.push(BitMove::new_promotion_capture(
            origin,
            target,
            PieceType::BISHOP,
        ));
        moves.push(BitMove::new_promotion_capture(
            origin,
            target,
            PieceType::KNIGHT,
        ));
    }

    fn add_promotion(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_promotion(origin, target, PieceType::QUEEN));
        moves.push(BitMove::new_promotion(origin, target, PieceType::ROOK));
        moves.push(BitMove::new_promotion(origin, target, PieceType::BISHOP));
        moves.push(BitMove::new_promotion(origin, target, PieceType::KNIGHT));
    }

    fn add_castle_kingside(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_castle_kingside(origin, target));
    }

    fn add_castle_queenside(&self, moves: &mut MoveList, origin: Square, target: Square) {
        moves.push(BitMove::new_castle_queenside(origin, target));
    }

    /// Returns a [`MoveList`](crate::MoveList) of all legal moves.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::{Position, ParsedMove};
    ///
    /// let mut pos = Position::new();
    /// let moves = pos.generate_legal_moves();
    ///
    /// let m1 = ParsedMove::from_coordinate_notation("e2e4").unwrap();
    /// let m2 = ParsedMove::from_coordinate_notation("e4e5").unwrap();
    ///
    /// assert!(moves.iter().any(|m| *m == m1));
    /// assert!(moves.iter().all(|m| *m != m2));
    /// ```
    pub fn generate_legal_moves(&mut self) -> MoveList {
        self.generate_pseudo_legal_moves()
            .into_iter()
            .filter(|candidate| {
                self.make_bit_move(*candidate);
                let result = !self.in_check(!self.side_to_move);
                self.undo_move();
                result
            })
            .collect()
    }

    fn generate_pseudo_legal_moves(&self) -> MoveList {
        let mut moves = MoveList::new();

        for i in 0..8 {
            for j in 0..8 {
                let square = Square::new(File::new(i), Rank::new(j));
                let piece = self.pieces[square];
                if piece.is_color(self.side_to_move) {
                    match piece.piece_type() {
                        PieceType::PAWN_W => {
                            self.generate_white_pawn_moves(&mut moves, square);
                        }
                        PieceType::PAWN_B => {
                            self.generate_black_pawn_moves(&mut moves, square);
                        }
                        PieceType::KNIGHT => {
                            self.generate_knight_moves(&mut moves, square);
                        }
                        PieceType::BISHOP => {
                            self.generate_bishop_moves(&mut moves, square);
                        }
                        PieceType::ROOK => {
                            self.generate_rook_moves(&mut moves, square);
                        }
                        PieceType::QUEEN => {
                            self.generate_bishop_moves(&mut moves, square);
                            self.generate_rook_moves(&mut moves, square);
                        }
                        PieceType::KING => {
                            self.generate_king_moves(&mut moves, square);
                        }
                        _ => {}
                    }
                }
            }
        }
        self.generate_castling_moves(&mut moves);
        if self.side_to_move == Color::WHITE {
            self.generate_en_passant_moves_white(&mut moves);
        } else {
            self.generate_en_passant_moves_black(&mut moves);
        }

        moves
    }

    fn generate_white_pawn_moves(&self, moves: &mut MoveList, origin: Square) {
        let index = origin.to_usize();
        let offset = WHITE_PAWN_OFFSET;
        let capture_offsets = WHITE_PAWN_CAPTURE_OFFSETS;
        let starting_rank = origin.rank() == Rank::SECOND;
        let promotion_rank = origin.rank() == Rank::SEVENTH;

        // captures
        for offset in &capture_offsets {
            let target = ((index as i8) + offset) as usize;
            if self.pieces[target].is_color(!self.side_to_move) {
                if promotion_rank {
                    self.add_promotion_capture(moves, origin, Square::from_index(target));
                } else {
                    self.add_capture(moves, origin, Square::from_index(target));
                }
            }
        }

        // push
        let target = Square::from_index(((index as i8) + offset) as usize);
        if self.pieces[target] == Piece::EMPTY {
            if promotion_rank {
                self.add_promotion(moves, origin, target);
            } else {
                self.add_quiet(moves, origin, target);
            }

            // double push
            if starting_rank {
                let target = Square::from_index(((index as i8) + 2 * offset) as usize);
                if self.pieces[target] == Piece::EMPTY {
                    self.add_double_pawn_push(moves, origin, target);
                }
            }
        }
    }

    fn generate_black_pawn_moves(&self, moves: &mut MoveList, origin: Square) {
        let index = origin.to_usize();
        let offset = BLACK_PAWN_OFFSET;
        let capture_offsets = BLACK_PAWN_CAPTURE_OFFSETS;
        let starting_rank = origin.rank() == Rank::SEVENTH;
        let promotion_rank = origin.rank() == Rank::SECOND;

        // captures
        for offset in &capture_offsets {
            let target = ((index as i8) + offset) as usize;
            if self.pieces[target].is_piece() && self.pieces[target].is_color(!self.side_to_move) {
                if promotion_rank {
                    self.add_promotion_capture(moves, origin, Square::from_index(target));
                } else {
                    self.add_capture(moves, origin, Square::from_index(target));
                }
            }
        }

        // push
        let target = Square::from_index(((index as i8) + offset) as usize);
        if self.pieces[target] == Piece::EMPTY {
            if promotion_rank {
                self.add_promotion(moves, origin, target);
            } else {
                self.add_quiet(moves, origin, target);
            }

            // double push
            if starting_rank {
                let target = Square::from_index(((index as i8) + 2 * offset) as usize);
                if self.pieces[target] == Piece::EMPTY {
                    self.add_double_pawn_push(moves, origin, target);
                }
            }
        }
    }

    fn generate_knight_moves(&self, moves: &mut MoveList, origin: Square) {
        for offset in &KNIGHT_OFFSETS {
            let target = (origin.to_i8() + offset) as usize;
            match self.pieces[target] {
                Piece::EMPTY => self.add_quiet(moves, origin, Square::from_index(target)),
                Piece::OFF_BOARD => continue,
                p if p.is_color(self.side_to_move) => continue,
                _ => self.add_capture(moves, origin, Square::from_index(target)),
            }
        }
    }

    fn generate_bishop_moves(&self, moves: &mut MoveList, origin: Square) {
        for offset in &BISHOP_OFFSETS {
            let mut target = (origin.to_i8() + offset) as usize;
            let mut piece = self.pieces[target];
            while piece != Piece::OFF_BOARD {
                if piece != Piece::EMPTY {
                    if piece.is_color(!self.side_to_move) {
                        self.add_capture(moves, origin, Square::from_index(target));
                    }
                    break;
                }
                self.add_quiet(moves, origin, Square::from_index(target));

                target = (target as i8 + offset) as usize;
                piece = self.pieces[target];
            }
        }
    }

    fn generate_rook_moves(&self, moves: &mut MoveList, origin: Square) {
        for offset in &ROOK_OFFSETS {
            let mut target = (origin.to_i8() + offset) as usize;
            let mut piece = self.pieces[target];
            while piece != Piece::OFF_BOARD {
                if piece != Piece::EMPTY {
                    if piece.is_color(!self.side_to_move) {
                        self.add_capture(moves, origin, Square::from_index(target));
                    }
                    break;
                }
                self.add_quiet(moves, origin, Square::from_index(target));

                target = (target as i8 + offset) as usize;
                piece = self.pieces[target];
            }
        }
    }

    fn generate_king_moves(&self, moves: &mut MoveList, origin: Square) {
        for offset in &KING_OFFSETS {
            let target = (origin.to_i8() + offset) as usize;
            match self.pieces[target] {
                Piece::EMPTY => self.add_quiet(moves, origin, Square::from_index(target)),
                Piece::OFF_BOARD => continue,
                p if p.is_color(self.side_to_move) => continue,
                _ => self.add_capture(moves, origin, Square::from_index(target)),
            }
        }
    }

    fn generate_castling_moves(&self, moves: &mut MoveList) {
        // TODO: dry
        match self.side_to_move {
            Color::WHITE => {
                if self.state.castling_rights.white_king_side() {
                    // NOTE: Might be faster to check first if both squares are empty since that is
                    // just a lookup.
                    if self.is_empty_and_not_attacked(Square::F1)
                        && self.is_empty_and_not_attacked(Square::G1)
                        && !self.is_check()
                    {
                        self.add_castle_kingside(moves, Square::E1, Square::G1);
                    }
                }
                if self.state.castling_rights.white_queen_side() {
                    // NOTE: Might be faster to check first if all squares are empty since that is
                    // just a lookup.

                    if self.pieces[Square::B1] == Piece::EMPTY
                        && self.is_empty_and_not_attacked(Square::C1)
                        && self.is_empty_and_not_attacked(Square::D1)
                        && !self.is_check()
                    {
                        self.add_castle_queenside(moves, Square::E1, Square::C1);
                    }
                }
            }
            Color::BLACK => {
                if self.state.castling_rights.black_king_side() {
                    // NOTE: Might be faster to check first if both squares are empty since that is
                    // just a lookup.
                    if self.is_empty_and_not_attacked(Square::F8)
                        && self.is_empty_and_not_attacked(Square::G8)
                        && !self.is_check()
                    {
                        self.add_castle_kingside(moves, Square::E8, Square::G8);
                    }
                }
                if self.state.castling_rights.black_queen_side() {
                    // NOTE: Might be faster to check first if all squares are empty since that is
                    // just a lookup.

                    if self.pieces[Square::B8] == Piece::EMPTY
                        && self.is_empty_and_not_attacked(Square::C8)
                        && self.is_empty_and_not_attacked(Square::D8)
                        && !self.is_check()
                    {
                        self.add_castle_queenside(moves, Square::E8, Square::C8);
                    }
                }
            }
        }
    }

    fn is_empty_and_not_attacked(&self, sq: Square) -> bool {
        self.pieces[sq] == Piece::EMPTY && !self.is_attacked(sq, !self.side_to_move)
    }

    fn generate_en_passant_moves_white(&self, moves: &mut MoveList) {
        if let Some(sq) = self.state.ep_square {
            // The offset is added to the target square. That's why it's the other way around.
            for offset in BLACK_PAWN_CAPTURE_OFFSETS {
                let target = (sq.to_i8() + offset) as usize;
                if self.pieces[target] == Piece::W_PAWN {
                    self.add_en_passant(moves, Square::from_index(target), sq);
                }
            }
        }
    }

    fn generate_en_passant_moves_black(&self, moves: &mut MoveList) {
        if let Some(sq) = self.state.ep_square {
            // The offset is added to the target square. That's why it's the other way around.
            for offset in WHITE_PAWN_CAPTURE_OFFSETS {
                let target = (sq.to_i8() + offset) as usize;
                if self.pieces[target] == Piece::B_PAWN {
                    self.add_en_passant(moves, Square::from_index(target), sq);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::utils;

    use super::*;

    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut ["a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3"]; "starting position")]
    #[test_case(utils::fen::KIWIPETE, &mut ["a2a3", "b2b3", "g2g3", "d5d6", "a2a4", "g2g4", "g2h3", "d5e6", "c3b1", "c3d1", "c3a4", "c3b5", "e5d3", "e5c4", "e5g4", "e5c6", "e5g6", "e5d7", "e5f7", "d2c1", "d2e3", "d2f4", "d2g5", "d2h6", "e2d1", "e2f1", "e2d3", "e2c4", "e2b5", "e2a6", "a1b1", "a1c1", "a1d1", "h1f1", "h1g1", "f3d3", "f3e3", "f3g3", "f3h3", "f3f4", "f3g4", "f3f5", "f3h5", "f3f6", "e1d1", "e1f1", "e1g1", "e1c1"]; "kiwipete")]
    // En passant move is not covered in kiwipete.
    #[test_case("rnbqkbnr/pppp2pp/8/3Ppp2/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3", &mut ["a2a3", "b2b3", "c2c3", "e2e3", "f2f3", "g2g3", "h2h3", "d5d6", "a2a4", "b2b4", "c2c4", "e2e4", "f2f4", "g2g4", "h2h4", "d5e6", "b1d2", "b1a3", "b1c3", "g1f3", "g1h3", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "d1d2", "d1d3", "d1d4", "e1d2", ]; "en passant")]
    // There was a bug in this position on commit 31459f2b8cee5d4ab8fd1d3152d1ca432b7df125.
    #[test_case("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/1R2K2R b Kkq - 1 1", &mut ["b4b3", "g6g5", "c7c6", "d7d6", "c7c5", "h3g2", "e6d5", "b4c3", "b6a4", "b6c4", "b6d5", "b6c8", "f6e4", "f6g4", "f6d5", "f6h5", "f6h7", "f6g8", "a6e2", "a6d3", "a6c4", "a6b5", "a6b7", "a6c8", "g7h6", "g7f8", "a8b8", "a8c8", "a8d8", "h8h4", "h8h5", "h8h6", "h8h7", "h8f8", "h8g8", "e7c5", "e7d6", "e7d8", "e7f8", "e8d8", "e8f8", "e8g8", "e8c8"]; "bug 1")]
    // There was a bug in these positions on commit 31459f2b8cee5d4ab8fd1d3152d1ca432b7df125.
    #[test_case("r3k2r/p1pNqpb1/bn2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1", &mut ["b4b3", "e6e5", "g6g5", "c7c6", "c7c5", "h3g2", "e6d5", "b4c3", "b6a4", "b6c4", "b6d5", "b6d7", "b6c8", "f6e4", "f6g4", "f6d5", "f6h5", "f6d7", "f6h7", "f6g8", "a6e2", "a6d3", "a6c4", "a6b5", "a6b7", "a6c8", "g7h6", "g7f8", "a8b8", "a8c8", "a8d8", "h8h4", "h8h5", "h8h6", "h8h7", "h8f8", "h8g8", "e7c5", "e7d6", "e7d7", "e7d8", "e7f8", "e8d7", "e8d8", "e8c8"]; "bug 2")]
    #[test_case("r3k2N/p1ppq1b1/1n2pnp1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 2", &mut ["b4b3", "e6e5", "g6g5", "a7a6", "c7c6", "d7d6", "a7a5", "c7c5", "h3g2", "e6d5", "b4c3", "b6a4", "b6c4", "b6d5", "b6c8", "f6e4", "f6g4", "f6d5", "f6h5", "f6h7", "f6g8", "b5e2", "b5d3", "b5a4", "b5c4", "b5a6", "b5c6", "g7h6", "g7f8", "g7h8", "a8b8", "a8c8", "a8d8", "e7c5", "e7d6", "e7f7", "e7d8", "e7f8", "e8c8", "e8d8", "e8f8"]; "bug 2.3")]
    // There was a bug in this position on commit 31459f2b8cee5d4ab8fd1d3152d1ca432b7df125.
    #[test_case("r3k2r/p1ppqpb1/1n2pnp1/3PN3/Pp2P3/2N2Q1p/bPPBBPPP/R3K2R w KQkq - 1 3", &mut ["b2b3", "g2g3", "a4a5", "d5d6", "g2g4", "g2h3", "d5e6", "c3b1", "c3d1", "c3a2", "c3b5", "e5d3", "e5c4", "e5g4", "e5c6", "e5g6", "e5d7", "e5f7", "d2c1", "d2e3", "d2f4", "d2g5", "d2h6", "e2d1", "e2f1", "e2d3", "e2c4", "e2b5", "e2a6", "a1b1", "a1c1", "a1d1", "a1a2", "h1f1", "h1g1", "f3d3", "f3e3", "f3g3", "f3h3", "f3f4", "f3g4", "f3f5", "f3h5", "f3f6", "e1d1", "e1f1", "e1g1", "e1c1"]; "bug 4.3")]
    #[test_case("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1R1K b kq - 1 1", &mut ["c7c6", "d7d6", "c7c5", "d7d5", "b2a1q", "b2a1r", "b2a1b", "b2a1n", "b2b1q", "b2b1r", "b2b1b", "b2b1n", "g7h6", "a5b3", "a5c4", "a5c6", "f6e4", "f6g4", "f6d5", "f6h5", "f6g8", "b6g1", "b6f2", "b6e3", "b6d4", "b6c5", "b6a7", "g6e4", "g6f5", "g6h5", "a8a7", "a8b8", "a8c8", "a8d8", "h8f8", "h8g8", "a3a2", "a3b3", "a3c3", "a3d3", "a3e3", "a3f3", "a3a4", "a3b4", "e8c8", "e8d8"]; "bug 5")]
    fn test_position_generate_legal_moves(fen: &str, expected_moves: &mut [&str]) {
        let mut pos = Position::from_fen(fen).expect("valid position");
        let mut moves: Vec<_> = pos
            .generate_legal_moves()
            .into_iter()
            .map(|m| m.to_string())
            .collect();
        expected_moves.sort_unstable();
        moves.sort_unstable();

        pretty_assertions::assert_eq!(moves, expected_moves);
    }
}
