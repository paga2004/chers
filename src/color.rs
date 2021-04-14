use std::fmt;
use std::ops;

/// Represents the color of a player or a piece.
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Creates a `Color` from its english letter or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Color;
    ///
    /// assert_eq!(Color::from_char('W'), Some(Color::White));
    /// assert_eq!(Color::from_char('w'), Some(Color::White));
    /// assert_eq!(Color::from_char('B'), Some(Color::Black));
    /// assert_eq!(Color::from_char('b'), Some(Color::Black));
    ///
    /// assert_eq!(Color::from_char('X'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'w' | 'W' => Some(Color::White),
            'b' | 'B' => Some(Color::Black),
            _ => None,
        }
    }
}

impl ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White")?,
            Color::Black => write!(f, "Black")?,
        }
        Ok(())
    }
}
