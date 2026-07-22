use dioxus::prelude::*;

use crate::{CssClass, DisplayText, InlineStyle, OptionalDisplayText};

#[component]
pub fn FeatureCard(
    #[props(into)] label: DisplayText,
    #[props(into)] title: DisplayText,
    #[props(into)] body: DisplayText,
    #[props(default, into)] style: InlineStyle,
) -> Element {
    let style = style.into_string();
    rsx! {
        article { class: "feature-card motion-reveal",
            style,
            span { class: "card-label", "{label}" }
            h3 { "{title}" }
            p { "{body}" }
        }
    }
}

#[component]
pub fn SectionHeader(
    #[props(default, into)] label: OptionalDisplayText,
    #[props(into)] title: DisplayText,
    #[props(default, into)] lead: OptionalDisplayText,
    #[props(default = CssClass::new("section-heading"), into)] class: CssClass,
) -> Element {
    let class = class.into_string();
    let label = label.into_option();
    let lead = lead.into_option();

    rsx! {
        div { class,
            if let Some(label) = label {
                span { class: "panel-label", "{label}" }
            }
            h2 { "{title}" }
            if let Some(lead) = lead {
                p { "{lead}" }
            }
        }
    }
}

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
