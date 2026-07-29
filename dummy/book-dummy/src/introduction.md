# Overview

`sum-numbers-ai` is a deterministic evaluation target for an auditable
AI-style addition API. Its Rust crate accepts ordered `i64` operands and returns
an `i128` total together with a request ID, route metadata, verification status,
and trace events.

All calculation, verification, and provider-style evidence is generated locally.
The examples do not contact the configured endpoint or invoke an AI model. Use
them to review a contract and its presentation across clients, not to measure
provider availability, latency, token use, or model accuracy.

## What you can evaluate

- The `sum-numbers-ai-dummy` crate provides the executable Rust contract.
- The Dioxus console displays an illustrative request, response, and trace.
- The Ratzilla terminal accepts JSON-style integer lists through a `clap`
  command parser.
- The Bevy UI and GPUI browser demos recompute the same Rust sum from three
  fields.
- The build produces the mdBook, an llms text set, a sitemap, and the static demo
  site from the same repository state.

When reviewers use the operands `8`, `13`, and `21`, every client reports a
total of `42` and a successful local verification.

## Choose your path

- Buyers and product reviewers can use [Product fit](positioning.md) to
  understand what this target does and what it cannot prove.
- Rust callers and API reviewers can use the
  [Rust API contract](api-contract.md) for defaults, fields, examples, and input
  boundaries.
- Demo reviewers can use
  [Evaluate and operate the demos](operating-model.md) to build the site and
  compare all four clients.
