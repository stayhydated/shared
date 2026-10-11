set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

mod dummy

default:
    @just --list --list-submodules

fmt:
    cargo sort-derives
    cargo fmt
    bun run fmt
    taplo fmt
    rumdl fmt .

fmt-check:
    cargo fmt --all -- --check
    cargo sort-derives --check
    taplo fmt --check
    rumdl check .
    typos
    actionlint
    git ls-files -z '*.sh' '*.bash' | xargs -0 --no-run-if-empty shellcheck
    bun run fmt --check
    bun run lint
    git diff --check

clippy:
    cargo clippy --workspace --all-features --all-targets --locked -- -D warnings

check:
    cargo check --workspace --all-features --locked

test:
    cargo test --workspace --all-features --all-targets --locked

cov:
    cargo llvm-cov --workspace --exclude xtask-dummy --exclude web-dummy --exclude sum-numbers-ai-dummy --exclude sum-numbers-ai-bevy-demo --exclude sum-numbers-ai-gpui-demo --all-features --all-targets
