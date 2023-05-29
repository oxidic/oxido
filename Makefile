default: run

run:
	cargo run

build:
	cargo build --release

wasm:
	wasm-pack build --target nodejs
	mkdir .wasm
	rm -rf pkg/README.md
	mv pkg/* .wasm

clear:
	rm -rf pkg 