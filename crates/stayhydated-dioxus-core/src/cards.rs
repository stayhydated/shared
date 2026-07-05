use dioxus::prelude::*;

use crate::{CssClass, DisplayText, InlineStyle, OptionalDisplayText};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureCardItem {
    pub label: DisplayText,
    pub title: DisplayText,
    pub body: DisplayText,
}

impl FeatureCardItem {
    pub fn new(
        label: impl Into<DisplayText>,
        title: impl Into<DisplayText>,
        body: impl Into<DisplayText>,
    ) -> Self {
        Self {
            label: label.into(),
            title: title.into(),
            body: body.into(),
        }
    }
}

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
