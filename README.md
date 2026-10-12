# shared

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]

`shared` provides reusable Rust crates and GitHub Actions workflows for authors
of public repository sites. It combines Dioxus UI primitives, configured
application shells, base-path-aware static-site assembly, and build automation
while each consumer owns its identity, content, routes, and sequencing.

Requires Rust 1.99 or newer for all workspace packages. The shared crates use
Dioxus 0.7 and are unpublished Git-sourced packages; consumers keep directly
used packages on one shared revision.

## Overview

The dummy applications exercise Dioxus, Bevy, GPUI, documentation, and static-site
integrations.

Use the [consumer adoption skill][adoption] for revision pins, route manifests,
build assembly, and deployment.

## Crates

Start with `stayhydated-dioxus` for a configured site shell, `stayhydated-site`
for the web launch and route manifest, and `stayhydated-xtask` in the consumer's
build task. Use the core package directly for individual UI primitives.

| Crate | Purpose |
| --- | --- |
| [`stayhydated-dioxus`][dioxus-readme] | Configured project-site shells and presets for consumer-owned identity and routes. |
| [`stayhydated-dioxus-core`][dioxus-core-readme] | Generic Dioxus components and shared visual assets, also reexported by the configured facade. |
| [`stayhydated-site`][site-readme] | Browser launch, base-path routing, route manifests, sitemaps, and static-output helpers. |
| [`stayhydated-xtask`][xtask-readme] | Reusable documentation, build, and preview helpers called by consumer-owned xtasks. |

## Example

The [dummy web entry point][dummy-entry] launches its consumer-owned `App`
through the shared browser boundary:

```rust
fn main() {
    stayhydated_site::launch(web_dummy::App);
}
```

This excerpt belongs to the `web-dummy` package; its library supplies the
Dioxus application, routes, and content.

The [dummy project configuration][dummy-site] supplies the identity used by its
Dioxus shell, while the [dummy build command][dummy-build] passes the exported
route manifest to `stayhydated-xtask`. The reusable [Pages workflow][pages-workflow]
accepts the consumer's build command and artifact path; consumers sequence their
own book, demo, and site builds.

[adoption]: skills/use-stayhydated-github-pages/SKILL.md
[ci-badge]: https://github.com/stayhydated/shared/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/shared/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg
[codecov]: https://app.codecov.io/github/stayhydated/shared
[dioxus-core-readme]: crates/stayhydated-dioxus-core/README.md
[dioxus-readme]: crates/stayhydated-dioxus/README.md
[site-readme]: crates/stayhydated-site/README.md
[xtask-readme]: crates/stayhydated-xtask/README.md
[dummy-site]: dummy/web-dummy/src/site/constants.rs
[dummy-build]: dummy/xtask-dummy/src/commands/build_web.rs
[pages-workflow]: .github/workflows/deploy-pages.yml
[dummy-entry]: dummy/web-dummy/src/main.rs
