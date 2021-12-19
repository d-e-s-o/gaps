// Copyright (C) 2020-2021 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

mod bounds;
mod gaps;
mod inc;

pub use crate::gaps::GapIter;
pub use crate::gaps::Gappable;
pub use crate::gaps::RangeGappable;
pub use crate::inc::Inc;

pub mod range {
  pub use crate::bounds::bounds;
}
