extern crate gutenberg_post_parser;

use gutenberg_post_parser::root;

use std::fs;
use std::path::Path;
use std::ffi::OsStr;

#[test]
fn compare_html_to_parser_output() {
    let fixtures_directory = Path::new("./tests/fixtures");
    let html = OsStr::new("html");

    for entry in fs::read_dir(fixtures_directory).unwrap() {
        let entry = entry.unwrap();
        let input_path = entry.path();

        if let Some(extension) = input_path.extension() {
            if extension == html {
                let html_content = fs::read_to_string(&input_path).unwrap();
                let output = format!("{:?}", root(html_content.as_bytes()));

                let expected_path = input_path.as_path().with_extension("output");
                let expected_content = fs::read_to_string(&expected_path).unwrap();

                assert_eq!(
                    output,
                    expected_content,
                    "The parser output for {:?} differs from what is expected: left represents the output of the parser, right represents the expected datum.",
                    input_path
                );
            }
        }
    }
}
