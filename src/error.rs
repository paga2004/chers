//! Types representing various errors.

use thiserror::Error;

/// Error returned by [`Position::from_fen`](crate::Position::from_fen).
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseFenError {
    /// FEN too short
    #[error("too short")]
    TooShort,
    /// Invalid piece character
    #[error("invalid piece ({0})")]
    InvalidPiece(char),
    /// Invalid color character
    #[error("invalid color ({0})")]
    InvalidColor(char),
    #[error("wrong number of files")]
    /// Wrong number of files in the first field of the fen
    WrongNumberOfFiles,
    /// Invalid castling rights
    #[error("invalid castling rights (unexpected charater {0})")]
    InvalidCastlingRights(char),
    /// Invalid en passant square
    #[error("invalid en passant square")]
    InvalidEnPassantSquare(#[from] ParseSquareError),
    /// Invalid halfmove clock
    #[error("invalid halfmove clock")]
    InvalidHalfMoveClock,
    /// Invalid fullmove number
    #[error("invalid fullmove number")]
    InvalidFullmoveNumber,
}

/// Error returned by [`ParsedMove::from_coordinate_notation`](crate::ParsedMove::from_coordinate_notation).
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMoveError {
    /// Move too short
    #[error("too short")]
    TooShort,
    /// Invalid file character
    #[error("invalid square ({0})")]
    InvalidSquare(#[from] ParseSquareError),
    /// Invalid promotion piece character
    #[error("invalid promotion piece ({0})")]
    InvalidPromotionPiece(char),
}

/// Error returned by [`Square::from_algebraic_notation`](crate::Square::from_algebraic_notation).
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseSquareError {
    /// Square too short
    #[error("too short")]
    TooShort,
    /// Invalid file character
    #[error("invalid file ({0})")]
    InvalidFile(char),
    /// Invalid rank character
    #[error("invalid rank ({0})")]
    InvalidRank(char),
}
