use std::{env, path::PathBuf};

fn main() {
    let dst = cmake::Config::new("open62541")
        // Use explicit paths here to avoid generating files where we do not expect them below.
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        // Some systems (Fedora) default to `lib64/` instead of `lib/` for 64-bit libraries.
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .build();

    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=open62541");

    let input = env::current_dir().unwrap().join("wrapper.h");

    println!("cargo:rerun-if-changed={}", input.display());

    let bindings = bindgen::Builder::default()
        .allowlist_function("(__)?UA_.*")
        .allowlist_type("(__)?UA_.*")
        .allowlist_var("(__)?UA_.*")
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
        .header(input.to_string_lossy())
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
    let statc_path = env::current_dir().unwrap().join("wrapper.c");
    let extc_path = env::temp_dir().join("bindgen").join("extern.c");

    cc::Build::new()
        .file(extc_path)
        .file(statc_path)
        .include(dst.join("include"))
        .include(input.parent().unwrap())
        // Disable warnings for `open62541`. Not much we can do anyway.
        .warnings(false)
        // Explicitly disable deprecation warnings (seem to be enabled even when other warnings have
        // been disabled above).
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-deprecated")
        .compile(ext_name);
}
