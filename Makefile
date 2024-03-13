RUSTUP:=$(shell which rustup)
CARGO:=$(shell which cargo)

run:
	$(CARGO) build
	sudo RUST_LOG=debug ./target/debug/tetragon-grpc-client
