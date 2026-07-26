set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[private]
default:
    @just --justfile dummy.just --list

web-build:
    env -u RUSTC_WRAPPER cargo run -p xtask-dummy -- build bevy-demo
    env -u RUSTC_WRAPPER cargo run -p xtask-dummy -- build gpui-demo
    cargo run -p xtask-dummy -- build book
    cargo run -p xtask-dummy -- build llms-txt
    cargo run -p xtask-dummy -- build web

web: web-build
    dx serve --package web-dummy

web-preview: web-build
    cargo run -p xtask-dummy -- preview web
