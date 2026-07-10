# AGENTS.md

This is the working guide for contributors and coding agents in the `stayhydated/shared` workspace.

Use it to decide:

- which workspace crate owns a change;
- which public exports, tests, fixtures, assets, or README files must move with it;
- which generated-output helper owns web/book/llms output behavior;
- which narrow repository command proves the edited surface.

## Start Here

- `Cargo.toml` defines the Rust workspace members and workspace lints.
- `justfile` is the local command index. Start with `just --list` before choosing a broad command.
- Public Rust exports start in each crate's `src/lib.rs`. Keep those exports aligned with the owning module and any compile-pass or compile-fail fixture that names the API.
- The workspace package version is `0.1.0`; keep docs, exports, examples, and tests focused on the current API shape unless a user or repository policy asks for a compatibility bridge.

## Workspace Map

- `crates/stayhydated-dioxus-core`
  Audience: Public integration.
  Role: generic Dioxus components, navigation/layout/card/tab APIs, shared newtypes, styles, and shader background assets.
  Sync: component API changes may need `src/lib.rs`, adjacent `*.css` or `*.wgsl` assets, and the trybuild fixtures under `crates/stayhydated-dioxus-core/tests/`.

- `crates/stayhydated-dioxus`
  Audience: Public integration.
  Role: stayhydated project-site wrapper, shared project registry, project header/footer helpers, and re-exports from `stayhydated-dioxus-core`.
  Sync: project inventory, package URLs, docs/source URLs, skill commands, header labels, and asset path behavior are tested in `src/projects.rs`, `src/header.rs`, and `src/app.rs`.

- `dummy/sum-numbers-ai-dummy`
  Audience: Local validation.
  Role: real dummy library crate for the sum-numbers-ai concept, including local sum behavior and provider-style trace data.
  Sync: API or positioning changes may need `src/lib.rs`, `dummy/web-dummy/src/lib.rs`, `dummy/web-dummy/src/terminal.rs`, `dummy/book-dummy/src/`, and project registry data in `crates/stayhydated-dioxus/src/projects.rs`.

- `dummy/web-dummy`
  Audience: Local validation.
  Role: runnable Dioxus web crate for the sum-numbers-ai dummy project, including a Ratzilla terminal demo and Bun preview script.
  Sync: generated static output is built by `dummy/xtask-dummy`; preview behavior is owned by `preview.ts`; project cards should stay aligned with `dummy/book-dummy`.

- `dummy/book-dummy`
  Audience: Documentation fixture.
  Role: mdBook source for the local sum-numbers-ai documentation target.
  Sync: generated book output behavior is invoked by `dummy/xtask-dummy`; website cards in `dummy/web-dummy/src/lib.rs` should stay aligned with book positioning.

- `dummy/xtask-dummy`
  Audience: Internal workflow.
  Role: local build commands for dummy book, llms output, and Dioxus static-site output under `dummy/web-dummy`.
  Sync: path changes under `dummy/book-dummy` or `dummy/web-dummy` may need command updates under `src/commands/` and root `justfile` recipes.

- `crates/stayhydated-site`
  Audience: Public integration.
  Role: base-path, href, sitemap, and generated route-cache helpers for static project sites.
  Sync: route path, sitemap static output, and route-cache cleanup behavior is encoded in module tests next to `src/routing.rs`, `src/sitemap.rs`, and `src/route_cache.rs`.

- `crates/stayhydated-xtask`
  Audience: Internal workflow and public repository tooling.
  Role: helper APIs for mdBook output, llms output, Dioxus static-site builds, and release publishing.
  Sync: web output paths and copied assets are owned by `src/book.rs`, `src/llms.rs`, and `src/web.rs`; release order and `cargo publish` behavior are owned by `src/release.rs` and its tests.

- `crates/stayhydated-dioxus-core/tests`
  Audience: Validation.
  Role: trybuild compile-pass and compile-fail coverage for the public component API.
  Sync: when expected diagnostics change, update the matching `tests/ui/*.stderr` file with the source fixture change.

## Synchronization Rules

- When changing a public Rust type, function, component prop, route helper, or exported constant, update the owning module, the crate `src/lib.rs` export surface, and any tests or trybuild fixtures that name the changed API.
- When changing `dx-components-theme.css` or `DX_COMPONENTS_THEME_FILE_NAME`, keep `crates/stayhydated-dioxus-core/src/styles.rs` and `crates/stayhydated-xtask/src/web.rs` aligned.
- When changing project registry data in `stayhydated-dioxus/src/projects.rs`, update tests for project options, URLs, package sets, support links, llms links, and skill commands in the same module.
- When changing sum-numbers-ai positioning, keep the library behavior in `dummy/sum-numbers-ai-dummy/src/lib.rs`, website cards in `dummy/web-dummy/src/lib.rs`, terminal rendering in `dummy/web-dummy/src/terminal.rs`, and book chapters under `dummy/book-dummy/src/` aligned.
- When changing sitemap, route-cache, book, llms, or static-site output behavior, update the helper that owns the output path plus the tests that encode the path or copied file list.
- When changing release publishing behavior in `stayhydated-xtask/src/release.rs`, update tests for publish order, command arguments, dirty-worktree guards, resume points, and registry handling.

## Validation

- Use `just --list` for the local recipe index.
- Local recipes currently include `just fmt`, `just clippy`, `just check`, `just test`, and `just cov`.
- CI runs `cargo fmt --all -- --check`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`, `cargo test --workspace --all-features` on Linux/macOS/Windows, `cargo-machete`, and coverage with `cargo llvm-cov --workspace --all-features --cobertura --output-path=target/cobertura.xml`.
- For trybuild changes, run `cargo test -p stayhydated-dioxus-core --all-features --test compile_pass` for `tests/pass/*` fixtures or `cargo test -p stayhydated-dioxus-core --all-features --test compile_fail` for `tests/ui/*` fixtures before broad workspace validation.
- For README-only or AGENTS.md-only changes, static review is sufficient unless a repository command directly covers the edited Markdown.
