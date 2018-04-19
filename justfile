wasm_directory = "bindings/wasm"

# Build a regular build.
build-library:
	cargo +nightly build --release

# Test the parser only (i.e. not the bindings to external languages).
test-library:
	cargo +nightly test

# Build the documentation.
build-doc:
	cargo +nightly doc --release --all-features

# Build the parser and the WASM binding.
build-wasm:
	cargo +nightly build --release --features "wasm" --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/parser.wasm {{wasm_directory}}
	cd {{wasm_directory}} && \
		wasm-gc parser.wasm && \
		wasm-opt -Oz -o parser_opt.wasm parser.wasm && \
		mv parser_opt.wasm parser.wasm && \
		gzip --best --stdout parser.wasm > parser.wasm.gz

# Pack the WASM binding and run an HTTP server to try it.
run-wasm: build-wasm
	cd {{wasm_directory}} && \
		npm install && \
		npm run serve

# Local Variables:
# mode: makefile
# End:
