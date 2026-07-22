use dioxus::prelude::*;
use stayhydated_dioxus::{
    NavigationTarget, StayhydatedProjectPortalShell, page_entry_reveal_style,
};

use crate::site::{
    constants::{PROJECT, VERSION},
    routing::{AppRoute, PageKind},
};

#[component]
fn DemoCardLink(route: AppRoute, title: &'static str) -> Element {
    let aria_label = format!("Open {title} demo");

    if try_router().is_some() {
        rsx! {
            Link {
                class: "demo-card",
                to: route,
                aria_label,
                h2 { class: "demo-card-title", "{title}" }
            }
        }
    } else {
        rsx! {
            a {
                class: "demo-card",
                href: route.to_string(),
                aria_label,
                h2 { class: "demo-card-title", "{title}" }
            }
        }
    }
}

#[component]
pub(crate) fn DemosPage() -> Element {
    let demos_style = page_entry_reveal_style().into_string();

    rsx! {
        StayhydatedProjectPortalShell {
            project: PROJECT,
            version: VERSION,
            home: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Home)),
            div { class: "demo-page demo-gallery",
                section {
                    class: "grid columns-2 demo-example-cards motion-reveal",
                    style: demos_style,
                    DemoCardLink {
                        route: crate::site::routing::app_route(PageKind::DioxusDemo),
                        title: "Dioxus",
                    }
                    DemoCardLink {
                        route: crate::site::routing::app_route(PageKind::TerminalDemo),
                        title: "Terminal",
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demos_page_renders_only_example_cards() {
        let html = dioxus::ssr::render_element(rsx! { DemosPage {} });

        assert!(html.contains("demo-page demo-gallery"));
        assert!(html.contains("demo-example-cards"));
        assert_eq!(html.matches("class=\"demo-card\"").count(), 2);
        assert_eq!(html.matches("class=\"demo-card-title\"").count(), 2);
        assert!(html.contains("Dioxus"));
        assert!(html.contains("Terminal"));
        assert!(!html.contains("Product console"));
        assert!(!html.contains("Operator CLI"));
        assert!(!html.contains("Open console"));
        assert!(!html.contains("Open CLI"));
        assert!(html.contains("class=\"project-portal\""));
        assert!(html.contains("portal-header"));
        assert!(html.contains("portal-skills-copy"));
        assert!(!html.contains("project-portal is-root"));
        assert!(!html.contains("page-header"));
        assert!(!html.contains("page-title-band"));
        assert!(!html.contains("site-footer"));
        assert!(!html.contains("Two clients, one AI contract"));
    }
}
