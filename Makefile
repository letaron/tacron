run:
	cargo run

fmt:
	cargo +nightly fmt

test:
	cargo test
	cargo +nightly fmt --all -- --check
