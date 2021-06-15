use std::fmt;

use crate::{ParsedMove, PieceType, Square};

/// Additional information for a move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFlags {
    Promotion { capture: bool, piece: PieceType },
    Capture { en_passant: bool },
    DoublePawnPush,
    Castle { kingside: bool },
    QuietMove,
}

/// Compressed representation of a chess move with additional information in just one 16 bit word.
///
/// This type is only used internally. To parse a move from diffrent formats see
/// [`ParsedMove`](crate::ParsedMove).
///
/// In addition to the origin and target square the following flags are stored.
///
/// | code | promotion | capture | special 1 | special 0 | kind of move         |
/// |------|-----------|---------|-----------|-----------|----------------------|
/// |  0   |     0     |    0    |     0     |     0     | quiet moves          |
/// |  1   |     0     |    0    |     0     |     1     | double pawn push     |
/// |  2   |     0     |    0    |     1     |     0     | king side castle     |
/// |  3   |     0     |    0    |     1     |     1     | queen side castle    |
/// |  4   |     0     |    1    |     0     |     0     | captures             |
/// |  5   |     0     |    1    |     0     |     1     | ep-capture           |
/// |  8   |     1     |    0    |     0     |     0     | knight-promotion     |
/// |  9   |     1     |    0    |     0     |     1     | bishop-promotion     |
/// |  10  |     1     |    0    |     1     |     0     | rook-promotion       |
/// |  11  |     1     |    0    |     1     |     1     | queen-promotion      |
/// |  12  |     1     |    1    |     0     |     0     | knight-promo capture |
/// |  13  |     1     |    1    |     0     |     1     | bishop-promo capture |
/// |  14  |     1     |    1    |     1     |     0     | rook-promo capture   |
/// |  15  |     1     |    1    |     1     |     1     | queen-promo capture  |
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitMove(u16);

impl BitMove {
    const QUIET: u16 = 0;
    const DOUBLE_PAWN_PUSH: u16 = 1;
    const KING_SIDE_CASTLE: u16 = 2;
    const QUEEN_SIDE_CASTLE: u16 = 3;
    const CAPTURE: u16 = 4;
    const EN_PASSANT: u16 = 5;
    const PROMOTION: u16 = 8;
    /// Creates a new `BitMove`.
    pub fn new(origin: Square, target: Square, flags: MoveFlags) -> Self {
        let flag_bits = match flags {
            MoveFlags::Promotion {
                capture: true,
                piece,
            } => Self::PROMOTION | Self::CAPTURE | Self::piece_to_code(piece),
            MoveFlags::Promotion {
                capture: false,
                piece,
            } => Self::PROMOTION | Self::piece_to_code(piece),
            MoveFlags::Capture { en_passant: true } => Self::EN_PASSANT,
            MoveFlags::Capture { en_passant: false } => Self::CAPTURE,
            MoveFlags::DoublePawnPush => Self::DOUBLE_PAWN_PUSH,
            MoveFlags::Castle { kingside: true } => Self::KING_SIDE_CASTLE,
            MoveFlags::Castle { kingside: false } => Self::QUEEN_SIDE_CASTLE,
            MoveFlags::QuietMove => Self::QUIET,
        };

        Self::from_flag_bits(origin, target, flag_bits)
    }

