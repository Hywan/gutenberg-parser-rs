#[cfg(feature = "c")] extern crate cbindgen;
#[cfg(feature = "nodejs")] extern crate neon_build;

#[cfg(feature = "c")]
fn main() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate C bindings.")
        .write_to_file("bindings/c/gutenberg_post_parser.h");
}

#[cfg(feature = "nodejs")]
fn main() {
    neon_build::setup();
}

#[cfg(all(not(feature = "c"), not(feature = "nodejs")))]
fn main() {
}
