# See https://just.systems/man/

export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "short")
export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings -A deprecated")

toolchain := ""
cargo := "cargo" + if toolchain != "" { " +" + toolchain } else { "" }

# If we're running in Github Actions and cargo-action-fmt is installed, then add
# a command suffix that formats errors.
_fmt := if env_var_or_default("GITHUB_ACTIONS", "") != "true" { "" } else {
    ```
    if command -v cargo-action-fmt >/dev/null 2>&1; then
        echo "--message-format=json | cargo-action-fmt"
    fi
    ```
}

default: fetch check-fmt deny clippy docs test-build test

fetch:
    {{ cargo }} fetch

check-fmt:
    {{ cargo }} fmt -- --check

check *flags:
    {{ cargo }} clippy --frozen --all-targets {{ flags }} {{ _fmt }}

clippy *flags:
    {{ cargo }} clippy --frozen --all-targets {{ flags }} {{ _fmt }}

deny:
    {{ cargo }} deny --all-features check

docs:
    {{ cargo }} doc --frozen --no-deps --features=k8s-openapi/v1_24 {{ _fmt }}

test-build *flags:
    {{ cargo }} test --no-run --frozen {{ flags }} {{ _fmt }}

test *flags:
    {{ cargo }} test --frozen {{ flags }}

publish *flags:
    {{ cargo }} publish --features=k8s-openapi/v1_24 {{ flags }}

# vim: set ft=make :
