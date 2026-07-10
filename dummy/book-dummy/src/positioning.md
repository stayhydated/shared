# Product Positioning

`sum-numbers-ai-dummy` is positioned as a managed AI integration for focused
addition workflows.

The product story is direct: a well-designed AI service should make provider
routing, verification, documentation, and operational evidence easy to inspect.

## Audience

The primary audience is a reviewer who cares about the shape around an AI
feature:

- Product leads evaluating whether the user-facing promise is specific.
- Platform engineers checking provider routing and operational evidence.
- Documentation owners checking that examples match implementation behavior.
- Client owners validating that multiple surfaces use one contract.

## Value Proposition

### Clear contract

The request accepts an ordered list of integers and a provider route. The
response returns the answer, the model-facing result, provider metadata, and
trace events as separate fields.

### Transparent delegation

The provider route is visible. Endpoint, model, latency, prompt tokens,
completion tokens, and trace codes all remain part of the response story.

### Deterministic verification

The crate computes the answer locally with `i128` accumulation. The provider
result is returned beside a `verified` flag so clients can show the AI boundary
without depending on an external service.

### Professional documentation target

The focused workload supports fast review, while the site and book still
exercise real documentation concerns: positioning, API examples, client surfaces,
generated book output, llms text, route metadata, and static web output.

## Message Framework

| Question | Answer |
| --- | --- |
| What is it? | A Rust API and product site for an AI-assisted sum workflow. |
| Why does it exist? | To make AI API integration discipline visible without domain noise. |
| What should reviewers inspect? | Request shape, response envelope, trace data, verification, and client parity. |
| What makes it credible? | The clients call the real local crate and render the response fields directly. |

## Success Criteria

A professional presentation of this product succeeds when a reader can answer
three questions quickly:

1. What does a caller send?
2. What evidence comes back?
3. Which generated docs and client surfaces prove the same contract?
