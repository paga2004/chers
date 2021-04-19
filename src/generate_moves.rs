use crate::position::BoardState;
use crate::position::{
    BISHOP_OFFSETS, BLACK_PAWN_CAPTURE_OFFSETS, BLACK_PAWN_OFFSET, KING_OFFSETS, KNIGHT_OFFSETS,
    ROOK_OFFSETS, WHITE_PAWN_CAPTURE_OFFSETS, WHITE_PAWN_OFFSET,
};
use crate::Color;
use crate::File;
use crate::Move;
use crate::MoveList;
use crate::PieceType;
use crate::Position;
use crate::Rank;
use crate::Square::{self, *};

impl Position {
    // Functions to add moves to the MoveList. They can later be used to assign diffrent scores to
    // the moves for move ordering.
    fn add_quiet_move(&self, moves: &mut MoveList, m: Move) {
        moves.push(m);
    }

    fn add_capture_move(&self, moves: &mut MoveList, m: Move) {
        moves.push(m);
    }

    fn add_en_passant_move(&self, moves: &mut MoveList, m: Move) {
        moves.push(m);
    }

    fn add_pawn_move(&self, moves: &mut MoveList, from: Square, to: Square) {
        if from.rank() == self.side_to_move.map(Rank::Seventh, Rank::Second) {
            self.add_capture_move(moves, Move::new(from, to, Some(PieceType::Queen)));
            self.add_capture_move(moves, Move::new(from, to, Some(PieceType::Rook)));
            self.add_capture_move(moves, Move::new(from, to, Some(PieceType::Bishop)));
            self.add_capture_move(moves, Move::new(from, to, Some(PieceType::Knight)));
        } else {
            self.add_quiet_move(moves, Move::new(from, to, None));
        }
    }

    fn add_pawn_capture_move(&self, moves: &mut MoveList, from: Square, to: Square) {
        if from.rank() == self.side_to_move.map(Rank::Seventh, Rank::Second) {
            self.add_quiet_move(moves, Move::new(from, to, Some(PieceType::Queen)));
            self.add_quiet_move(moves, Move::new(from, to, Some(PieceType::Rook)));
            self.add_quiet_move(moves, Move::new(from, to, Some(PieceType::Bishop)));
            self.add_quiet_move(moves, Move::new(from, to, Some(PieceType::Knight)));
        } else {
            self.add_capture_move(moves, Move::new(from, to, None));
        }
    }

    /// Returns a [`MoveList`](crate::MoveList) of all legal moves.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::{Position, Move};
    ///
    /// let pos = Position::new();
    /// let moves = pos.generate_legal_moves();
    ///
    /// let m1 = Move::from_coordinate_notation("e2e4").unwrap();
    /// let m2 = Move::from_coordinate_notation("e4e5").unwrap();
    ///
    /// assert!(moves.contains(&m1));
    /// assert!(!moves.contains(&m2));
    /// ```
    pub fn generate_legal_moves(&self) -> MoveList {
        // FIXME: This is really slow.
        self.generate_pseudo_legal_moves()
            .into_iter()
            .filter(|candidate| {
                let mut clone = self.clone();
                clone.make_move(candidate);
                !clone.in_check(self.side_to_move)
            })
            .collect()
    }

    fn generate_pseudo_legal_moves(&self) -> MoveList {
        let mut moves = MoveList::new();

        for i in 0..8 {
            for j in 0..8 {
                let square = Square::new(File::new(i), Rank::new(j));
                if let BoardState::Piece(piece) = self.pieces[square] {
                    if piece.is_color(self.side_to_move) {
                        match piece.piece_type {
                            PieceType::Pawn => {
                                self.generate_pawn_moves(&mut moves, square);
                            }
                            PieceType::Knight => {
                                self.generate_knight_moves(&mut moves, square);
                            }
                            PieceType::Bishop => {
                                self.generate_bishop_moves(&mut moves, square);
                            }
                            PieceType::Rook => {
                                self.generate_rook_moves(&mut moves, square);
                            }
                            PieceType::Queen => {
                                self.generate_bishop_moves(&mut moves, square);
                                self.generate_rook_moves(&mut moves, square);
                            }
                            PieceType::King => {
                                self.generate_king_moves(&mut moves, square);
                            }
                        }
                    }
                }
            }
        }
        self.generate_castling_moves(&mut moves);
        self.generate_en_passant_moves(&mut moves);

        moves
    }

