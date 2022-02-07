// Copyright (C) 2020-2022 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;

use gaps::range::bounds;
use gaps::Gappable as _;
use gaps::RangeGappable as _;

use maplit::btreemap;
use maplit::btreeset;


#[test]
fn set_gap_iteration_empty() {
  let r = BTreeSet::<usize>::new();
  assert_eq!(
    r.gaps(0..=0).collect::<Vec<_>>(),
    vec![(Included(0), Included(0))]
  );
  assert_eq!(
    r.gaps(0..=1).collect::<Vec<_>>(),
    vec![(Included(0), Included(1))]
  );
  assert_eq!(
    r.gaps(0..=2).collect::<Vec<_>>(),
    vec![(Included(0), Included(2))]
  );
  assert_eq!(
    r.gaps(1..=3).collect::<Vec<_>>(),
    vec![(Included(1), Included(3))]
  );
  assert_eq!(
    r.gaps(0..).collect::<Vec<_>>(),
    vec![(Included(0), Unbounded)]
  );
  assert_eq!(
    r.gaps(..0).collect::<Vec<_>>(),
    vec![(Unbounded, Excluded(0))]
  );
  assert_eq!(
    r.gaps(..=0).collect::<Vec<_>>(),
    vec![(Unbounded, Included(0))]
  );
  assert_eq!(r.gaps(..).collect::<Vec<_>>(), vec![(Unbounded, Unbounded)]);
}

#[test]
fn set_gap_iteration_1() {
  let mut r = BTreeSet::<usize>::new();
  r.extend(btreeset! {
    2,
  });

  assert_eq!(
    r.gaps(0..=0).collect::<Vec<_>>(),
    vec![(Included(0), Included(0))]
  );
  assert_eq!(
    r.gaps(0..=1).collect::<Vec<_>>(),
    vec![(Included(0), Included(1))]
  );
  assert_eq!(
    r.gaps(0..=2).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(2))]
  );
  assert_eq!(
    r.gaps(1..=2).collect::<Vec<_>>(),
    vec![(Included(1), Excluded(2))]
  );
  assert_eq!(
    r.gaps(0..=3).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(2)), (Excluded(2), Included(3))]
  );
  assert_eq!(
    r.gaps(0..).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(2)), (Excluded(2), Unbounded)]
  );
  assert_eq!(
    r.gaps(..9).collect::<Vec<_>>(),
    vec![(Unbounded, Excluded(2)), (Excluded(2), Excluded(9))]
  );
  assert_eq!(
    r.gaps(..).collect::<Vec<_>>(),
    vec![(Unbounded, Excluded(2)), (Excluded(2), Unbounded)]
  );
}

#[test]
fn set_gap_iteration_2() {
  let mut r = BTreeSet::<usize>::new();
  r.extend(btreeset! {
    2,
    1,
  });

  assert_eq!(
    r.gaps(0..=0).collect::<Vec<_>>(),
    vec![(Included(0), Included(0))]
  );
  assert_eq!(
    r.gaps(0..=1).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1))]
  );
  assert_eq!(
    r.gaps(0..=2).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1))]
  );
  assert_eq!(r.gaps(1..=2).collect::<Vec<_>>(), vec![]);
  assert_eq!(
    r.gaps(0..=3).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1)), (Excluded(2), Included(3))]
  );
  assert_eq!(
    r.gaps(0..=6).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1)), (Excluded(2), Included(6))]
  );
  assert_eq!(
    r.gaps(0..).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1)), (Excluded(2), Unbounded)]
  );
  assert_eq!(
    r.gaps(..9).collect::<Vec<_>>(),
    vec![(Unbounded, Excluded(1)), (Excluded(2), Excluded(9))]
  );
  assert_eq!(
    r.gaps(..).collect::<Vec<_>>(),
    vec![(Unbounded, Excluded(1)), (Excluded(2), Unbounded)]
  );
}

#[test]
fn set_gap_iteration_3() {
  let mut r = BTreeSet::<usize>::new();
  r.extend(btreeset! {
    2,
    1,
    4
  });

  assert_eq!(
    r.gaps(0..=0).collect::<Vec<_>>(),
    vec![(Included(0), Included(0))]
  );
  assert_eq!(
    r.gaps(0..=1).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1))]
  );
  assert_eq!(
    r.gaps(0..=2).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1))]
  );
  assert_eq!(r.gaps(1..=2).collect::<Vec<_>>(), vec![]);
  assert_eq!(
    r.gaps(0..=3).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1)), (Excluded(2), Included(3))]
  );
  assert_eq!(
    r.gaps(0..=4).collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1)), (Excluded(2), Excluded(4))]
  );
  assert_eq!(
    r.gaps(0..=5).collect::<Vec<_>>(),
    vec![
      (Included(0), Excluded(1)),
      (Excluded(2), Excluded(4)),
      (Excluded(4), Included(5))
    ]
  );
  assert_eq!(
    r.gaps(0..=6).collect::<Vec<_>>(),
    vec![
      (Included(0), Excluded(1)),
      (Excluded(2), Excluded(4)),
      (Excluded(4), Included(6))
    ]
  );

  assert_eq!(
    r.gaps(0..).collect::<Vec<_>>(),
    vec![
      (Included(0), Excluded(1)),
      (Excluded(2), Excluded(4)),
      (Excluded(4), Unbounded)
    ]
  );
  assert_eq!(
    r.gaps(..9).collect::<Vec<_>>(),
    vec![
      (Unbounded, Excluded(1)),
      (Excluded(2), Excluded(4)),
      (Excluded(4), Excluded(9))
    ]
  );
  assert_eq!(
    r.gaps(..).collect::<Vec<_>>(),
    vec![
      (Unbounded, Excluded(1)),
      (Excluded(2), Excluded(4)),
      (Excluded(4), Unbounded)
    ]
  );
}

#[test]
fn iterator_gap_iteration() {
  let mut r = BTreeMap::<usize, &str>::new();
  assert_eq!(
    r.iter()
      .map(|(x, _)| x)
      .copied()
      .gaps(0..=0)
      .collect::<Vec<_>>(),
    vec![(Included(0), Included(0))]
  );

  r.extend(btreemap! { 1 => "foo", 99 => "bar" });
  assert_eq!(
    r.iter()
      .map(|(x, _)| x)
      .copied()
      .gaps(0..2)
      .collect::<Vec<_>>(),
    vec![(Included(0), Excluded(1))]
  );
  assert_eq!(
    r.iter()
      .map(|(x, _)| x)
      .copied()
      .gaps(0..)
      .collect::<Vec<_>>(),
    vec![
      (Included(0), Excluded(1)),
      (Excluded(1), Excluded(99)),
      (Excluded(99), Unbounded),
    ]
  );
}

/// Check that we can clone a [`GapIter`] object and iteration still
/// works as expected.
#[test]
fn gap_iterator_cloning() {
  let set = btreeset! {
    1usize,
    2,
    4,
    7,
  };

  let mut it1 = set.gaps(1..=7);
  assert_eq!(it1.next().unwrap(), (Excluded(2), Excluded(4)));

  let mut it2 = it1.clone();
  assert_eq!(it1.next().unwrap(), (Excluded(4), Excluded(7)));
  assert_eq!(it2.next().unwrap(), (Excluded(4), Excluded(7)));

  assert_eq!(it1.next(), None);
  assert_eq!(it2.next(), None);
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
