test:
	cargo test

build:
	cargo build

clean:
	rm -rf target

example:
	cargo build
	cargo run

doc:
	cargo doc --open --no-deps
