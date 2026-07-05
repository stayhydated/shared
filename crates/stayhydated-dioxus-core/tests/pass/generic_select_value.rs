use dioxus::prelude::*;
use stayhydated_dioxus_core::select;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Locale {
    En,
    Fr,
}

fn app() -> Element {
    rsx! {
        select::Select::<Locale> {
            default_value: Locale::En,
            select::SelectTrigger {
                select::SelectValue {
                    placeholder: "Locale",
                }
            }
            select::SelectList {
                select::SelectOption::<Locale> {
                    index: 0usize,
                    value: Locale::En,
                    text_value: Some("English".to_string()),
                    "English"
                }
                select::SelectOption::<Locale> {
                    index: 1usize,
                    value: Locale::Fr,
                    text_value: Some("French".to_string()),
                    "French"
                }
            }
        }
    }
}

fn main() {
    let _ = app;
}
