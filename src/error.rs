//! Types representing various errors.

use thiserror::Error;

/// Error returned by [`Position::from_fen`](../struct.Position.html#method.from_fen)
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenParsingError {
    /// FEN too short
    #[error("FEN too short")]
    TooShort,
    /// Invalid piece character
    #[error("invalid piece: {0}")]
    InvalidPiece(char),
    /// Invalid color character
    #[error("invalid color: {0}")]
    InvalidColor(char),
    #[error("wrong number of files")]
    /// Wrong number of files in the first field
    WrongNumberOfFiles,
    /// Invalid castling rights
    #[error("invalid castling rights")]
    InvalidCastlingRights,
    /// Invalid en passant square
    #[error("invalid en passant square")]
    InvalidEnPassantSquare,
    /// Invalid halfmove clock
    #[error("invalid halfmove clock")]
    InvalidHalfMoveClock,
    /// Invalid fullmove number
    #[error("invalid fullmove number")]
    InvalidFullmoveNumber,
}

/// Error returned by [`Move::from_smith_notation`](../struct.Move.html#method.from_smith_notation)
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveParsingError {
    /// Move too short
    #[error("Move too short")]
    TooShort,
    /// Invalid file character
    #[error("invalid file: {0}")]
    InvalidFile(char),
    /// Invalid rank character
    #[error("invalid rank: {0}")]
    InvalidRank(char),
    /// Invalid promotion piece character
    #[error("invalid capture indicator: {0}")]
    InvalidCaptureIndicatior(char),
    /// Invalid promotion piece character
    #[error("invalid promotion piece: {0}")]
    InvalidPromotionPiece(char),
}