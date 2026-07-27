# stayhydated GitHub Pages adoption patterns

## Contents

- [Evidence map](#evidence-map)
- [Current consumer baseline](#current-consumer-baseline)
- [Workspace dependencies](#workspace-dependencies)
- [Consumer-owned project configuration](#consumer-owned-project-configuration)
- [Web package](#web-package)
- [Base-path-safe app and routing](#base-path-safe-app-and-routing)
- [Shared pages and components](#shared-pages-and-components)
- [Dioxus and asset configuration](#dioxus-and-asset-configuration)
- [GitHub Pages build task](#github-pages-build-task)
- [Static preview](#static-preview)
- [Deployment workflow](#deployment-workflow)
- [Revision automation](#revision-automation)
- [Validation checklist](#validation-checklist)

## Evidence map

Inspect these paths in the `shared` checkout at the consumer's target
revision:

- `crates/stayhydated-site/src/routing.rs`: `BasePath`, `BaseHref`,
  `RoutePath`, `OutputDir`, `Href`, and `SiteUrl`.
- `crates/stayhydated-site/src/sitemap.rs`: sitemap generation.
- `crates/stayhydated-dioxus/src/`: application wrappers, consumer-configured
  project pages, and component exports.
- `crates/stayhydated-dioxus-core/src/`: framework-neutral Dioxus components
  and shared styles.
- `crates/stayhydated-xtask/src/web.rs`: Pages build configuration.
- `crates/stayhydated-xtask/src/preview.rs`: base-path-aware preview.
- `crates/stayhydated-xtask/src/book.rs`, `llms.rs`, and `trunk.rs`: optional
  artifact builders.
- `dummy/web-dummy` and `dummy/xtask-dummy`: production-shaped integration
  examples.
- `.github/workflows/deploy-pages.yml`: reusable deployment contract.

When Koruma or es-fluent is locally available, inspect its `web/`, `xtask/`,
`justfile`, and `.github/workflows/` at the consumer's checked-out branch.
Treat those repositories as consumer evidence, not as sources for shared
project metadata.

## Current consumer baseline

Koruma and es-fluent share these surfaces:

- a `web` library plus feature-gated binary;
- `Project`, `StayhydatedRouterApp`, `StayhydatedProjectPageMetadata`,
  `StayhydatedProjectPortal`, and `StayhydatedProjectPortalShell`;
- one route model for Dioxus dispatch, fallback paths, metadata, and sitemap
  inputs;
- `cargo xtask build book`, `cargo xtask build llms-txt`, and
  `cargo xtask build web`;
- `just web-build`, `just web`, and `just web-preview`;
- a Bun `web/preview.ts` that serves `web/dist` under the root package name;
- an explicit Pages workflow and the shared revision-update workflow.

Preserve their build variants:

| Surface | Koruma | es-fluent |
| --- | --- | --- |
| Web command | Root-driven `.package("web")` | Runs from `web/` |
| Public assets | Default `web/public` copy | Explicit localization and stylesheet inputs |
| Browser demos | Dioxus routes | Bevy and GPUI Trunk outputs plus Dioxus routes |
| Extra toolchains | Dioxus CLI | Dioxus CLI, Trunk, and nightly GPUI build |

Neither current consumer uses `StayhydatedProjectLanding`; treat it as an
optional component rather than part of the required adoption.

## Workspace dependencies

Place one synchronized shared revision in the consumer workspace:

```toml
[workspace.dependencies.stayhydated-dioxus]
git = "https://github.com/stayhydated/shared"
rev = "<40-character-sha>"

[workspace.dependencies.stayhydated-site]
git = "https://github.com/stayhydated/shared"
rev = "<40-character-sha>"

[workspace.dependencies.stayhydated-xtask]
git = "https://github.com/stayhydated/shared"
rev = "<40-character-sha>"
```

Use workspace inheritance in the web and xtask packages:

```toml
# web/Cargo.toml
[features]
default = ["web"]
web = ["dioxus/web", "stayhydated-site/web"]

[dependencies]
dioxus = { features = ["launch", "lib", "router"], workspace = true }
stayhydated-dioxus = { workspace = true }
stayhydated-site = { workspace = true }

# xtask/Cargo.toml
[dependencies]
anyhow = { workspace = true }
stayhydated-xtask = { workspace = true }
web = { workspace = true }
```

Add Dioxus with its `ssr` feature under web dev-dependencies only when
project-owned route, metadata, or content render tests need it.

Update the lockfile without broad dependency upgrades:

```sh
cargo update \
  -p stayhydated-dioxus \
  -p stayhydated-site \
  -p stayhydated-xtask
```

`stayhydated-dioxus-core` arrives transitively unless the consumer directly
uses core-only APIs.

## Consumer-owned project configuration

Keep each repository's identity and destinations local. Shared provides the
`Project` value type and reusable rendering components, but it does not catalog
consumer repositories.

Define the identity in the consumer:

```rust
use stayhydated_dioxus::Project;

pub(crate) const PROJECT: Project =
    Project::new("my-project", "A concise project tagline.")
        .with_skill_command("npx skills add my-organization/my-project");
pub(crate) const SITE_URL: &str =
    "https://my-organization.github.io/my-project/";
pub(crate) const RUSTDOC_URL: &str = "https://docs.rs/my-project/";
pub(crate) const SOURCE_URL: &str =
    "https://github.com/my-organization/my-project";
```

Omit `.with_skill_command(...)` when the project does not publish an agent
skill. Keep book and demo destinations base-path-aware through the consumer's
routing helpers.

When updating a consumer pinned before the consumer-owned `Project` API:

1. replace its catalog enum variant with `Project::new(...)`;
2. replace `PROJECT.site_url()` with a local `SITE_URL`;
3. add local Rustdoc and source URL constants;
4. pass `docs`, `book`, and `source` to `StayhydatedProjectPortal`.

Existing `StayhydatedProjectPageMetadata` and
`StayhydatedProjectPortalShell` call sites continue to accept the local
`PROJECT` value.

## Web package

A feature-gated binary can launch the shared app:

```rust
#[cfg(not(feature = "web"))]
compile_error!("web must be built with the `web` feature enabled");

#[cfg(not(feature = "web"))]
fn main() {}

#[cfg(feature = "web")]
fn main() {
    stayhydated_site::launch(
        stayhydated_site::SiteApp::builder()
            .app(web::App)
            .build(),
    );
}
```

Keep `PROJECT`, `SITE_URL`, destination constants, and `VERSION` together in
the consumer's site constants module.

Export the build inputs from the web library:

```rust
pub use site::app::App;

pub fn route_paths() -> Vec<String> {
    site::routing::all_routes()
        .into_iter()
        .map(|route| route.path().into_string())
        .collect()
}

pub fn sitemap_xml() -> String {
    site::render::render_sitemap()
}
```

## Base-path-safe app and routing

Let the Dioxus CLI provide the runtime project prefix:

```rust
use dioxus::cli_config;
use stayhydated_site::routing::{BaseHref, BasePath};

pub(crate) fn app_base_href() -> BaseHref {
    let base_path = cli_config::base_path();
    let base_path = base_path.as_deref().map(BasePath::new);
    stayhydated_site::routing::base_href(base_path.as_ref())
}
```

Render the router through the shared app wrapper so shared and project
stylesheets use the same base:

```rust
use dioxus::prelude::*;
use stayhydated_dioxus::StayhydatedRouterApp;

#[component]
pub fn App() -> Element {
    let base_href = crate::site::routing::app_base_href();

    rsx! {
        StayhydatedRouterApp::<AppRoute> {
            base_href: base_href.to_string(),
        }
    }
}
```

Model project routes once. Use that model for:

- the `Routable` enum;
- route-to-page dispatch;
- page title and description;
- exported fallback paths;
- sitemap paths.

The route paths supplied to `WebBuildConfig` are root-relative application
paths such as `/`, `/demos/`, and `/demos/example/`. Do not prefix them with
the repository slug.

Wrap each route's content with canonical metadata:

```rust
rsx! {
    StayhydatedProjectPageMetadata {
        project: PROJECT,
        page_title: route.page.title(),
        description: route.page.description(),
    }
    {pages::route_content(route)}
}
```

Create the sitemap from the same routes:

```rust
use stayhydated_site::routing::SiteUrl;

pub(crate) fn render_sitemap() -> String {
    let paths = crate::site::routing::all_routes()
        .into_iter()
        .map(|route| route.path())
        .collect::<Vec<_>>();

    stayhydated_site::sitemap::render_project(&SiteUrl::new(SITE_URL), paths)
}
```

## Shared pages and components

Current consumers render `StayhydatedProjectPortal` on the home route. Pass
every destination explicitly:

```rust
StayhydatedProjectPortal::<AppRoute> {
    project: PROJECT,
    version: VERSION,
    home: NavigationTarget::Internal(app_route(PageKind::Home)),
    docs: Href::new(RUSTDOC_URL),
    book: book_href(),
    demos: NavigationTarget::Internal(app_route(PageKind::Demos)),
    source: Href::new(SOURCE_URL),
}
```

Use `StayhydatedProjectPortal` when the standard Docs, Book, Demos, and Git
labels fit. Use `StayhydatedProjectPortalShell` for demo pages and other
project-specific content inside the shared frame.

Use `StayhydatedProjectLanding` only when the consumer requests a compact
landing instead of the portal:

```rust
StayhydatedProjectLanding {
    project: PROJECT,
    eyebrow: "my-organization / Rust",
    links: vec![
        LandingLink::new("book/", "Read the book"),
        LandingLink::new(RUSTDOC_URL, "Rust API docs"),
        LandingLink::new(SOURCE_URL, "Source"),
    ],
    theme: LandingTheme::Cyan,
}
```

Use typed targets:

```rust
NavigationTarget::Internal(app_route(PageKind::Demos))
NavigationTarget::<AppRoute>::External("https://example.test/".to_owned())
```

Use shared demo cards for galleries:

```rust
let demo_count = demos.len();

for (position, (route, title, shader_id, time_offset)) in
    demos.into_iter().enumerate()
{
    DemoCard::<AppRoute> {
        target: NavigationTarget::Internal(route),
        accent: DemoCardAccent::for_position(position, demo_count),
        title,
        shader_id,
        time_offset,
    }
}
```

Prefer other exported shared components—tabs, selects, fullscreen frames,
shader backgrounds, landing links, and reveal styles—over local equivalents.

## Dioxus and asset configuration

Set the repository slug as the Dioxus base path:

```toml
[application]
asset_dir = "public"
name = "my-project"
out_dir = "dist"

[web.app]
base_path = "my-project"
title = "my-project | Home"

[web.watcher]
index_on_404 = true
reload_html = true
watch_path = ["src", "public"]
```

`StayhydatedRouterApp` inserts:

- the shared component styles;
- the project stylesheet at `<base>/assets/site.css`;
- the generated component theme at `<base>/dx-components-theme.css`.

Current consumers keep `web/public/.nojekyll` and
`web/public/dx-components-theme.css` available to local `dx serve`. Preserve
those checked-in development inputs during an ordinary shared-revision update.
`WebBuildConfig::github_pages` writes authoritative copies into `web/dist` and
normally copies `web/public/assets` into `web/dist/assets`.

Keep shared component selectors and tokens in shared. Put project-specific
rules in `web/public/assets/site.css`, or explicitly configure another asset
source in the build task.

## GitHub Pages build task

Use the root-driven default-assets build for the Koruma-shaped consumer:

```rust
use stayhydated_xtask::web::WebBuildConfig;

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;

    stayhydated_xtask::web::build(
        WebBuildConfig::github_pages(&workspace_root)
            .package("web")
            .route_fallback_paths(web::route_paths())
            .sitemap_xml(web::sitemap_xml())
            .build(),
    )
}
```

Use the explicit-assets variant for an es-fluent-shaped consumer:

```rust
use stayhydated_xtask::web::WebBuildConfig;

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;

    stayhydated_xtask::web::build(
        WebBuildConfig::github_pages(&workspace_root)
            .command_current_dir(workspace_root.join("web"))
            .no_public_assets_dir()
            .extra_dir("web/assets", "assets")
            .extra_file("web/public/assets/site.css", "assets/site.css")
            .extra_dir("web/public/bevy-demo", "bevy-demo")
            .extra_dir("web/public/gpui-demo", "gpui-demo")
            .route_fallback_paths(web::route_paths())
            .sitemap_xml(web::sitemap_xml())
            .build(),
    )
}
```

Keep only extra inputs the consumer actually produces. In particular, do not
add localization assets, Bevy output, GPUI output, Trunk, or nightly setup to a
consumer that does not use them.

The builder:

- runs a release Dioxus web SSG build;
- assembles `web/dist`;
- copies `web/public/assets`, `book`, `llms`, `llms.txt`, and
  `llms-full.txt` when present;
- writes `.nojekyll` and `dx-components-theme.css`;
- writes route fallback `index.html` files;
- copies the root index to `404.html`;
- writes the supplied sitemap.

Use `.extra_dir(source, destination)` and `.extra_file(source, destination)`
for project-owned static demos or artifacts. Use `.no_public_assets_dir()` or
`.public_assets_dir(path)` only when the repository intentionally assembles
assets explicitly.

If the Dioxus command runs from `web/`, use
`.command_current_dir(workspace_root.join("web"))`; otherwise prefer the
root-driven `.package("web")` shape.

Build only the prerequisite outputs owned by the consumer before the final web
task:

```sh
# es-fluent browser demos
cargo xtask build bevy-demo
cargo xtask build gpui-demo

# common documentation outputs
cargo xtask build book
cargo xtask build llms-txt

# final Pages artifact
cargo xtask build web
```

Use `stayhydated_xtask::book`, `llms`, and `trunk` helpers rather than copying
their implementation into the consumer.

## Static preview

Current consumers build first and then run the Bun preview from `web/`:

```just
web-preview: web-build
    cd web && bun run preview
```

`web/package.json` maps `preview` to `bun run preview.ts`. The script:

- serves `web/dist`;
- derives the Pages base path from the root package name;
- redirects `/` to that base path;
- resolves direct files and directory `index.html` files;
- falls back to `404.html`;
- honors `HOST` and `PORT`, selecting the next available port when needed.

Preserve this preview path during ordinary shared adoption work. Use
`stayhydated_xtask::preview::StaticSitePreviewConfig` only when the consumer
already uses the Rust preview or the task explicitly requests that migration.

Preview the assembled artifact, not only `dx serve`, because direct navigation,
fallback files, copied books/demos, and the repository base path are Pages
contracts.

## Deployment workflow

Current consumers own an explicit `.github/workflows/gh-pages.yml` with
separate build and deploy jobs. Preserve that workflow during ordinary shared
adoption and revision updates.

Keep its project-specific setup aligned with the build:

- Koruma installs the Dioxus CLI and runs book, llms.txt, and web builds.
- es-fluent also installs Trunk and nightly, builds Bevy and GPUI demos, then
  builds book, llms.txt, and web output.
- both upload `web/dist` and deploy it through GitHub Pages actions.

Use the shared reusable workflow only for a new consumer or an explicitly
requested workflow migration:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [master]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  deploy:
    uses: stayhydated/shared/.github/workflows/deploy-pages.yml@master
    with:
      build-command: |
        cargo xtask build book
        cargo xtask build llms-txt
        cargo xtask build web
      artifact-path: web/dist
```

For the reusable workflow, set `install-trunk: true` for Trunk-built browser
demos and `install-nightly: true` for GPUI wasm builds that require nightly.
Add those demo build commands before the web build.

Follow repository policy for reusable-workflow refs. A full commit pin is
appropriate for immutable workflow policy; `@master` follows the pattern used
by current stayhydated reusable workflows. Cargo dependencies remain pinned to
one full commit SHA either way.

Keep an explicit workflow when it owns setup that the reusable inputs do not
cover. Align its Rust target, Dioxus/Trunk versions, Pages permissions, artifact
path, and deploy actions with the consumer's actual build.

## Revision automation

The shared reusable updater can keep Cargo revisions current:

```yaml
name: update shared revisions

on:
  schedule:
    - cron: "0 0 * * *"
  workflow_dispatch:

permissions:
  contents: write
  pull-requests: write

jobs:
  update:
    uses: stayhydated/shared/.github/workflows/update-shared-revisions.yml@master
```

It updates Cargo dependencies sourced from `stayhydated/shared` and regenerates
the lockfile. It does not update immutable reusable-workflow SHAs.

## Validation checklist

Run the consumer's repository-standard commands first. A typical focused
sequence is:

```sh
just fmt
just check
just clippy
just test
cargo test -p web --lib --no-default-features --locked
just web-build
git diff --check
```

Adapt the feature set and generated prerequisites to repository evidence. Run
the consumer's focused Dioxus/localization feature matrix when its CI or
`justfile` defines one.

Inspect `web/dist` for:

- `.nojekyll`, `index.html`, and `404.html`;
- `dx-components-theme.css` and `assets/site.css`;
- every route fallback path;
- `sitemap.xml` with the canonical project URL;
- requested `book/`, `llms.txt`, `llms-full.txt`, `llms/`, and demo outputs.

Exercise the root and at least one nested route through a preview mounted at
`/<project-slug>/` with `just web-preview`. Test project-owned integration;
rely on shared tests for generic component styling and rendering.
