use std::path::PathBuf;
use std::{env, process};

fn main() {
    let dst = cmake::build("open62541");

    println!("cargo:rustc-link-search={}/lib", dst.display());
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .wrap_static_fns(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let ext_name = "open62541-extern";
    let exto_path = out_path.join(format!("{ext_name}.o"));
    let extc_path = env::temp_dir().join("bindgen").join("extern.c");

    let clang_output = process::Command::new("cc")
        .arg("-c")
        .arg("-O")
        .arg(format!("-I{}", input.parent().unwrap().display()))
        .arg(format!("-I{}", dst.join("include").display()))
        .arg(format!("-o{}", exto_path.display()))
        .arg(extc_path)
        .output()
        .unwrap();

    if !clang_output.status.success() {
        panic!(
            "Could not compile object file:\n{}",
            String::from_utf8_lossy(&clang_output.stderr)
        );
    }

    #[cfg(not(target_os = "windows"))]
    let lib_output = process::Command::new("ar")
        .arg("rcs")
        .arg(out_path.join(format!("lib{ext_name}.a")))
        .arg(exto_path)
        .output()
        .unwrap();
    #[cfg(target_os = "windows")]
    let lib_output = process::Command::new("LIB")
        .arg(obj_path)
        .arg(format!(
            "/OUT:{}",
            out_path.join(format!("lib{ext_name}.lib")).display()
        ))
        .output()
        .unwrap();

    if !lib_output.status.success() {
        panic!(
            "Could not emit library file:\n{}",
            String::from_utf8_lossy(&lib_output.stderr)
        );
    }

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=static={}", ext_name);
}
