# See https://just.systems/man/

export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "short")
export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings -A deprecated")

default: fetch deny lint test-build test

lint: check-fmt clippy docs md-lint action-lint action-dev-check

md-lint:
    markdownlint-cli2 '**/*.md' '!**/target'

fetch:
    just-cargo fetch

check-fmt:
    just-cargo fmt -- --check

clippy:
    just-cargo clippy --frozen --workspace --all-targets --all-features

deny:
    cargo-deny --all-features check

docs:
    just-cargo doc --frozen --no-deps --features=k8s-openapi/v1_25

test-build *flags:
    just-cargo test-build --frozen {{ flags }}

test *flags:
    just-cargo test --frozen {{ flags }}

publish *flags:
    cargo publish --features=k8s-openapi/v1_25 {{ flags }}

action-lint:
    just-dev lint-actions

action-dev-check:
    just-dev check-action-images
    just-dev pull-action-images
