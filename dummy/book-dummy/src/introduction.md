# Fixture overview

This book is source material for the `sum-numbers-ai` documentation fixture in
the `stayhydated/shared` workspace. Maintainers use it to exercise mdBook, LLM
text generation, static-site assembly, and links between several Rust browser
clients.

The fixture models an AI-style addition request, but every value is computed
locally. The reserved `.invalid` endpoint, provider metadata, and trace events
exist to give the shared presentation and generation code stable content.

## What the fixture exercises

- `sum-numbers-ai-dummy` supplies one deterministic Rust contract.
- Dioxus and Ratzilla present the contract as console and terminal experiences.
- Bevy UI and GPUI present the same contract as browser demos.
- The dummy xtask builds this book, LLM text files, route fallbacks, a sitemap,
  and the assembled site.

The canonical scenario uses `8`, `13`, and `21`. Each client should display a
sum of `42` and successful local verification.

## Source and generated output

Edit chapters under `dummy/book-dummy/src`. The dummy xtask writes rendered book
and LLM artifacts under `dummy/web-dummy/public`; treat those files as generated
output and rebuild them from this source.

See [Scenario coverage](positioning.md) for the surfaces under review,
[Dummy Rust contract](api-contract.md) for the shared data shape, and
[Build and inspect](operating-model.md) for repository commands.
