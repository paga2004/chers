use std::fmt;
use std::ops::Add;
use std::ops::Sub;

/// A rank (otherwise known as row) on the board.
#[derive(Clone, Copy, PartialEq)]
pub struct Rank(u8);

#[allow(missing_docs)]
impl Rank {
    pub const FIRST: Self = Self(0);
    pub const SECOND: Self = Self(1);
    pub const THIRD: Self = Self(2);
    pub const FOURTH: Self = Self(3);
    pub const FIFTH: Self = Self(4);
    pub const SIXTH: Self = Self(5);
    pub const SEVENTH: Self = Self(6);
    pub const EIGHTH: Self = Self(7);

    /// Creates a new `Rank`.
    ///
    /// # Panics
    ///
    /// Panics if the index is not in the range `0..=7`.
    #[inline]
    pub fn new(index: u8) -> Self {
        debug_assert!(index < 8);
        Self(index)
    }

    /// Creates a new `Rank` from a character or returns `None` if `c` is not in the range
    /// `1..=8`.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::FIRST),
            '2' => Some(Self::SECOND),
            '3' => Some(Self::THIRD),
            '4' => Some(Self::FOURTH),
            '5' => Some(Self::FIFTH),
            '6' => Some(Self::SIXTH),
            '7' => Some(Self::SEVENTH),
            '8' => Some(Self::EIGHTH),
            _ => None,
        }
    }

    /// Returns the letter representing the `Rank`
    pub fn to_char(self) -> char {
        (self.0 + b'1') as char
    }

    #[inline]
    pub(crate) fn to_u8(self) -> u8 {
        self.0
    }

    #[inline]
    pub(crate) fn to_u16(self) -> u16 {
        self.0 as u16
    }

    /// Retruns the number in the range 0..8 corresponding to the rank
    #[inline]
    pub fn to_i32(self) -> i32 {
        self.0 as i32
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0 + 1)
    }
}

impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Add<u8> for Rank {
    type Output = Self;

    /// Adds a `Rank` and a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if `self as u8 + rhs >= 8`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::Rank;
    /// assert_eq!(Rank::FIFTH + 2, Rank::SEVENTH);
    /// ```
    fn add(self, rhs: u8) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl Sub<u8> for Rank {
    type Output = Self;

    /// Subtracts a `u8` from a `Rank`.
    ///
    /// # Panics
    ///
    /// Panics if `self as u8 - rhs` < 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::Rank;
    /// assert_eq!(Rank::FIFTH - 2, Rank::THIRD);
    /// ```
    fn sub(self, rhs: u8) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_rank_new() {
        for i in 0..8 {
            let r = Rank::new(i);
            assert_eq!(r.0, i);
        }
    }
}
