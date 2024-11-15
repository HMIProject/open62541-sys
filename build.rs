#![allow(clippy::panic)] // Panic only during build time.

use std::{
    env,
    fs::File,
    io::{self, Write as _},
    path::{Path, PathBuf},
};

/// Target path in CMake build for include files.
const CMAKE_INCLUDE: &str = "include";
/// Target path in CMake build for lib files.
const CMAKE_LIB: &str = "lib";

/// Name of target library from `open62541` build. This must be `open62541` as it is being generated
/// by the CMake build.
const LIB_BASE: &str = "open62541";
/// Name of library from `extern.c` and `wrapper.c` that holds additional helpers, in particular the
/// compilation of static (inline) functions from `open62541` itself. This may be an arbitrary name;
/// the `cc` build adds it as `rustc-link-lib` automatically.
const LIB_EXT: &str = "open62541-ext";

/// Pattern to search for compatibility with Edition 2024.
///
/// See also [`LEGACY_EXTERN_REPLACEMENT`].
const LEGACY_EXTERN_PATTERN: &str = r#"extern "C" {"#;

/// Replacement to use for compatibility with Edition 2024.
///
/// See also [`LEGACY_EXTERN_PATTERN`].
const LEGACY_EXTERN_REPLACEMENT: &str = r#"unsafe extern "C" {"#;

fn main() {
    let with_mbedtls =
        matches!(env::var("CARGO_FEATURE_MBEDTLS"), Ok(mbedtls) if !mbedtls.is_empty());
    let with_openssl =
        matches!(env::var("CARGO_FEATURE_OPENSSL"), Ok(openssl) if !openssl.is_empty());
    // For now, we do not actually announce feature flag `openssl` in `Cargo.toml`.
    let encryption = match (with_mbedtls, with_openssl) {
        (false, false) => None,
        (true, false) => Some(Encryption::MbedTls),
        (false, true) => Some(Encryption::OpenSsl),
        _ => panic!("conflicting encryption feature flags, only one must be enabled"),
    };

    let src = env::current_dir().expect("should get current directory");

    // Get derived paths relative to `src`.
    let src_mbedtls = src.join("mbedtls");
    let src_open62541 = src.join("open62541");
    let src_wrapper_c = src.join("wrapper.c");
    let src_wrapper_h = src.join("wrapper.h");

    // Rerun build when files in `src` change.
    println!("cargo:rerun-if-changed={}", src_open62541.display());
    println!("cargo:rerun-if-changed={}", src_wrapper_c.display());
    println!("cargo:rerun-if-changed={}", src_wrapper_h.display());

    // Build related encryption libraries.
    let encryption_dst = encryption.map(|encryption| match encryption {
        Encryption::MbedTls => prepare_mbedtls(src_mbedtls),
        Encryption::OpenSsl => prepare_openssl(),
    });

    // Build `open62541` library.
    let dst = build_open62541(src_open62541, encryption_dst.as_ref());

    // Get derived paths relative to `dst`.
    let dst_include = dst.join(CMAKE_INCLUDE);
    let dst_lib = dst.join(CMAKE_LIB);

    if matches!(env::var("CARGO_CFG_TARGET_OS"), Ok(os) if os == "windows") {
        // We require the `Iphlpapi` library on Windows builds to avoid errors (regarding the use of
        // `if_nametoindex`, see https://github.com/open62541/open62541/issues/5622).
        println!("cargo:rustc-link-lib=Iphlpapi");
    }

    println!("cargo:rustc-link-search={}", dst_lib.display());
    println!("cargo:rustc-link-lib={LIB_BASE}");

    // For encryption support enabled, we add the libraries that have to be used as dependencies for
    // the final build artifact.
    //
    // Note: These must come _after_ adding `LIB_BASE` above for linker to resolve dependencies.
    if let Some(encryption_dst) = encryption_dst {
        encryption_dst.rustc_link_search();
        encryption_dst.rustc_link_lib();
    }

    let out = PathBuf::from(env::var("OUT_DIR").expect("should have OUT_DIR"));

    // Get derived paths relative to `out`.
    let out_bindings_rs = out.join("bindings.rs");
    let out_extern_c = out.join("extern.c");

    let builder = bindgen::Builder::default()
        // Include our wrapper functions.
        .allowlist_function("(__)?RS_.*")
        .allowlist_function("(__)?UA_.*")
        // Include our wrapper types.
        .allowlist_type("(__)?RS_.*")
        .allowlist_type("(__)?UA_.*")
        // Include our wrapper vars.
        .allowlist_var("(__)?RS_.*")
        .allowlist_var("(__)?UA_.*")
        // Explicitly set C99 standard to force Windows variants of `vsnprintf()` to conform to this
        // standard. This also matches the expected (or supported) C standard of `open62541` itself.
        .clang_arg("-std=c99")
        .clang_arg(format!("-I{}", dst_include.display()))
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        // Use explicit Rust target version that matches or is older than the entry in `Cargo.toml`.
        .rust_target(bindgen::RustTarget::Stable_1_71)
        // Do not derive `Copy` because most of the data types are not copy-safe (they own memory by
        // pointers and need to be cloned manually to duplicate that memory).
        .derive_copy(false)
        // We want to initialize some types statically. This is used in `open62541`, we require that
        // as well to mirror some of the functionality.
        .derive_default(true)
        // The auto-derived comments are not particularly useful because they often do not match the
        // declaration they belong to.
        .generate_comments(false)
        .header(src_wrapper_h.to_str().expect("should be valid path"))
        // Activate parse callbacks. This causes cargo to invalidate the generated bindings when any
        // of the included files change. It also enables us to rename items in the final bindings.
        .parse_callbacks(Box::new(CustomCallbacks { dst }))
        // We may use `core` instead of `std`. This might be useful for `no_std` environments.
        .use_core()
        // Wrap static functions. These are used in several places for inline helpers and we want to
        // preserve those in the generated bindings. This outputs `extern.c` which we compile below.
        .wrap_static_fns(true)
        // Make sure to specify the location of the resulting `extern.c`. By default `bindgen` would
        // place it in the temporary directory.
        .wrap_static_fns_path(out_extern_c.to_str().expect("should be valid path"));

    let bindings = builder
        .generate()
        .expect("should generate `Bindings` instance");

    bindings
        .write_to_file(out_bindings_rs.clone())
        .expect("should write `bindings.rs`");

    // Until <https://github.com/rust-lang/rust-bindgen/issues/2901> is resolved, we replace `extern
    // "C"` with `unsafe extern "C"` manually here. Remove this when `bindgen` is able to do it.
    if version_check::is_min_version("1.82.0") == Some(true) {
        // We can only use `unsafe extern` starting with Rust 1.82.0. See
        // <https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html#safe-items-with-unsafe-extern>.
        replace_in_file(
            &out_bindings_rs,
            LEGACY_EXTERN_PATTERN,
            LEGACY_EXTERN_REPLACEMENT,
        )
        .expect("should add unsafe to extern statements");
    }

    // Build `extern.c` and our custom `wrapper.c` that both hold additional helpers that we want to
    // link in addition to the base `open62541` library.
    cc::Build::new()
        .file(out_extern_c)
        .file(src_wrapper_c)
        .include(dst_include)
        // Disable warnings for `open62541`. Not much we can do anyway.
        .warnings(false)
        // Explicitly disable deprecation warnings (seem to be enabled even when other warnings have
        // been disabled above).
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-deprecated")
        .compile(LIB_EXT);
}

