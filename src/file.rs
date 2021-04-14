use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

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
