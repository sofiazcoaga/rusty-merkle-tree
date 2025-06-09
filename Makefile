test:
	cargo test

build:
	cargo build

clean:
	cargo clean

example:
	cargo build
	cargo run

doc:
	cargo doc --open --no-deps