    fn generate_pawn_moves(&self, moves: &mut MoveList, from: Square) {
        let index = from as usize;
        let offset;
        let starting_rank;
        let capture_offsets;
        match self.side_to_move {
            Color::White => {
                offset = WHITE_PAWN_OFFSET;
                capture_offsets = WHITE_PAWN_CAPTURE_OFFSETS;
                starting_rank = from.rank() == Rank::Second;
            }
            Color::Black => {
                offset = BLACK_PAWN_OFFSET;
                capture_offsets = BLACK_PAWN_CAPTURE_OFFSETS;
                starting_rank = from.rank() == Rank::Seventh;
            }
        }

        // captures
        for offset in &capture_offsets {
            let target = ((index as i8) + offset) as usize;
            match self.pieces[target] {
                BoardState::Piece(p) if p.is_color(!self.side_to_move) => {
                    self.add_pawn_capture_move(moves, from, Square::from_index(target));
                }
                _ => {}
            }
        }

        // push
        let target = Square::from_index(((index as i8) + offset) as usize);
        if let BoardState::Empty = self.pieces[target] {
            self.add_pawn_move(moves, from, target);
            // double push
            if starting_rank {
                let target = Square::from_index(((index as i8) + 2 * offset) as usize);
                if let BoardState::Empty = self.pieces[target] {
                    self.add_pawn_move(moves, from, target);
                }
            }
        }
    }

    fn generate_knight_moves(&self, moves: &mut MoveList, from: Square) {
        for offset in &KNIGHT_OFFSETS {
            let target = (from as i8 + offset) as usize;
            match self.pieces[target] {
                BoardState::Piece(p) if p.is_color(self.side_to_move) => continue,
                BoardState::OffBoard => continue,
                _ => {}
            }
            moves.push(Move::new(from, Square::from_index(target), None));
        }
    }

    fn generate_bishop_moves(&self, moves: &mut MoveList, from: Square) {
        for offset in &BISHOP_OFFSETS {
            let mut target = (from as i8 + offset) as usize;
            let mut state = self.pieces[target];
            while state != BoardState::OffBoard {
                if let BoardState::Piece(p) = state {
                    if p.is_color(!self.side_to_move) {
                        moves.push(Move::new(from, Square::from_index(target), None));
                    }
                    break;
                }
                moves.push(Move::new(from, Square::from_index(target), None));

                target = (target as i8 + offset) as usize;
                state = self.pieces[target];
            }
        }
    }

    fn generate_rook_moves(&self, moves: &mut MoveList, from: Square) {
        for offset in &ROOK_OFFSETS {
            let mut target = (from as i8 + offset) as usize;
            let mut state = self.pieces[target];
            while state != BoardState::OffBoard {
                if let BoardState::Piece(p) = state {
                    if p.is_color(!self.side_to_move) {
                        moves.push(Move::new(from, Square::from_index(target), None));
                    }
                    break;
                }
                moves.push(Move::new(from, Square::from_index(target), None));

                target = (target as i8 + offset) as usize;
                state = self.pieces[target];
            }
        }
    }

    fn generate_king_moves(&self, moves: &mut MoveList, from: Square) {
        for offset in &KING_OFFSETS {
            let target = (from as i8 + offset) as usize;
            match self.pieces[target] {
                BoardState::Piece(p) if p.is_color(self.side_to_move) => continue,
                BoardState::OffBoard => continue,
                _ => {}
            }
            moves.push(Move::new(from, Square::from_index(target), None));
        }
    }

