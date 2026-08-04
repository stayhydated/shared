# Scenario coverage

Use the fixture to check that shared site behavior remains consistent across
frameworks and generated documentation. Each surface intentionally presents the
same small request in a different form.

| Surface | Scenario | Expected evidence |
| --- | --- | --- |
| Rust library | Sum ordered `i64` operands | An `i128` total, route labels, and synthetic trace data |
| Dioxus console | Edit up to three operands | Request, response, and trace panels update together |
| Ratzilla terminal | Enter a JSON-style integer list | Parsed operands and the local response are printed |
| Bevy UI | Edit three operand fields | The total updates after every field parses |
| GPUI | Edit or reset three operand fields | The total updates and reset restores `8 + 13 + 21` |
| Documentation pipeline | Build the book and LLM outputs | Generated pages describe the same fixture contract |

## Review boundary

Interpret provider names, latency, token counts, request IDs, and trace messages
as deterministic fixture data. The executable path performs local arithmetic
and opens no provider connection. Use separate integration evidence for network,
authentication, retry, model-quality, cost, or service-limit behavior.

A useful fixture review confirms that all clients agree on the result, the route
manifest includes the expected application and static paths, and generated
documentation comes from the current book source.
