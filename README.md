# shared

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]

`shared` provides reusable Rust crates and GitHub Actions workflows for authors
of public repository sites. It combines Dioxus UI primitives, configured
application shells, base-path-aware static-site assembly, and build automation
while each consumer owns its identity, content, routes, and sequencing.

## Overview

Consumer repositories define their project configuration and route manifest;
the shared crates keep application routes, generated outputs, and GitHub Pages
assembly aligned. Runnable fixtures under `dummy/` exercise Dioxus, Bevy, GPUI,
documentation, and static-site integrations.

The [consumer adoption skill][adoption] covers revision synchronization,
application setup, build assembly, deployment, and integration audits.

## Crates

| Crate | Purpose | Source |
| --- | --- | --- |
| `stayhydated-dioxus` | Configured project-site shells and presets | [README][dioxus-readme] |
| `stayhydated-dioxus-core` | Generic Dioxus components and shared visual assets | [README][dioxus-core-readme] |
| `stayhydated-site` | Base-path routing, route manifests, sitemaps, and static-output helpers | [README][site-readme] |
| `stayhydated-xtask` | Reusable documentation, build, preview, and release helpers | [README][xtask-readme] |

[adoption]: skills/use-stayhydated-github-pages/SKILL.md
[ci-badge]: https://github.com/stayhydated/shared/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/shared/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg
[codecov]: https://app.codecov.io/github/stayhydated/shared
[dioxus-core-readme]: crates/stayhydated-dioxus-core/README.md
[dioxus-readme]: crates/stayhydated-dioxus/README.md
[site-readme]: crates/stayhydated-site/README.md
[xtask-readme]: crates/stayhydated-xtask/README.md
