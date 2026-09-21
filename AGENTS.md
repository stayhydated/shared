# AGENTS.md

`stayhydated/shared` provides reusable site crates and workflows. Consumer
repositories own project identity, content, routes, and build sequencing. The
`dummy/` applications exercise those integrations locally.

Start with `just --list --list-submodules` for repository commands and each
crate's `src/lib.rs` for its public exports.

## Change ownership

| Surface | Owner and related evidence |
| --- | --- |
| Generic Dioxus components and visual assets | `crates/stayhydated-dioxus-core/src/`; render tests and `tests/pass/` cover public component use |
| Project identity, configured shells, and presets | `crates/stayhydated-dioxus/src/`; module and render tests cover `Project`, `ProjectSite`, and wrappers |
| Base paths, route manifests, sitemaps, and route caches | `crates/stayhydated-site/src/`; adjacent module tests define path and output behavior |
| Reusable build and preview helpers | `crates/stayhydated-xtask/src/`; each module owns its command contract, embedded assets, and tests |
| Downstream revision updates | `xtask/src/commands/update_shared_revisions.rs`; its tests, composite action, and reusable workflow define the integration |
| Dummy arithmetic contract | `dummy/sum-numbers-ai-dummy/src/lib.rs`; consumed by the Dioxus, Ratzilla, Bevy, and GPUI clients |
| Static browser clients | `dummy/bevy-demo/` and `dummy/gpui-demo/`; their `build_bevy_demo.rs` and `build_gpui_demo.rs` commands under `dummy/xtask-dummy/src/commands/` stage Trunk inputs |
| Dummy site and generated outputs | `dummy/web-dummy/` and `dummy/xtask-dummy/src/commands/`; `dummy.just` sequences their builds |
| Maintainer documentation fixture | `dummy/book-dummy/src/`; describes the dummy contract and exercises documentation generation |

## Keep related surfaces aligned

- For public API changes, update the owning module, its `src/lib.rs` exports,
  affected crate READMEs, and tests or compile-pass fixtures that use the API.
- For shared theme changes, keep adjacent CSS/WGSL assets, the declarations and
  token tests in `crates/stayhydated-dioxus-core/src/styles.rs`, and render tests
  aligned. Consumers use the bundled theme asset.
- For `Project` or `ProjectSite` changes, update wrapper tests and the consumer
  configuration in `dummy/web-dummy/src/site/constants.rs`.
- For dummy contract changes, update all four clients, the pages under
  `dummy/web-dummy/src/pages/`, and `dummy/book-dummy/src/`. Interactive clients
  expose at most `MAX_DEMO_INPUTS` operands and preserve their order.
- For book, LLM, web, Trunk, loader, or preview behavior, update the owning
  `stayhydated-xtask` module, its tests and embedded assets, and the affected
  commands in `dummy/xtask-dummy/src/commands/`. Generate output through those
  commands rather than editing rendered files.
- For consumer-facing Pages workflows or helper APIs, update the affected
  READMEs and `skills/use-stayhydated-github-pages`. Update fixture chapters
  when their documented commands or behavior change.
- For revision updates, keep the owning Rust command,
  `.github/actions/update-shared-revisions/action.yml`, and
  `.github/workflows/update-shared-revisions.yml` aligned.

## Validation

Choose checks for the edited surface before running the workspace suite:

- Component compile-pass fixtures:
  `cargo test -p stayhydated-dioxus-core --all-features --test compile_pass`.
- Revision updater: `cargo test -p xtask`.
- Dummy documentation:
  `MDBOOK_BUILD__CREATE_MISSING=false cargo run -p xtask-dummy -- build book`;
  rebuild LLM output with `cargo run -p xtask-dummy -- build llms-txt`.
- Dummy site assembly: `just dummy web-build`; inspect it with
  `just dummy web-preview` when routes, assets, or browser behavior change.
- Markdown: use the repository's `rumdl` tooling on the edited files and run
  `git diff --check`.

The root recipes provide formatting, checking, Clippy, tests, and coverage.
CI checks formatting and Clippy, runs workspace tests on Linux, macOS, and
Windows, checks unused dependencies, and collects coverage for the shared crates.
