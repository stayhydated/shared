use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};

pub const DEFAULT_ENDPOINT: &str = "https://api.sum-numbers-ai.invalid/v1/responses";
pub const DEFAULT_MODEL: &str = "sum-numbers-ai/addition-router-2026-07";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SumRequest {
    numbers: Vec<i64>,
    endpoint: String,
    model: String,
}

impl SumRequest {
    pub fn new(numbers: impl IntoIterator<Item = i64>) -> Self {
        Self {
            numbers: numbers.into_iter().collect(),
            endpoint: DEFAULT_ENDPOINT.to_owned(),
            model: DEFAULT_MODEL.to_owned(),
        }
    }

    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn numbers(&self) -> &[i64] {
        &self.numbers
    }

    pub fn endpoint_url(&self) -> &str {
        &self.endpoint
    }

    pub fn model_name(&self) -> &str {
        &self.model
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SumResponse {
    pub request_id: String,
    pub numbers: Vec<i64>,
    pub sum: i128,
    pub model_result: String,
    pub verified: bool,
    pub provider: ProviderMetadata,
    pub trace: Vec<TraceEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderMetadata {
    pub endpoint: String,
    pub model: String,
    pub latency_ms: u16,
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceEvent {
    pub code: &'static str,
    pub message: String,
}

pub fn sum(numbers: impl IntoIterator<Item = i64>) -> SumResponse {
    sum_with_request(&SumRequest::new(numbers))
}

pub fn request_from_entropy(entropy: u64) -> SumRequest {
    SumRequest::new(numbers_from_entropy(entropy))
}

pub fn numbers_from_entropy(entropy: u64) -> Vec<i64> {
    let offset = (entropy & 0x0fff) as i64;
    let wide_offset = ((entropy >> 12) & 0xffff) as i64;

    match entropy % 5 {
        0 => vec![8, 13, 21],
        1 => vec![i64::MAX - offset, wide_offset, 34],
        2 => vec![i64::MIN + offset, -wide_offset, -55],
        3 => vec![i64::MAX - offset, i64::MIN + wide_offset, 987_654_321],
        _ => vec![144, -89, 377, -233, 610],
    }
}

pub fn sum_with_request(request: &SumRequest) -> SumResponse {
    let request_id = request_id(request);
    let local_sum = request
        .numbers
        .iter()
        .copied()
        .map(i128::from)
        .sum::<i128>();
    let mut trace = Vec::new();

    record(
        &mut trace,
        "ai.endpoint.resolve",
        format!(
            "Resolved AI sum endpoint {} for request {request_id}",
            request.endpoint_url()
        ),
    );
    record(&mut trace, "ai.transport.open", transport_message(request));
    record(
        &mut trace,
        "ai.prompt.contract",
        format!(
            "Serialized {} operands into strict JSON response contract sum.v1",
            request.numbers.len()
        ),
    );
    record(
        &mut trace,
        "ai.model.dispatch",
        format!(
            "Dispatched addition prompt to model {} with deterministic verifier attached",
            request.model_name()
        ),
    );
    record(
        &mut trace,
        "ai.response.verify",
        format!("Parsed provider answer {local_sum} and matched local guardrail"),
    );

    let provider = ProviderMetadata {
        endpoint: request.endpoint.clone(),
        model: request.model.clone(),
        latency_ms: latency_ms(request),
        prompt_tokens: prompt_tokens(request),
        completion_tokens: 3,
    };

    SumResponse {
        request_id,
        numbers: request.numbers.clone(),
        sum: local_sum,
        model_result: local_sum.to_string(),
        verified: true,
        provider,
        trace,
    }
}

fn record(trace: &mut Vec<TraceEvent>, code: &'static str, message: String) {
    tracing::info!(target: "sum_numbers_ai_dummy", operation = code, "{message}");
    trace.push(TraceEvent { code, message });
}

fn request_id(request: &SumRequest) -> String {
    format!("sum_{}", request_hash(request))
}

fn request_hash(request: &SumRequest) -> u64 {
    let mut hasher = DefaultHasher::new();
    request.numbers.hash(&mut hasher);
    request.endpoint.hash(&mut hasher);
    request.model.hash(&mut hasher);
    hasher.finish()
}

fn transport_message(request: &SumRequest) -> String {
    const POOLS: [&str; 4] = [
        "ai-sum-gateway-us-east-01",
        "ai-sum-gateway-us-west-02",
        "ai-sum-gateway-eu-central-01",
        "ai-sum-gateway-edge-cache-04",
    ];
    const MODES: [&str; 4] = [
        "opened warm TLS channel",
        "reused HTTP/2 stream from pool",
        "attached provider request budget",
        "negotiated structured-output capability",
    ];

    let hash = request_hash(request);
    let pool = POOLS[(hash as usize) % POOLS.len()];
    let mode = MODES[((hash >> 8) as usize) % MODES.len()];

    format!("{mode} through {pool}")
}

fn latency_ms(request: &SumRequest) -> u16 {
    150 + (request_hash(request) % 90) as u16
}

fn prompt_tokens(request: &SumRequest) -> u16 {
    28 + request.numbers.len() as u16 * 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_numbers_locally_after_fake_ai_round_trip() {
        let response = sum([8, 13, 21]);

        assert_eq!(response.sum, 42);
        assert_eq!(response.model_result, "42");
        assert!(response.verified);
        assert_eq!(response.numbers, [8, 13, 21]);
    }

    #[test]
    fn sums_extreme_operands_without_i64_overflow() {
        let response = sum([i64::MAX, 1, 8]);

        assert_eq!(response.sum, i128::from(i64::MAX) + 9);
        assert_eq!(response.model_result, "9223372036854775816");
    }

    #[test]
    fn entropy_numbers_include_extreme_operands() {
        assert!(
            numbers_from_entropy(1)
                .into_iter()
                .any(|number| number > i64::MAX - 4096)
        );
        assert!(
            numbers_from_entropy(2)
                .into_iter()
                .any(|number| number < i64::MIN + 4096)
        );
    }

    #[test]
    fn trace_mentions_endpoint_and_provider_steps() {
        let request = SumRequest::new([1, 2, 3]).endpoint("https://example.test/ai/sum");
        let response = sum_with_request(&request);

        assert!(
            response.trace[0]
                .message
                .contains("https://example.test/ai/sum")
        );
        assert!(
            response
                .trace
                .iter()
                .any(|event| event.code == "ai.model.dispatch")
        );
    }
}
