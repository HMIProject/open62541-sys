use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("open62541");

    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=open62541");

    let input = env::current_dir().unwrap().join("wrapper.h");

    println!("cargo:rerun-if-changed={}", input.display());

    let bindings = bindgen::Builder::default()
        .allowlist_function("(__)?UA_.*")
        .allowlist_type("(__)?UA_.*")
        .allowlist_var("(__)?UA_.*")
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .generate_comments(false)
        .header(input.to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

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
        // Explicitly disable warning (seems to be enabled by default).
        .flag("-Wno-deprecated")
        .compile(ext_name);
}
