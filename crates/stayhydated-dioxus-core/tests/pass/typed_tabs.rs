use dioxus::prelude::*;
use stayhydated_dioxus_core::{TabContent, TabList, TabTrigger, Tabs, TabsOrientation};

fn app() -> Element {
    rsx! {
        Tabs {
            default_value: "overview",
            orientation: TabsOrientation::Horizontal,
            TabList {
                TabTrigger {
                    value: "overview",
                    index: 0usize,
                    extra_class: "is-first",
                    "Overview"
                }
            }
            TabContent {
                value: "overview",
                index: 0usize,
                "Overview content"
            }
        }
    }
}

fn main() {
    let _ = app;
}
