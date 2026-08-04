# Dummy Rust contract

The workspace-local `sum-numbers-ai-dummy` crate supplies deterministic data to
every fixture client. Construct a `SumRequest` and pass it to
`sum_with_request`, or call `sum` when the default route labels are sufficient.

```rust,ignore
use sum_numbers_ai_dummy::{SumRequest, sum_with_request};

let request = SumRequest::new([8, 13, 21]);
let response = sum_with_request(&request);

assert_eq!(response.numbers, [8, 13, 21]);
assert_eq!(response.sum, 42);
assert_eq!(response.model_result, "42");
assert!(response.verified);
```

## Request and response shape

`SumRequest::new` preserves the order of supplied `i64` operands. The optional
`.endpoint(...)` and `.model(...)` setters replace labels used in the request ID
and synthetic provider metadata.

| Response field | Fixture meaning |
| --- | --- |
| `request_id` | Correlation value derived from operands and route labels |
| `numbers` | Operands in their original order |
| `sum` | Local `i128` accumulation |
| `model_result` | Decimal string of the local total |
| `verified` | Local result and result string agree |
| `provider` | Endpoint, model, latency, and token fixture values |
| `trace` | Five ordered provider-style events |

The default endpoint is
`https://api.sum-numbers-ai.invalid/v1/responses`, and the default model is
`sum-numbers-ai/addition-router-2026-07`. Both are labels; the crate performs no
network operation.

## Fixture invariants

- Convert each `i64` operand to `i128` before accumulation.
- Preserve operand order in the response.
- Emit the endpoint, transport, prompt, model, and verification trace events in
  that order.
- Keep `MAX_DEMO_INPUTS` as the three-input limit for interactive clients.
- Allow the library contract to accept workloads outside the interactive limit,
  including an empty iterator, which sums to `0`.

`numbers_from_entropy` and `request_from_entropy` provide deterministic
workloads for generated examples. They include extreme and mixed-sign operands
so clients can exercise the `i128` result boundary.
