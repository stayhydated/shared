# shared

Reusable Rust crates and workflows for stayhydated GitHub Pages sites. This
repository is primarily a source dependency and maintenance workspace; project
identity and content stay in each consumer repository.

[![Build Status](https://github.com/stayhydated/shared/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/shared/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/github/stayhydated/shared/graph/badge.svg?token=34CV04UOU1)](https://codecov.io/github/stayhydated/shared)

## Workspace crates

- [`stayhydated-dioxus-core`](crates/stayhydated-dioxus-core/README.md) provides
  generic Dioxus components and shared visual assets.
- [`stayhydated-dioxus`](crates/stayhydated-dioxus/README.md) provides configured
  project-site shells and presets.
- [`stayhydated-site`](crates/stayhydated-site/README.md) owns base-path routing,
  route manifests, sitemaps, and static-output helpers.
- [`stayhydated-xtask`](crates/stayhydated-xtask/README.md) provides reusable
  build and preview helpers for consumer xtasks.

Use the
[`use-stayhydated-github-pages`](skills/use-stayhydated-github-pages/SKILL.md)
skill for the complete adoption and maintenance workflow. Contributors can run
`just --list` from the repository root to see the available workspace tasks.
