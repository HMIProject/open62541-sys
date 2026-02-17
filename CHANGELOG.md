# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.2] - 2026-02-17

### Changed

- Upgrade to open62541 version
  [1.4.15](https://github.com/open62541/open62541/releases/tag/v1.4.15).

## [0.5.1] - 2025-10-21

### Changed

- Upgrade to open62541 version
  [1.4.14](https://github.com/open62541/open62541/releases/tag/v1.4.14).
- Upgrade to Mbed TLS version [3.6.5](https://github.com/Mbed-TLS/mbedtls/releases/tag/v3.6.5).

## [0.5.0] - 2025-10-13

### Changed

- Breaking: Bump Minimum Supported Rust Version (MSRV) to 1.85 (Edition 2024).

### Fixed

- Disable LTO on `x86_64-unknown-linux-gnu` to fix unresolved symbols when linking with `lld`
  (default in Rust 1.90).

## [0.4.18] - 2025-08-18

### Changed

- Upgrade to open62541 version
  [1.4.13](https://github.com/open62541/open62541/releases/tag/v1.4.13).

## [0.4.17] - 2025-07-15

### Changed

- Upgrade to Mbed TLS version [3.6.4](https://github.com/Mbed-TLS/mbedtls/releases/tag/v3.6.4).

### Fixed

- Fix cross-compilation with [`cross`](https://github.com/cross-rs/cross) and
  [`MinGW`](https://mingw.osdn.io).

## [0.4.16] - 2025-05-30

### Changed

- Upgrade to open62541 version
  [1.4.12](https://github.com/open62541/open62541/releases/tag/v1.4.12).
- Upgrade to Mbed TLS version [3.6.3.1](https://github.com/Mbed-TLS/mbedtls/releases/tag/v3.6.3.1).

## [0.4.15] - 2025-03-29

### Changed

- Upgrade to Mbed TLS version
  [3.6.3](https://github.com/Mbed-TLS/mbedtls/releases/tag/mbedtls-3.6.3).

## [0.4.14] - 2025-03-27

### Fixed

- Fix builds on GCC 15 and Clang 20 when feature flag `mbedtls` is enabled.

## [0.4.13] - 2025-03-19

### Changed

- Upgrade to open62541 version
  [1.4.11](https://github.com/open62541/open62541/releases/tag/v1.4.11.1).

## [0.4.12] - 2025-03-19 [YANKED]

## [0.4.11] - 2025-02-06

### Changed

- Upgrade to open62541 version
  [1.4.10](https://github.com/open62541/open62541/releases/tag/v1.4.10).

## [0.4.10] - 2025-01-24

### Changed

- Upgrade to open62541 version [1.4.9](https://github.com/open62541/open62541/releases/tag/v1.4.9).
  This includes <https://github.com/open62541/open62541/pull/7042> that fixes builds on macOS and
  musl libc targets.

## [0.4.9] - 2024-12-13

### Fixed

- Fix Mbed TLS build on certain macOS versions.

## [0.4.8] - 2024-11-26

### Added

- Include bindings for functions from header files `plugin/create_certificate.h` (when feature flag
  `mbedtls` is enabled).

## [0.4.7] - 2024-11-25

### Changed

- Upgrade to open62541 version [1.4.8](https://github.com/open62541/open62541/releases/tag/v1.4.8).

## [0.4.6] - 2024-11-21

### Added

- Include bindings for functions from header files `plugin/accesscontrol.h`,
  `plugin/accesscontrol_default.h`.

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

[Unreleased]: https://github.com/HMIProject/open62541-sys/compare/v0.5.2...HEAD
[0.5.2]: https://github.com/HMIProject/open62541-sys/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/HMIProject/open62541-sys/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/HMIProject/open62541-sys/compare/v0.4.18...v0.5.0
[0.4.18]: https://github.com/HMIProject/open62541-sys/compare/v0.4.17...v0.4.18
[0.4.17]: https://github.com/HMIProject/open62541-sys/compare/v0.4.16...v0.4.17
[0.4.16]: https://github.com/HMIProject/open62541-sys/compare/v0.4.15...v0.4.16
[0.4.15]: https://github.com/HMIProject/open62541-sys/compare/v0.4.14...v0.4.15
[0.4.14]: https://github.com/HMIProject/open62541-sys/compare/v0.4.13...v0.4.14
[0.4.13]: https://github.com/HMIProject/open62541-sys/compare/v0.4.12...v0.4.13
[0.4.12]: https://github.com/HMIProject/open62541-sys/compare/v0.4.11...v0.4.12
[0.4.11]: https://github.com/HMIProject/open62541-sys/compare/v0.4.10...v0.4.11
[0.4.10]: https://github.com/HMIProject/open62541-sys/compare/v0.4.9...v0.4.10
[0.4.9]: https://github.com/HMIProject/open62541-sys/compare/v0.4.8...v0.4.9
[0.4.8]: https://github.com/HMIProject/open62541-sys/compare/v0.4.7...v0.4.8
[0.4.7]: https://github.com/HMIProject/open62541-sys/compare/v0.4.6...v0.4.7
[0.4.6]: https://github.com/HMIProject/open62541-sys/compare/v0.4.5...v0.4.6
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
