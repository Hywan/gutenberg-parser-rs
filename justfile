cwd = `pwd`
c_directory = "bindings/c"
nodejs_directory = "bindings/nodejs"
wasm_directory = "bindings/wasm"

# Build a regular library..
build-library:
	cargo build --no-default-features --release

# Build a regular binary.
build-binary:
	cargo build --no-default-features --features "bin" --release

# Build the parser and produce a WASM binary.
build-wasm:
	RUSTFLAGS='-g' cargo +nightly build --release --no-default-features --features "wasm" --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/gutenberg_post_parser.wasm {{wasm_directory}}
	cd {{wasm_directory}} && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code gutenberg_post_parser.wasm -o gutenberg_post_parser_snipped.wasm && \
		mv gutenberg_post_parser_snipped.wasm gutenberg_post_parser.wasm && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-opt -g -Oz -o gutenberg_post_parser.debug.wasm gutenberg_post_parser.wasm && \
		wasm-opt -Oz -o gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		mv gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		gzip --best --stdout gutenberg_post_parser.wasm > gutenberg_post_parser.wasm.gz

# Start an HTTP server to serve WASM demo.
start-wasm-server:
	cd {{wasm_directory}} && php -S localhost:8888 -t . server.php

# Build the parser and produce a C binary.
build-c:
	cargo build --no-default-features --features "c" --release
	cd {{c_directory}} && \
		clang \
			-Wall \
			-o gutenberg-post-parser \
			gutenberg_post_parser.c \
			-L {{cwd}}/target/release/ \
			-l gutenberg_post_parser \
			-l System \
			-l pthread \
			-l c \
			-l m

# Build the parser and produce a NodeJS native module.
build-nodejs:
	RUSTFLAGS='--cfg feature="nodejs"' neon build --debug --rust nightly --path {{nodejs_directory}}/

# Run all tests.
test: test-library-unit test-library-integration test-documentation

# Run the unit tests.
test-library-unit:
	cargo test --lib --no-default-features

# Run the integration tests.
test-library-integration:
	cargo test --test integration --no-default-features

# Run the documentation tests.
test-documentation:
	cargo test --doc --no-default-features

# Build the documentation.
build-doc:
	cargo doc --release --no-default-features --package gutenberg_post_parser

# Open the documentation.
open-doc: build-doc
	open target/doc/gutenberg_post_parser/index.html

# Build the readme automatically.
build-readme:
	cargo readme --input src/lib.rs --template README.tpl > README.md

# Local Variables:
# mode: makefile
# End:
