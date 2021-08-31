use std::fmt;
use std::ops::Add;
use std::ops::Sub;

/// A file (otherwise known as column) on the board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct File(u8);

#[allow(missing_docs)]
impl File {
    pub const A: Self = Self(0);
    pub const B: Self = Self(1);
    pub const C: Self = Self(2);
    pub const D: Self = Self(3);
    pub const E: Self = Self(4);
    pub const F: Self = Self(5);
    pub const G: Self = Self(6);
    pub const H: Self = Self(7);

    /// Creates a new `File`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not in the range `0..=7`.
    pub fn new(index: u8) -> Self {
        debug_assert!(index < 8);
        Self(index)
    }

    /// Creates a new `File` from a character or returns [`None`] if `c` is not in the range
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

    pub(crate) fn to_u8(self) -> u8 {
        self.0
    }

    pub(crate) fn to_u16(self) -> u16 {
        self.0 as u16
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (b'a' + self.0) as char)
    }
}

impl Add<u8> for File {
    type Output = Self;

    /// Adds a `File` and a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if `self as u8 + rhs >= 8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::File;
    ///
    /// assert_eq!(File::E + 2, File::G);
    /// ```
    fn add(self, rhs: u8) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl Sub<u8> for File {
    type Output = Self;

    /// Subtracts a `u8` from a `File`.
    ///
    /// # Panics
    ///
    /// Panics if `self as u8 - rhs` < 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::File;
    ///
    /// assert_eq!(File::E - 2, File::C);
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
    fn test_file_new() {
        for i in 0..8 {
            let f = File::new(i);
            assert_eq!(f.0, i);
        }
    }
}
