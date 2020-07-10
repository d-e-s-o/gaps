// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Bound;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;
use std::ops::RangeBounds;

use crate::Inc;


/// Check whether a "start" bound is less than another one.
pub(crate) fn start_lt_start<T>(b1: &Bound<T>, b2: &Bound<T>) -> bool
where
  T: Copy + Ord + Inc,
{
  match (b1, b2) {
    (Unbounded, _) => false,
    (_, Unbounded) => true,
    (Included(b1), Included(b2)) => *b1 < *b2,
    (Included(b1), Excluded(b2)) => *b1 <= *b2,
    (Excluded(b1), Included(b2)) => b1.inc() < *b2,
    (Excluded(b1), Excluded(b2)) => *b1 < *b2,
  }
}

/// Check whether a "start" bound is less than or equal to another one.
pub(crate) fn start_le_start<T>(b1: &Bound<T>, b2: &Bound<T>) -> bool
where
  T: Copy + Ord + Inc,
{
  match (b1, b2) {
    (_, Unbounded) => true,
    (Unbounded, _) => false,
    (Included(b1), Included(b2)) => *b1 <= *b2,
    (Included(b1), Excluded(b2)) => *b1 <= b2.inc(),
    (Excluded(b1), Included(b2)) => b1.inc() <= *b2,
    (Excluded(b1), Excluded(b2)) => *b1 <= *b2,
  }
}

/// Check whether a "start" bound is less than or equal to an "end"
/// bound.
pub(crate) fn start_le_end<T>(b1: &Bound<T>, b2: &Bound<T>) -> bool
where
  T: Copy + Ord + Inc,
{
  match (b1, b2) {
    (_, Unbounded) => true,
    (Unbounded, _) => true,
    (Included(b1), Included(b2)) => *b1 <= *b2,
    (Included(b1), Excluded(b2)) => *b1 < *b2,
    (Excluded(b1), Included(b2)) => *b1 < *b2,
    (Excluded(b1), Excluded(b2)) => {
      // This case is a bit tricky in that we can't fudge it with merely
      // comparison operations. `(1..2)`, for example, should not result
      // in `true` being reported. We need to increment the start value
      // by one to get the proper check.
      b1.inc() < *b2
    },
  }
}

/// Check whether an "end" bound is less than another one.
pub(crate) fn end_lt_end<T>(b1: &Bound<T>, b2: &Bound<T>) -> bool
where
  T: Copy + Ord + Inc,
{
  match (b1, b2) {
    (Unbounded, _) => false,
    (_, Unbounded) => true,
    (Included(b1), Included(b2)) => *b1 < *b2,
    (Included(b1), Excluded(b2)) => b1.inc() < *b2,
    (Excluded(b1), Included(b2)) => *b1 <= *b2,
    (Excluded(b1), Excluded(b2)) => *b1 < *b2,
  }
}


/// Extract the bounds from a range, cloning the inner values.
// TODO: This function should use `Bound::cloned` once it is stable.
pub(crate) fn bounds<R, T>(range: &R) -> (Bound<T>, Bound<T>)
where
  R: RangeBounds<T>,
  T: Copy,
{
  let start = range.start_bound();
  let end = range.end_bound();

  match (start, end) {
    (Included(start), Included(end)) => (Included(*start), Included(*end)),
    (Included(start), Excluded(end)) => (Included(*start), Excluded(*end)),
    (Included(start), Unbounded) => (Included(*start), Unbounded),
    (Excluded(start), Included(end)) => (Excluded(*start), Included(*end)),
    (Excluded(start), Excluded(end)) => (Excluded(*start), Excluded(*end)),
    (Excluded(start), Unbounded) => (Excluded(*start), Unbounded),
    (Unbounded, Included(end)) => (Unbounded, Included(*end)),
    (Unbounded, Excluded(end)) => (Unbounded, Excluded(*end)),
    (Unbounded, Unbounded) => (Unbounded, Unbounded),
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn start_less_than_start() {
    assert!(start_lt_start(&Included(0), &Included(2)));
    assert!(start_lt_start(&Included(1), &Included(2)));
    assert!(!start_lt_start(&Included(2), &Included(2)));

    assert!(start_lt_start(&Included(2), &Excluded(3)));
    assert!(start_lt_start(&Included(3), &Excluded(3)));
    assert!(!start_lt_start(&Included(4), &Excluded(3)));

    assert!(start_lt_start(&Included(4), &Unbounded));

    assert!(start_lt_start(&Excluded(1), &Included(4)));
    assert!(start_lt_start(&Excluded(2), &Included(4)));
    assert!(!start_lt_start(&Excluded(3), &Included(4)));

    assert!(start_lt_start(&Excluded(0), &Excluded(2)));
    assert!(start_lt_start(&Excluded(1), &Excluded(2)));
    assert!(!start_lt_start(&Excluded(2), &Excluded(2)));

    assert!(start_lt_start(&Excluded(2), &Unbounded));

    assert!(!start_lt_start(&Unbounded, &Included(2)));
    assert!(!start_lt_start(&Unbounded, &Excluded(2)));
    assert!(!start_lt_start::<u8>(&Unbounded, &Unbounded));
  }

  #[test]
  fn start_less_than_or_equal_start() {
    assert!(start_le_start(&Included(1), &Included(2)));
    assert!(start_le_start(&Included(2), &Included(2)));
    assert!(!start_le_start(&Included(3), &Included(2)));

    assert!(start_le_start(&Included(3), &Excluded(3)));
    assert!(start_le_start(&Included(4), &Excluded(3)));
    assert!(!start_le_start(&Included(5), &Excluded(3)));

    assert!(start_le_start(&Included(4), &Unbounded));

    assert!(start_le_start(&Excluded(2), &Included(4)));
    assert!(start_le_start(&Excluded(3), &Included(4)));
    assert!(!start_le_start(&Excluded(4), &Included(4)));

    assert!(start_le_start(&Excluded(1), &Excluded(2)));
    assert!(start_le_start(&Excluded(2), &Excluded(2)));
    assert!(!start_le_start(&Excluded(3), &Excluded(2)));

    assert!(start_le_start(&Excluded(2), &Unbounded));

    assert!(!start_le_start(&Unbounded, &Included(2)));
    assert!(!start_le_start(&Unbounded, &Excluded(2)));
    assert!(start_le_start::<u8>(&Unbounded, &Unbounded));
  }

  #[test]
  fn start_less_or_equal_end() {
    assert!(start_le_end(&Included(1), &Included(3)));
    assert!(start_le_end(&Included(3), &Included(3)));
    assert!(!start_le_end(&Included(3), &Included(2)));

    assert!(start_le_end(&Included(2), &Excluded(4)));
    assert!(start_le_end(&Included(3), &Excluded(4)));
    assert!(!start_le_end(&Included(3), &Excluded(3)));

    assert!(start_le_end(&Included(3), &Unbounded));

    assert!(start_le_end(&Excluded(3), &Included(5)));
    assert!(start_le_end(&Excluded(3), &Included(4)));
    assert!(!start_le_end(&Excluded(4), &Included(4)));

    assert!(start_le_end(&Excluded(2), &Excluded(5)));
    assert!(start_le_end(&Excluded(2), &Excluded(4)));
    assert!(!start_le_end(&Excluded(3), &Excluded(4)));

    assert!(start_le_end(&Excluded(3), &Unbounded));

    assert!(start_le_end(&Unbounded, &Included(5)));
    assert!(start_le_end(&Unbounded, &Excluded(5)));
    assert!(start_le_end::<u8>(&Unbounded, &Unbounded));
  }

  #[test]
  fn end_less_than_end() {
    assert!(end_lt_end(&Included(0), &Included(2)));
    assert!(end_lt_end(&Included(1), &Included(2)));
    assert!(!end_lt_end(&Included(2), &Included(2)));

    assert!(end_lt_end(&Included(0), &Excluded(3)));
    assert!(end_lt_end(&Included(1), &Excluded(3)));
    assert!(!end_lt_end(&Included(2), &Excluded(3)));

    assert!(end_lt_end(&Included(4), &Unbounded));

    assert!(end_lt_end(&Excluded(1), &Included(2)));
    assert!(end_lt_end(&Excluded(2), &Included(2)));
    assert!(!end_lt_end(&Excluded(3), &Included(2)));

    assert!(end_lt_end(&Excluded(0), &Excluded(2)));
    assert!(end_lt_end(&Excluded(1), &Excluded(2)));
    assert!(!end_lt_end(&Excluded(2), &Excluded(2)));

    assert!(end_lt_end(&Excluded(2), &Unbounded));

    assert!(!end_lt_end(&Unbounded, &Included(2)));
    assert!(!end_lt_end(&Unbounded, &Excluded(2)));
    assert!(!end_lt_end::<u8>(&Unbounded, &Unbounded));
  }

  #[test]
  fn extract_bounds() {
    assert_eq!(bounds(&(2..=5)), (Included(2), Included(5)));
    assert_eq!(bounds(&(1..4)), (Included(1), Excluded(4)));
    assert_eq!(bounds(&(42..)), (Included(42), Unbounded));

    assert_eq!(
      bounds(&(Excluded(2), Included(5))),
      (Excluded(2), Included(5))
    );
    assert_eq!(
      bounds(&(Excluded(1), Excluded(4))),
      (Excluded(1), Excluded(4))
    );
    assert_eq!(bounds(&(Excluded(8), Unbounded)), (Excluded(8), Unbounded));

    assert_eq!(bounds(&(..=5)), (Unbounded, Included(5)));
    assert_eq!(bounds(&(..4)), (Unbounded, Excluded(4)));
    assert_eq!(bounds::<_, u8>(&(..)), (Unbounded, Unbounded));
  }
}
