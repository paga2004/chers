use std::fmt;
use std::ops;

/// The color of a player or a piece.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color(bool); // using bool instead of u8 allows easier match statements and possibly further optimizations

impl Color {
    /// White
    pub const WHITE: Self = Self(false);
    /// White
    pub const BLACK: Self = Self(true);
    /// Creates a `Color` from its english letter or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Color;
    ///
    /// assert_eq!(Color::from_char('W'), Some(Color::WHITE));
    /// assert_eq!(Color::from_char('w'), Some(Color::WHITE));
    /// assert_eq!(Color::from_char('B'), Some(Color::BLACK));
    /// assert_eq!(Color::from_char('b'), Some(Color::BLACK));
    ///
    /// assert_eq!(Color::from_char('X'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'w' | 'W' => Some(Self::WHITE),
            'b' | 'B' => Some(Self::BLACK),
            _ => None,
        }
    }

    /// Returns `white` if `self == Color::WHITE` and `black` if `self == Color::BLACK`
    ///
    /// # Examples
    ///
    /// ```
    /// use chers::Color;
    ///
    /// let color = Color::WHITE;
    ///
    /// assert_eq!(color.map("white", "black"), "white");
    /// ```
    #[inline]
    pub fn map<T>(self, white: T, black: T) -> T {
        match self {
            Self::WHITE => white,
            Self::BLACK => black,
        }
    }

    #[inline]
    pub(crate) const fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub(crate) const fn to_u8(self) -> u8 {
        self.0 as u8
    }

    #[inline]
    pub(crate) fn from_bool(b: bool) -> Self {
        Self(b)
    }
}

impl ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.map(Color::BLACK, Color::WHITE)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map("white", "black"))
    }
}
