set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    bun run fmt
    taplo fmt
    rumdl fmt .

clippy:
    cargo clippy --workspace --all-features

check:
    cargo check --workspace --all-features

test:
    cargo test --workspace --all-features --all-targets

cov:
    cargo llvm-cov --workspace --exclude xtask --exclude web --all-features --all-targets
