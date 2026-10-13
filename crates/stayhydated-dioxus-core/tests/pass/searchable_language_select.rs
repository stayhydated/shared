use dioxus::prelude::*;
use stayhydated_dioxus_core::{SharedStyles, select};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Locale {
    En,
    Fr,
}

#[component]
fn App() -> Element {
    let mut locale = use_signal(|| Some(Locale::En));

    rsx! {
        SharedStyles {}
        select::SearchableSelect::<Locale> {
            items: vec![
                select::SelectItem::new(Locale::En, "English"),
                select::SelectItem::new(Locale::Fr, "Français"),
            ],
            aria_label: "Language",
            placeholder: "Search languages...",
            value: Some(locale.into()),
            on_value_change: move |value| locale.set(value),
        }
    }
}

fn main() {
    let _ = App;
}
