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
                let expected_path = input_path.as_path().with_extension("json");
                let expected_content = fs::read_to_string(&expected_path).unwrap();

                let output =
                    Command::new("./bin/gutenberg-post-parser")
                    .arg("--emit-json")
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

                let mut output_content = String::from_utf8(output.stdout).unwrap();

                assert!(expected_content == output_content, format!("file = {:?}", input_path));
            }
        }
    }
}
