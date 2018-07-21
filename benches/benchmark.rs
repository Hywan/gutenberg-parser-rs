extern crate gutenberg_post_parser as parser;
#[macro_use] extern crate criterion;

use criterion::Criterion;
use std::{fs, path::Path};


macro_rules! criterion_for (
    ($name:ident, $input_path:expr) => (
        fn $name(c: &mut Criterion) {
            let input_path = Path::new($input_path);
            let content = fs::read_to_string(&input_path).unwrap();

            c.bench_function(
                stringify!($name),
                move |b| {
                    b.iter(|| parser::root(content.as_bytes()).unwrap())
                }
            );
        }
    );
);

criterion_for!(autoclosing_block, "./tests/fixtures/autoclosing-block.html");
criterion_for!(early_adopting_the_future, "./tests/fixtures/early-adopting-the-future.html");
criterion_for!(gutenberg_demo, "./tests/fixtures/gutenberg-demo.html");
criterion_for!(moby_dick_parsed, "./tests/fixtures/moby-dick-parsed.html");
criterion_for!(pygmalian_raw_html, "./tests/fixtures/pygmalian-raw-html.html");
criterion_for!(redesigning_chrome_desktop, "./tests/fixtures/redesigning-chrome-desktop.html");
criterion_for!(shortcode_shortcomings, "./tests/fixtures/shortcode-shortcomings.html");
criterion_for!(web_at_maximum_fps, "./tests/fixtures/web-at-maximum-fps.html");

criterion_group!(
    benches,
    autoclosing_block,
    early_adopting_the_future,
    gutenberg_demo,
    moby_dick_parsed,
    pygmalian_raw_html,
    redesigning_chrome_desktop,
    shortcode_shortcomings,
    web_at_maximum_fps
);

criterion_main!(benches);
