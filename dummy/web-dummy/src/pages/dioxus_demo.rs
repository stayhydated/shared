use crate::components::{FooterPanel, PageHeader};
use crate::site::routing::PageKind;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdArrowDown, LdArrowUp, LdPlus, LdRotateCcw, LdTrash2},
};
use dioxus_primitives::drag_and_drop_list::{
    DragAndDropInstructions, DragAndDropList, DragAndDropListItems, DragAndDropLiveRegion,
    use_drag_and_drop_list_items,
};
use stayhydated_dioxus::{
    CodeBlock, PageTitleBand, ProjectPageShell, ProjectSurfaceSection, surface_reveal_style,
};
use sum_numbers_ai_dummy::{SumRequest, SumResponse, sum_with_request};

#[component]
pub(crate) fn DioxusDemoPage() -> Element {
    let mut numbers = use_signal(default_number_inputs);
    let mut list_version = use_signal(|| 0_u64);
    let input_values = numbers();
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
    let sortable_items = input_values
        .iter()
        .map(|input| {
            let id = input.id;
            rsx! {
                NumberInputRow {
                    key: "{id}",
                    id,
                    numbers,
                    list_version,
                }
            }
        })
        .collect::<Vec<_>>();
    let result_summary = response_summary(response.as_ref(), &parsed_numbers);
    let surface_style = surface_reveal_style();

    use_effect(move || {
        install_number_list_key_guard();
    });

    rsx! {
        ProjectPageShell {
            header: rsx!(PageHeader { current_page: PageKind::DioxusDemo }),
            footer: Some(rsx!(FooterPanel {})),
            PageTitleBand {
                label: "Dioxus console",
                title: "A buyer-facing console for the managed AI provider",
                lead: "Change the operand workload and watch the same request contract, response envelope, and trace evidence update from the local library.",
            }
            ProjectSurfaceSection {
                label: "Workload",
                title: "Customer request builder",
                lead: "The focused integer workload keeps validation, ordering, and repeatable request generation easy to inspect.",
                content_class: "sum-demo-workbench",
                style: surface_style,
                div { class: "sum-number-editor",
                    div { class: "sum-number-toolbar",
                        button {
                            class: "sum-action-button",
                            r#type: "button",
                            onclick: move |_| {
                                add_number_input(&mut numbers, &mut list_version);
                            },
                            Icon {
                                class: "sum-button-icon".to_string(),
                                width: 17,
                                height: 17,
                                icon: LdPlus,
                            }
                            "Add"
                        }
                        button {
                            class: "sum-action-button",
                            r#type: "button",
                            onclick: move |_| {
                                reset_number_inputs(&mut numbers, &mut list_version);
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
                    DragAndDropList {
                        key: "{list_version()}",
                        class: "sum-number-dnd",
                        aria_label: Some("Number inputs".to_owned()),
                        items: sortable_items,
                        SyncedNumberOrder {
                            numbers,
                        }
                        DragAndDropInstructions {}
                        DragAndDropListItems {
                            aria_label: "Number inputs".to_owned(),
                        }
                        DragAndDropLiveRegion {}
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
            ProjectSurfaceSection {
                label: "Request",
                title: "HTTP-style request facade",
                lead: "The preview projects the Rust SumRequest into the wire shape a managed API would document.",
                content_class: "sum-code-grid",
                CodeBlock {
                    code: request_code,
                }
            }
            ProjectSurfaceSection {
                label: "Response",
                title: "Verified response envelope",
                lead: "The answer is computed locally, then returned beside the provider result, usage, latency, and verification status.",
                content_class: "sum-code-grid",
                CodeBlock {
                    code: response_code,
                }
            }
            ProjectSurfaceSection {
                label: "Trace",
                title: "Provider evidence trail",
                lead: "Trace events make the AI boundary observable enough for docs, reviews, and demo parity checks.",
                content_class: "sum-code-grid",
                CodeBlock {
                    code: trace_code,
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
fn NumberInputRow(
    id: u64,
    mut numbers: Signal<Vec<NumberInput>>,
    mut list_version: Signal<u64>,
) -> Element {
    let snapshot = numbers();
    let Some(index) = snapshot.iter().position(|input| input.id == id) else {
        return rsx! {};
    };
    let input = snapshot[index].clone();
    let position = index + 1;
    let can_remove = snapshot.len() > 1;
    let can_move_up = index > 0;
    let can_move_down = index + 1 < snapshot.len();

    rsx! {
        div { class: "sum-number-row",
            span { class: "sum-number-position", "{position}" }
            input {
                class: "sum-number-input",
                r#type: "number",
                value: "{input.value}",
                "aria-label": "Number {position}",
                onkeydown: move |event: KeyboardEvent| {
                    stop_reorder_key_bubbling(event);
                },
                oninput: move |event| {
                    set_number_value(&mut numbers, id, event.value());
                },
            }
            div { class: "sum-number-actions",
                button {
                    class: "sum-icon-button",
                    r#type: "button",
                    disabled: !can_move_up,
                    title: "Move number {position} up",
                    "aria-label": "Move number {position} up",
                    onclick: move |_| {
                        move_number_input(&mut numbers, &mut list_version, id, MoveDirection::Up);
                    },
                    Icon {
                        class: "sum-button-icon".to_string(),
                        width: 16,
                        height: 16,
                        icon: LdArrowUp,
                    }
                }
                button {
                    class: "sum-icon-button",
                    r#type: "button",
                    disabled: !can_move_down,
                    title: "Move number {position} down",
                    "aria-label": "Move number {position} down",
                    onclick: move |_| {
                        move_number_input(&mut numbers, &mut list_version, id, MoveDirection::Down);
                    },
                    Icon {
                        class: "sum-button-icon".to_string(),
                        width: 16,
                        height: 16,
                        icon: LdArrowDown,
                    }
                }
                button {
                    class: "sum-icon-button",
                    r#type: "button",
                    disabled: !can_remove,
                    title: "Remove number {position}",
                    "aria-label": "Remove number {position}",
                    onclick: move |_| {
                        remove_number_input(&mut numbers, &mut list_version, id);
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

#[component]
fn SyncedNumberOrder(mut numbers: Signal<Vec<NumberInput>>) -> Element {
    let ordered_ids = use_drag_and_drop_list_items()
        .into_iter()
        .filter_map(|item| item.key.parse::<u64>().ok())
        .collect::<Vec<_>>();

    use_effect(move || {
        sync_number_order(&mut numbers, &ordered_ids);
    });

    rsx! {}
}

fn stop_reorder_key_bubbling(event: KeyboardEvent) {
    event.stop_propagation();
}

#[cfg(target_arch = "wasm32")]
fn install_number_list_key_guard() {
    let _ = dioxus::document::eval(
        r#"
        if (!globalThis.__sumNumbersDndKeyGuard) {
            globalThis.__sumNumbersDndKeyGuard = true;
            document.addEventListener("keydown", (event) => {
                const target = event.target;
                if (!(target instanceof Element)) {
                    return;
                }
                if (!target.closest(".sum-number-dnd")) {
                    return;
                }
                if (event.key !== "Backspace" && event.key !== "Delete") {
                    return;
                }

                const editable = target.matches("input, textarea, [contenteditable='true']");
                if (!editable) {
                    event.preventDefault();
                }
                event.stopPropagation();
            }, true);
        }
        "#,
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn install_number_list_key_guard() {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MoveDirection {
    Up,
    Down,
}

fn default_number_inputs() -> Vec<NumberInput> {
    [8, 13, 21]
        .into_iter()
        .enumerate()
        .map(|(index, value)| NumberInput::new(index as u64, value.to_string()))
        .collect()
}

fn add_number_input(numbers: &mut Signal<Vec<NumberInput>>, list_version: &mut Signal<u64>) {
    let next_id = numbers
        .read()
        .iter()
        .map(|input| input.id)
        .max()
        .unwrap_or_default()
        + 1;
    numbers.write().push(NumberInput::new(next_id, "0"));
    bump_list_version(list_version);
}

fn reset_number_inputs(numbers: &mut Signal<Vec<NumberInput>>, list_version: &mut Signal<u64>) {
    numbers.set(default_number_inputs());
    bump_list_version(list_version);
}

fn set_number_value(numbers: &mut Signal<Vec<NumberInput>>, id: u64, value: String) {
    if let Some(input) = numbers.write().iter_mut().find(|input| input.id == id) {
        input.value = value;
    }
}

fn move_number_input(
    numbers: &mut Signal<Vec<NumberInput>>,
    list_version: &mut Signal<u64>,
    id: u64,
    direction: MoveDirection,
) {
    let mut inputs = numbers.write();
    let Some(index) = inputs.iter().position(|input| input.id == id) else {
        return;
    };
    match direction {
        MoveDirection::Up if index > 0 => inputs.swap(index, index - 1),
        MoveDirection::Down if index + 1 < inputs.len() => inputs.swap(index, index + 1),
        _ => return,
    }
    drop(inputs);
    bump_list_version(list_version);
}

fn remove_number_input(
    numbers: &mut Signal<Vec<NumberInput>>,
    list_version: &mut Signal<u64>,
    id: u64,
) {
    let mut inputs = numbers.write();
    if inputs.len() <= 1 {
        return;
    }
    inputs.retain(|input| input.id != id);
    drop(inputs);
    bump_list_version(list_version);
}

fn bump_list_version(list_version: &mut Signal<u64>) {
    *list_version.write() += 1;
}

fn sync_number_order(numbers: &mut Signal<Vec<NumberInput>>, ordered_ids: &[u64]) {
    let current = numbers.read();
    if current.len() != ordered_ids.len() {
        return;
    }

    let current_ids = current.iter().map(|input| input.id).collect::<Vec<_>>();
    if current_ids == ordered_ids {
        return;
    }

    let mut reordered = Vec::with_capacity(current.len());
    for id in ordered_ids {
        let Some(input) = current.iter().find(|input| input.id == *id) else {
            return;
        };
        reordered.push(input.clone());
    }
    drop(current);

    numbers.set(reordered);
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
