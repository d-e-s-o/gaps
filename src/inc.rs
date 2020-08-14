// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add as _;


/// A trait representing the capability to increment a value.
pub trait Inc {
  /// Increment self and return the new value.
  fn inc(self) -> Self;
}

macro_rules! inc {
  ( $t:ty ) => {
    impl Inc for $t {
      fn inc(self) -> Self {
        self.add(1)
      }
    }
  };
}

inc!(u8);
inc!(i8);
inc!(u16);
inc!(i16);
inc!(u32);
inc!(i32);
inc!(u64);
inc!(i64);
inc!(u128);
inc!(i128);
inc!(usize);
inc!(isize);


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn increment() {
    fn inc<T>(x: T) -> T
    where
      T: Inc,
    {
      x.inc()
    }

    assert_eq!(inc(1u8), 2);
    assert_eq!(inc(-1i16), 0);
    assert_eq!(inc(129_012u32), 129_013);
    assert_eq!(inc(42usize), 43);
  }
}
