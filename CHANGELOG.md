Unreleased
----------
- Switched to using GitHub Actions as CI provider


0.3.1
-----
- Implemented `Clone` and `Debug` for `GapIter`
- Removed overly strong (debug) assertion from `GapIter::new`
- Added categories and keywords to `Cargo.toml`


0.3.0
-----
- Moved `Copied` type from `RangeGappable::gaps` return type to
  `RangeGappable::Iter` associated type
- Fixed "implementation of `FnOnce` is not general enough" error seen in
  some client use-cases


0.2.3
-----
- Exported `GapIter::new` constructor


0.2.2
-----
- Made `GapIter` publicly accessible


0.2.1
-----
- Fixed compile time failure on release builds


0.2.0
-----
- Changed `GapIter` to assume non-reference types in wrapped iterator
- Bumped minimum required Rust version to `1.36.0`
- Enabled CI pipeline comprising building, testing, linting, and
  coverage collection of the project
  - Added badges indicating pipeline status and code coverage percentage


0.1.1
-----
- Introduced `RangeGappable` trait for collections with `range` method
- Exported `bounds` function for extracting the bounds of a range


0.1.0
-----
- Initial release
