use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

/// Represents a square on the board.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[allow(missing_docs)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// Creates a `Square` from file and rank.
    ///
    /// # Panics
    ///
    /// Panics if `file` or `rank` are not in the range 0..=7
    pub fn new(file: usize, rank: usize) -> Self {
        assert!(file < 8);
        assert!(rank < 8);
        Self::from_index(file + 8 * rank)
    }

    pub(crate) fn from_index(index: usize) -> Self {
        Self::try_from(index as u8).unwrap()
    }

    /// Returns the file of the field as an integer
    ///
    /// # Examples
    /// ```
    /// # use chers::Square;
    /// assert_eq!(Square::A1.file(), 0);
    /// assert_eq!(Square::E8.file(), 4);
    /// assert_eq!(Square::H4.file(), 7);
    /// ```
    pub fn file(self) -> usize {
        self as usize % 8
    }

    /// Returns the rank of the field as an integer
    ///
    /// # Examples
    /// ```
    /// # use chers::Square;
    /// assert_eq!(Square::A1.rank(), 0);
    /// assert_eq!(Square::E8.rank(), 7);
    /// assert_eq!(Square::H4.rank(), 3);
    /// ```
    pub fn rank(self) -> usize {
        self as usize / 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_new() {
        use Square::*;
        assert_eq!(Square::new(0, 0), A1);
        assert_eq!(Square::new(0, 7), A8);
        assert_eq!(Square::new(7, 0), H1);
        assert_eq!(Square::new(7, 7), H8);
    }

    #[test]
    fn test_square_from_index() {
        for i in 0..64 {
            let f = Square::from_index(i);
            assert_eq!(i, f as usize);
        }
    }

    #[test]
    #[should_panic]
    fn test_square_new_out_of_bounds_square() {
        let _ = Square::new(8, 0);
    }

    #[test]
    #[should_panic]
    fn test_square_new_out_of_bounds_rank() {
        let _ = Square::new(0, 8);
    }

    #[test]
    fn test_square_file() {
        use Square::*;
        assert_eq!(A1.file(), 0);
        assert_eq!(A2.file(), 0);
        assert_eq!(A8.file(), 0);
        assert_eq!(B1.file(), 1);
        assert_eq!(B2.file(), 1);
        assert_eq!(B8.file(), 1);
        assert_eq!(H1.file(), 7);
        assert_eq!(H2.file(), 7);
        assert_eq!(H8.file(), 7);
    }

    #[test]
    fn test_square_rank() {
        use Square::*;
        assert_eq!(A1.rank(), 0);
        assert_eq!(B1.rank(), 0);
        assert_eq!(H1.rank(), 0);
        assert_eq!(A2.rank(), 1);
        assert_eq!(B2.rank(), 1);
        assert_eq!(H2.rank(), 1);
        assert_eq!(A8.rank(), 7);
        assert_eq!(B8.rank(), 7);
        assert_eq!(H8.rank(), 7);
    }
}
