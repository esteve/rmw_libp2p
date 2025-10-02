extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     Path::new(&crate_dir)
    //         .parent()
    //         .unwrap()
    //         .parent()
    //         .unwrap()
    //         .parent()
    //         .unwrap()
    //         .parent()
    //         .unwrap()
    //         .join("build")
    //         .join("rmw_libp2p_cpp")
    //         .display()
    // );
    // println!("cargo:rustc-link-lib=dylib=rmw_libp2p_cpp");
    // cbindgen::Builder::new()
    //   .with_crate(crate_dir)
    //   .generate()
    //   .expect("Unable to generate bindings")
    //   .write_to_file("../src/impl/rmw_libp2p_rs.h");
}
