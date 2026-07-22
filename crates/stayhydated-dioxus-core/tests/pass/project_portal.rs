use dioxus::prelude::*;
use stayhydated_dioxus_core::{PortalAccent, PortalDestination, ProjectPortal};

#[derive(Clone, Debug, PartialEq, Routable)]
enum AppRoute {
    #[route("/", Home)]
    Home {},
    #[route("/demos/", Demos)]
    Demos {},
}

#[component]
fn Home() -> Element {
    rsx! {}
}

#[component]
fn Demos() -> Element {
    rsx! {}
}

fn portal() -> Element {
    rsx! {
        ProjectPortal::<AppRoute> {
            project_name: "example-project",
            version: "0.1.0",
            tagline: "A focused project description",
            home: stayhydated_dioxus_core::NavigationTarget::Internal(AppRoute::Home {}),
            shader_id_prefix: "example-portal",
            destinations: vec![
                PortalDestination::href("/book/api.html", "Docs", PortalAccent::Yellow),
                PortalDestination::href("/book/", "Book", PortalAccent::Cyan),
                PortalDestination::route(AppRoute::Demos {}, "Demos", PortalAccent::Magenta),
                PortalDestination::href(
                    "https://github.com/example/project",
                    "Git",
                    PortalAccent::White,
                ),
            ],
        }
    }
}

fn main() {
    let _ = portal;
}
