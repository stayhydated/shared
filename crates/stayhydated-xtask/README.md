# stayhydated-xtask

Reusable helpers for repository-owned Rust xtasks. The crate builds mdBook and
LLM text outputs, assembles Dioxus static sites, builds Trunk browser demos,
serves assembled artifacts, and coordinates workspace release publishing.

Call these helpers from a consumer's own xtask so project paths, packages,
routes, and optional outputs remain consumer-owned. The
[`use-stayhydated-github-pages`](../../skills/use-stayhydated-github-pages/SKILL.md)
skill documents the standard GitHub Pages integration.
