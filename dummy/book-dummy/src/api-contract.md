# API Contract

The API has one operation: sum an ordered list of integers through a
provider-shaped request and return the verified answer with evidence.

The crate exposes a Rust boundary. The web and terminal demos project that
boundary into buyer-facing request and response examples.

## Rust Boundary

```rust
use sum_numbers_ai_dummy::{SumRequest, sum_with_request};

let request = SumRequest::new([8, 13, 21])
    .endpoint("https://api.sum-numbers-ai.invalid/v1/responses")
    .model("sum-numbers-ai/addition-router-2026-07");

let response = sum_with_request(&request);

assert_eq!(response.sum, 42);
assert_eq!(response.model_result, "42");
assert!(response.verified);
```

`SumRequest` owns:

- `numbers`: ordered `i64` operands.
- `endpoint`: provider endpoint URL, defaulting to the configured endpoint.
- `model`: provider model name, defaulting to the configured addition router.

`SumResponse` returns:

- `request_id`: deterministic identifier for the request shape.
- `numbers`: normalized operands echoed for audit.
- `sum`: local `i128` total.
- `model_result`: provider-style answer string.
- `verified`: local guardrail result.
- `provider`: endpoint, model, latency, and token metadata.
- `trace`: ordered provider-style events.

## HTTP-Style Facade

The Dioxus console renders the Rust request as a documented wire facade:

```http
POST /v1/sum
Content-Type: application/json
```

```json
{
  "numbers": [8, 13, 21],
  "strategy": "llm-delegated",
  "verification": "local-cross-check",
  "endpoint": "https://api.sum-numbers-ai.invalid/v1/responses",
  "model": "sum-numbers-ai/addition-router-2026-07"
}
```

The response separates the answer from the provider metadata:

```json
{
  "request_id": "sum_11772416564322390563",
  "sum": 42,
  "model_result": "42",
  "verified": true,
  "latency_ms": 173,
  "usage": {
    "prompt_tokens": 37,
    "completion_tokens": 3
  }
}
```

Trace output stays line-oriented so both the web code block and the terminal can
render it directly:

```text
ai.endpoint.resolve  Resolved AI sum endpoint https://api.sum-numbers-ai.invalid/v1/responses for request sum_11772416564322390563
ai.transport.open  attached provider request budget through ai-sum-gateway-edge-cache-04
ai.prompt.contract  Serialized 3 operands into strict JSON response contract sum.v1
ai.model.dispatch  Dispatched addition prompt to model sum-numbers-ai/addition-router-2026-07 with deterministic verifier attached
ai.response.verify  Parsed provider answer 42 and matched local guardrail
```

## Caller Validation

The web and terminal clients validate caller input before constructing a
`SumRequest`. Invalid numeric input uses a small error envelope:

```json
{
  "error": {
    "code": "invalid_number_input",
    "message": "Review input 2"
  }
}
```

Provider-facing implementations should keep transport failures distinct from
caller validation failures:

```json
{
  "error": {
    "code": "provider_unavailable",
    "message": "The delegated sum request could not be completed.",
    "retryable": true
  }
}
```

## Contract Rules

1. Preserve operand order in the request and response.
2. Accumulate the local answer in `i128` so extreme `i64` operands remain safe to
   demonstrate.
3. Keep provider endpoint and model visible.
4. Return token and latency fields as first-class metadata.
5. Attach trace events in the order a reviewer should read them.
6. Use one local crate boundary for every demo client.
