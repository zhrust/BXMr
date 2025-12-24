# BXMr Makefile
# rIME-Squirrel BXM code table management tool
# 表形码管理工具

.PHONY: all build release clean test doc clippy check fmt run help

# Default target
all: build

# Development build
build:
	cargo build

# Release build with optimizations
release:
	cargo build --release

# Run the application
run:
	cargo run

# Run clippy linter with warnings as errors
clippy:
	cargo clippy -- -D warnings

# Run all checks (clippy, fmt, build)
check: fmt clippy build
	@echo "All checks passed!"

# Format code with rustfmt
fmt:
	cargo fmt

# Format check (for CI)
fmt-check:
	cargo fmt --check

# Generate documentation
doc:
	cargo doc --no-deps --open

# Generate documentation without opening browser
doc-build:
	cargo doc --no-deps

# Clean build artifacts
clean:
	cargo clean

# Run tests (when available)
test:
	cargo test

# Install to ~/.cargo/bin
install:
	cargo install --path .

# Show help
help:
	@echo "BXMr Makefile Commands:"
	@echo "  make build    - Development build"
	@echo "  make release  - Release build with optimizations"
	@echo "  make run      - Run the application"
	@echo "  make clippy   - Run clippy linter"
	@echo "  make check    - Run all checks (fmt, clippy, build)"
	@echo "  make fmt      - Format code with rustfmt"
	@echo "  make doc      - Generate and open documentation"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make test     - Run tests"
	@echo "  make install  - Install to ~/.cargo/bin"
	@echo "  make help     - Show this help"
