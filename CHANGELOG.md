# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2024-01-22

[0.2.2]: https://github.com/HMIProject/open62541-sys/compare/v0.2.1...v0.2.2

### Changed

- Add binding for `va_end()`, adjust behavior of `vsnprintf()` to call `va_copy()` internally
  (HMIProject/open62541#30).

## [0.2.1] - 2024-01-19

[0.2.1]: https://github.com/HMIProject/open62541-sys/compare/v0.2.0...v0.2.1

### Fixed

- Add wrapper for `vsnprintf()` to support older C library versions (before UCRT in Visual Studio
  2015 and Windows 10).

## [0.2.0] - 2024-01-17

[0.2.0]: https://github.com/HMIProject/open62541-sys/compare/v0.1.3...v0.2.0

### Added

- Include binding for `vsnprintf()` from `stdio.h` to simplify formatting of log messages.

### Changed

- Breaking: Rename `UA_EMPTY_ARRAY_SENTINEL_` back to `UA_EMPTY_ARRAY_SENTINEL` without trailing
  underscore.

## [0.1.3] - 2024-01-12

[0.1.3]: https://github.com/HMIProject/open62541-sys/releases/tag/v0.1.3

### Added

- First public release.
