use dioxus::prelude::*;

use crate::{CssClass, DisplayText};

#[component]
pub fn CodeBlock(
    #[props(into)] code: DisplayText,
    #[props(default = CssClass::new("code-sample"), into)] class: CssClass,
) -> Element {
    let class = class.into_string();
    rsx! {
        pre { class, code { "{code}" } }
    }
}
