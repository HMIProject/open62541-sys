use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let src = env::current_dir().unwrap();

    // Rebuild when any of the files in the source directory changes.
    println!("cargo:rerun-if-changed={}", src.join("open62541").display());
    println!("cargo:rerun-if-changed={}", src.join("wrapper.c").display());
    println!("cargo:rerun-if-changed={}", src.join("wrapper.h").display());

    // Build bundled copy of `open62541` with CMake.
    let dst = cmake::Config::new(src.join("open62541"))
        // Use explicit paths here to avoid generating files where we do not expect them below.
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        // Some systems (Fedora) default to `lib64/` instead of `lib/` for 64-bit libraries.
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        // Explicitly set C99 standard to force Windows variants of `vsnprintf()` to conform to this
        // standard. This also matches the expected (or supported) C standard of `open62541` itself.
        .define("C_STANDARD", "99")
        // Python defaults to creating bytecode in `__pycache__` directories. During build, this may
        // happen when the tool `nodeset_compiler` is called. When we package a crate, builds should
        // never modify files outside of `OUT_DIR`, so we disable the cache to prevent this.
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .build();

    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=open62541");

    let bindings = bindgen::Builder::default()
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
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        // Use explicit Rust target version that matches the entry in `Cargo.toml`.
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
        .header(src.join("wrapper.h").to_str().unwrap())
        // Activate parse callbacks. This causes cargo to invalidate the generated bindings when any
        // of the included files change. It also enables us to rename items in the final bindings.
        .parse_callbacks(Box::new(CustomCallbacks { dst: dst.clone() }))
        // We may use `core` instead of `std`. This might be useful for `no_std` environments.
        .use_core()
        // Wrap static functions. These are used in several places for inline helpers and we want to
        // preserve those in the generated bindings. This outputs `extern.c` which we compile below.
        .wrap_static_fns(true)
        .generate()
        .expect("should generate `Bindings` instance");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("should write `bindings.rs`");

    let ext_name = "open62541-extern";
    let statc_path = src.join("wrapper.c");
    let extc_path = env::temp_dir().join("bindgen").join("extern.c");

    cc::Build::new()
        .file(extc_path)
        .file(statc_path)
        .include(dst.join("include"))
        // Disable warnings for `open62541`. Not much we can do anyway.
        .warnings(false)
        // Explicitly disable deprecation warnings (seem to be enabled even when other warnings have
        // been disabled above).
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-deprecated")
        .compile(ext_name);
}

#[derive(Debug)]
struct CustomCallbacks {
    /// Destination of CMake build of `open62541`.
    dst: PathBuf,
}

// Include `cargo:rerun-if` instructions just like `bindgen::CargoCallbacks` does. In addition, make
// necessary adjustments to the names of items for the final bindings.
impl bindgen::callbacks::ParseCallbacks for CustomCallbacks {
    fn header_file(&self, filename: &str) {
        if !Path::new(filename).starts_with(&self.dst) {
            // Make sure to rerun build when dependency changes (but do not force rebuild when files
            // generated by CMake change, it's not necessary because we already watch the sources).
            println!("cargo:rerun-if-changed={}", filename);
        }
    }

    fn include_file(&self, filename: &str) {
        if !Path::new(filename).starts_with(&self.dst) {
            // Make sure to rerun build when dependency changes (but do not force rebuild when files
            // generated by CMake change, it's not necessary because we already watch the sources).
            println!("cargo:rerun-if-changed={}", filename);
        }
    }

    fn read_env_var(&self, key: &str) {
        // Make sure to rerun build when environment variable changes.
        println!("cargo:rerun-if-env-changed={}", key);
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        // Rename our wrapped custom exports to their intended names.
        original_item_name.strip_prefix("RS_").map(str::to_owned)
    }
}
