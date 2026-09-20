# stayhydated-xtask

[![Codecov: stayhydated-xtask][codecov-badge]][codecov]

`stayhydated-xtask` provides reusable helpers for maintainers of repository-owned
Rust xtasks. It generates documentation and LLM text outputs, assembles Dioxus
static sites, builds Trunk browser demos, serves assembled artifacts, and
coordinates workspace release publishing.

Call these helpers from a consumer's own xtask so project paths, packages,
routes, and optional outputs remain consumer-owned. The
[consumer adoption skill][adoption] documents the standard GitHub Pages
integration.

## Example

For the standard `web/` layout, pass the web library's manifest to the GitHub
Pages builder from the consumer's xtask command:

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

This runs a release Dioxus Web build with static-site generation and assembles
`web/dist`. Build generated inputs before this command. Add consumer-owned
demo outputs with `.extra_dir(...)`, then serve the assembled artifact through
`preview::serve` to check direct navigation under the project base path.

[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/SKILL.md
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-xtask
[codecov]: https://app.codecov.io/github/stayhydated/shared
