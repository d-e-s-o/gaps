Unreleased
----------
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
