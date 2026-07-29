# Evaluate and operate the demos

Build and preview the repository's static site to compare the four clients
against the same Rust request. A successful review shows `42` and
`verified true` for the operands `8`, `13`, and `21`.

## Prerequisites

Run the commands in this chapter from the `shared` repository root. The complete
site build requires:

- a Rust toolchain compatible with the workspace's Rust `1.96` requirement;
- the `wasm32-unknown-unknown` target for stable Rust;
- nightly Rust with the `wasm32-unknown-unknown` target for the GPUI demo;
- `just`, Trunk, and the Dioxus CLI (`dx`) on `PATH`; and
- Bun to run the local preview server.

The mdBook-only build uses the repository's Rust wrapper and does not require a
separate `mdbook` executable.

## Build the documentation only

Run:

```console
cargo run -p xtask-dummy -- build book
```

The command succeeds when it prints `mdBook built successfully` and writes
`dummy/web-dummy/public/book/index.html`.

## Build and preview the complete site

1. Build the Bevy UI demo, GPUI demo, mdBook, llms outputs, and Dioxus static
   site:

   ```console
   just dummy web-build
   ```

2. Start the preview server without rebuilding:

   ```console
   cargo run -p xtask-dummy -- preview web
   ```

3. Open the URL printed by the command. The default is
   `http://127.0.0.1:8081/sum-numbers-ai/`; if port `8081` is busy, the server
   selects the next available port.

`just dummy web-preview` combines the complete build and preview steps.

## Compare the client surfaces

| Client | Review action | Success signal |
| --- | --- | --- |
| Dioxus | Change, remove, or restore up to three operands | Request, response, and trace panels update together |
| Terminal | Enter `[8,13,21]`, `sum [8,13,21]`, or `run [8,13,21]` | Output includes request ID, operands, sum, verification, model, latency, and five trace lines |
| Bevy UI | Edit any of the three operand fields | Total recomputes after all fields parse as `i64` |
| GPUI + gpui-component | Edit the three fields, then use the reset button | Total recomputes and reset restores `8 + 13 + 21` |

The visual clients expose at most three operands and no reordering controls.
The Rust crate itself accepts longer workloads, as described in the
[Rust API contract](api-contract.md#public-helpers-and-constants).

## Generated outputs

| Command target | Output |
| --- | --- |
| `build bevy-demo` | `dummy/web-dummy/public/bevy-demo` |
| `build gpui-demo` | `dummy/web-dummy/public/gpui-demo` |
| `build book` | `dummy/web-dummy/public/book` |
| `build llms-txt` | `public/llms.txt`, `public/llms-full.txt`, and the `public/llms` Markdown mirror under `dummy/web-dummy` |
| `build web` | Complete static site in `dummy/web-dummy/dist`, including copied generated outputs, route fallbacks, `404.html`, and `sitemap.xml` |

## Troubleshoot the local build

| Symptom | Action |
| --- | --- |
| The Bevy or GPUI build cannot start `trunk` | Install Trunk and confirm `trunk --version` succeeds |
| The GPUI build reports a missing toolchain or target | Install nightly Rust and add its `wasm32-unknown-unknown` target |
| The web build cannot start `dx` | Install the Dioxus CLI and confirm `dx --version` succeeds |
| The preview cannot start `bun` | Install Bun and confirm `bun --version` succeeds |
| A demo reports invalid input | Restore one to three `i64` values in Dioxus or terminal, or all three fields in Bevy UI or GPUI |
