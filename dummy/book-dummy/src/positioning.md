# Product fit

`sum-numbers-ai` helps teams review the shape and presentation of an auditable
AI integration without requiring a live provider. It is best suited to contract,
documentation, and cross-client evaluations; its local evidence is not a
substitute for production-provider testing.

## Intended reviewers

- Product leads can test whether the addition workflow and its evidence are
  understandable.
- Platform and API reviewers can inspect route fields, response metadata, trace
  ordering, and local verification.
- Documentation owners can compare the book's examples with the public Rust
  types and generated outputs.
- Client owners can confirm that Dioxus, terminal, Bevy UI, and GPUI surfaces
  call one Rust boundary.

## Evidence and limits

| Review concern | Evidence in this target | What the evidence means |
| --- | --- | --- |
| Request contract | Ordered `i64` operands plus endpoint and model strings | Callers can inspect the intended route alongside the workload. |
| Result correctness | Local `i128` accumulation and a matching result string | `verified: true` confirms local consistency only. |
| Operational visibility | Synthetic latency, token counts, and five trace events | Reviewers can assess field naming and presentation, not live operations. |
| Client parity | Four clients use `sum_with_request` | The same `8`, `13`, and `21` inputs produce `42` across frameworks. |
| Generated documentation | mdBook, llms files, sitemap, and static site | Reviewers can compare outputs produced from one source tree. |

The configured endpoint uses the reserved `.invalid` domain, and the crate never
opens a network connection. Authentication, retries, timeouts, provider errors,
and real cost or performance data require separate integration evidence.

## Decision criteria

This target is a fit when the decision is about:

- the clarity of a Rust request and response contract;
- the visibility of provider-routing fields and audit events;
- deterministic demonstrations across multiple UI frameworks; or
- alignment between product prose, API examples, and generated documentation.

Use a provider-backed prototype when the decision depends on network behavior,
credentials, model quality, service limits, fallback policy, or observed
latency and token usage.

An evaluation is successful when reviewers can identify what a caller sends,
which fields come back, which values are synthetic, and whether all four clients
show the same result.
