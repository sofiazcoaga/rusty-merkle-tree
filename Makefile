test:
	cargo test

build:
	cargo build

clean:
	cargo clean

example:
	cargo run --example merkle_tree_example

doc:
	cargo doc --open --no-deps
