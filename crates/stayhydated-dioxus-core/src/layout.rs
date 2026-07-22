use dioxus::prelude::*;

use crate::{DisplayText, Href};

#[component]
pub fn FullscreenDemoFrame(
    #[props(into)] src: Href,
    #[props(into)] title: DisplayText,
    #[props(default)] allowfullscreen: bool,
) -> Element {
    rsx! {
        div { class: "fullscreen-demo",
            iframe {
                class: "fullscreen-demo-frame",
                src: src.as_str(),
                title: title.as_str(),
                allowfullscreen,
            }
        }
    }
}
