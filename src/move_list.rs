use crate::Move;

/// A container for moves.
///
/// # Examples
///
/// ```
/// # use chers::{MoveList, Move};
///
/// let m1 = Move::from_smith_notation("e2e4").unwrap();
/// let m2 = Move::from_smith_notation("e7e5").unwrap();
///
/// let mut list = MoveList::new();
///
/// list.push(m1);
/// list.push(m2);
///
/// assert_eq!(list.len(), 2);
/// assert_eq!(list[0], m1);
///
/// assert_eq!(list.pop(), Some(m2));
/// assert_eq!(list.len(), 1);
///
/// for m in &list {
///     println!("{:?}", m);
/// }
/// ```
pub type MoveList = Vec<Move>;
