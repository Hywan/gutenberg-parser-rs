wasm_directory = "bindings/wasm"

test-library:
	cargo +nightly test

build-wasm:
	cargo +nightly build --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/parser.wasm --out-dir {{wasm_directory}}
	cd {{wasm_directory}} && \
		wasm-gc parser_bg.wasm && \
		wasm-opt -Oz -o parser_bg_opt.wasm parser_bg.wasm && \
		mv parser_bg_opt.wasm parser_bg.wasm && \
		npm install && \
		npm run serve

# Local Variables:
# mode: makefile
# End:
