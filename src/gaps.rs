// Copyright (C) 2020-2022 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::btree_map::Range as BTreeMapRange;
use std::collections::btree_set::Range as BTreeSetRange;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::Copied;
use std::iter::Map;
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
#[derive(Clone, Debug)]
pub struct GapIter<I, T> {
  /// The iterator that we wrap.
  iter: Option<I>,
  /// The start of the remaining range we iterate. This start bound will
  /// change as the iterator produces new items, always just excluding
  /// the previously produced one.
  start: Bound<T>,
  /// The end of the range to iterate over.
  end: Bound<T>,
  #[cfg(debug_assertions)]
  last: Option<T>,
}

impl<I, T> GapIter<I, T>
where
  I: Iterator<Item = T>,
  T: Copy + Ord + Inc,
{
  /// Create a new `GapIter` wrapping the provided iterator and yielding
  /// ranges identifying the gaps between the elements, if any.
  ///
  /// # Notes
  /// - the provided iterator is assumed to yield elements in ascending
  ///   order
  pub fn new(iter: I, start: Bound<T>, end: Bound<T>) -> Self {
    Self {
      iter: Some(iter),
      start,
      end,
      #[cfg(debug_assertions)]
      last: None,
    }
  }
}

impl<I, T> Iterator for GapIter<I, T>
where
  I: Iterator<Item = T>,
  T: Copy + Ord + Inc,
{
  type Item = (Bound<T>, Bound<T>);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.iter.as_mut() {
        Some(iter) => {
          let (start, end) = if let Some(this) = iter.next() {
            #[cfg(debug_assertions)]
            {
              debug_assert!(
                self.last.unwrap_or(this) <= this,
                "sequence is not ascending"
              );
              self.last = Some(this);
            }

            let end = Excluded(this);
            if self.start != Unbounded && start_le_start(&Included(this), &self.start) {
              // As long as our current element is still less than or
              // even equal to the actual start of the range that we
              // consider, we just continue.
              if !start_lt_start(&Included(this), &self.start) {
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
/// `0..=6` would be: `[0..1, 2..3, 5..=6]`.
///
/// ```rust
/// use std::ops::Bound;
/// # use gaps::Gappable as _;
///
/// let vec = vec![1, 3, 4];
/// let mut gaps = vec.iter().copied().gaps(0..=6);
/// assert_eq!(gaps.next(), Some((Bound::Included(0), Bound::Excluded(1))));
/// assert_eq!(gaps.next(), Some((Bound::Excluded(1), Bound::Excluded(3))));
/// assert_eq!(gaps.next(), Some((Bound::Excluded(4), Bound::Included(6))));
/// assert_eq!(gaps.next(), None);
/// ```
pub trait Gappable<I, T> {
  /// Create a new [`GapIter`] that yields ranges identifying the gaps
  /// in a certain range of a collection.
  fn gaps<R>(self, range: R) -> GapIter<I, T>
  where
    R: RangeBounds<T>;
}

impl<I, T> Gappable<I, T> for I
where
  I: Iterator<Item = T>,
  T: Copy + Ord + Inc,
{
  fn gaps<R>(self, range: R) -> GapIter<I, T>
  where
    R: RangeBounds<T>,
  {
    let (start, end) = bounds(&range);
    GapIter::new(self, start, end)
  }
}


/// An extension trait that provides range based access to the "gaps" in
/// collections with a `range` method.
///
/// `BTreeSet` and `BTreeMap` are the two most prominent examples of
/// such collections.
///
/// ```rust
/// use std::ops::Bound;
/// # use maplit::btreeset;
/// # use gaps::RangeGappable as _;
///
/// let set = btreeset!{1, 3, 4};
/// let mut gaps = set.gaps(0..=6);
/// assert_eq!(gaps.next(), Some((Bound::Included(0), Bound::Excluded(1))));
/// assert_eq!(gaps.next(), Some((Bound::Excluded(1), Bound::Excluded(3))));
/// assert_eq!(gaps.next(), Some((Bound::Excluded(4), Bound::Included(6))));
/// assert_eq!(gaps.next(), None);
/// ```
pub trait RangeGappable<'s, T> {
  /// The type of the wrapped iterator.
  type Iter;

  /// Create a new [`GapIter`] that yields ranges identifying the gaps
  /// in a certain range of a collection.
  fn gaps<R>(&'s self, range: R) -> GapIter<Self::Iter, T>
  where
    R: RangeBounds<T>;
}

impl<'s, V> RangeGappable<'s, V> for BTreeSet<V>
where
  V: Copy + Ord + Inc + 's,
{
  type Iter = Copied<BTreeSetRange<'s, V>>;

  fn gaps<R>(&'s self, range: R) -> GapIter<Self::Iter, V>
  where
    R: RangeBounds<V>,
  {
    let (start, end) = bounds(&range);
    let range = self.range(range).copied();
    GapIter::new(range, start, end)
  }
}


impl<'s, K, V> RangeGappable<'s, K> for BTreeMap<K, V>
where
  K: Copy + Ord + Inc + 's,
  V: 's,
{
  #[allow(clippy::type_complexity)]
  type Iter = Map<BTreeMapRange<'s, K, V>, fn((&'_ K, &'_ V)) -> K>;

  fn gaps<R>(&'s self, range: R) -> GapIter<Self::Iter, K>
  where
    R: RangeBounds<K>,
  {
    fn map<I, J>(x: (&I, &J)) -> I
    where
      I: Copy,
    {
      *x.0
    }

    let (start, end) = bounds(&range);
    let range = self.range(range).map(map as _);
    GapIter::new(range, start, end)
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  #[cfg(debug_assertions)]
  #[should_panic(expected = "sequence is not ascending")]
  fn panic_when_non_ascending() {
    vec![1, 2, 1, 4, 5]
      .iter()
      .copied()
      .gaps(..)
      .for_each(|_| ());
  }
}
