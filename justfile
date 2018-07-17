cwd = `pwd`
cargo_no_std = "cargo/no_std/Cargo.toml"
cargo_std = "cargo/std/Cargo.toml"
asmjs_directory = "bindings/asmjs"
c_directory = "bindings/c"
nodejs_directory = "bindings/nodejs"
php_directory = "bindings/php"
wasm_directory = "bindings/wasm"
fuzz_directory = "fuzz"

# Build a regular library..
build-library:
	cargo build --manifest-path {{cargo_std}} --release

# Build a regular binary.
build-binary:
	cargo build --manifest-path {{cargo_std}} --features "bin" --release

# Check that the WASM binary can be build.
check-wasm:
	# Checking `rustup` is installed…
	@which rustup > /dev/null || \
		(echo 'Please, install `rustup`, check https://rustup.rs/.' && exit 1)
	# Checking nightly toolchain is installed…
	@rustup toolchain list | \
		grep 'nightly-' > /dev/null || \
		(echo 'Please, install the nightly toolchain for `rustc` with `rustup install nightly`.' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `wasm32-unknown-unknown` target is installed…
	@rustup target list --toolchain nightly | \
		grep 'wasm32-unknown-unknown (installed)' > /dev/null || \
		(echo 'Please, install the `wasm32-unknown-unknown` target with `rustup target add --toolchain nightly wasm32-unknown-unknown.' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `wasm-gc` is installed…
	@which wasm-gc > /dev/null || \
		(echo 'Please, install `wasm-gc` with `cargo install wasm-gc`.' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `wasm-snip` is installed…
	@which wasm-snip > /dev/null || \
		(echo 'Please, install `wasm-snip` with `cargo install wasm-snip`.' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `wasm-opt` is installed…
	@which wasm-opt > /dev/null || \
		(echo 'Please, install `wasm-opt`, check https://github.com/WebAssembly/binaryen (prebuilds are attached to releases).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `gzip` is installed…
	@which gzip > /dev/null || \
		(echo 'Please, install `gzip`, check http://www.gzip.org/ (it is very likely that the package manager of your OS can install it).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `brotli` is installed…
	@which brotli > /dev/null || \
		(echo 'Please, install `brotli`, check https://github.com/google/brotli (it is very likely that the package manager of your OS can install it).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m

# Build the parser and produce a WASM binary.
build-wasm: check-wasm
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
	cd {{wasm_directory}}/web && php -S localhost:8888 -t . server.php

# Check that the ASM.js module can be build.
check-asmjs:
	# Checking `wasm2es6js` is installed…
	@which wasm2es6js > /dev/null || \
		(echo 'Please, install `wasm2es6js` with `cargo install wasm-bindgen-cli`.' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m

# Build the parser and produce an ASM.js module.
build-asmjs: check-asmjs build-wasm
	wasm2es6js --wasm2asm --output {{asmjs_directory}}/bin/gutenberg_post_parser.asm.js {{wasm_directory}}/bin/gutenberg_post_parser.wasm
	cd {{asmjs_directory}}/bin/ && \
		sed -i '' '1s/^/function GUTENBERG_POST_PARSER_ASM_MODULE() {/; s/export //; s/const /var /; s/let /var /' gutenberg_post_parser.asm.js && \
		echo 'return { root: root, alloc: alloc, dealloc: dealloc, memory: memory }; }' >> gutenberg_post_parser.asm.js && \
		uglifyjs --mangle --output .temp.asm.js gutenberg_post_parser.asm.js && \
		mv .temp.asm.js gutenberg_post_parser.asm.js && \
		gzip --best --stdout gutenberg_post_parser.asm.js > gutenberg_post_parser.asm.js.gz && \
		brotli --best --stdout --lgwin=24 gutenberg_post_parser.asm.js > gutenberg_post_parser.asm.js.br

# Start an HTTP server to serve ASM.js demo.
start-asmjs-server:
	cd {{asmjs_directory}}/web && php -S localhost:8888 -t . server.php

# Check that the C binary can be build.
check-c:
	# Checking `clang` is installed…
	@which clang > /dev/null || \
		(echo 'Please, install `clang`, check https://clang.llvm.org/ (it is very likely that the package manager of your OS can install it).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m

# Build the parser and produce a C binary.
build-c: check-c
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

# Check that the NodeJS native module can be build.
check-nodejs:
	# Checking `neon` is installed…
	@which neon > /dev/null || \
		(echo 'Please, install `neon` with `npm install --global neon-cli` (`npm` is the NodeJS Package Manager, see https://www.nodejs.org/).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m

# Build the parser and produce a NodeJS native module.
build-nodejs: check-nodejs
	cd {{nodejs_directory}} && neon build

# Check that the PHP extension can be build.
check-php:
	# Checking `php` is installed…
	@which php > /dev/null || \
		(echo 'Please, install `php`, check http://php.net/ (it is very likely that the package manager of your OS can install it).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m
	# Checking `make` is installed…
	@which make > /dev/null || \
		(echo 'Please, install `make` (it is very likely that the package manager of your OS can install it).' && exit 1)
	# ^^^^^^^^ ~~> [32mOK[0m

# Build the parser and produce a PHP extension.
build-php php_prefix_bin='/usr/local/bin': check-php
	cd {{c_directory}} && cargo build --release
	cd {{php_directory}}/extension/gutenberg_post_parser/ && \
		{{php_prefix_bin}}/phpize --clean && \
		{{php_prefix_bin}}/phpize && \
		./configure --with-php-config={{php_prefix_bin}}/php-config && \
		sudo make install

# Test everything.
test: test-library test-wasm test-c test-php

# Run all tests for the parser.
test-library: build-library test-library-unit test-library-integration test-documentation

# Run the unit tests of the parser.
test-library-unit:
	cargo test --manifest-path {{cargo_std}} --lib

# Run the documentation tests.
test-documentation:
	cargo test --manifest-path {{cargo_std}} --doc

# Run the integration tests of the parser.
test-library-integration:
	cargo test --manifest-path {{cargo_std}} --test integration

# Run all tests for the WASM binary.
test-wasm: build-wasm test-wasm-integration

# Run the integration tests of the WASM binary.
test-wasm-integration:
	cd {{wasm_directory}} && cargo +nightly test --test integration

# Run all tests for the C binding.
test-c: build-c test-c-unit test-c-integration

# Run the unit tests of the C binding.
test-c-unit:
	cd {{c_directory}} && cargo test --lib

# Run the integration tests of the C binding.
test-c-integration:
	cd {{c_directory}} && cargo test --test integration

# Run all tests for the PHP binding.
test-php: test-php-integration

# Run the integration tests of the PHP binding.
test-php-integration:
	cd {{php_directory}} && cargo test --test integration

# Run a fuzzer on the library.
fuzz-library:
	cd {{fuzz_directory}} && \
		cargo afl build --release && \
		cargo afl fuzz -i inputs -o output ../target/release/fuzz

# Build the documentation.
build-doc:
	cargo doc --manifest-path {{cargo_std}} --release --no-default-features --package gutenberg_post_parser

# Open the documentation.
open-doc: build-doc
	open target/doc/gutenberg_post_parser/index.html

# Build the readme automatically.
build-readme:
	cargo readme --project-root {{cargo_std}} --input src/lib.rs --template README.tpl > README.md

# Local Variables:
# mode: makefile
# End:
