.PHONY: build run

run: build
	cargo run

build: 
	cargo build