# makefile borrowed from https://github.com/killercup/wasm-experiments

WAT_ARGS ?= --fold-exprs --inline-exports --generate-names


all: demo.rs
	rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib demo.rs
	mv demo.wasm demo-bloated.wasm
	wasm-gc demo-bloated.wasm demo.wasm
	wasm2wat $(WAT_ARGS) demo.wasm > demo.wat
