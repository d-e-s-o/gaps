// Copyright (C) 2020-2022 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Functionality for working with gaps/holes in ordered sequences of
//! values.

#![allow(unknown_lints)]
#![warn(
  bad_style,
  broken_intra_doc_links,
  dead_code,
  future_incompatible,
  illegal_floating_point_literal_pattern,
  improper_ctypes,
  late_bound_lifetime_arguments,
  missing_copy_implementations,
  missing_debug_implementations,
  missing_docs,
  no_mangle_generic_items,
  non_shorthand_field_patterns,
  nonstandard_style,
  overflowing_literals,
  path_statements,
  patterns_in_fns_without_body,
  private_in_public,
  proc_macro_derive_resolution_fallback,
  renamed_and_removed_lints,
  rust_2018_compatibility,
  rust_2018_idioms,
  stable_features,
  trivial_bounds,
  trivial_numeric_casts,
  type_alias_bounds,
  tyvar_behind_raw_pointer,
  unaligned_references,
  unconditional_recursion,
  unreachable_code,
  unreachable_patterns,
  unreachable_pub,
  unstable_features,
  unstable_name_collisions,
  unused,
  unused_comparisons,
  unused_import_braces,
  unused_lifetimes,
  unused_qualifications,
  unused_results,
  where_clauses_object_safety,
  while_true
)]

mod bounds;
mod gaps;
mod inc;

pub use crate::gaps::GapIter;
pub use crate::gaps::Gappable;
pub use crate::gaps::RangeGappable;
pub use crate::inc::Inc;

/// A module providing utility functionality for working with ranges.
pub mod range {
  pub use crate::bounds::bounds;
}
