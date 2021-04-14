use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

/// Represents a rank on the board.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[allow(missing_docs)]
#[rustfmt::skip]
pub enum Rank{
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    /// Creates a new `Rank`
    ///
    /// # Panics
    ///
    /// Panics if the index is not in the range `0..=7`
    pub fn new(index: u8) -> Self {
        Self::try_from(index).unwrap()
    }

    /// Creates a new `Rank` from a char.
    ///
    /// # Panics
    ///
    /// Panics if `c` is not in the range `'1'..='8'`
    pub fn from_char(c: char) -> Self {
        match c {
            '1' => Self::First,
            '2' => Self::Second,
            '3' => Self::Third,
            '4' => Self::Fourth,
            '5' => Self::Fifth,
            '6' => Self::Sixth,
            '7' => Self::Seventh,
            '8' => Self::Eighth,
            _ => panic!("Invalid char {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_new() {
        for i in 0..8 {
            let r = Rank::new(i);
            assert_eq!(r as u8, i);
        }
    }
}