#[derive(Debug)]
enum Encryption {
    MbedTls,
    OpenSsl,
}

#[derive(Debug)]
enum EncryptionDst {
    MbedTls {
        dst: PathBuf,
        libs: Vec<&'static str>,
    },
    OpenSsl {
        search: Option<&'static str>,
        libs: Vec<&'static str>,
    },
}

impl EncryptionDst {
    const fn search(&self) -> Option<&'static str> {
        match self {
            EncryptionDst::MbedTls { .. } => None,
            EncryptionDst::OpenSsl { search, .. } => *search,
        }
    }

    fn libs(&self) -> &[&'static str] {
        match self {
            EncryptionDst::MbedTls { libs, .. } | EncryptionDst::OpenSsl { libs, .. } => libs,
        }
    }

    fn rustc_link_search(&self) {
        if let Some(search) = self.search() {
            println!("cargo:rustc-link-search={search}");
        }
    }

    fn rustc_link_lib(&self) {
        for lib in self.libs() {
            println!("cargo:rustc-link-lib={lib}");
        }
    }
}

fn prepare_mbedtls(src: PathBuf) -> EncryptionDst {
    // Build bundled copy of `mbedtls` with CMake.
    let mut cmake = cmake::Config::new(src);
    cmake
        // Use explicit paths here to avoid generating files where we do not expect them below.
        .define("CMAKE_INSTALL_INCLUDEDIR", CMAKE_INCLUDE)
        // Some systems (Fedora) default to `lib64/` instead of `lib/` for 64-bit libraries.
        .define("CMAKE_INSTALL_LIBDIR", CMAKE_LIB)
        // Use same C99 standard as is used for building `open62541`.
        .define("C_STANDARD", "99")
        // Skip building binary programs unnecessary for linking library.
        .define("ENABLE_PROGRAMS", "OFF")
        // Skip building test programs that we are not going to run anyway.
        .define("ENABLE_TESTING", "OFF");

    let dst = cmake.build();

    // The set of MbedTLS libraries that must be linked to work with `open62541` has been taken from
    // <https://github.com/open62541/open62541/blob/master/tools/cmake/FindMbedTLS.cmake>.
    let libs = vec!["mbedtls", "mbedx509", "mbedcrypto"];

    EncryptionDst::MbedTls { dst, libs }
}

