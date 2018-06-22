cwd = `pwd`
cargo_no_std = "cargo/no_std/Cargo.toml"
cargo_std = "cargo/std/Cargo.toml"
asmjs_directory = "bindings/asmjs"
c_directory = "bindings/c"
nodejs_directory = "bindings/nodejs"
php_directory = "bindings/php"
wasm_directory = "bindings/wasm"

# Build a regular library..
build-library:
	cargo build --manifest-path {{cargo_std}} --no-default-features --release

# Build a regular binary.
build-binary:
	cargo build --manifest-path {{cargo_std}} --no-default-features --features "bin" --release

# Build the parser and produce a WASM binary.
build-wasm:
	cd {{wasm_directory}} && RUSTFLAGS='-g' cargo +nightly build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/gutenberg_post_parser_wasm.wasm {{wasm_directory}}/bin/gutenberg_post_parser.wasm
	cd {{wasm_directory}}/bin && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code gutenberg_post_parser.wasm -o gutenberg_post_parser_snipped.wasm && \
		mv gutenberg_post_parser_snipped.wasm gutenberg_post_parser.wasm && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-opt -g -Oz -o gutenberg_post_parser.debug.wasm gutenberg_post_parser.wasm && \
		wasm-opt -Oz -o gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		mv gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		gzip --best --stdout gutenberg_post_parser.wasm > gutenberg_post_parser.wasm.gz && \
		brotli --best --stdout --lgwin=24 gutenberg_post_parser.wasm > gutenberg_post_parser.wasm.br

# Start an HTTP server to serve WASM demo.
start-wasm-server:
	cd {{wasm_directory}}/bin && php -S localhost:8888 -t . server.php

# Build the parser and produce an ASM.js file.
build-asmjs: build-wasm
	wasm2es6js --wasm2asm --output {{asmjs_directory}}/gutenberg_post_parser.asm.js {{wasm_directory}}/bin/gutenberg_post_parser.wasm
	cd {{asmjs_directory}} && \
		sed -i '' '1s/^/function GUTENBERG_POST_PARSER_ASM_MODULE() {/; s/export //; s/const /var /; s/let /var /' gutenberg_post_parser.asm.js && \
		echo 'return { root: root, alloc: alloc, dealloc: dealloc, memory: memory }; }' >> gutenberg_post_parser.asm.js && \
		uglifyjs --mangle --output .temp.asm.js gutenberg_post_parser.asm.js && \
		mv .temp.asm.js gutenberg_post_parser.asm.js && \
		gzip --best --stdout gutenberg_post_parser.asm.js > gutenberg_post_parser.asm.js.gz && \
		brotli --best --stdout --lgwin=24 gutenberg_post_parser.asm.js > gutenberg_post_parser.asm.js.br

# Build the parser and produce a C binary.
build-c:
	cd {{c_directory}} && cargo build --release
	cd {{c_directory}}/bin && \
		clang \
			-Wall \
			-o gutenberg-post-parser \
			gutenberg_post_parser.c \
			-L {{cwd}}/target/release/ \
			-l gutenberg_post_parser_c \
			-l System \
			-l pthread \
			-l c \
			-l m

# Build the parser and produce a NodeJS native module.
build-nodejs:
	cd {{nodejs_directory}} && neon build

# Build the parser and produce a PHP extension.
build-php:
	cd {{c_directory}} && cargo build --release
	cd {{php_directory}}/extension/gutenberg_post_parser/ && \
		phpize && \
		./configure && \
		sudo make install

# Run all tests.
test: test-library-unit test-library-integration test-documentation

# Run the unit tests.
test-library-unit:
	cargo test --manifest-path {{cargo_std}} --lib --no-default-features

# Run the integration tests.
test-library-integration:
	cargo test --manifest-path {{cargo_std}} --test integration --no-default-features

# Run the documentation tests.
test-documentation:
	cargo test --manifest-path {{cargo_std}} --doc --no-default-features

# Build the documentation.
build-doc:
	cargo doc --manifest-path {{cargo_std}} --release --no-default-features --package gutenberg_post_parser

# Open the documentation.
open-doc: build-doc
	open target/doc/gutenberg_post_parser/index.html

# Build the readme automatically.
build-readme:
	cargo readme --input src/lib.rs --template README.tpl > README.md

# Local Variables:
# mode: makefile
# End:
