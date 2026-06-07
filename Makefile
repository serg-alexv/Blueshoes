.PHONY: all build clean test

# Top-level Makefile for orchestrating the Blueshoes project

all: build

build:
	@echo "Building bs-edge-agent..."
	cd runtime/bs-edge-agent && cargo build --release

test:
	@echo "Running unit tests..."
	cd runtime/bs-edge-agent && cargo test

clean:
	@echo "Cleaning workspace..."
	cd runtime/bs-edge-agent && cargo clean
