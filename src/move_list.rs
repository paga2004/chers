use arrayvec::ArrayVec;

use crate::BitMove;

/// A container for moves.
///
/// # Examples
///
/// ```
/// use chers::{MoveList, BitMove, Square};
///
/// let m1 = BitMove::new_quiet(Square::E2, Square::E3);
/// let m2 = BitMove::new_quiet(Square::E7, Square::E6);
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
pub type MoveList = ArrayVec<BitMove, 256>;
