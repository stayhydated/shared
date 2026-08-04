# Build and inspect

Build the dummy outputs from the `shared` repository root. A successful fixture
review produces the requested artifact and shows `42` for the operands `8`,
`13`, and `21` in every client.

## Prerequisites

Run the commands in this chapter from the `shared` repository root. The complete
site build requires:

- a Rust toolchain compatible with the workspace's Rust `1.96` requirement;
- the `wasm32-unknown-unknown` target for stable Rust;
- nightly Rust with the `wasm32-unknown-unknown` target for the GPUI demo;
- `just`, Trunk, and the Dioxus CLI (`dx`) on `PATH`; and
- Bun to run the local preview server.

The book build runs through the Rust helper and needs no standalone `mdbook`
binary.

## Build an output

| Output | Command |
| --- | --- |
| mdBook | `cargo run -p xtask-dummy -- build book` |
| LLM text files | `cargo run -p xtask-dummy -- build llms-txt` |
| Bevy browser demo | `cargo run -p xtask-dummy -- build bevy-demo` |
| GPUI browser demo | `cargo run -p xtask-dummy -- build gpui-demo` |
| Dioxus static site | `cargo run -p xtask-dummy -- build web` |

Run `just dummy web-build` to build the demos, book, LLM outputs, and final site
in dependency order.

## Preview the assembled site

Run `just dummy web-preview` to rebuild and serve the complete artifact. To
serve an existing `dummy/web-dummy/dist` without rebuilding, run:

```console
cargo run -p xtask-dummy -- preview web
```

Open the base-path URL printed by the preview command.

## Inspect the result

- `dummy/web-dummy/public/book/index.html` is the rendered book entry point.
- `dummy/web-dummy/public/llms.txt`, `llms-full.txt`, and `llms/` are generated
  from the book source.
- `dummy/web-dummy/public/bevy-demo` and `gpui-demo` contain the Trunk artifacts.
- `dummy/web-dummy/dist` contains the assembled site, route fallbacks,
  `404.html`, and `sitemap.xml`.
- The Dioxus, terminal, Bevy, and GPUI clients should agree on the canonical
  result.

## Troubleshoot prerequisites

| Symptom | Action |
| --- | --- |
| A browser demo cannot start `trunk` | Install Trunk and confirm `trunk --version` succeeds |
| The GPUI build reports a missing toolchain or target | Install nightly Rust and its `wasm32-unknown-unknown` target |
| The web build cannot start `dx` | Install the Dioxus CLI and confirm `dx --version` succeeds |
| The preview cannot start Bun | Install Bun and confirm it is available on `PATH` |
