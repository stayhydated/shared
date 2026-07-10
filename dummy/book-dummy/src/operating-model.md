# Operating Model

The operating model is designed to make the AI boundary visible enough for
product review, implementation review, and generated-output validation.

## Request Lifecycle

1. Accept an ordered list of integer operands from the client.
2. Build a `SumRequest` with the selected endpoint and model.
3. Generate a deterministic request identifier from operands, endpoint, and
   model.
4. Record provider-style trace events through `tracing` and the response trace
   vector.
5. Accumulate the answer locally with `i128`.
6. Return provider metadata, the model-style result, verification status, and
   trace evidence.

## Evidence Model

Every successful response carries three evidence layers:

- `provider`: endpoint, model, latency, prompt tokens, and completion tokens.
- `trace`: ordered events for endpoint resolution, transport, prompt contract,
  model dispatch, and verification.
- `verified`: a boolean summary that the provider-style result matched the local
  guardrail.

Those fields support the managed AI workflow presentation while keeping the
runtime deterministic.

## Provider Policy

The default provider route is explicit:

```text
endpoint: https://api.sum-numbers-ai.invalid/v1/responses
model: sum-numbers-ai/addition-router-2026-07
```

The local implementation uses stable provider-style metadata so reviewers can
discuss model routing, timeout budgets, token costs, and fallback policy without
calling a network service.

## Client Surfaces

### Dioxus console

The Dioxus page owns editable operand state, validates each input, and renders
three review panels:

- Request facade
- Verified response envelope
- Provider evidence trail

### Terminal CLI

The Ratzilla terminal starts with starter commands and routes input through a
`clap` parser. Operators can run either form:

```text
[1,2,3]
sum [4, 5, 6]
```

The terminal returns the same request identity, sum, verification status, model,
latency, and trace lines as the web console.

## Generated Outputs

`xtask-dummy` owns the generated project artifacts:

- `cargo run -p xtask-dummy -- build book` writes the mdBook output into
  `dummy/web-dummy/public/book`.
- `cargo run -p xtask-dummy -- build llms-txt` writes the llms text output into
  `dummy/web-dummy/public/llms.txt`.
- `cargo run -p xtask-dummy -- build web` writes the static Dioxus site into
  `dummy/web-dummy/dist`.

The `just dummy web-build` recipe runs all three steps in order. The preview
script serves `dummy/web-dummy/dist` under the `/sum-numbers-ai/` base path used
by the project registry.

## Review Checklist

Use this checklist when changing the product story:

1. The home page pitch explains the focused addition workflow.
2. The book examples name the same fields returned by `SumResponse`.
3. The Dioxus console and terminal client call `sum-numbers-ai-dummy` directly.
4. The generated book, llms text, sitemap, and static site remain aligned.
