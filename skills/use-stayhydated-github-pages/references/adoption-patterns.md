# stayhydated GitHub Pages adoption patterns

## Contents

- [Workspace dependencies](#workspace-dependencies)
- [Featureless web package](#featureless-web-package)
- [Single-page project portal](#single-page-project-portal)
- [Multi-route sites](#multi-route-sites)
- [Shared pages and components](#shared-pages-and-components)
- [Assets and Dioxus configuration](#assets-and-dioxus-configuration)
- [GitHub Pages build task](#github-pages-build-task)
- [Browser demo builds](#browser-demo-builds)
- [Static preview](#static-preview)
- [Deployment workflow](#deployment-workflow)
- [Revision automation](#revision-automation)
- [Validation checklist](#validation-checklist)

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
[dependencies]
dioxus = { features = ["launch", "lib", "router", "web"], workspace = true }
stayhydated-dioxus = { workspace = true }
stayhydated-site = { workspace = true }

# xtask/Cargo.toml
[dependencies]
anyhow = { workspace = true }
stayhydated-xtask = { workspace = true }
web = { workspace = true }
```

The web package has one browser shape. Do not add a `web` feature, a native
fallback `main`, or a Dioxus SSR dev-dependency. The published artifact is
always produced by the shared `dx build --platform web --ssg` command.

Update the lockfile without broad dependency upgrades:

```sh
cargo update \
  -p stayhydated-dioxus \
  -p stayhydated-site \
  -p stayhydated-xtask
```

`stayhydated-dioxus-core` arrives transitively unless the consumer directly
uses core-only APIs.

## Featureless web package

The binary launches the only supported application shape directly:

```rust
fn main() {
    stayhydated_site::launch(web::App);
}
```

Keep consumer-owned identity, canonical URLs, and versions in the web library
or its site constants module. Do not put project identity in shared.

## Single-page project portal

Use `ProjectSite` with `StayhydatedSinglePageProjectApp` when `/` is the only
Dioxus route. The preset owns the standard metadata, router, portal,
base-path-aware Book and optional direct Demos destination, and route manifest while
leaving values in the consumer:

```rust
use dioxus::prelude::*;
use stayhydated_dioxus::{
    Project, ProjectSite, StayhydatedSinglePageProjectApp,
};

const PROJECT: Project = Project::new(
    "my-project",
    "A concise project tagline.",
)
.with_skill_command("npx skills add my-organization/my-project");
const SITE_URL: &str = "https://my-organization.github.io/my-project/";
const RUSTDOC_URL: &str = "https://docs.rs/my-project/";
const SOURCE_URL: &str = "https://github.com/my-organization/my-project";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn site() -> ProjectSite {
    ProjectSite::builder()
        .project(PROJECT)
        .site_url(SITE_URL)
        .rustdoc_url(RUSTDOC_URL)
        .source_url(SOURCE_URL)
        .version(VERSION)
        .demo_path("gpui-demo")
        .build()
}

#[component]
pub fn App() -> Element {
    rsx! { StayhydatedSinglePageProjectApp { site: site() } }
}

pub fn route_manifest() -> stayhydated_site::SiteRouteManifest {
    site().single_page_route_manifest()
}
```

Omit `.with_skill_command(...)` when no skill is published. Omit
`.demo_path(...)` when the site has no static demo. Add
`.site_stylesheet_path("assets/site.css")` only when the consumer owns real
project-specific CSS.

Test the owned configuration and manifest directly:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_tracks_the_static_demo() {
        assert_eq!(site().demo_path(), Some("gpui-demo"));
        assert_eq!(site().rustdoc_url(), RUSTDOC_URL);
        assert!(
            route_manifest()
                .static_paths()
                .iter()
                .any(|path| path.as_str() == "/gpui-demo/")
        );
    }
}
```

Do not enable Dioxus SSR to test consumer configuration. Shared component
tests own generic rendered markup behavior.

Use `StayhydatedEmbeddedDemoProjectApp` when the portal should keep its shared
header while a single static browser demo runs inside the page. Configure the
raw artifact with `demo_path`, link the portal to the preset's `/demo/` route,
and assemble both the application route and static output from one manifest:

```rust
use dioxus::prelude::*;
use stayhydated_dioxus::{
    Project, ProjectSite, StayhydatedEmbeddedDemoProjectApp,
};

#[component]
pub fn App() -> Element {
    rsx! { StayhydatedEmbeddedDemoProjectApp { site: site() } }
}

pub fn route_manifest() -> stayhydated_site::SiteRouteManifest {
    site().embedded_demo_route_manifest()
}
```

The manifest treats `/demo/` as a Dioxus application route and the configured
`demo_path` as the raw static iframe source.

## Multi-route sites

Treat the `Routable` enum as the application-route source of truth. Route
manifest paths are root-relative (`/`, `/demos/`, `/demos/example/`) and do
not include the repository slug. `Routable::static_routes()` excludes dynamic
segments, so add any dynamic paths that need generated fallbacks through an
explicit manifest source.

Define the same general configuration used by the single-page preset in the
consumer's site constants module:

```rust
use stayhydated_dioxus::{Project, ProjectSite};

pub(crate) const PROJECT: Project = Project::new(
    "my-project",
    "A concise project tagline.",
);
pub(crate) const SITE_URL: &str = "https://my-organization.github.io/my-project/";
pub(crate) const RUSTDOC_URL: &str = "https://docs.rs/my-project/";
pub(crate) const SOURCE_URL: &str = "https://github.com/my-organization/my-project";
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn site() -> ProjectSite {
    ProjectSite::builder()
        .project(PROJECT)
        .site_url(SITE_URL)
        .rustdoc_url(RUSTDOC_URL)
        .source_url(SOURCE_URL)
        .version(VERSION)
        .site_stylesheet_path("assets/site.css")
        .build()
}
```

Export one manifest from the web library:

```rust
pub fn route_manifest() -> stayhydated_site::SiteRouteManifest {
    site::constants::site()
        .route_manifest::<site::routing::AppRoute>()
        .with_static_paths(["/bevy-demo/", "/gpui-demo/"])
}
```

Static paths are included in the sitemap but do not receive application-route
fallback files. The project manifest automatically includes `/book/`,
`/llms.txt`, and `/llms-full.txt`.

Render through the shared router. The base href comes from the Dioxus CLI
inside shared:

```rust
use dioxus::prelude::*;
use stayhydated_dioxus::StayhydatedProjectApp;

#[component]
pub fn App() -> Element {
    rsx! {
        StayhydatedProjectApp::<AppRoute> { site: site() }
    }
}
```

Omit `site_stylesheet_path` from `ProjectSite` when the consumer has no project
CSS. Resolve project-owned static destinations from the same configuration:

```rust
let gpui_demo = site().static_href("gpui-demo");
```

Wrap each route's content with canonical metadata:

```rust
rsx! {
    StayhydatedProjectPageMetadata {
        project: PROJECT,
        page_title: page.title(),
        description: page.description(),
    }
    {pages::route_content(page)}
}
```

## Shared pages and components

For a custom multi-route home page, use `StayhydatedProjectSitePortal`. It
reads Docs, Book, Git, version, and identity from `ProjectSite`; pass `demos`
only when a gallery exists:

```rust
StayhydatedProjectSitePortal::<AppRoute> {
    site: site(),
    home: NavigationTarget::Internal(app_route(PageKind::Home)),
    demos: NavigationTarget::Internal(app_route(PageKind::Demos)),
}
```

Use the lower-level `StayhydatedProjectPortal` when the page intentionally
needs destinations that differ from `ProjectSite`.

Use `StayhydatedProjectPortalShell` for demo pages and other project-specific
content inside the shared frame. Use `StayhydatedProjectLanding` only when a
compact landing is intentionally preferred over the portal.

Use `DemoGallery` instead of rebuilding the common grid, accent selection,
shader offset, and reveal behavior:

```rust
let items = vec![
    DemoGalleryItem::route(
        app_route(PageKind::Dioxus),
        "Dioxus",
        "dioxus-demo-card-shader",
    ),
    DemoGalleryItem::href(
        site().static_href("gpui-demo"),
        "GPUI",
        "gpui-demo-card-shader",
    ),
];

rsx! {
    DemoGallery::<AppRoute> {
        items,
        columns: DemoGalleryColumns::Three,
    }
}
```

Two columns are the default. Prefer other exported shared components—tabs,
selects, fullscreen frames, shader backgrounds, landing links, and reveal
styles—over local equivalents.

## Assets and Dioxus configuration

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

Keep `[application].name` and `[web.app].base_path` aligned to the same
non-empty project slug. Keep Cargo repository metadata aligned when present.

`StayhydatedProjectApp` and the single-page preset insert the shared component
styles, including the bundled Dioxus component theme. Do not keep a copied
`web/public/dx-components-theme.css`. A consumer stylesheet is optional; keep
`web/public/assets/site.css` only when it contains project-specific rules and
configure it explicitly in the app.

`WebBuildConfig::github_pages` writes `.nojekyll` into the assembled artifact
and normally copies `web/public/assets` when that directory exists.

## GitHub Pages build task

Use the root-driven builder and pass the manifest as one contract:

```rust
use stayhydated_xtask::web::WebBuildConfig;

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;

    stayhydated_xtask::web::build(
        WebBuildConfig::github_pages(&workspace_root)
            .package("web")
            .route_manifest(web::route_manifest())
            .build(),
    )
}
```

Use explicit inputs only when the consumer needs them:

```rust
WebBuildConfig::github_pages(&workspace_root)
    .command_current_dir(workspace_root.join("web"))
    .no_public_assets_dir()
    .extra_dir("web/assets", "assets")
    .extra_file("web/public/assets/site.css", "assets/site.css")
    .extra_dir("web/public/bevy-demo", "bevy-demo")
    .extra_dir("web/public/gpui-demo", "gpui-demo")
    .route_manifest(web::route_manifest())
    .build()
```

The builder always runs a release Dioxus Web SSG build, assembles `web/dist`,
copies available book/LLM/public assets, writes route fallbacks and `404.html`,
and renders `sitemap.xml` from the manifest. It has no SSR build mode.

## Browser demo builds

Use `stayhydated_xtask::trunk` instead of copying a Trunk subprocess, output
verifier, HTML shell, or initializer into the consumer. Generate the standard
fullscreen page and shared loader with:

```rust
use stayhydated_xtask::trunk::{TrunkDemoBuildConfig, TrunkDemoPageConfig};

let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
stayhydated_xtask::trunk::build(
    &TrunkDemoBuildConfig::builder()
        .workspace_root(workspace_root)
        .example_dir("examples/gpui-demo")
        .output_dir("web/public/gpui-demo")
        .example_name("gpui-demo")
        .required_marker("my-project-gpui-demo")
        .toolchain("nightly")
        .generated_page(
            TrunkDemoPageConfig::builder()
                .title("my-project GPUI demo")
                .demo_name("GPUI")
                .build(),
        )
        .build(),
)
```

Add optional `TrunkDemoCopyDir` values when the demo needs copied assets. The
helper stages generated inputs under
`target/stayhydated-trunk/<example-name>/` and verifies JavaScript, Wasm, and
the required marker.

Keep only demos the consumer actually produces. Do not add Trunk or nightly to
a site without a matching browser build.

## Static preview

Keep live development, publication assembly, and static preview distinct:

```just
web-build:
    cargo xtask build book
    cargo xtask build llms-txt
    cargo xtask build web

web: web-build
    dx serve --package web

web-preview: web-build
    cargo xtask preview web
```

Wire `PreviewCommand::Web` through `StaticSitePreviewConfig` with `web/dist`
and the project base path. Exercise the assembled artifact because direct
navigation, fallback files, copied books/demos, and the base path are Pages
contracts that `dx serve` alone does not prove.

## Deployment workflow

Use the shared reusable workflow. The consumer owns only its prerequisite build
sequence and tool requirements:

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
        env -u RUSTC_WRAPPER cargo xtask build gpui-demo
        cargo xtask build book
        cargo xtask build llms-txt
        cargo xtask build web
      install-trunk: true
      install-nightly: true
```

Omit `install-trunk` and `install-nightly` when unused. The default artifact is
`web/dist`; override `artifact-path` only for a genuinely different layout.
Keep an explicit workflow only when it owns setup the reusable inputs cannot
express.

## Revision automation

Every pinned consumer should use the reusable updater:

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

It updates dependencies sourced from `stayhydated/shared` and regenerates the
lockfile. It does not update immutable reusable-workflow SHAs.

## Validation checklist

Run the consumer's repository-standard commands first. A focused sequence is:

```sh
just fmt
cargo test -p web --lib --locked
cargo check -p xtask --locked
just web-build
cargo xtask preview web --help
python3 <shared-checkout>/skills/use-stayhydated-github-pages/scripts/audit_consumer.py \
  . \
  --dist \
  --expected-shared-revision <40-character-sha> \
  --site-url https://my-organization.github.io/my-project/
git diff --check
```

Run `just check`, `just clippy`, and `just test` when repository guidance or
the change scope calls for them. Run the consumer's localization or other
library feature matrices separately; the `web` package itself has no feature
matrix.

Inspect `web/dist` for:

- `.nojekyll`, `index.html`, `404.html`, and `sitemap.xml`;
- every application-route fallback;
- requested `book/`, `llms.txt`, `llms-full.txt`, `llms/`, and demo outputs;
- a hashed `assets/dx-components-theme*.css` bundled by Dioxus;
- `assets/site.css` only when the app explicitly configures project CSS.

The shared component theme is bundled into Dioxus assets, so a root
`dx-components-theme.css` is not part of the assembled contract.

The audit validates declared sitemap entries but cannot infer a missing
consumer-owned static destination. Keep direct manifest tests for every book,
LLM, or browser-demo path the repository promises.
