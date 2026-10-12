# stayhydated-xtask

[![Codecov: stayhydated-xtask][codecov-badge]][codecov]

`stayhydated-xtask` provides reusable helpers for maintainers of repository-owned
Rust xtasks. It generates documentation and LLM text outputs, assembles Dioxus
static sites, builds Trunk browser demos, and serves assembled artifacts.

Call these helpers from a consumer's own xtask so project paths, packages,
routes, and optional outputs remain consumer-owned. The
[consumer adoption skill][adoption] documents the standard GitHub Pages
integration.

Requires Rust 1.99 or newer. This is an unpublished Git-sourced package; keep
directly used shared packages on the same revision. Web assembly invokes the
consumer's Dioxus 0.7 Web build and expects its generated inputs to exist.

## Example

For a consumer using the standard `web/` layout, construct a build plan from
its route manifest. This example inspects the plan without invoking the
external Dioxus CLI:

```rust
use stayhydated_site::{SiteRouteManifest, routing::SiteUrl};
use stayhydated_xtask::web::WebBuildConfig;

fn main() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    let manifest = SiteRouteManifest::new(
        SiteUrl::new("https://example.test/project/"),
        ["/", "/demos/"],
    );

    let config = WebBuildConfig::github_pages(&workspace_root)
        .package("web")
        .route_manifest(manifest)
        .build();

    assert_eq!(config.dist_dir, workspace_root.join("web/dist"));
    Ok(())
}
```

Passing that configuration to `stayhydated_xtask::web::build` runs a release
Dioxus Web build with static-site generation and assembles `web/dist`. Dioxus
builds use `--locked` and require an up-to-date consumer `Cargo.lock`. Build
generated inputs first and declare consumer-owned demo outputs with
`.extra_dir(...)`. The [dummy build command][dummy-build] demonstrates explicit
assembly for its nonstandard layout, while the [web builder tests][tests]
cover the standard GitHub Pages plan.

[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/SKILL.md
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-xtask
[codecov]: https://app.codecov.io/github/stayhydated/shared
[dummy-build]: https://github.com/stayhydated/shared/blob/master/dummy/xtask-dummy/src/commands/build_web.rs
[tests]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-xtask/src/web.rs
