use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

use crate::error::ParseSquareError;
use crate::Piece;
use crate::{File, Rank};

/// A square on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square(u8);

#[allow(missing_docs)]
impl Square {
    pub const A1: Self = Self(21);
    pub const B1: Self = Self(22);
    pub const C1: Self = Self(23);
    pub const D1: Self = Self(24);
    pub const E1: Self = Self(25);
    pub const F1: Self = Self(26);
    pub const G1: Self = Self(27);
    pub const H1: Self = Self(28);
    pub const A2: Self = Self(31);
    pub const B2: Self = Self(32);
    pub const C2: Self = Self(33);
    pub const D2: Self = Self(34);
    pub const E2: Self = Self(35);
    pub const F2: Self = Self(36);
    pub const G2: Self = Self(37);
    pub const H2: Self = Self(38);
    pub const A3: Self = Self(41);
    pub const B3: Self = Self(42);
    pub const C3: Self = Self(43);
    pub const D3: Self = Self(44);
    pub const E3: Self = Self(45);
    pub const F3: Self = Self(46);
    pub const G3: Self = Self(47);
    pub const H3: Self = Self(48);
    pub const A4: Self = Self(51);
    pub const B4: Self = Self(52);
    pub const C4: Self = Self(53);
    pub const D4: Self = Self(54);
    pub const E4: Self = Self(55);
    pub const F4: Self = Self(56);
    pub const G4: Self = Self(57);
    pub const H4: Self = Self(58);
    pub const A5: Self = Self(61);
    pub const B5: Self = Self(62);
    pub const C5: Self = Self(63);
    pub const D5: Self = Self(64);
    pub const E5: Self = Self(65);
    pub const F5: Self = Self(66);
    pub const G5: Self = Self(67);
    pub const H5: Self = Self(68);
    pub const A6: Self = Self(71);
    pub const B6: Self = Self(72);
    pub const C6: Self = Self(73);
    pub const D6: Self = Self(74);
    pub const E6: Self = Self(75);
    pub const F6: Self = Self(76);
    pub const G6: Self = Self(77);
    pub const H6: Self = Self(78);
    pub const A7: Self = Self(81);
    pub const B7: Self = Self(82);
    pub const C7: Self = Self(83);
    pub const D7: Self = Self(84);
    pub const E7: Self = Self(85);
    pub const F7: Self = Self(86);
    pub const G7: Self = Self(87);
    pub const H7: Self = Self(88);
    pub const A8: Self = Self(91);
    pub const B8: Self = Self(92);
    pub const C8: Self = Self(93);
    pub const D8: Self = Self(94);
    pub const E8: Self = Self(95);
    pub const F8: Self = Self(96);
    pub const G8: Self = Self(97);
    pub const H8: Self = Self(98);

    /// Creates a `Square` from file and rank.
    #[inline]
    pub fn new(file: File, rank: Rank) -> Self {
        Self(21 + file.to_u8() + 10 * rank.to_u8())
    }

    /// Creates a new `Square` from a `&str` in algebraic notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::{Square, error::ParseSquareError};
    ///
    /// assert_eq!(Square::from_algebraic_notation("a1"), Ok(Square::A1));
    /// assert_eq!(Square::from_algebraic_notation("e4"), Ok(Square::E4));
    /// assert_eq!(Square::from_algebraic_notation("g8"), Ok(Square::G8));
    ///
    /// assert_eq!(Square::from_algebraic_notation(""), Err(ParseSquareError::TooShort));
    /// assert_eq!(Square::from_algebraic_notation("a"), Err(ParseSquareError::TooShort));
    /// assert_eq!(Square::from_algebraic_notation("aa"), Err(ParseSquareError::InvalidRank('a')));
    /// ```
    pub fn from_algebraic_notation(s: &str) -> Result<Self, ParseSquareError> {
        let mut chars = s.chars();
        let f = chars.next().ok_or(ParseSquareError::TooShort)?;
        let r = chars.next().ok_or(ParseSquareError::TooShort)?;
        let file = File::from_char(f).ok_or(ParseSquareError::InvalidFile(f))?;
        let rank = Rank::from_char(r).ok_or(ParseSquareError::InvalidRank(r))?;

        Ok(Square::new(file, rank))
    }

    #[inline]
    pub(crate) fn from_index(index: usize) -> Self {
        Self(index as u8)
    }

    /// Returns the file of the field.
    ///
    /// # Examples
    /// ```
    /// use chers::{Square, File};
    ///
    /// assert_eq!(Square::A1.file(), File::A);
    /// assert_eq!(Square::E8.file(), File::E);
    /// assert_eq!(Square::H4.file(), File::H);
    /// ```
    #[inline]
    pub fn file(self) -> File {
        File::new(self.0 % 10 - 1)
    }

    /// Returns the rank of the field.
    ///
    /// # Examples
    /// ```
    /// use chers::{Square, Rank};
    ///
    /// assert_eq!(Square::A1.rank(), Rank::FIRST);
    /// assert_eq!(Square::E8.rank(), Rank::EIGHTH);
    /// assert_eq!(Square::H4.rank(), Rank::FOURTH);
    /// ```
    #[inline]
    pub fn rank(self) -> Rank {
        Rank::new(self.0 / 10 - 2)
    }

    #[inline]
    pub(crate) fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub(crate) fn to_i8(self) -> i8 {
        self.0 as i8
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl<T> Index<Square> for [T; 120] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl IndexMut<Square> for [Piece; 120] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_square_new() {
        assert_eq!(Square::new(File::A, Rank::FIRST), Square::A1);
        assert_eq!(Square::new(File::A, Rank::EIGHTH), Square::A8);
        assert_eq!(Square::new(File::H, Rank::FIRST), Square::H1);
        assert_eq!(Square::new(File::H, Rank::EIGHTH), Square::H8);
    }

    #[test]
    fn test_square_from_index() {
        for i in 2..=9 {
            for j in 1..=8 {
                let index = 10 * i + j;
                let s = Square::from_index(index);
                assert_eq!(s.0 as usize, index);
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
        assert_eq!(Square::A1.rank(), Rank::FIRST);
        assert_eq!(Square::B1.rank(), Rank::FIRST);
        assert_eq!(Square::H1.rank(), Rank::FIRST);
        assert_eq!(Square::A2.rank(), Rank::SECOND);
        assert_eq!(Square::B2.rank(), Rank::SECOND);
        assert_eq!(Square::H2.rank(), Rank::SECOND);
        assert_eq!(Square::A8.rank(), Rank::EIGHTH);
        assert_eq!(Square::B8.rank(), Rank::EIGHTH);
        assert_eq!(Square::H8.rank(), Rank::EIGHTH);
    }

    #[test]
    fn test_square_display() {
        assert_eq!(format!("{}", Square::A1), "a1");
    }
}
