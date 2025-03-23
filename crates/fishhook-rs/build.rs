use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo::rerun-if-changed=fishhook/fishhook.c");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=fishhook");

    // Compile fishhook.c
    cc::Build::new()
        .file("fishhook/fishhook.c")
        .compile("fishhook");

    // Generate bindings so Rust code can call functions in fishhook.h
    bindgen::Builder::default()
        .header("fishhook/fishhook.h")
        .clang_arg("-DFISHHOOK_EXPORT")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("fishhook_bindings.rs")
        )
        .expect("Couldn't write bindings");
}
