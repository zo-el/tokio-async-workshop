# 0.1.3 (September 12, 2019)

### Fixed

- Fixed `#[instrument]`ed async functions not compiling on `nightly-2019-09-11`
  or newer (#342)

# 0.1.2 (August 19, 2019)

### Changed

- Updated `syn` and `quote` dependencies to 1.0 (#292)
- Removed direct dependency on `proc-macro2` to avoid potential version
  conflicts (#296)

### Fixed

- Outdated idioms in examples (#271, #273)

# 0.1.1 (August 9, 2019)

### Changed

- Using the `#[instrument]` attribute on `async fn`s no longer requires a
  feature flag (#258)

### Fixed

- The `#[instrument]` macro now works on generic functions (#262)

# 0.1.0 (August 8, 2019)

- Initial release
