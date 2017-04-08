extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let libdrm = pkg_config::Config::new()
        .atleast_version("2.4.77")
        .probe("libdrm")
        .expect("Unable to find libdrm");

    // Transform include_paths into arguments to the compiler
    let include_args = libdrm.include_paths.iter()
        .map(|path| path.to_str().expect("Failed to convert path to string"))
        .map(|path| format!("-I{}", path));

    // Configure and generate bindings.
    let bindings = bindgen::Builder::default()
        .no_unstable_rust();

    let bindings = include_args
        .fold(bindings, |bindings, arg| bindings.clang_arg(arg))
        .header("src/wrapper.h")
        .link("drm")
        .generate()
        .expect("Unable to generate bindings");

    // Write the generated bindings to an output file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
