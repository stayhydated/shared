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
- Public Rust exports start in each library crate's `src/lib.rs`. Keep those exports aligned with the owning module and any `tests/pass/*` compile-pass fixture that names the API.
- The workspace package version is `0.1.0`; keep docs, exports, examples, and tests focused on the current API shape unless a user or repository policy asks for a compatibility bridge.

## Workspace Map

- `crates/stayhydated-dioxus-core`
  Audience: Public integration.
  Role: generic Dioxus landing page, portal, shader demo card, full-screen demo frame, code block, tab, select, metadata, shared value types, styles, and shader background assets.
  Sync: component API changes may need `src/lib.rs`, adjacent `*.css` or `*.wgsl` assets, and the trybuild fixtures under `crates/stayhydated-dioxus-core/tests/`.

- `crates/stayhydated-dioxus`
  Audience: Public integration.
  Role: Dioxus application wrapper, consumer-configured project identity, landing and portal helpers, page metadata, and selected re-exports from `stayhydated-dioxus-core`.
  Sync: project identity and presentation behavior are tested in adjacent module tests and `tests/render_components.rs`; asset path behavior is tested in `src/app.rs`.

- `dummy/sum-numbers-ai-dummy`
  Audience: Local validation.
  Role: real dummy library crate for the sum-numbers-ai concept, including local sum behavior and provider-style trace data.
  Sync: API or positioning changes may need `src/lib.rs`, all four dummy clients, `dummy/book-dummy/src/`, and consumer-owned site constants under `dummy/web-dummy/src/site/`.

- `dummy/web-dummy`
  Audience: Local validation.
  Role: runnable Dioxus web crate and demo gallery for the sum-numbers-ai dummy project, including a Ratzilla terminal demo and links to the two static WebAssembly examples.
  Sync: generated static output and preview behavior are invoked by `dummy/xtask-dummy`; project cards in `src/pages/demos.rs`, static demo sitemap entries, and `dummy/book-dummy` should stay aligned.

- `dummy/bevy-demo`
  Audience: Local validation.
  Role: Bevy UI WebAssembly example with three editable operands for the dummy sum contract.
  Sync: shared Trunk page inputs are staged from `dummy/xtask-dummy/src/commands/build_bevy_demo.rs`, which also owns the build output; gallery positioning lives in `dummy/web-dummy/src/pages/demos.rs`.

- `dummy/gpui-demo`
  Audience: Local validation.
  Role: GPUI WebAssembly example using gpui-component inputs for the dummy sum contract.
  Sync: shared Trunk page inputs are staged from `dummy/xtask-dummy/src/commands/build_gpui_demo.rs`, which also owns the build output; gallery positioning lives in `dummy/web-dummy/src/pages/demos.rs`.

- `dummy/book-dummy`
  Audience: Documentation fixture.
  Role: mdBook source for the local sum-numbers-ai documentation target.
  Sync: generated book output behavior is invoked by `dummy/xtask-dummy`; website cards in `dummy/web-dummy/src/pages/demos.rs` should stay aligned with book positioning.

- `dummy/xtask-dummy`
  Audience: Internal workflow.
  Role: local build and preview commands for dummy book, llms output, Bevy and GPUI Trunk output, and Dioxus static-site output under `dummy/web-dummy`.
  Sync: path changes under any dummy client, `dummy/book-dummy`, or `dummy/web-dummy` may need command updates under `src/commands/` and in `dummy.just`, which the root `justfile` imports.

- `crates/stayhydated-site`
  Audience: Public integration.
  Role: UI-neutral base-path, href, sitemap, and generated route-cache helpers for static project sites.
  Sync: route path, sitemap static output, and route-cache cleanup behavior is encoded in module tests next to `src/routing.rs`, `src/sitemap.rs`, and `src/route_cache.rs`.

- `crates/stayhydated-xtask`
  Audience: Internal workflow and public repository tooling.
  Role: helper APIs for mdBook output, llms output, Dioxus static-site builds, Trunk demos, static preview serving, and release publishing.
  Sync: generated web behavior is owned by `src/book.rs`, `src/llms.rs`, `src/web.rs`, `src/trunk.rs`, and `src/preview.rs`; keep their embedded JavaScript assets and module tests aligned. Release order and `cargo publish` behavior are owned by `src/release.rs` and its tests.

- `xtask`
  Audience: Internal workflow.
  Role: repository-owned maintenance commands, including the GitHub Action that updates downstream Cargo revisions to `stayhydated/shared` `master`.
  Sync: revision-update behavior is owned by `src/commands/update_shared_revisions.rs`; keep its tests, `.github/actions/update-shared-revisions/action.yml`, and `.github/workflows/update-shared-revisions.yml` aligned.

- `crates/stayhydated-dioxus-core/tests`
  Audience: Validation.
  Role: render and trybuild compile-pass coverage for the public component API.
  Sync: public component changes may need the matching render test or `tests/pass/*` fixture.

## Synchronization Rules

- When changing a public Rust type, function, component prop, route helper, or exported constant, update the owning module, the crate `src/lib.rs` export surface, and any tests or trybuild fixtures that name the changed API.
- When changing `dx-components-theme.css` or `DX_COMPONENTS_THEME_FILE_NAME`, keep `crates/stayhydated-dioxus-core/src/styles.rs` and `crates/stayhydated-xtask/src/web.rs` aligned.
- When changing the consumer-owned `Project` identity or its Dioxus wrappers, update `crates/stayhydated-dioxus/src/project.rs`, the crate export surface, adjacent wrapper tests, and the dummy identity in `dummy/web-dummy/src/site/constants.rs`.
- When changing sum-numbers-ai positioning, keep the library behavior in `dummy/sum-numbers-ai-dummy/src/lib.rs`, all four demo clients, website pages under `dummy/web-dummy/src/pages/`, book chapters under `dummy/book-dummy/src/`, and consumer-owned constants under `dummy/web-dummy/src/site/` aligned. Interactive demos expose at most three operands and do not provide operand reordering.
- When changing sitemap, route-cache, book, llms, Dioxus build, Trunk build, generated loader, or preview behavior, update the owning `stayhydated-site` or `stayhydated-xtask` module plus tests that encode its output or command contract.
- When changing release publishing behavior in `stayhydated-xtask/src/release.rs`, update tests for publish order, command arguments, dirty-worktree guards, resume points, and registry handling.
- When changing downstream revision-update behavior, keep `xtask/src/commands/update_shared_revisions.rs`, its tests, `.github/actions/update-shared-revisions/action.yml`, and `.github/workflows/update-shared-revisions.yml` aligned.

## Validation

- Use `just --list` for the local recipe index.
- Local recipes currently include `just fmt`, `just clippy`, `just check`, `just test`, and `just cov`.
- CI runs `cargo fmt --all -- --check`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`, `cargo test --workspace --all-features` on Linux/macOS/Windows, `cargo-machete`, and coverage with dummy applications and xtasks excluded from instrumentation.
- For trybuild changes, run `cargo test -p stayhydated-dioxus-core --all-features --test compile_pass` for `tests/pass/*` fixtures before broad workspace validation.
- For downstream revision-update changes, run `cargo test -p xtask` before broad workspace validation.
- For README-only or AGENTS.md-only changes, static review is sufficient unless a repository command directly covers the edited Markdown.
