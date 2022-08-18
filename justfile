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

default: fetch deny lint test-build test

lint: check-fmt clippy docs md-lint action-lint action-dev-check

md-lint:
    markdownlint-cli2 '**/*.md' '!**/node_modules' '!**/target'

fetch:
    {{ cargo }} fetch

check-fmt:
    {{ cargo }} fmt -- --check

clippy:
    {{ cargo }} clippy --frozen --workspace --all-targets --all-features {{ _fmt }}

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


##
## GitHub Actions
##

# Format actionlint output for Github Actions if running in CI.
_actionlint-fmt := if env_var_or_default("GITHUB_ACTIONS", "") != "true" { "" } else {
  '{{range $err := .}}::error file={{$err.Filepath}},line={{$err.Line}},col={{$err.Column}}::{{$err.Message}}%0A```%0A{{replace $err.Snippet "\\n" "%0A"}}%0A```\n{{end}}'
}

# Lints all GitHub Actions workflows
action-lint:
    actionlint {{ if _actionlint-fmt != '' { "-format '" + _actionlint-fmt + "'" } else { "" } }} .github/workflows/*

# Ensure all devcontainer versions are in sync
action-dev-check:
    action-dev-check

# vim: set ft=make :
