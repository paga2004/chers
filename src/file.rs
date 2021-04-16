use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::ops::Add;
use std::ops::Sub;

/// Represents a file on the board.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[allow(missing_docs)]
#[rustfmt::skip]
pub enum File {
    A, B, C, D, E, F, G, H,
}

impl File {
    /// Creates a new `File`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not in the range `0..=7`
    pub fn new(index: u8) -> Self {
        Self::try_from(index).unwrap()
    }

    /// Creates a new `File` from a character or returns `None` if `c` is not in the range
    /// `'a'..='h'`.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(Self::A),
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'd' => Some(Self::D),
            'e' => Some(Self::E),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'h' => Some(Self::H),
            _ => None,
        }
    }
}

impl Add<u8> for File {
    type Output = Self;

    /// Adds a `File` and a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if `self as u8 + rhs >= 8`
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::File;
    /// assert_eq!(File::E + 2, File::G);
    /// ```
    fn add(self, rhs: u8) -> Self::Output {
        Self::new(self as u8 + rhs)
    }
}

impl Sub<u8> for File {
    type Output = Self;

    /// Subtracts a `u8` from a `File`.
    ///
    /// # Panics
    ///
    /// Panics if `self as u8 - rhs` < 0`
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::File;
    /// assert_eq!(File::E - 2, File::C);
    /// ```
    fn sub(self, rhs: u8) -> Self::Output {
        Self::new(self as u8 - rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_new() {
        for i in 0..8 {
            let f = File::new(i);
            assert_eq!(f as u8, i);
        }
    }
}
