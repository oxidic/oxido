default: run

run:
	cargo run

build:
	cargo build --release

wasm:
	wasm-pack build --target web
	rm -rf pkg/README.md .wasm
	mkdir .wasm
	mv pkg/* .wasm

clear:
	rm -rf pkg 