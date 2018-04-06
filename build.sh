#!/bin/sh

set -ex

cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/parser.wasm --out-dir .
npm install
npm run serve