    fn generate_castling_moves(&self, moves: &mut MoveList) {
        match self.side_to_move {
            Color::White => {
                if self.castling_rights.white_king_side {
                    // NOTE: Might be faster to check first if both squares are empty since that is
                    // just a lookup.
                    if self.is_empty_and_not_attacked(F1)
                        && self.is_empty_and_not_attacked(G1)
                        && !self.is_check()
                    {
                        self.add_quiet_move(moves, Move::new(E1, G1, None));
                    }
                }
                if self.castling_rights.white_queen_side {
                    // NOTE: Might be faster to check first if all squares are empty since that is
                    // just a lookup.

                    if self.is_empty_and_not_attacked(B1)
                        && self.is_empty_and_not_attacked(C1)
                        && self.is_empty_and_not_attacked(D1)
                        && !self.is_check()
                    {
                        self.add_quiet_move(moves, Move::new(E1, C1, None));
                    }
                }
            }
            Color::Black => {
                if self.castling_rights.white_king_side {
                    // NOTE: Might be faster to check first if both squares are empty since that is
                    // just a lookup.
                    if self.is_empty_and_not_attacked(F8)
                        && self.is_empty_and_not_attacked(G8)
                        && !self.is_check()
                    {
                        self.add_quiet_move(moves, Move::new(E8, G8, None));
                    }
                }
                if self.castling_rights.white_queen_side {
                    // NOTE: Might be faster to check first if all squares are empty since that is
                    // just a lookup.

                    if self.is_empty_and_not_attacked(B8)
                        && self.is_empty_and_not_attacked(C8)
                        && self.is_empty_and_not_attacked(D8)
                        && !self.is_check()
                    {
                        self.add_quiet_move(moves, Move::new(E8, C8, None));
                    }
                }
            }
        }
    }

    fn is_empty_and_not_attacked(&self, sq: Square) -> bool {
        self.pieces[sq] == BoardState::Empty && !self.is_attacked(sq, !self.side_to_move)
    }

    fn generate_en_passant_moves(&self, moves: &mut MoveList) {
        if let Some(sq) = self.en_passant_square {
            // The offset is added to the to square. That's why this is the other way around.
            for offset in &self
                .side_to_move
                .map(BLACK_PAWN_CAPTURE_OFFSETS, WHITE_PAWN_CAPTURE_OFFSETS)
            {
                let target = (sq as i8 + offset) as usize;
                if let BoardState::Piece(p) = self.pieces[target] {
                    if p.is_color(self.side_to_move) && p.is_type(PieceType::Pawn) {
                        self.add_en_passant_move(
                            moves,
                            Move::new(Square::from_index(target), sq, None),
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    /// Creates a function to test `Position::new_move`.
    ///
    /// Curly braces are necessary for rustfmt to work, which is nice because it can automatically
    /// wrap long lines.
    macro_rules! test_position_generate_legal_moves {
        ({ $($name:ident($fen:expr, $moves:expr $(,)?);)+ }) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    let mut expected = $moves;
                    let pos = Position::from_fen($fen).expect("valid position");
                    let mut moves : Vec<_> = pos.generate_legal_moves().into_iter()
                        .map(|m| m.to_string())
                        .collect();
                    expected.sort_unstable();
                    moves.sort_unstable();

                    assert_eq!(moves, expected);
                }
            )*
        };
        () => {};
    }

    test_position_generate_legal_moves!({
        test_position_generate_legal_moves_starting_position(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            [
                "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4",
                "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
            ],
        );

        test_position_generate_legal_moves_en_passant(
            "rnbqkbnr/pppp2pp/8/3Ppp2/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3",
            [
                "a2a3", "b2b3", "c2c3", "e2e3", "f2f3", "g2g3", "h2h3", "d5d6", "a2a4", "b2b4",
                "c2c4", "e2e4", "f2f4", "g2g4", "h2h4", "d5e6", "b1d2", "b1a3", "b1c3", "g1f3",
                "g1h3", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6", "d1d2", "d1d3", "d1d4", "e1d2",
            ],
        );

        test_position_generate_legal_moves_kiwipete(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
            [
                "a2a3", "b2b3", "g2g3", "d5d6", "a2a4", "g2g4", "g2h3", "d5e6", "c3b1", "c3d1",
                "c3a4", "c3b5", "e5d3", "e5c4", "e5g4", "e5c6", "e5g6", "e5d7", "e5f7", "d2c1",
                "d2e3", "d2f4", "d2g5", "d2h6", "e2d1", "e2f1", "e2d3", "e2c4", "e2b5", "e2a6",
                "a1b1", "a1c1", "a1d1", "h1f1", "h1g1", "f3d3", "f3e3", "f3g3", "f3h3", "f3f4",
                "f3g4", "f3f5", "f3h5", "f3f6", "e1d1", "e1f1", "e1g1", "e1c1",
            ],
        );
    });
}