    /// Creates a new quiet move (i.e no capture, promotion, castle, or double pawn push).
    pub fn new_quiet(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::QUIET)
    }

    /// Creates a new double pawn push move.
    pub fn new_pawn_push(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::DOUBLE_PAWN_PUSH)
    }

    /// Creates a new capture move.
    pub fn new_capture(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::CAPTURE)
    }

    /// Creates a new en passant capture move.
    pub fn new_en_passant(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::EN_PASSANT)
    }

    /// Creates a new pawn promotion capture move.
    pub fn new_promotion_capture(origin: Square, target: Square, piece: PieceType) -> Self {
        Self::from_flag_bits(
            origin,
            target,
            Self::PROMOTION | Self::CAPTURE | Self::piece_to_code(piece),
        )
    }

    /// Creates a new pawn promotion move.
    pub fn new_promotion(origin: Square, target: Square, piece: PieceType) -> Self {
        Self::from_flag_bits(origin, target, Self::PROMOTION | Self::piece_to_code(piece))
    }

    /// Creates a new king side castle move.
    pub fn new_castle_kingside(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::KING_SIDE_CASTLE)
    }

    /// Creates a new queen side castle move.
    pub fn new_castle_queenside(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::QUEEN_SIDE_CASTLE)
    }

    fn from_flag_bits(origin: Square, target: Square, flags: u16) -> Self {
        Self(Self::square_to_code(origin) | (Self::square_to_code(target) << 6) | flags << 12)
    }

    // TODO: remove this
    fn square_to_code(sq: Square) -> u16 {
        (sq.file() as u16) | ((sq.rank() as u16) << 3)
    }

    // TODO: remove this
    // a Square should be represented as 0..=63 and propbably not be an enum in the first place
    fn square_from_code(code: u16) -> Square {
        use crate::File;
        use crate::Rank;
        let file = File::new((code & 7) as u8);
        let rank = Rank::new(((code >> 3) & 7) as u8);
        Square::new(file, rank)
    }

    fn piece_to_code(piece: PieceType) -> u16 {
        debug_assert_ne!(piece, PieceType::Pawn);
        debug_assert_ne!(piece, PieceType::King);

        match piece {
            PieceType::Knight => 0,
            PieceType::Bishop => 1,
            PieceType::Rook => 2,
            // TODO: is this really better for performance?
            _ => 3,
        }
    }

    fn piece_from_code(code: u16) -> PieceType {
        debug_assert!(code < 4);

        match code {
            0 => PieceType::Knight,
            1 => PieceType::Bishop,
            2 => PieceType::Rook,
            // TODO: is this really better for performance?
            _ => PieceType::Queen,
        }
    }

    /// Returns the origin square.
    pub fn origin(self) -> Square {
        Self::square_from_code(self.0 & 0b111111)
    }

    /// Returns the target square.
    pub fn target(self) -> Square {
        Self::square_from_code((self.0 >> 6) & 0b111111)
    }

    fn flags(self) -> u16 {
        self.0 >> 12
    }

    /// Returns if the move is a capture.
    pub fn is_capture(self) -> bool {
        self.flags() & Self::CAPTURE != 0
    }

    /// Returns if the move is quiet (i.e no capture, promotion, castle, or double pawn push).
    pub fn is_quiet(self) -> bool {
        self.flags() == 0
    }

    /// Returns if the move is a promotion.
    pub fn is_promotion(self) -> bool {
        self.flags() & Self::PROMOTION != 0
    }

    /// Returns if the move is a castle.
    pub fn is_castle(self) -> bool {
        self.flags() >> 1 == 1
    }

    /// Returns if the move is a king side castle.
    pub fn is_king_side_castle(self) -> bool {
        self.flags() == Self::KING_SIDE_CASTLE
    }

    /// Returns if the move is a queen side castle.
    pub fn is_queen_side_castle(self) -> bool {
        self.flags() == Self::QUEEN_SIDE_CASTLE
    }

    /// Returns if the move is an en passant capture.
    pub fn is_en_passant(self) -> bool {
        self.flags() == Self::EN_PASSANT
    }

    /// Return if the move is a double pawn push.
    pub fn is_double_push(self) -> bool {
        self.flags() == Self::DOUBLE_PAWN_PUSH
    }

    /// Retruns the promotion piece.
    ///
    /// # Saftey
    ///
    /// This method should only be called if the move is a promotion, otherwise garbage will be
    /// returned.
    pub fn promotion_piece(self) -> PieceType {
        Self::piece_from_code(self.flags() & 0b0011)
    }

    // TODO: move_type
}

impl PartialEq<ParsedMove> for BitMove {
    fn eq(&self, other: &ParsedMove) -> bool {
        let promotion_piece = if self.is_promotion() {
            Some(self.promotion_piece())
        } else {
            None
        };

        self.origin() == other.origin
            && self.target() == other.target
            && promotion_piece == other.promotion_piece
    }
}

