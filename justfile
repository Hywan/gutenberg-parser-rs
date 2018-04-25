wasm_directory = "bindings/wasm"

# Build a regular build.
build-library:
	cargo +nightly build --release

# Test the parser only (i.e. not the bindings to external languages) and its documentation.
test-library:
	cargo +nightly test

# Build the documentation.
build-doc:
	cargo +nightly doc --release --package gutenberg_post_parser

# Build the parser and the WASM binding.
build-wasm:
	cargo +nightly build --release --features "wasm" --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/gutenberg_post_parser.wasm {{wasm_directory}}
	cd {{wasm_directory}} && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-opt -Oz -o gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		mv gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		gzip --best --stdout gutenberg_post_parser.wasm > gutenberg_post_parser.wasm.gz

# Pack the WASM binding and run an HTTP server to try it.
run-wasm: build-wasm
	cd {{wasm_directory}} && \
		npm install && \
		npm run serve

# Local Variables:
# mode: makefile
# End:
