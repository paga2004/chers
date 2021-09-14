use std::fmt;

use crate::Square;

/// Compressed representation of the castling_rights of both players in just 4 bits. This way they
/// can be easily updated with a castle mask.
///
/// | bin  | dec | castling rights                         |
/// |------|-----|-----------------------------------------|
/// | 0001 |  1  | white king can castle to the king side  |
/// | 0010 |  2  | white king can castle to the queen side |
/// | 0100 |  4  | black king can castle to the king side  |
/// | 1000 |  8  | black king can castle to the queen side |
///
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CastlingRights(u8);

impl CastlingRights {
    /// Creates a new `CastlingRights`.
    pub fn new(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        Self(
            white_king_side as u8
                | (white_queen_side as u8) << 1
                | (black_king_side as u8) << 2
                | (black_queen_side as u8) << 3,
        )
    }
}

impl CastlingRights {
    /// Returns wether the white king can castle kingside.
    #[inline]
    pub fn white_king_side(self) -> bool {
        self.0 & 1 != 0
    }

    /// Returns wether the white king can castle queenside.
    #[inline]
    pub fn white_queen_side(self) -> bool {
        self.0 & 2 != 0
    }

    /// Returns wether the black king can castle kingside.
    #[inline]
    pub fn black_king_side(self) -> bool {
        self.0 & 4 != 0
    }

    /// Returns wether the black king can castle queenside.
    #[inline]
    pub fn black_queen_side(self) -> bool {
        self.0 & 8 != 0
    }

    /// Update the castling rights with a castling mask.
    ///
    /// | move                      | castling right | move update | new castling right |
    /// |---------------------------|----------------|-------------|--------------------|
    /// | king & rooks didn't move: | 1111           |  & 1111     |  =  1111    (15)   |
    /// |                           |                |             |
    /// | white king  moved:        | 1111           |  & 1100     |  =  1100    (12)   |
    /// | white king's rook moved:  | 1111           |  & 1110     |  =  1110    (14)   |
    /// | white queen's rook moved: | 1111           |  & 1101     |  =  1101    (13)   |
    /// |                           |                |             |                    |
    /// | black king moved:         | 1111           |  & 0011     |  =  1011    (3)    |
    /// | black king's rook moved:  | 1111           |  & 1011     |  =  1011    (11)   |
    /// | black queen's rook moved: | 1111           |  & 0111     |  =  0111    (7)    |
    ///
    #[inline]
    pub fn update(&mut self, sq: Square) {
        #[rustfmt::skip]
        const CASTLE_MASK: [u8; 120] = [
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 13, 15, 15, 15, 12, 15, 15, 14, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15,  7, 15, 15, 15,  3, 15, 15, 11, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
        ];

        self.0 &= CASTLE_MASK[sq];
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(15)
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == CastlingRights::new(false, false, false, false) {
            return write!(f, "-");
        }
        if self.white_king_side() {
            write!(f, "K")?;
        }
        if self.white_queen_side() {
            write!(f, "Q")?;
        }
        if self.black_king_side() {
            write!(f, "k")?;
        }
        if self.black_queen_side() {
            write!(f, "q")?;
        }

        Ok(())
    }
}
