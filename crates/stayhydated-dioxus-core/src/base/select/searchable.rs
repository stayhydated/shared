use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdCheck, icons::ld_icons::LdSearch};
use dioxus_primitives::combobox;

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
    let base = dioxus_primitives::dioxus_attributes::attributes!(div {
        class: "dx-select dx-select-searchable"
    });
    let attributes = dioxus_primitives::merge_attributes(vec![base, attributes]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        combobox::Combobox::<T> {
            value,
            default_value,
            on_value_change,
            disabled,
            default_open,
            default_query,
            attributes,
            div { class: "dx-select-search-field",
                combobox::ComboboxInput {
                    class: "dx-select-trigger dx-select-search-input",
                    aria_label: aria_label.clone(),
                    placeholder,
                }
                span { class: "dx-select-search-icon", aria_hidden: "true",
                    Icon { width: 18, height: 18, icon: LdSearch }
                }
            }
            combobox::ComboboxList { class: "dx-select-list", aria_label,
                for (index, item) in items.into_iter().enumerate() {
                    combobox::ComboboxOption::<T> {
                        key: "{index}",
                        class: "dx-select-option",
                        index,
                        value: item.value,
                        text_value: item.label.clone(),
                        disabled: item.disabled,
                        "{item.label}"
                        combobox::ComboboxItemIndicator {
                            Icon {
                                class: "dx-select-check-icon".to_owned(),
                                width: 16,
                                height: 16,
                                icon: LdCheck,
                            }
                        }
                    }
                }
                combobox::ComboboxEmpty {
                    class: "dx-select-empty",
                    aria_live: "polite",
                    "{empty_message}"
                }
            }
        }
    }
}
