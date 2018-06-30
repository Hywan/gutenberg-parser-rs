use std::fs;
use std::env;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::process::Command;

#[test]
fn run_all_fixtures() {
    let mut fixtures_directory = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fixtures_directory.push("../../tests/fixtures");

    let html = OsStr::new("html");

    for entry in fs::read_dir(fixtures_directory).unwrap() {
        let entry = entry.unwrap();
        let input_path = entry.path();

        if let Some(extension) = input_path.extension() {
            if extension == html {
                let output =
                    Command::new("./bin/gutenberg-post-parser")
                    .arg(&input_path)
                    .output()
                    .expect("Failed to execute `gutenberg-post-parser`.");

                assert!(
                    output.status.success(),
                    format!(
                        "Failed to parse {:?}\nStatus: {}\nOutput: {}",
                        input_path,
                        output.status,
                        String::from_utf8_lossy(&output.stdout.as_slice())
                    )
                );
            }
        }
    }
}
