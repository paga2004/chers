use std::fmt;

use crate::File;
use crate::ParsedMove;
use crate::PieceType;
use crate::Rank;
use crate::Square;

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
///
/// The advantage of this flags is that we can order the moves in a reasonable way simply by
/// interpreting the move as a 16-bit number(promotion > captures > quiet).
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct BitMove(u16);

impl BitMove {
    pub(crate) const NULL: Self = Self(0);

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
    #[inline]
    pub fn new_quiet(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::QUIET)
    }

    /// Creates a new double pawn push move.
    #[inline]
    pub fn new_pawn_push(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::DOUBLE_PAWN_PUSH)
    }

    /// Creates a new capture move.
    #[inline]
    pub fn new_capture(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::CAPTURE)
    }

    /// Creates a new en passant capture move.
    #[inline]
    pub fn new_en_passant(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::EN_PASSANT)
    }

    /// Creates a new pawn promotion capture move.
    #[inline]
    pub fn new_promotion_capture(origin: Square, target: Square, piece: PieceType) -> Self {
        Self::from_flag_bits(
            origin,
            target,
            Self::PROMOTION | Self::CAPTURE | Self::piece_to_code(piece),
        )
    }

    /// Creates a new pawn promotion move.
    #[inline]
    pub fn new_promotion(origin: Square, target: Square, piece: PieceType) -> Self {
        Self::from_flag_bits(origin, target, Self::PROMOTION | Self::piece_to_code(piece))
    }

    /// Creates a new king side castle move.
    #[inline]
    pub fn new_castle_kingside(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::KING_SIDE_CASTLE)
    }

    /// Creates a new queen side castle move.
    #[inline]
    pub fn new_castle_queenside(origin: Square, target: Square) -> Self {
        Self::from_flag_bits(origin, target, Self::QUEEN_SIDE_CASTLE)
    }

    #[inline]
    fn from_flag_bits(origin: Square, target: Square, flags: u16) -> Self {
        Self(
            Self::square_to_promotion_code(origin)
                | (Self::square_to_promotion_code(target) << 6)
                | flags << 12,
        )
    }

    #[inline]
    fn square_to_promotion_code(sq: Square) -> u16 {
        sq.file().to_u16() | (sq.rank().to_u16() << 3)
    }

    #[inline]
    fn square_from_code(code: u16) -> Square {
        let file = File::new((code & 7) as u8);
        let rank = Rank::new(((code >> 3) & 7) as u8);
        Square::new(file, rank)
    }

    #[inline]
    fn piece_to_code(piece: PieceType) -> u16 {
        match piece {
            PieceType::KNIGHT => 0,
            PieceType::BISHOP => 1,
            PieceType::ROOK => 2,
            PieceType::QUEEN => 3,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn piece_from_code(code: u16) -> PieceType {
        debug_assert!(code < 4);

        match code {
            0 => PieceType::KNIGHT,
            1 => PieceType::BISHOP,
            2 => PieceType::ROOK,
            3 => PieceType::QUEEN,
            _ => unreachable!(),
        }
    }

    /// Returns the origin square.
    #[inline]
    pub fn origin(self) -> Square {
        Self::square_from_code(self.0 & 0b111111)
    }

    /// Returns the target square.
    #[inline]
    pub fn target(self) -> Square {
        Self::square_from_code((self.0 >> 6) & 0b111111)
    }

    #[inline]
    fn flags(self) -> u16 {
        self.0 >> 12
    }

    /// Returns if the move is a capture.
    #[inline]
    pub fn is_capture(self) -> bool {
        self.flags() & Self::CAPTURE != 0
    }

    /// Returns if the move is quiet (i.e no capture, promotion, castle, or double pawn push).
    #[inline]
    pub fn is_quiet(self) -> bool {
        self.flags() == 0
    }

    /// Returns if the move is a promotion.
    #[inline]
    pub fn is_promotion(self) -> bool {
        self.flags() & Self::PROMOTION != 0
    }

    /// Returns if the move is a castle.
    #[inline]
    pub fn is_castle(self) -> bool {
        self.flags() >> 1 == 1
    }

    /// Returns if the move is a king side castle.
    #[inline]
    pub fn is_king_side_castle(self) -> bool {
        self.flags() == Self::KING_SIDE_CASTLE
    }

    /// Returns if the move is a queen side castle.
    #[inline]
    pub fn is_queen_side_castle(self) -> bool {
        self.flags() == Self::QUEEN_SIDE_CASTLE
    }

    /// Returns if the move is an en passant capture.
    #[inline]
    pub fn is_en_passant(self) -> bool {
        self.flags() == Self::EN_PASSANT
    }

    /// Return if the move is a double pawn push.
    #[inline]
    pub fn is_double_push(self) -> bool {
        self.flags() == Self::DOUBLE_PAWN_PUSH
    }

    /// Retruns the promotion piece.
    ///
    /// # Saftey
    ///
    /// This method should only be called if the move is a promotion, otherwise garbage will be
    /// returned.
    #[inline]
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

    #[test_case(Square::E2, Square::E3, QuietMove)]
    #[test_case(Square::E2, Square::E4, DoublePawnPush)]
    #[test_case(Square::F7, Square::F8, Promotion { piece: PieceType::KNIGHT, capture: false })]
    #[test_case(Square::F7, Square::F8, Promotion { piece: PieceType::ROOK, capture: false })]
    #[test_case(Square::F7, Square::F8, Promotion { piece: PieceType::BISHOP, capture: false })]
    #[test_case(Square::F7, Square::F8, Promotion { piece: PieceType::QUEEN, capture: false })]
    #[test_case(Square::F7, Square::G8, Promotion { piece: PieceType::QUEEN, capture: true })]
    #[test_case(Square::C5, Square::D4, Capture { en_passant: false })]
    #[test_case(Square::D4, Square::C3, Capture { en_passant: true })]
    #[test_case(Square::E1, Square::G1, Castle { kingside: true })]
    #[test_case(Square::E1, Square::C1, Castle { kingside: false })]
    #[test_case(Square::E8, Square::G8, Castle { kingside: true })]
    #[test_case(Square::E8, Square::C8, Castle { kingside: false })]
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
        let expected = BitMove::new(Square::E2, Square::E3, QuietMove);
        assert_eq!(expected, BitMove::new_quiet(Square::E2, Square::E3));
    }

    #[test]
    fn bitmove_new_pawn_push() {
        let expected = BitMove::new(Square::E2, Square::E4, DoublePawnPush);
        assert_eq!(expected, BitMove::new_pawn_push(Square::E2, Square::E4));
    }

    #[test]
    fn bitmove_new_capture() {
        let expected = BitMove::new(Square::G4, Square::H3, Capture { en_passant: false });
        assert_eq!(expected, BitMove::new_capture(Square::G4, Square::H3));
    }

    #[test]
    fn bitmove_new_en_passant() {
        let expected = BitMove::new(Square::G4, Square::H3, Capture { en_passant: true });
        assert_eq!(expected, BitMove::new_en_passant(Square::G4, Square::H3));
    }

    #[test]
    fn bitmove_new_promotion_capture() {
        let expected = BitMove::new(
            Square::E7,
            Square::F8,
            Promotion {
                capture: true,
                piece: PieceType::QUEEN,
            },
        );
        assert_eq!(
            expected,
            BitMove::new_promotion_capture(Square::E7, Square::F8, PieceType::QUEEN)
        );
    }

    #[test]
    fn bitmove_new_promotion() {
        let expected = BitMove::new(
            Square::E7,
            Square::E8,
            Promotion {
                capture: false,
                piece: PieceType::QUEEN,
            },
        );
        assert_eq!(
            expected,
            BitMove::new_promotion(Square::E7, Square::E8, PieceType::QUEEN)
        );
    }

    #[test]
    fn bitmove_new_castle_kingside() {
        let expected = BitMove::new(Square::E1, Square::G1, Castle { kingside: true });
        assert_eq!(
            expected,
            BitMove::new_castle_kingside(Square::E1, Square::G1)
        );
    }

    #[test]
    fn bitmove_new_castle_queenside() {
        let expected = BitMove::new(Square::E1, Square::C1, Castle { kingside: false });
        assert_eq!(
            expected,
            BitMove::new_castle_queenside(Square::E1, Square::C1)
        );
    }
}
