# Rust API contract

The executable API is the `sum-numbers-ai-dummy` Rust crate. Construct a
`SumRequest`, optionally replace its route labels, and pass it to
`sum_with_request` to receive the local total and synthetic audit evidence.

The crate returns a `SumResponse` directly rather than a `Result`. It does not
perform network I/O or validate the endpoint and model strings.

## Run a request

The package is a workspace-local, non-published fixture. The following example
assumes `sum-numbers-ai-dummy` is already available as a Cargo dependency:

```rust,ignore
use sum_numbers_ai_dummy::{
    DEFAULT_ENDPOINT, DEFAULT_MODEL, SumRequest, sum_with_request,
};

let request = SumRequest::new([8, 13, 21]);
let response = sum_with_request(&request);

assert_eq!(response.sum, 42);
assert_eq!(response.model_result, "42");
assert!(response.verified);
assert_eq!(response.provider.endpoint, DEFAULT_ENDPOINT);
assert_eq!(response.provider.model, DEFAULT_MODEL);
```

`SumRequest::new` preserves operand order and applies these defaults:

| Value | Type | Default or behavior |
| --- | --- | --- |
| Operands | `Vec<i64>` | Collected from the supplied iterator without changing order |
| Endpoint | `String` | `https://api.sum-numbers-ai.invalid/v1/responses` |
| Model | `String` | `sum-numbers-ai/addition-router-2026-07` |

Use the `.endpoint(...)` and `.model(...)` builders to replace the route labels.
Read the stored values through `numbers()`, `endpoint_url()`, and `model_name()`.
These labels affect the request ID and synthetic metadata, but they do not cause
a provider call.

## Response fields

| Field | Type | Meaning |
| --- | --- | --- |
| `request_id` | `String` | Correlation ID derived from operands, endpoint, and model |
| `numbers` | `Vec<i64>` | Operands echoed in their original order |
| `sum` | `i128` | Locally accumulated total |
| `model_result` | `String` | Decimal string of the same local total |
| `verified` | `bool` | Always `true` because the local result string matches the local total |
| `provider` | `ProviderMetadata` | Route labels and synthetic latency and token values |
| `trace` | `Vec<TraceEvent>` | Five synthetic events in review order |

Treat `request_id` as a demo correlation value. It is deterministic for the same
request in the current implementation, but it is not an interoperability or
persistence key.

`ProviderMetadata` exposes:

| Field | Type | Current value |
| --- | --- | --- |
| `endpoint` | `String` | Request endpoint label |
| `model` | `String` | Request model label |
| `latency_ms` | `u16` | Synthetic value from `150` through `239` |
| `prompt_tokens` | `u16` | Synthetic value derived from operand count; `37` for three operands |
| `completion_tokens` | `u16` | Synthetic value `3` |

Each `TraceEvent` has a static `code` and a `message`. Successful responses use
this order:

1. `ai.endpoint.resolve`
2. `ai.transport.open`
3. `ai.prompt.contract`
4. `ai.model.dispatch`
5. `ai.response.verify`

## Public helpers and constants

| Item | Purpose |
| --- | --- |
| `sum(numbers)` | Creates a default request and returns its response |
| `sum_with_request(&request)` | Runs a request with explicit route labels |
| `numbers_from_entropy(entropy)` | Produces deterministic demonstration operands |
| `request_from_entropy(entropy)` | Wraps deterministic demonstration operands in a default request |
| `DEFAULT_ENDPOINT` | Default endpoint label |
| `DEFAULT_MODEL` | Default model label |
| `MAX_DEMO_INPUTS` | Shared three-input limit for interactive clients |

The three-input limit belongs to the demos, not `SumRequest`.
`SumRequest::new` does not enforce an operand count and accepts an empty
iterator; an empty request returns `0`. Some entropy-generated workloads also
contain more than three operands.

## Illustrative HTTP display

The Dioxus console renders an HTTP-style request for API review:

```http
POST /v1/sum
Content-Type: application/json

{
  "numbers": [8, 13, 21],
  "strategy": "llm-delegated",
  "verification": "local-cross-check",
  "endpoint": "https://api.sum-numbers-ai.invalid/v1/responses",
  "model": "sum-numbers-ai/addition-router-2026-07"
}
```

`POST /v1/sum`, `strategy`, and `verification` are display-only wire concepts;
they are not fields or routes exposed by the Rust crate. The console displays
this response shape:

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

For the same default request, the displayed trace is:

```text
ai.endpoint.resolve  Resolved AI sum endpoint https://api.sum-numbers-ai.invalid/v1/responses for request sum_11772416564322390563
ai.transport.open  attached provider request budget through ai-sum-gateway-edge-cache-04
ai.prompt.contract  Serialized 3 operands into strict JSON response contract sum.v1
ai.model.dispatch  Dispatched addition prompt to model sum-numbers-ai/addition-router-2026-07 with deterministic verifier attached
ai.response.verify  Parsed provider answer 42 and matched local guardrail
```

## Demo input behavior

Each client validates text before constructing a request. None of the clients
offers operand reordering.

| Client | Accepted workload | Invalid-input signal |
| --- | --- | --- |
| Dioxus | One to three editable `i64` values | JSON envelope with `invalid_number_input` and the positions to review |
| Terminal | One to three comma-separated `i64` values inside `[]` | Error line followed by the accepted command forms |
| Bevy UI | Exactly three editable `i64` values | `Review the three integer inputs` |
| GPUI | Exactly three editable `i64` values | `Review the three integer inputs` |

The terminal accepts `[1,2,3]`, `sum [1,2,3]`, and the `run [1,2,3]` alias. Use
`help` to display its `clap` command reference.
