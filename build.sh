#!/bin/sh

set -ex

cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/parser.wasm --out-dir .
wasm-gc parser_bg.wasm
wasm-opt -Oz -o parser_bg_opt.wasm parser_bg.wasm
mv parser_bg_opt.wasm parser_bg.wasm
npm install
npm run serve
