// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Bound;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;
use std::ops::RangeBounds;

use crate::bounds::bounds;
use crate::bounds::end_lt_end;
use crate::bounds::start_le_end;
use crate::bounds::start_le_start;
use crate::bounds::start_lt_start;
use crate::Inc;


/// An iterator over the gaps in a sequence represented by an iterator.
pub struct GapIter<I, T> {
  /// The iterator that we wrap.
  iter: Option<I>,
  /// The start of the remaining range we iterate. This start bound will
  /// change as the iterator produces new items, always just excluding
  /// the previously produced one.
  start: Bound<T>,
  /// The end of the range to iterate over.
  end: Bound<T>,
}

impl<'i, I, T> GapIter<I, T>
where
  I: Iterator<Item = &'i T>,
  T: Copy + Ord + Inc + 'i,
{
  fn new(iter: I, start: Bound<T>, end: Bound<T>) -> Self {
    debug_assert!(start_le_end(&start, &end));

    Self {
      iter: Some(iter),
      start,
      end,
    }
  }
}

impl<'i, I, T> Iterator for GapIter<I, T>
where
  I: Iterator<Item = &'i T>,
  T: Copy + Ord + Inc + 'i,
{
  type Item = (Bound<T>, Bound<T>);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.iter.as_mut() {
        Some(iter) => {
          let (start, end) = if let Some(this) = iter.next() {
            let end = Excluded(*this);
            if self.start != Unbounded && start_le_start(&Included(*this), &self.start) {
              // As long as our current element is still less than or
              // even equal to the actual start of the range that we
              // consider, we just continue.
              if !start_lt_start(&Included(*this), &self.start) {
                // But if it is equal to the start bound then we adjust
                // the start bound to exclude this element.
                self.start = end;
              }
              continue
            }

            let start = self.start;
            self.start = end;

            if !end_lt_end(&end, &self.end) {
              // Once we see an element being produced that is at or
              // past our overarching range's end, we are done.
              self.iter = None;
              (start, self.end)
            } else {
              if !start_le_end(&self.start, &self.end) {
                // If our start has caught up with our end, we are done.
                self.iter = None;
              }
              (start, end)
            }
          } else {
            // The iterator is out of items and we are done.
            self.iter = None;
            (self.start, self.end)
          };

          // We could still end up with a range that is empty (or even
          // descending). Don't report those.
          if start_le_end(&start, &end) {
            break Some((start, end))
          }
        },
        None => break None,
      }
    }
  }
}


/// An extension trait that provides range based access to the "gaps"
/// between ordered elements yielded by an iterator.
///
/// E.g., given an ordered set {1, 3, 4}, the gaps in the range
/// `0..=6` would be: `[0..1, 2..3, 5..6]`.
///
/// ```rust
/// use std::ops::Bound;
/// # use gaps::Gappable as _;
///
/// let vec = vec![1, 3, 4];
/// let mut gaps = vec.iter().gaps(0..=6);
/// assert_eq!(gaps.next(), Some((Bound::Included(0), Bound::Excluded(1))));
/// assert_eq!(gaps.next(), Some((Bound::Excluded(1), Bound::Excluded(3))));
/// assert_eq!(gaps.next(), Some((Bound::Excluded(4), Bound::Included(6))));
/// assert_eq!(gaps.next(), None);
/// ```
pub trait Gappable<'s, I, T> {
  fn gaps<R>(self, range: R) -> GapIter<I, T>
  where
    R: RangeBounds<T>;
}

impl<'s, I, T> Gappable<'s, I, T> for I
where
  I: Iterator<Item = &'s T>,
  T: Copy + Ord + Inc + 's,
{
  fn gaps<R>(self, range: R) -> GapIter<I, T>
  where
    R: RangeBounds<T>,
  {
    let (start, end) = bounds(&range);
    GapIter::new(self, start, end)
  }
}
