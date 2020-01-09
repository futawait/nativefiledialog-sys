use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .include("nativefiledialog/src/include")
        .files(&[
            "nativefiledialog/src/nfd_common.c",
            "nativefiledialog/src/nfd_cocoa.m",
        ])
        .compile("libnfd.a");
    println!("cargo:rustc-link-lib=framework=AppKit");
    // println!("cargo:rustc-link-search=out");
    println!("cargo:rustc-link-lib=static=nfd");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
