# Executive Brief

`sum-numbers-ai-dummy` packages integer addition as a managed AI workflow with a
clear request contract, provider metadata, verified responses, and generated
documentation.

The product surface is designed around operational clarity. A caller submits an
ordered integer workload, the service records the provider route, and the
response returns the answer beside verification and trace evidence.

## What the crate provides

The crate computes the sum locally and returns provider-style metadata. That
keeps every demo deterministic while exercising the shape of an AI-backed API:

- A caller creates a `SumRequest` from an ordered list of `i64` operands.
- The request carries the endpoint and model that would identify the provider
  route.
- `sum_with_request` returns a `SumResponse` with the numeric answer, provider
  metadata, verification status, and trace events.
- The web console and terminal CLI call the same library boundary.
- The mdBook, llms output, sitemap, and static web build all document the same
  current behavior.

## Why the product is focused

An addition-focused AI workflow gives reviewers a compact surface for evaluating
the parts of the product that matter operationally:

- Product reviewers can evaluate the value proposition without learning a
  domain model.
- API reviewers can see the full contract in one page.
- Documentation reviewers can compare examples against the actual Rust types.
- Demo reviewers can verify that the Dioxus and terminal clients agree.

The focused workload keeps the contract easy to inspect while preserving the
same evidence model expected from a larger AI-backed service.
