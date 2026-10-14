use std::rc::Rc;

use dioxus::core::{ElementId, Mutation, NoOpMutations};
use dioxus::html::{
    HasFileData, HasFormData, HasKeyboardData, ModifiersInteraction, input_data::keyboard_types,
};
use dioxus::prelude::*;
use stayhydated_dioxus_core::select::{SearchableSelect, SelectItem};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Locale {
    En,
    Fr,
    De,
}

#[component]
fn LanguageSelect() -> Element {
    let mut locale = use_signal(|| Some(Locale::En));

    rsx! {
        SearchableSelect::<Locale> {
            items: vec![
                SelectItem::new(Locale::En, "English"),
                SelectItem::new(Locale::Fr, "Français"),
                SelectItem { disabled: true, ..SelectItem::new(Locale::De, "Deutsch") },
            ],
            aria_label: "Language",
            placeholder: "Search languages...",
            empty_message: "No languages found",
            value: Some(locale.into()),
            on_value_change: move |value| locale.set(value),
        }
        output { "{locale():?}" }
    }
}

fn settle(dom: &mut VirtualDom) {
    // Effects register option labels and update popup visibility after rendering.
    for _ in 0..10 {
        dom.render_immediate(&mut NoOpMutations);
    }
}

fn language_select() -> (VirtualDom, ElementId) {
    select_app(LanguageSelect)
}

fn select_app(app: fn() -> Element) -> (VirtualDom, ElementId) {
    static EVENTS: std::sync::Once = std::sync::Once::new();
    EVENTS.call_once(|| dioxus::html::set_event_converter(Box::new(TestEventConverter)));
    let mut dom = VirtualDom::new(app);
    let mutations = dom.rebuild_to_vec();
    let input = mutations
        .edits
        .iter()
        .find_map(|edit| match edit {
            Mutation::NewEventListener { name, id } if name == "input" => Some(*id),
            _ => None,
        })
        .expect("search input has an input handler");
    settle(&mut dom);
    (dom, input)
}

fn type_query(dom: &mut VirtualDom, input: ElementId, query: &str) {
    let data = dioxus::html::PlatformEventData::new(Box::new(query.to_owned()));
    dom.runtime()
        .handle_event("input", Event::new(Rc::new(data), true).into_any(), input);
    settle(dom);
}

fn press_key(dom: &mut VirtualDom, input: ElementId, key: Key) {
    let data = dioxus::html::PlatformEventData::new(Box::new(key));
    dom.runtime()
        .handle_event("keydown", Event::new(Rc::new(data), true).into_any(), input);
    settle(dom);
}

// Renderer event data drives the actual component handlers in the virtual DOM.
struct TestEventConverter;

