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

    /// Returns `white` if `self == Color::White` and `black` if `self == Color::Black`
    ///
    /// # Examples
    ///
    /// ```
    /// # use chers::Color;
    /// let color = Color::White;
    ///
    /// assert_eq!(color.map("white", "black"), "white");
    /// ```
    pub fn map<T>(self, white: T, black: T) -> T {
        match self {
            Self::White => white,
            Self::Black => black,
        }
    }
}

impl ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.map(Color::Black, Color::White)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map("white", "black"))
    }
}
