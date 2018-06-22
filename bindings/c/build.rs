extern crate cbindgen;

fn main() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate C bindings.")
        .write_to_file("bin/gutenberg_post_parser.h");
}
