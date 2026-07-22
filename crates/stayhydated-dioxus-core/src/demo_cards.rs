use dioxus::prelude::*;

use crate::{
    CssClass, DisplayText, Href, InlineStyle,
    layout::{GridColumns, GridSection},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoCard<R> {
    pub target: NavigationTarget<R>,
    pub label: DisplayText,
    pub title: DisplayText,
    pub body: DisplayText,
    pub action: DisplayText,
    pub body_class: Option<CssClass>,
}

impl<R> DemoCard<R> {
    pub fn new(
        target: NavigationTarget<R>,
        label: impl Into<DisplayText>,
        title: impl Into<DisplayText>,
        body: impl Into<DisplayText>,
        action: impl Into<DisplayText>,
    ) -> Self {
        Self {
            target,
            label: label.into(),
            title: title.into(),
            body: body.into(),
            action: action.into(),
            body_class: None,
        }
    }

    pub fn route(
        route: R,
        label: impl Into<DisplayText>,
        title: impl Into<DisplayText>,
        body: impl Into<DisplayText>,
        action: impl Into<DisplayText>,
    ) -> Self {
        Self::new(
            NavigationTarget::Internal(route),
            label,
            title,
            body,
            action,
        )
    }

    pub fn href(
        href: impl Into<Href>,
        label: impl Into<DisplayText>,
        title: impl Into<DisplayText>,
        body: impl Into<DisplayText>,
        action: impl Into<DisplayText>,
    ) -> Self {
        Self::new(
            NavigationTarget::External(href.into().into_string()),
            label,
            title,
            body,
            action,
        )
    }

    pub fn with_body_class(mut self, body_class: impl Into<CssClass>) -> Self {
        self.body_class = Some(body_class.into());
        self
    }
}

#[component]
pub fn DemoCardGrid<R: Routable + Clone + PartialEq + 'static>(
    cards: Vec<DemoCard<R>>,
    #[props(default)] columns: Option<GridColumns>,
    #[props(default, into)] extra_class: CssClass,
    #[props(default, into)] style: InlineStyle,
    #[props(default, into)] body_class: CssClass,
) -> Element {
    rsx! {
        GridSection {
            columns,
            extra_class,
            style,
            for card in cards {
                {
                    let key = card.target.to_string();

                    rsx! {
                        RouteCardLink::<R> {
                            key: "{key}",
                            target: card.target,
                            label: card.label,
                            title: card.title,
                            body: card.body,
                            body_class: card.body_class.unwrap_or_else(|| body_class.clone()),
                            action: card.action,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RouteCardLink<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    #[props(into)] label: DisplayText,
    #[props(into)] title: DisplayText,
    #[props(into)] body: DisplayText,
    #[props(into)] body_class: CssClass,
    #[props(into)] action: DisplayText,
) -> Element {
    let body_class = body_class.into_string();

    match target {
        NavigationTarget::Internal(route) if try_router().is_some() => {
            rsx! {
                Link {
                    class: "demo-card",
                    to: route,
                    div { class: "card-label", "{label}" }
                    h2 { "{title}" }
                    p { class: body_class, "{body}" }
                    span { class: "card-link", "{action}" }
                }
            }
        },
        NavigationTarget::Internal(route) => {
            let href = route.to_string();
            rsx! {
                a {
                    class: "demo-card",
                    href,
                    div { class: "card-label", "{label}" }
                    h2 { "{title}" }
                    p { class: body_class, "{body}" }
                    span { class: "card-link", "{action}" }
                }
            }
        },
        NavigationTarget::External(href) => {
            rsx! {
                a {
                    class: "demo-card",
                    href,
                    div { class: "card-label", "{label}" }
                    h2 { "{title}" }
                    p { class: body_class, "{body}" }
                    span { class: "card-link", "{action}" }
                }
            }
        },
    }
}
