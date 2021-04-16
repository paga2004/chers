use crate::position::BoardState;
use crate::{File, Rank};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::ops::Index;
use std::ops::IndexMut;

/// Represents a square on the board.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[allow(missing_docs)]
#[rustfmt::skip]
pub enum Square {
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// Creates a `Square` from file and rank.
    pub fn new(file: File, rank: Rank) -> Self {
        Self::from_index(21 + file as usize + 10 * rank as usize)
    }

    pub(crate) fn from_index(index: usize) -> Self {
        Self::try_from(index as u8).unwrap()
    }

    /// Returns the file of the field as an integer
    ///
    /// # Examples
    /// ```
    /// # use chers::Square;
    /// # use chers::File;
    /// assert_eq!(Square::A1.file(), File::A);
    /// assert_eq!(Square::E8.file(), File::E);
    /// assert_eq!(Square::H4.file(), File::H);
    /// ```
    pub fn file(self) -> File {
        // self as u8  % 8 is always in the range 0..=7 so the unwrap will never panic
        File::try_from(self as u8 % 10 - 1).unwrap()
    }

    /// Returns the rank of the field as an integer
    ///
    /// # Examples
    /// ```
    /// # use chers::Square;
    /// # use chers::Rank;
    /// assert_eq!(Square::A1.rank(), Rank::First);
    /// assert_eq!(Square::E8.rank(), Rank::Eighth);
    /// assert_eq!(Square::H4.rank(), Rank::Fourth);
    /// ```
    pub fn rank(self) -> Rank {
        // self as u8 / 8 is always in the range 0..=7 because self as u8 is always in the range
        // 0..=63 so the unwrap will never panic
        Rank::try_from(self as u8 / 10 - 2).unwrap()
    }
}

impl Index<Square> for [BoardState; 120] {
    type Output = BoardState;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Square> for [BoardState; 120] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_square_new() {
        assert_eq!(Square::new(File::A, Rank::First), Square::A1);
        assert_eq!(Square::new(File::A, Rank::Eighth), Square::A8);
        assert_eq!(Square::new(File::H, Rank::First), Square::H1);
        assert_eq!(Square::new(File::H, Rank::Eighth), Square::H8);
    }

    #[test]
    fn test_square_from_index() {
        for i in 2..=9 {
            for j in 1..=8 {
                let index = 10 * i + j;
                let s = Square::from_index(index);
                assert_eq!(s as usize, index);
            }
        }
    }

    #[test]
    fn test_square_file() {
        assert_eq!(Square::A1.file(), File::A);
        assert_eq!(Square::A2.file(), File::A);
        assert_eq!(Square::A8.file(), File::A);
        assert_eq!(Square::B1.file(), File::B);
        assert_eq!(Square::B2.file(), File::B);
        assert_eq!(Square::B8.file(), File::B);
        assert_eq!(Square::H1.file(), File::H);
        assert_eq!(Square::H2.file(), File::H);
        assert_eq!(Square::H8.file(), File::H);
    }

    #[test]
    fn test_square_rank() {
        assert_eq!(Square::A1.rank(), Rank::First);
        assert_eq!(Square::B1.rank(), Rank::First);
        assert_eq!(Square::H1.rank(), Rank::First);
        assert_eq!(Square::A2.rank(), Rank::Second);
        assert_eq!(Square::B2.rank(), Rank::Second);
        assert_eq!(Square::H2.rank(), Rank::Second);
        assert_eq!(Square::A8.rank(), Rank::Eighth);
        assert_eq!(Square::B8.rank(), Rank::Eighth);
        assert_eq!(Square::H8.rank(), Rank::Eighth);
    }
}
