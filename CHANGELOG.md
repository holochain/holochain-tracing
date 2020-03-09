# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.23] - 2020-03-09

### Added

### Changed
newrelic is behind compile time flag.

### Deprecated

### Removed

### Fixed

### Security

## [0.0.22] - 2020-03-05

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.21] - 2020-03-05

### Added
##### follow
Easily follow on from a SpanWrap. Good for when a span crosses a thread boundary.
##### follow_encoded
Same as follow but for encoded. Good for following after crossing a process boundary.
##### follow_encoded_tag
As above but allows adding a tag. Good for including the data in the span.
##### wrap
Easily wrap up some data into a SpanWrap. This will always create a SpanWrap but use
null if there's no span on the stack.
This is good for when messages require this type.
##### wrap_with_tag
As above but allows adding a tag. Good for including the data in the span.
##### here!
This is a proc macro that prints a neat string of the location it is called as file:line_number.
More reliable and easier then `format!("{}:{}", file!(), line!())`.


### Changed
Cleaned up file / line prints

### Deprecated

### Removed

### Fixed

### Security

## [0.0.20] - 2020-02-13

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.19] - 2020-02-11

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.17] - 2020-02-10

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.16] - 2020-01-30

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.15] - 2020-01-30

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.15] - 2020-01-30

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.13] - 2020-01-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.12] - 2020-01-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.11] - 2020-01-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.10] - 2020-01-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.9] - 2020-01-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.8] - 2020-01-17

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security