fn prepare_openssl() -> EncryptionDst {
    // For macOS, we require the precise link path because we expect OpenSSL to be provided by using
    // Homebrew.
    let search = matches!(env::var("CARGO_CFG_TARGET_OS"), Ok(os) if os == "macos")
        .then_some("/opt/homebrew/opt/openssl/lib");

    let libs = vec!["ssl", "crypto"];

    EncryptionDst::OpenSsl { search, libs }
}

fn build_open62541(src: PathBuf, encryption: Option<&EncryptionDst>) -> PathBuf {
    // Build bundled copy of `open62541` with CMake.
    let mut cmake = cmake::Config::new(src);
    cmake
        // Use explicit paths here to avoid generating files where we do not expect them below.
        .define("CMAKE_INSTALL_INCLUDEDIR", CMAKE_INCLUDE)
        // Some systems (Fedora) default to `lib64/` instead of `lib/` for 64-bit libraries.
        .define("CMAKE_INSTALL_LIBDIR", CMAKE_LIB)
        // Explicitly set C99 standard to force Windows variants of `vsnprintf()` to conform to this
        // standard. This also matches the expected (or supported) C standard of `open62541` itself.
        .define("C_STANDARD", "99")
        // Python defaults to creating bytecode in `__pycache__` directories. During build, this may
        // happen when the tool `nodeset_compiler` is called. When we package a crate, builds should
        // never modify files outside of `OUT_DIR`, so we disable the cache to prevent this.
        .env("PYTHONDONTWRITEBYTECODE", "1");

    if matches!(env::var("CARGO_CFG_TARGET_ENV"), Ok(env) if env == "musl") {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("should have CARGO_CFG_TARGET_ARCH");
        // We require includes from the Linux headers which are not provided automatically when musl
        // is targeted (see https://github.com/open62541/open62541/issues/6360).
        // TODO: Remove this when `open62541` enables us to build without including Linux headers.
        cmake
            .cflag("-idirafter/usr/include")
            .cflag(format!("-idirafter/usr/include/{arch}-linux-gnu"));
    }

    // When enabled, we build `open62541` with encryption support. This changes the library and also
    // changes the resulting `bindings.rs`.
    let encryption = match encryption {
        None => "OFF",
        Some(EncryptionDst::MbedTls { dst, .. }) => {
            // Skip auto-detection and use explicit folders from `mbedtls` build.
            cmake
                .define("MBEDTLS_FOLDER_INCLUDE", dst.join(CMAKE_INCLUDE))
                .define("MBEDTLS_FOLDER_LIBRARY", dst.join(CMAKE_LIB));
            "MBEDTLS"
        }
        Some(EncryptionDst::OpenSsl { .. }) => "OPENSSL",
    };

    cmake.define("UA_ENABLE_ENCRYPTION", encryption);

    cmake.build()
}

#[derive(Debug)]
struct CustomCallbacks {
    /// Destination of CMake build of `open62541`.
    dst: PathBuf,
}

impl CustomCallbacks {
    /// Checks if `filename` is inside CMake destination.
    ///
    /// This may be used to ensure that we do not run a rebuild when files generated by CMake change
    /// (it is not necessary to include those files because we already watch the CMake _sources_ and
    /// trigger a rebuild when they change).
    fn inside_dst(&self, filename: &str) -> bool {
        Path::new(filename).starts_with(&self.dst)
    }
}

// Include `cargo:rerun-if` instructions just like `bindgen::CargoCallbacks` does. In addition, make
// necessary adjustments to the names of items for the final bindings.
impl bindgen::callbacks::ParseCallbacks for CustomCallbacks {
    fn header_file(&self, filename: &str) {
        // Make sure to rerun build when dependency outside of `dst/` changes.
        if !self.inside_dst(filename) {
            println!("cargo:rerun-if-changed={filename}");
        }
    }

    fn include_file(&self, filename: &str) {
        // Make sure to rerun build when dependency outside of `dst/` changes.
        if !self.inside_dst(filename) {
            println!("cargo:rerun-if-changed={filename}");
        }
    }

    fn read_env_var(&self, key: &str) {
        // Make sure to rerun build when environment variable changes.
        println!("cargo:rerun-if-env-changed={key}");
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        // Rename our wrapped custom exports to their intended names.
        original_item_name.strip_prefix("RS_").map(str::to_owned)
    }
}

/// Replaces all occurrences of pattern in file.
///
/// Note that this is not particularly efficient because it reads the entire file into memory before
/// writing it back. Care should be taken when operating on large files.
fn replace_in_file(path: &Path, pattern: &str, replacement: &str) -> io::Result<()> {
    let buf = io::read_to_string(File::open(path)?)?;

    let buf = buf.replace(pattern, replacement);

    File::create(path)?.write_all(buf.as_bytes())?;

    Ok(())
}
