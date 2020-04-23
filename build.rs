extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=physfs-static");
    println!("cargo:rustc-link-lib=static=shell32");
    println!("cargo:rustc-link-lib=static=libvcruntime");
    println!("cargo:rerun-if-changed=physfs.h");

    let bindings = bindgen::Builder::default()
        .header("physfs.h")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
