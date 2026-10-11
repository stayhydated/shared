set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[private]
default:
    @just --justfile dummy.just --list

web-build:
    env -u RUSTC_WRAPPER cargo run --locked -p xtask-dummy -- build bevy-demo
    env -u RUSTC_WRAPPER cargo run --locked -p xtask-dummy -- build gpui-demo
    cargo run --locked -p xtask-dummy -- build book
    cargo run --locked -p xtask-dummy -- build llms-txt
    cargo run --locked -p xtask-dummy -- build web

web: web-build
    dx serve --package web-dummy

web-preview: web-build
    cargo run --locked -p xtask-dummy -- preview web
