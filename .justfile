# List available commands
default:
    @just --list

# Run tests
test:
    cargo test

# Build the project
build:
    cargo build --release

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Format code
format:
    cargo fmt

# Check formatting
check-format:
    cargo fmt -- --check

# Run all checks (test, lint, format)
check: test lint check-format

# Clean build artifacts
clean:
    cargo clean

# Bump version (usage: just bump-version [major|minor|patch])
bump-version TYPE:
    cargo install cargo-edit
    cargo set-version --bump {{TYPE}}

# Publish to crates.io
publish:
    cargo publish

# Generate and update README.md
update-readme:
    # Add your command to generate/update README.md here

# Deploy (run checks, bump version, update readme, and publish)
deploy TYPE:
    just check
    just bump-version {{TYPE}}
    just update-readme
    just publish

# Install CDW locally
install:
    cargo install --path .

# Uninstall CDW
uninstall:
    cargo uninstall cdw

# Generate shell completions
generate-completions:
    # Add commands to generate shell completions for different shells

# Build and run CDW
run *ARGS:
    cargo run -- {{ARGS}}