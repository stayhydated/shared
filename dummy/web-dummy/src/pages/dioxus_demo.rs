use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdPlus, LdRotateCcw, LdTrash2},
};
use stayhydated_dioxus::{CodeBlock, NavigationTarget, StayhydatedProjectPortalShell};
use sum_numbers_ai_dummy::{MAX_DEMO_INPUTS, SumRequest, SumResponse, sum_with_request};

use crate::site::{
    constants::{PROJECT, VERSION},
    routing::PageKind,
};

#[component]
pub(crate) fn DioxusDemoPage() -> Element {
    let mut numbers = use_signal(default_number_inputs);
    let input_values = numbers();
    let can_add = input_values.len() < MAX_DEMO_INPUTS;
    let parsed_numbers = parse_number_inputs(&input_values);
    let response = parsed_numbers
        .as_ref()
        .map(|numbers| sum_with_request(&SumRequest::new(numbers.clone())))
        .ok();
    let request_code = match &response {
        Some(response) => request_example(response),
        None => input_error_example(&parsed_numbers),
    };
    let response_code = match &response {
        Some(response) => response_example(response),
        None => input_error_example(&parsed_numbers),
    };
    let trace_code = response
        .as_ref()
        .map(trace_example)
        .unwrap_or_else(|| input_error_example(&parsed_numbers));
    let input_rows = input_values
        .iter()
        .map(|input| {
            let id = input.id;
            rsx! {
                NumberInputRow {
                    key: "{id}",
                    id,
                    numbers,
                }
            }
        })
        .collect::<Vec<_>>();
    let result_summary = response_summary(response.as_ref(), &parsed_numbers);
    rsx! {
        StayhydatedProjectPortalShell {
            project: PROJECT,
            version: VERSION,
            home: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Home)),
            div { class: "demo-page sum-console-demo",
                section { class: "sum-demo-workbench", aria_label: "Sum console",
                    div { class: "sum-number-editor",
                        div { class: "sum-number-toolbar",
                            button {
                                class: "sum-action-button",
                                r#type: "button",
                                disabled: !can_add,
                                title: if can_add {
                                    "Add a number"
                                } else {
                                    "A maximum of three inputs is supported"
                                },
                                onclick: move |_| {
                                    add_number_input(&mut numbers);
                                },
                                Icon {
                                    class: "sum-button-icon".to_string(),
                                    width: 17,
                                    height: 17,
                                    icon: LdPlus,
                                }
                            }
                            button {
                                class: "sum-action-button",
                                r#type: "button",
                                onclick: move |_| {
                                    reset_number_inputs(&mut numbers);
                                },
                                Icon {
                                    class: "sum-button-icon".to_string(),
                                    width: 17,
                                    height: 17,
                                    icon: LdRotateCcw,
                                }
                                "Reset"
                            }
                        }
                        div {
                            class: "sum-number-list",
                            aria_label: "Number inputs",
                            {input_rows.into_iter()}
                        }
                    }
                    div { class: "sum-result-panel",
                        div { class: "sum-result-metric",
                            span { "Total" }
                            strong { "{result_summary.total}" }
                        }
                        div { class: "sum-result-metric",
                            span { "Operands" }
                            strong { "{result_summary.operands}" }
                        }
                        div { class: "sum-result-metric",
                            span { "Verification" }
                            strong { "{result_summary.verification}" }
                        }
                        p { class: "{result_summary.class_name}", "{result_summary.detail}" }
                    }
                }
                section { class: "sum-code-grid", aria_label: "Request",
                    CodeBlock {
                        code: request_code,
                    }
                }
                section { class: "sum-code-grid", aria_label: "Response",
                    CodeBlock {
                        code: response_code,
                    }
                }
                section { class: "sum-code-grid", aria_label: "Trace",
                    CodeBlock {
                        code: trace_code,
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NumberInput {
    id: u64,
    value: String,
}

impl NumberInput {
    fn new(id: u64, value: impl Into<String>) -> Self {
        Self {
            id,
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NumberInputError {
    invalid_positions: Vec<usize>,
}

impl NumberInputError {
    fn summary(&self) -> String {
        format!(
            "Review input {}",
            self.invalid_positions
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResponseSummary {
    total: String,
    operands: String,
    verification: String,
    detail: String,
    class_name: &'static str,
}

#[component]
fn NumberInputRow(id: u64, mut numbers: Signal<Vec<NumberInput>>) -> Element {
    let snapshot = numbers();
    let Some(index) = snapshot.iter().position(|input| input.id == id) else {
        return rsx! {};
    };
    let input = snapshot[index].clone();
    let position = index + 1;
    let can_remove = snapshot.len() > 1;

    rsx! {
        div { class: "sum-number-row",
            span { class: "sum-number-position", "{position}" }
            input {
                class: "sum-number-input",
                r#type: "number",
                value: "{input.value}",
                "aria-label": "Number {position}",
                oninput: move |event| {
                    set_number_value(&mut numbers, id, event.value());
                },
            }
            div { class: "sum-number-actions",
                button {
                    class: "sum-icon-button",
                    r#type: "button",
                    disabled: !can_remove,
                    title: "Remove number {position}",
                    "aria-label": "Remove number {position}",
                    onclick: move |_| {
                        remove_number_input(&mut numbers, id);
                    },
                    Icon {
                        class: "sum-button-icon".to_string(),
                        width: 16,
                        height: 16,
                        icon: LdTrash2,
                    }
                }
            }
        }
    }
}

fn default_number_inputs() -> Vec<NumberInput> {
    [8, 13, 21]
        .into_iter()
        .enumerate()
        .map(|(index, value)| NumberInput::new(index as u64, value.to_string()))
        .collect()
}

fn next_number_input(inputs: &[NumberInput]) -> Option<NumberInput> {
    if inputs.len() >= MAX_DEMO_INPUTS {
        return None;
    }

    let next_id = inputs
        .iter()
        .map(|input| input.id)
        .max()
        .unwrap_or_default()
        + 1;
    Some(NumberInput::new(next_id, "0"))
}

fn add_number_input(numbers: &mut Signal<Vec<NumberInput>>) {
    let next = next_number_input(&numbers.read());
    if let Some(next) = next {
        numbers.write().push(next);
    }
}

fn reset_number_inputs(numbers: &mut Signal<Vec<NumberInput>>) {
    numbers.set(default_number_inputs());
}

fn set_number_value(numbers: &mut Signal<Vec<NumberInput>>, id: u64, value: String) {
    if let Some(input) = numbers.write().iter_mut().find(|input| input.id == id) {
        input.value = value;
    }
}

fn remove_number_input(numbers: &mut Signal<Vec<NumberInput>>, id: u64) {
    let mut inputs = numbers.write();
    if inputs.len() <= 1 {
        return;
    }
    inputs.retain(|input| input.id != id);
}

fn parse_number_inputs(inputs: &[NumberInput]) -> Result<Vec<i64>, NumberInputError> {
    let mut numbers = Vec::with_capacity(inputs.len());
    let mut invalid_positions = Vec::new();

    for (index, input) in inputs.iter().enumerate() {
        let trimmed = input.value.trim();
        match trimmed.parse::<i64>() {
            Ok(number) => numbers.push(number),
            Err(_) => invalid_positions.push(index + 1),
        }
    }

    if invalid_positions.is_empty() {
        Ok(numbers)
    } else {
        Err(NumberInputError { invalid_positions })
    }
}

fn response_summary(
    response: Option<&SumResponse>,
    parsed_numbers: &Result<Vec<i64>, NumberInputError>,
) -> ResponseSummary {
    match response {
        Some(response) => ResponseSummary {
            total: response.sum.to_string(),
            operands: response.numbers.len().to_string(),
            verification: if response.verified {
                "Matched".to_owned()
            } else {
                "Review".to_owned()
            },
            detail: format!(
                "{} ms through {}",
                response.provider.latency_ms, response.provider.model
            ),
            class_name: "sum-result-detail",
        },
        None => {
            let detail = parsed_numbers
                .as_ref()
                .err()
                .map(NumberInputError::summary)
                .unwrap_or_else(|| "Review input".to_owned());
            ResponseSummary {
                total: "Pending".to_owned(),
                operands: "0".to_owned(),
                verification: "Review".to_owned(),
                detail,
                class_name: "sum-result-detail is-error",
            }
        },
    }
}

fn input_error_example(parsed_numbers: &Result<Vec<i64>, NumberInputError>) -> String {
    let message = parsed_numbers
        .as_ref()
        .err()
        .map(NumberInputError::summary)
        .unwrap_or_else(|| "Review input".to_owned());

    format!(
        r#"{{
  "error": {{
    "code": "invalid_number_input",
    "message": "{}"
  }}
}}"#,
        message
    )
}

fn request_example(response: &SumResponse) -> String {
    format!(
        r#"POST /v1/sum
Content-Type: application/json

{{
  "numbers": [{}],
  "strategy": "llm-delegated",
  "verification": "local-cross-check",
  "endpoint": "{}",
  "model": "{}"
}}"#,
        response
            .numbers
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        response.provider.endpoint,
        response.provider.model
    )
}

fn response_example(response: &SumResponse) -> String {
    format!(
        r#"{{
  "request_id": "{}",
  "sum": {},
  "model_result": "{}",
  "verified": {},
  "latency_ms": {},
  "usage": {{
    "prompt_tokens": {},
    "completion_tokens": {}
  }}
}}"#,
        response.request_id,
        response.sum,
        response.model_result,
        response.verified,
        response.provider.latency_ms,
        response.provider.prompt_tokens,
        response.provider.completion_tokens,
    )
}

fn trace_example(response: &SumResponse) -> String {
    response
        .trace
        .iter()
        .map(|event| format!("{}  {}", event.code, event.message))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dioxus_demo_renders_only_the_console() {
        let html = dioxus::ssr::render_element(rsx! { DioxusDemoPage {} });

        assert!(html.contains("demo-page sum-console-demo"));
        assert!(html.contains("sum-demo-workbench"));
        assert_eq!(html.matches("class=\"code-sample\"").count(), 3);
        assert!(html.contains("class=\"project-portal\""));
        assert!(html.contains("portal-header"));
        assert!(html.contains("portal-skills-copy"));
        assert!(!html.contains("project-portal is-root"));
        assert!(!html.contains("page-header"));
        assert!(!html.contains("page-title-band"));
        assert!(!html.contains("project-surface-header"));
        assert!(!html.contains("site-footer"));
    }

    #[test]
    fn number_inputs_stop_at_the_shared_demo_limit_without_reordering() {
        let inputs = default_number_inputs();
        let html = dioxus::ssr::render_element(rsx! { DioxusDemoPage {} });

        assert_eq!(inputs.len(), MAX_DEMO_INPUTS);
        assert_eq!(next_number_input(&inputs), None);
        assert!(!html.contains("Move number"));
        assert!(!html.contains("data-drag"));
    }
}
