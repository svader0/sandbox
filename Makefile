.PHONY: build run

run: build
	cargo run --release

debug: build
	cargo run

build: 
	cargo build

clean:
	cargo clean