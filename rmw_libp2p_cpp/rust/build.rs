extern crate cbindgen;

use std::env;

fn main() {
    let _crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // cbindgen::Builder::new()
    //   .with_crate(crate_dir)
    //   .generate()
    //   .expect("Unable to generate bindings")
    //   .write_to_file("../src/impl/rmw_libp2p_rs.h");
}
