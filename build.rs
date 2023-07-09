use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("open62541");

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=open62541");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .allowlist_function("(__)?UA_.*")
        .allowlist_type("(__)?UA_.*")
        .allowlist_var("(__)?UA_.*")
        .clang_arg(format!("-I{}/include", dst.display()))
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