impl fmt::Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.origin(), self.target())?;
        if self.is_promotion() {
            write!(f, "{}", self.promotion_piece().to_char())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    use MoveFlags::*;
    use PieceType::*;
    use Square::*;

    #[test_case(E2, E3, QuietMove)]
    #[test_case(E2, E4, DoublePawnPush)]
    #[test_case(F7, F8, Promotion { piece: Knight, capture: false })]
    #[test_case(F7, F8, Promotion { piece: Rook, capture: false })]
    #[test_case(F7, F8, Promotion { piece: Bishop, capture: false })]
    #[test_case(F7, F8, Promotion { piece: Queen, capture: false })]
    #[test_case(F7, G8, Promotion { piece: Queen, capture: true })]
    #[test_case(C5, D4, Capture { en_passant: false })]
    #[test_case(D4, C3, Capture { en_passant: true })]
    #[test_case(E1, G1, Castle { kingside: true })]
    #[test_case(E1, C1, Castle { kingside: false })]
    #[test_case(E8, G8, Castle { kingside: true })]
    #[test_case(E8, C8, Castle { kingside: false })]
    fn bitmove_new(origin: Square, target: Square, flags: MoveFlags) {
        let bm = BitMove::new(origin, target, flags);

        assert_eq!(origin, bm.origin());
        assert_eq!(target, bm.target());

        let mut quiet = false;
        let mut capture = false;
        let mut en_passant = false;
        let mut promotion = false;
        let mut castle = false;
        let mut kingside_castle = false;
        let mut queenside_castle = false;
        let mut double_push = false;

        match flags {
            MoveFlags::Promotion {
                capture: c,
                piece: p,
            } => {
                promotion = true;
                capture = c;
                assert_eq!(p, bm.promotion_piece());
            }
            MoveFlags::Capture { en_passant: ep } => {
                capture = true;
                en_passant = ep;
            }
            MoveFlags::DoublePawnPush => {
                double_push = true;
            }
            MoveFlags::Castle { kingside } => {
                castle = true;
                kingside_castle = kingside;
                queenside_castle = !kingside;
            }
            MoveFlags::QuietMove => {
                quiet = true;
            }
        }

        assert_eq!(quiet, bm.is_quiet());
        assert_eq!(capture, bm.is_capture());
        assert_eq!(en_passant, bm.is_en_passant());
        assert_eq!(promotion, bm.is_promotion());
        assert_eq!(castle, bm.is_castle());
        assert_eq!(kingside_castle, bm.is_king_side_castle());
        assert_eq!(queenside_castle, bm.is_queen_side_castle());
        assert_eq!(double_push, bm.is_double_push());
    }

    #[test]
    fn bitmove_new_quiet() {
        let expected = BitMove::new(E2, E3, QuietMove);
        assert_eq!(expected, BitMove::new_quiet(E2, E3));
    }

    #[test]
    fn bitmove_new_pawn_push() {
        let expected = BitMove::new(E2, E4, DoublePawnPush);
        assert_eq!(expected, BitMove::new_pawn_push(E2, E4));
    }

    #[test]
    fn bitmove_new_capture() {
        let expected = BitMove::new(G4, H3, Capture { en_passant: false });
        assert_eq!(expected, BitMove::new_capture(G4, H3));
    }

    #[test]
    fn bitmove_new_en_passant() {
        let expected = BitMove::new(G4, H3, Capture { en_passant: true });
        assert_eq!(expected, BitMove::new_en_passant(G4, H3));
    }

    #[test]
    fn bitmove_new_promotion_capture() {
        let expected = BitMove::new(
            E7,
            F8,
            Promotion {
                capture: true,
                piece: Queen,
            },
        );
        assert_eq!(expected, BitMove::new_promotion_capture(E7, F8, Queen));
    }

    #[test]
    fn bitmove_new_promotion() {
        let expected = BitMove::new(
            E7,
            E8,
            Promotion {
                capture: false,
                piece: Queen,
            },
        );
        assert_eq!(expected, BitMove::new_promotion(E7, E8, Queen));
    }

    #[test]
    fn bitmove_new_castle_kingside() {
        let expected = BitMove::new(E1, G1, Castle { kingside: true });
        assert_eq!(expected, BitMove::new_castle_kingside(E1, G1));
    }

    #[test]
    fn bitmove_new_castle_queenside() {
        let expected = BitMove::new(E1, C1, Castle { kingside: false });
        assert_eq!(expected, BitMove::new_castle_queenside(E1, C1));
    }
}