macro_rules! event_converter {
    (enum Event {
        $(
            #[convert = $converter:ident]
            #[events = [$($events:tt)*]]
            $(#[raw = [$($raw:tt)*]])?
            $group:ident($data:ident),
        )*
    }) => {
        impl dioxus::html::HtmlEventConverter for TestEventConverter {
            $(
                fn $converter(&self, event: &dioxus::html::PlatformEventData) -> dioxus::html::$data {
                    event_converter!(@data $data, event)
                }
            )*
        }
    };
    (@data FormData, $event:ident) => {
        FormData::new(InputData($event.downcast::<String>().unwrap().clone()))
    };
    (@data KeyboardData, $event:ident) => {
        KeyboardData::new(KeyData($event.downcast::<Key>().unwrap().clone()))
    };
    (@data $data:ident, $event:ident) => {{
        let _ = $event;
        panic!("unsupported test event: {}", stringify!($data))
    }};
}

dioxus::html::with_html_event_groups!(event_converter);

struct InputData(String);

impl HasFileData for InputData {
    fn files(&self) -> Vec<dioxus::html::FileData> {
        Vec::new()
    }
}

impl HasFormData for InputData {
    fn value(&self) -> String {
        self.0.clone()
    }

    fn valid(&self) -> bool {
        true
    }

    fn values(&self) -> Vec<(String, dioxus::html::FormValue)> {
        Vec::new()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct KeyData(Key);

impl ModifiersInteraction for KeyData {
    fn modifiers(&self) -> keyboard_types::Modifiers {
        keyboard_types::Modifiers::empty()
    }
}

impl HasKeyboardData for KeyData {
    fn key(&self) -> Key {
        self.0.clone()
    }

    fn code(&self) -> keyboard_types::Code {
        keyboard_types::Code::Unidentified
    }

    fn location(&self) -> keyboard_types::Location {
        keyboard_types::Location::Standard
    }

    fn is_auto_repeating(&self) -> bool {
        false
    }

    fn is_composing(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[test]
fn search_filters_labels_and_reports_empty_results_without_changing_selection() {
    let (mut dom, input) = language_select();
    let initial = dioxus::ssr::render(&dom);
    assert!(initial.contains("role=\"combobox\""));
    assert!(initial.contains("aria-label=\"Language\""));
    assert!(initial.contains("placeholder=\"Search languages...\""));
    assert!(initial.contains("value=\"English\""));

    type_query(&mut dom, input, " fRaN ");
    let filtered = dioxus::ssr::render(&dom);
    assert!(filtered.contains("Français"));
    assert!(!filtered.contains("English"));
    assert!(!filtered.contains("Deutsch"));
    assert!(!filtered.contains("No languages found"));
    assert!(filtered.contains("Some(En)"));

    type_query(&mut dom, input, "no such language");
    let empty = dioxus::ssr::render(&dom);
    assert!(empty.contains("No languages found"));
    assert!(!empty.contains("role=\"option\""));
    assert!(empty.contains("Some(En)"));

    type_query(&mut dom, input, "");
    let cleared = dioxus::ssr::render(&dom);
    assert_eq!(cleared.matches("role=\"option\"").count(), 3);
    assert!(!cleared.contains("No languages found"));
}

#[test]
fn keyboard_selects_filtered_typed_value_and_reopening_clears_search() {
    let (mut dom, input) = language_select();
    type_query(&mut dom, input, "fran");
    press_key(&mut dom, input, Key::ArrowDown);
    let focused = dioxus::ssr::render(&dom);
    assert!(focused.contains("aria-activedescendant="));
    assert!(focused.contains("data-highlighted=true"), "{focused}");

    press_key(&mut dom, input, Key::Enter);
    let selected = dioxus::ssr::render(&dom);
    assert!(selected.contains("Some(Fr)"));
    assert!(selected.contains("value=\"Français\""));
    assert!(selected.contains("aria-expanded=false"));

    press_key(&mut dom, input, Key::ArrowDown);
    let reopened = dioxus::ssr::render(&dom);
    assert!(reopened.contains("value=\"\""));
    assert_eq!(reopened.matches("role=\"option\"").count(), 3);
}

#[test]
fn disabled_match_cannot_be_selected_and_escape_restores_previous_label() {
    let (mut dom, input) = language_select();
    type_query(&mut dom, input, "deutsch");
    let filtered = dioxus::ssr::render(&dom);
    assert!(filtered.contains("Deutsch"));
    assert!(filtered.contains("aria-disabled=true"), "{filtered}");

    press_key(&mut dom, input, Key::ArrowDown);
    press_key(&mut dom, input, Key::Enter);
    let unchanged = dioxus::ssr::render(&dom);
    assert!(unchanged.contains("Some(En)"));
    assert!(unchanged.contains("aria-expanded=true"));

    press_key(&mut dom, input, Key::Escape);
    let closed = dioxus::ssr::render(&dom);
    assert!(closed.contains("Some(En)"));
    assert!(closed.contains("value=\"English\""));
    assert!(closed.contains("aria-expanded=false"));
}

#[component]
fn ChangingItemsSelect() -> Element {
    let mut selected = use_signal(|| None::<Locale>);
    let items = if selected() == Some(Locale::Fr) {
        vec![SelectItem::new(Locale::Fr, "Français canadien")]
    } else {
        vec![
            SelectItem::new(Locale::En, "English"),
            SelectItem::new(Locale::Fr, "Français"),
        ]
    };
    rsx! {
        SearchableSelect::<Locale> {
            items, aria_label: "Language", value: Some(selected.into()),
            default_value: Locale::En,
            on_value_change: move |next| selected.set(next),
        }
        output { "{selected():?}" }
    }
}

#[test]
fn controlled_none_overrides_default_and_replacement_items_refresh_the_index_and_label() {
    let (mut dom, input) = select_app(ChangingItemsSelect);
    let initial = dioxus::ssr::render(&dom);
    assert!(initial.contains("value=\"\""));
    assert!(!initial.contains("role=\"option\""));
    type_query(&mut dom, input, "fran");
    press_key(&mut dom, input, Key::ArrowDown);
    press_key(&mut dom, input, Key::Enter);
    let selected = dioxus::ssr::render(&dom);
    assert!(selected.contains("Some(Fr)"));
    assert!(selected.contains("value=\"Français canadien\""));
    press_key(&mut dom, input, Key::ArrowDown);
    let reopened = dioxus::ssr::render(&dom);
    assert_eq!(reopened.matches("role=\"option\"").count(), 1);
    assert!(reopened.contains("Français canadien"));
    type_query(&mut dom, input, "english");
    assert!(dioxus::ssr::render(&dom).contains("No matches found"));
}

#[component]
fn LargeSelect() -> Element {
    rsx! {
        SearchableSelect::<usize> {
            items: (0..5_027).map(|index| SelectItem::new(index, format!("Area {index:04}"))).collect(),
            aria_label: "Area", default_value: 0,
        }
    }
}

#[test]
fn large_catalog_search_reaches_the_last_item_and_closed_popup_has_no_option_rows() {
    let (mut dom, input) = select_app(LargeSelect);
    assert!(!dioxus::ssr::render(&dom).contains("role=\"option\""));
    type_query(&mut dom, input, "area 5026");
    let filtered = dioxus::ssr::render(&dom);
    assert_eq!(filtered.matches("role=\"option\"").count(), 1);
    press_key(&mut dom, input, Key::End);
    press_key(&mut dom, input, Key::Enter);
    let selected = dioxus::ssr::render(&dom);
    assert!(selected.contains("value=\"Area 5026\""));
    assert!(!selected.contains("role=\"option\""));
}

#[test]
fn navigation_wraps_skips_disabled_options_and_handles_no_matches() {
    let (mut dom, input) = language_select();
    press_key(&mut dom, input, Key::ArrowUp);
    press_key(&mut dom, input, Key::Enter);
    assert!(dioxus::ssr::render(&dom).contains("Some(Fr)"));
    press_key(&mut dom, input, Key::ArrowDown);
    press_key(&mut dom, input, Key::ArrowUp);
    press_key(&mut dom, input, Key::Enter);
    assert!(dioxus::ssr::render(&dom).contains("Some(Fr)"));
    type_query(&mut dom, input, "no match");
    for key in [
        Key::ArrowDown,
        Key::ArrowUp,
        Key::Home,
        Key::End,
        Key::Enter,
    ] {
        press_key(&mut dom, input, key);
    }
    let empty = dioxus::ssr::render(&dom);
    assert!(empty.contains("Some(Fr)"));
    assert!(!empty.contains("aria-activedescendant="));
}
