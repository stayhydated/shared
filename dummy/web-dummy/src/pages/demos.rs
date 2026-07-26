use dioxus::prelude::*;
use stayhydated_dioxus::{
    DemoCard, DemoCardAccent, NavigationTarget, StayhydatedProjectPortalShell,
    page_entry_reveal_style,
};

use crate::site::{
    constants::{PROJECT, VERSION},
    routing::{AppRoute, PageKind},
};

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
                    DemoCard::<AppRoute> {
                        target: NavigationTarget::Internal(
                            crate::site::routing::app_route(PageKind::DioxusDemo),
                        ),
                        accent: DemoCardAccent::Red,
                        title: "Dioxus",
                        shader_id: "dioxus-demo-card-shader",
                        time_offset: 0.0,
                    }
                    DemoCard::<AppRoute> {
                        target: NavigationTarget::Internal(
                            crate::site::routing::app_route(PageKind::TerminalDemo),
                        ),
                        accent: DemoCardAccent::Yellow,
                        title: "Terminal",
                        shader_id: "terminal-demo-card-shader",
                        time_offset: 13.0,
                    }
                    DemoCard::<AppRoute> {
                        target: NavigationTarget::External(
                            crate::site::routing::static_demo_href("bevy-demo").into_string(),
                        ),
                        accent: DemoCardAccent::Green,
                        title: "Bevy UI",
                        shader_id: "bevy-demo-card-shader",
                        time_offset: 26.0,
                    }
                    DemoCard::<AppRoute> {
                        target: NavigationTarget::External(
                            crate::site::routing::static_demo_href("gpui-demo").into_string(),
                        ),
                        accent: DemoCardAccent::Cyan,
                        title: "GPUI + gpui-component",
                        shader_id: "gpui-demo-card-shader",
                        time_offset: 39.0,
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
        assert_eq!(
            html.matches("class=\"demo-card demo-card-accent-").count(),
            4
        );
        assert!(html.contains("demo-card-accent-red"));
        assert!(html.contains("demo-card-accent-yellow"));
        assert!(html.contains("demo-card-accent-green"));
        assert!(html.contains("demo-card-accent-cyan"));
        assert_eq!(html.matches("class=\"demo-card-title\"").count(), 4);
        assert_eq!(html.matches("class=\"demo-card-tint\"").count(), 4);
        assert_eq!(
            html.matches("data-shader-background=\"loading\"").count(),
            4
        );
        assert!(html.contains("id=\"dioxus-demo-card-shader\""));
        assert!(html.contains("id=\"terminal-demo-card-shader\""));
        assert!(html.contains("id=\"bevy-demo-card-shader\""));
        assert!(html.contains("id=\"gpui-demo-card-shader\""));
        assert!(html.contains("Dioxus"));
        assert!(html.contains("Terminal"));
        assert!(html.contains("Bevy UI"));
        assert!(html.contains("GPUI + gpui-component"));
        assert!(html.contains("href=\"/bevy-demo/\""));
        assert!(html.contains("href=\"/gpui-demo/\""));
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
