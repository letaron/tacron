init:
	if [ ! -f "config.yaml" ]; then cp config.yaml.dist config.yaml; fi

run:
	cargo run

fmt:
	cargo +nightly fmt

test:
	cargo test
	cargo +nightly fmt --all -- --check
