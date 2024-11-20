# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Upgrade to build dependency bindgen version
  [0.70.1](https://github.com/rust-lang/rust-bindgen/releases/tag/v0.70.1).

## [0.4.5] - 2024-11-20

### Added

- Add feature flag `mbedtls` to build with encryption support by using bundled Mbed TLS version
  [3.6.2](https://github.com/Mbed-TLS/mbedtls/releases/tag/mbedtls-3.6.2).
- Include bindings for functions from header files `plugin/log.h`, `plugin/log_stdout.h`,
  `plugin/pki.h`, `plugin/pki_default.h`, `plugin/securitypolicy.h`.

## [0.4.4] - 2024-11-15

### Changed

- Upgrade to open62541 version [1.4.7](https://github.com/open62541/open62541/releases/tag/v1.4.7).

## [0.4.3] - 2024-10-14

### Changed

- Upgrade to open62541 version [1.4.6](https://github.com/open62541/open62541/releases/tag/v1.4.6).

## [0.4.2] - 2024-08-09

### Changed

- Upgrade to open62541 version [1.4.4](https://github.com/open62541/open62541/releases/tag/v1.4.4).

## [0.4.1] - 2024-07-29

### Changed

- Upgrade to open62541 version [1.4.3](https://github.com/open62541/open62541/releases/tag/v1.4.3).

## [0.4.0] - 2024-07-12

### Changed

- Breaking: Upgrade to open62541 version 1.4. While mostly compatible, this introduces some API
  changes. See open62541 [release notes](https://github.com/open62541/open62541/releases) for
  details.

## [0.4.0-pre.6] - 2024-07-04

### Changed

- Upgrade to open62541 version [1.4.2](https://github.com/open62541/open62541/releases/tag/v1.4.2).

## [0.4.0-pre.5] - 2024-05-30

### Changed

- Upgrade to open62541 version [1.4.1](https://github.com/open62541/open62541/releases/tag/v1.4.1).

## [0.4.0-pre.4] - 2024-04-29

### Changed

- Upgrade to open62541 version [1.4.0](https://github.com/open62541/open62541/releases/tag/v1.4.0).

## [0.4.0-pre.3] - 2024-04-04

### Changed

- Upgrade to latest
  [commit](https://github.com/open62541/open62541/commit/43ea708216e0460d9d50348a140a952cca34fe81)
  from open62541 version 1.4.

## [0.4.0-pre.2] - 2024-03-27

### Changed

- Add work-around to re-enable builds for [musl libc](https://www.musl-libc.org) environment.

## [0.4.0-pre.1] - 2024-03-26

### Changed

- Breaking: Upgrade to open62541 version 1.4. While mostly compatible, this introduces some API
  changes. See open62541 [release notes](https://github.com/open62541/open62541/releases) for
  details.

## [0.3.3] - 2024-02-12

### Fixed

- Avoid unnecessary rebuilds when only rebuilt files have changed.

## [0.3.2] - 2024-01-23

### Fixed

- Fix export of `UA_EMPTY_ARRAY_SENTINEL` constant.

## [0.3.1] - 2024-01-23

### Fixed

- Export `vsnprintf_va_copy()` and `vsnprintf_va_end()` as intended.

## [0.3.0] - 2024-01-23

### Changed

- Breaking: Rename `vsnprintf()` to `vsnprintf_va_copy()` to clarify implicit behavior.
- Breaking: Rename `va_end()` to `vsnprintf_va_end()`.

## [0.2.2] - 2024-01-22

### Changed

- Add binding for `va_end()`, adjust behavior of `vsnprintf()` to call `va_copy()` internally
  (HMIProject/open62541#30).

## [0.2.1] - 2024-01-19

### Fixed

- Add wrapper for `vsnprintf()` to support older C library versions (before UCRT in Visual Studio
  2015 and Windows 10).

## [0.2.0] - 2024-01-17

### Added

- Include binding for `vsnprintf()` from `stdio.h` to simplify formatting of log messages.

### Changed

- Breaking: Rename `UA_EMPTY_ARRAY_SENTINEL_` back to `UA_EMPTY_ARRAY_SENTINEL` without trailing
  underscore.

## [0.1.3] - 2024-01-12

### Added

- First public release.

[0.4.5]: https://github.com/HMIProject/open62541-sys/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/HMIProject/open62541-sys/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/HMIProject/open62541-sys/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/HMIProject/open62541-sys/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/HMIProject/open62541-sys/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/HMIProject/open62541-sys/compare/v0.3.3...v0.4.0
[0.4.0-pre.6]: https://github.com/HMIProject/open62541-sys/compare/v0.4.0-pre.5...v0.4.0-pre.6
[0.4.0-pre.5]: https://github.com/HMIProject/open62541-sys/compare/v0.4.0-pre.4...v0.4.0-pre.5
[0.4.0-pre.4]: https://github.com/HMIProject/open62541-sys/compare/v0.4.0-pre.3...v0.4.0-pre.4
[0.4.0-pre.3]: https://github.com/HMIProject/open62541-sys/compare/v0.4.0-pre.2...v0.4.0-pre.3
[0.4.0-pre.2]: https://github.com/HMIProject/open62541-sys/compare/v0.4.0-pre.1...v0.4.0-pre.2
[0.4.0-pre.1]: https://github.com/HMIProject/open62541-sys/compare/v0.3.3...v0.4.0-pre.1
[0.3.3]: https://github.com/HMIProject/open62541-sys/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/HMIProject/open62541-sys/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/HMIProject/open62541-sys/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/HMIProject/open62541-sys/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/HMIProject/open62541-sys/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/HMIProject/open62541-sys/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/HMIProject/open62541-sys/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/HMIProject/open62541-sys/releases/tag/v0.1.3
