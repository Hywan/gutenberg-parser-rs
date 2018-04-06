#!/bin/sh

set -ex

cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/parser.wasm --out-dir .
wasm-gc parser_bg.wasm
npm install
npm run serve
