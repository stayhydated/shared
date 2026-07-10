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

clippy:
    cargo clippy --workspace --all-features --all-targets -- -D warnings

check:
    cargo check --workspace --all-features

test:
    cargo test --workspace --all-features --all-targets

cov:
    cargo llvm-cov --workspace --exclude xtask --exclude xtask-dummy --exclude web-dummy --exclude sum-numbers-ai-dummy --all-features --all-targets
