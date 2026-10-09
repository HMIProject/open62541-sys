# open62541-sys

[![crates.io](https://img.shields.io/crates/v/open62541-sys.svg)](https://crates.io/crates/open62541-sys)
[![Docs](https://docs.rs/open62541-sys/badge.svg)](https://docs.rs/open62541-sys)
[![Dependencies](https://deps.rs/repo/github/HMIProject/open62541-sys/status.svg)](https://deps.rs/repo/github/HMIProject/open62541-sys)
[![Testing](https://github.com/HMIProject/open62541-sys/actions/workflows/test.yaml/badge.svg)](https://github.com/HMIProject/open62541-sys/actions/workflows/test.yaml)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)

This crate provides low-level, unsafe bindings for the C11 library
[open62541](https://www.open62541.org), an open source and free implementation of
[OPC UA](https://opcfoundation.org/about/opc-technologies/opc-ua/).

## Overview

Use this crate when you want to implement your own safe Rust abstractions over the unsafe C API
provided by open62541.

If you are looking for safe Rust bindings that can be used directly, see crate `open62541`
([crates.io](https://crates.io/crates/open62541),
[GitHub](https://github.com/HMIProject/open62541)).

## Development

Make sure to check out the repository `open62541-sys` with submodules:

```shell
git submodule update --init --recursive
```

### Address sanitizer

When the Rust code is built with AddressSanitizer enabled (nightly `-Zsanitizer=address`), the bundled C
libraries are instrumented as well (with `-fsanitize=address -fno-omit-frame-pointer`). No other
configuration is necessary:

```sh
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test \
--target "$(rustc +nightly -vV | sed -n 's/^host: //p')"
```

The sanitizer runtime is provided by the LLVM version bundled with Rust, so the C compiler must be
compatible with it. Otherwise, linking fails with an undefined symbol like this one (seen with the
default Apple Clang on macOS):

```text
Undefined symbols for architecture arm64:
"___asan_version_mismatch_check_apple_clang_2100", referenced from: ...
```

To resolve this, select a Clang matching the LLVM major version that `rustc +nightly -vV` reports
(line `LLVM version`) by setting `CC` (or `CC_<target>`, as supported by the
[`cc`](https://docs.rs/cc) crate). For example, with LLVM from Homebrew on macOS:

```sh
CC="$(brew --prefix llvm)/bin/clang" RUSTFLAGS="-Zsanitizer=address" cargo +nightly test \
--target "$(rustc +nightly -vV | sed -n 's/^host: //p')"
```

