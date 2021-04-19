use std::fmt;

/// Represents the castling rights for both players.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CastlingRights {
    pub(crate) white_king_side: bool,
    pub(crate) white_queen_side: bool,
    pub(crate) black_king_side: bool,
    pub(crate) black_queen_side: bool,
}

impl CastlingRights {
    /// Creates a new `CastlingRights`.
    pub fn new(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        Self {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == CastlingRights::new(false, false, false, false) {
            return write!(f, "-");
        }
        if self.white_king_side {
            write!(f, "K")?;
        }
        if self.white_queen_side {
            write!(f, "Q")?;
        }
        if self.black_king_side {
            write!(f, "k")?;
        }
        if self.black_queen_side {
            write!(f, "q")?;
        }

        Ok(())
    }
}
