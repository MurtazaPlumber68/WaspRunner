
# Justfile for WasmRunner development tasks

# Default recipe
default:
    @just --list

# Build all crates
build:
    cargo build --workspace

# Build release version
build-release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Run tests with coverage
test-coverage:
    cargo tarpaulin --workspace --out Html

# Run clippy lints
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run security audit
audit:
    cargo audit

# Build examples
build-examples:
    cd examples/hello-world && cargo build --target wasm32-wasi --release
    cd examples/http-service && cargo build --target wasm32-wasi --release
    cd examples/db-plugin && cargo build --target wasm32-wasi --release

# Install CLI locally
install:
    cargo install --path crates/wasmrunner-cli

# Run integration tests
integration-test: build-examples install
    wasmrunner --version
    wasmrunner run examples/hello-world/target/wasm32-wasi/release/hello-world.wasm

# Start development environment
dev:
    cargo watch -x "build" -x "test"

# Clean build artifacts
clean:
    cargo clean

# Generate documentation
docs:
    cargo doc --workspace --no-deps --open

# Run fuzz tests
fuzz TARGET:
    cargo fuzz run {{TARGET}}

# Benchmark performance
bench:
    cargo bench --workspace

# Check dependencies for updates
deps-update:
    cargo update

# Release preparation
release-prep VERSION:
    #!/usr/bin/env bash
    echo "Preparing release {{VERSION}}"
    sed -i 's/version = ".*"/version = "{{VERSION}}"/' Cargo.toml
    sed -i 's/version.workspace = true/version = "{{VERSION}}"/' crates/*/Cargo.toml
    echo "Updated version to {{VERSION}}"
    echo "Don't forget to update CHANGELOG.md!"

# Deploy to registry
deploy: build-release
    cargo publish -p wasmrunner-core
    sleep 10
    cargo publish -p wasmrunner-runtime
    sleep 10
    cargo publish -p wasmrunner-sandbox
    sleep 10
    cargo publish -p wasmrunner-plugins
    sleep 10
    cargo publish -p wasmrunner-store
    sleep 10
    cargo publish -p wasmrunner-cli
