use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdCheck, icons::ld_icons::LdSearch};
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::html::input_data::MouseButton;

/// A typed value and its display/search label in a [`SearchableSelect`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectItem<T> {
    /// Value supplied to the selection callback.
    pub value: T,
    /// Label displayed in the list and searched by the input.
    pub label: String,
    /// Keeps the option visible while preventing selection.
    pub disabled: bool,
}

impl<T> SelectItem<T> {
    /// Creates an enabled option.
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: false,
        }
    }
}

/// A single-value select whose text input filters option labels.
///
/// Search uses case-insensitive substring matching and ignores surrounding
/// whitespace. Arrow keys navigate matching enabled options, Enter selects,
/// and Escape closes the list. Closing restores the selected label; opening
/// the field again starts a fresh search.
#[component]
pub fn SearchableSelect<T: Clone + PartialEq + 'static>(
    items: Vec<SelectItem<T>>,
    /// Accessible name of the search input.
    aria_label: String,
    /// Controlled selection. `None` in the signal means no selection.
    #[props(default)]
    value: Option<ReadSignal<Option<T>>>,
    #[props(default)] default_value: Option<T>,
    #[props(default)] on_value_change: Callback<Option<T>>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default = "Search options...".to_owned())] placeholder: String,
    #[props(default = "No matches found".to_owned())] empty_message: String,
    /// Initial popup state for an uncontrolled select.
    #[props(default)]
    default_open: ReadSignal<bool>,
    /// Initial filter text for an uncontrolled select.
    #[props(default)]
    default_query: ReadSignal<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    // Cache normalized labels when the supplied data changes, rather than
    // registering and refiltering a reactive component for every option.
    let data = use_memo(use_reactive((&items,), |(items,)| SearchData::new(items)));
    let mut internal_value = use_signal(|| default_value);
    let selected = use_memo(use_reactive((&value,), move |(value,)| match value {
        Some(value) => value.cloned(),
        None => internal_value.cloned(),
    }));
    let selected_label = use_memo(move || {
        let selected = selected();
        data.read()
            .items
            .iter()
            .find(|item| Some(&item.value) == selected.as_ref())
            .map(|item| item.label.clone())
            .unwrap_or_default()
    });
    let mut open = use_signal(move || default_open.cloned());
    let mut query = use_signal(move || default_query.cloned());
    let mut active = use_signal(|| None::<usize>);
    let mut scroll_target = use_signal(|| None::<usize>);
    let mut pointer_start = use_signal(|| None::<(usize, f64, f64)>);
    let matches = use_memo(move || data.read().matching(&query()));
    let enabled = use_memo(move || {
        matches
            .read()
            .iter()
            .copied()
            .filter(|index| !data.read().items[*index].disabled)
            .collect::<Vec<_>>()
    });
    let id = use_hook(|| {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        format!("dx-search-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
    });
    let list_id = format!("{id}-list");
    let expanded = open() && !disabled();
    let active_index = active().filter(|index| enabled.read().contains(index));
    let active_id = expanded
        .then(|| active_index.map(|index| format!("{id}-{index}")))
        .flatten();
    let display = if expanded { query() } else { selected_label() };

    let close = use_callback(move |()| {
        open.set(false);
        active.set(None);
        scroll_target.set(None);
        pointer_start.set(None);
    });
    let commit = use_callback(move |index: usize| {
        if disabled() {
            return;
        }
        let next = data
            .read()
            .items
            .get(index)
            .filter(|item| !item.disabled)
            .map(|item| item.value.clone());
        if let Some(next) = next {
            internal_value.set(Some(next.clone()));
            on_value_change.call(Some(next));
            close.call(());
        }
    });
    use_effect(use_reactive((&disabled,), move |(disabled,)| {
        if disabled() {
            close.call(());
        }
    }));
    use_effect({
        let id = id.clone();
        move || {
            if !open() {
                return;
            }
            let Some(index) = scroll_target() else {
                return;
            };
            #[cfg(target_arch = "wasm32")]
            {
                let target_id = format!("{id}-{index}");
                spawn(async move {
                    let eval = document::eval(
                        "const id = await dioxus.recv(); document.getElementById(id)?.scrollIntoView({block: 'nearest'});",
                    );
                    let _ = eval.send(target_id);
                    let _ = eval.await;
                });
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (&id, index);
        }
    });

    let base = dioxus_primitives::dioxus_attributes::attributes!(div {
        class: "dx-select dx-select-searchable"
    });
    let attributes = dioxus_primitives::merge_attributes(vec![base, attributes]);
    let onkeydown = move |event: KeyboardEvent| {
        if disabled() {
            return;
        }
        let navigation = match event.key() {
            Key::ArrowDown => Some(Navigation::Next),
            Key::ArrowUp => Some(Navigation::Previous),
            Key::Home if open() => Some(Navigation::First),
            Key::End if open() => Some(Navigation::Last),
            Key::Enter if open() => {
                if let Some(index) = active().filter(|index| enabled.read().contains(index)) {
                    commit.call(index);
                }
                event.prevent_default();
                event.stop_propagation();
                return;
            },
            Key::Escape if open() => {
                close.call(());
                event.prevent_default();
                event.stop_propagation();
                return;
            },
            _ => None,
        };
        if let Some(navigation) = navigation {
            let next = if open() {
                navigate(&enabled.read(), active(), navigation)
            } else {
                query.set(String::new());
                let all_enabled = data
                    .read()
                    .items
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| (!item.disabled).then_some(index))
                    .collect::<Vec<_>>();
                navigate(&all_enabled, None, navigation)
            };
            open.set(true);
            active.set(next);
            scroll_target.set(next);
            event.prevent_default();
            event.stop_propagation();
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { "data-state": if expanded { "open" } else { "closed" }, "data-disabled": disabled(), ..attributes,
            div { class: "dx-select-search-field",
                input {
                    class: "dx-select-trigger dx-select-search-input",
                    r#type: "text", role: "combobox", aria_autocomplete: "list", aria_haspopup: "listbox",
                    aria_label, aria_expanded: expanded, aria_controls: list_id,
                    aria_activedescendant: active_id, value: display, placeholder,
                    autocomplete: "off", spellcheck: "false", disabled: disabled(),
                    onclick: move |_| { if !open() { query.set(String::new()); active.set(None); open.set(true); } },
                    oninput: move |event| {
                        let incoming = event.value();
                        let next = if open() { incoming } else {
                            incoming.strip_prefix(&selected_label()).map(str::to_owned).unwrap_or(incoming)
                        };
                        query.set(next); active.set(None); scroll_target.set(None); open.set(true);
                    },
                    onkeydown,
                    onblur: move |_| close.call(()),
                }
                span { class: "dx-select-search-icon", aria_hidden: "true",
                    Icon { width: 18, height: 18, icon: LdSearch }
                }
            }
            if expanded {
                div { class: "dx-select-list", id: list_id.clone(), role: "listbox", aria_label: aria_label.clone(), "data-state": "open",
                    onpointercancel: move |_| pointer_start.set(None),
                    for index in matches() {
                        {
                            let (label, item_disabled, is_selected) = {
                                let data = data.read();
                                let item = &data.items[index];
                                (item.label.clone(), item.disabled, selected().as_ref() == Some(&item.value))
                            };
                            rsx! {
                                div {
                                    key: "{index}", id: "{id}-{index}", class: "dx-select-option", role: "option",
                                    aria_selected: is_selected, aria_disabled: item_disabled,
                                    "data-highlighted": active_index == Some(index),
                                    "data-disabled": item_disabled, "data-selected": is_selected,
                                    onmouseenter: move |_| { if !item_disabled { active.set(Some(index)); } },
                                    onpointerdown: move |event| {
                                        if item_disabled || event.trigger_button() != Some(MouseButton::Primary) { return; }
                                        event.prevent_default();
                                        let p = event.client_coordinates();
                                        pointer_start.set(Some((index, p.x, p.y)));
                                    },
                                    onpointerup: move |event| {
                                        let Some((start_index, x, y)) = pointer_start.take() else { return; };
                                        if item_disabled || start_index != index || event.trigger_button() != Some(MouseButton::Primary) { return; }
                                        let p = event.client_coordinates();
                                        if event.pointer_type() != "touch" || (p.x-x).powi(2) + (p.y-y).powi(2) <= 25.0 {
                                            commit.call(index);
                                        }
                                    },
                                    "{label}"
                                    if is_selected { Icon { class: "dx-select-check-icon".to_owned(), width: 16, height: 16, icon: LdCheck } }
                                }
                            }
                        }
                    }
                    if matches.read().is_empty() {
                        div { class: "dx-select-empty", aria_live: "polite", "{empty_message}" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct SearchData<T> {
    items: Vec<SelectItem<T>>,
    normalized_labels: Vec<String>,
}

impl<T> SearchData<T> {
    fn new(items: Vec<SelectItem<T>>) -> Self {
        let normalized_labels = items.iter().map(|item| item.label.to_lowercase()).collect();
        Self {
            items,
            normalized_labels,
        }
    }

    fn matching(&self, query: &str) -> Vec<usize> {
        let query = query.trim().to_lowercase();
        self.normalized_labels
            .iter()
            .enumerate()
            .filter_map(|(index, label)| {
                (query.is_empty() || label.contains(&query)).then_some(index)
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
enum Navigation {
    Next,
    Previous,
    First,
    Last,
}

fn navigate(enabled: &[usize], active: Option<usize>, navigation: Navigation) -> Option<usize> {
    let position = active.and_then(|index| enabled.iter().position(|value| *value == index));
    match navigation {
        Navigation::First => enabled.first().copied(),
        Navigation::Last => enabled.last().copied(),
        Navigation::Next => enabled
            .get(position.map_or(0, |p| (p + 1) % enabled.len()))
            .copied(),
        Navigation::Previous => position.map_or_else(
            || enabled.last().copied(),
            |p| {
                enabled
                    .get((p + enabled.len() - 1) % enabled.len())
                    .copied()
            },
        ),
    }
}
