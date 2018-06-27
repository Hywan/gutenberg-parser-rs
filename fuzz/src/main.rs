extern crate gutenberg_post_parser;
#[macro_use] extern crate afl;

fn main() {
    fuzz!(
        |data: &[u8]| {
            let _ = gutenberg_post_parser::root(data);
        }
    );
}
