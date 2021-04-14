//! A chess engine written in rust.

// Make writing "unsafe" in code a compilation error.
#![forbid(unsafe_code)]
// Warn on generally recommended lints that are not enabled by default.
#![warn(future_incompatible, rust_2018_idioms, unused, macro_use_extern_crate)]
// Warn on unnecessary code.
#![warn(unused_lifetimes, single_use_lifetimes, unreachable_pub, trivial_casts)]
// Warn on missing implementations which make life easier.
#![warn(missing_debug_implementations, missing_copy_implementations)]
// Warn on missing docs.
#![warn(missing_docs)]

mod color;
mod fen;
mod file;
mod r#move;
mod piece;
mod position;
mod rank;
mod square;

pub mod error;

pub use file::File;
pub use rank::Rank;
pub use square::Square;

pub use color::Color;
pub use piece::Piece;
pub use piece::PieceType;

pub use r#move::Move;

pub use position::Position;
