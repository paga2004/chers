use std::sync::Arc;

use crate::{BitMove, CastlingRights, Piece, Square};

#[derive(Clone, Debug)]
pub(crate) struct PositionState {
    pub(crate) castling_rights: CastlingRights,
    pub(crate) ep_square: Square,

    pub(crate) halfmove_clock: u16,

    pub(crate) prev_move: BitMove,
    pub(crate) captured_piece: Piece,
    pub(crate) prev_state: Option<Arc<PositionState>>,
}

impl PositionState {
    pub(crate) fn new(
        castling_rights: CastlingRights,
        ep_square: Square,
        halfmove_clock: u16,
    ) -> Self {
        Self {
            castling_rights,
            ep_square,
            halfmove_clock,
            prev_move: BitMove::NULL,
            captured_piece: Piece::EMPTY,
            prev_state: None,
        }
    }
}

impl PartialEq for PositionState {
    // don't compare prev_move and captured_piece
    fn eq(&self, other: &Self) -> bool {
        self.castling_rights == other.castling_rights
            && self.ep_square == other.ep_square
            && self.halfmove_clock == other.halfmove_clock
    }
}
