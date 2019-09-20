# 0.1.6 (September 12, 2019)

### Added

- Internal APIs to support performance optimizations (#326)

### Fixed

- Clarified wording in `field::display` documentation (#340)

# 0.1.6 (August 16, 2019)

### Added

- `std::error::Error` as a new primitive `Value` type (#277)
- `Event::new` and `Event::new_child_of` to manually construct `Event`s (#281)

# 0.1.4 (August 9, 2019)

### Added

- Support for `no-std` + `liballoc` (#256)

### Fixed

- Broken links in RustDoc (#259)

# 0.1.3 (August 8, 2019)

### Added

- `std::fmt::Display` implementation for `Level` (#194)
- `std::str::FromStr` implementation for `Level` (#195)

# 0.1.2 (July 10, 2019)

### Deprecated

- `Subscriber::drop_span` in favor of new `Subscriber::try_close` (#168)

### Added

- `Into<Option<&Id>>`, `Into<Option<Id>>`, and
  `Into<Option<&'static Metadata<'static>>>` impls for `span::Current` (#170)
- `Subscriber::try_close` method (#153)
- Improved documentation for `dispatcher` (#171)

# 0.1.1 (July 6, 2019)

### Added

- `Subscriber::current_span` API to return the current span (#148).
- `span::Current` type, representing the `Subscriber`'s view of the current
  span (#148).

### Fixed

- Typos and broken links in documentation (#123, #124, #128, #154)

# 0.1.0 (June 27, 2019)

- Initial release
