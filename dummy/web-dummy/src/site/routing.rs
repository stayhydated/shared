use crate::{pages, site::constants::PROJECT};
use dioxus::prelude::*;
use stayhydated_dioxus::StayhydatedProjectPageMetadata;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PageKind {
    Home,
    Demos,
    DioxusDemo,
    TerminalDemo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PageMetadata {
    title: &'static str,
    description: &'static str,
}

impl PageKind {
    const fn metadata(self) -> PageMetadata {
        match self {
            Self::Home => PageMetadata {
                title: "Home",
                description: "A production-shaped documentation and demo target for an AI-assisted sum API.",
            },
            Self::Demos => PageMetadata {
                title: "Demos",
                description: "Dioxus, terminal, Bevy UI, and GPUI clients for inspecting the sum-numbers-ai API contract.",
            },
            Self::DioxusDemo => PageMetadata {
                title: "Dioxus Demo",
                description: "A Dioxus console for request, response, and trace review in sum-numbers-ai.",
            },
            Self::TerminalDemo => PageMetadata {
                title: "Terminal Demo",
                description: "A Ratzilla operator CLI with a clap parser for sum-numbers-ai workloads.",
            },
        }
    }

    const fn title(self) -> &'static str {
        self.metadata().title
    }

    const fn description(self) -> &'static str {
        self.metadata().description
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Routable)]
#[rustfmt::skip]
pub(crate) enum AppRoute {
    #[route("/", HomeRoute)]
    Home {},
    #[route("/demos/", DemosRoute)]
    Demos {},
    #[route("/demos/dioxus/", DioxusDemoRoute)]
    DioxusDemo {},
    #[route("/demos/terminal/", TerminalDemoRoute)]
    TerminalDemo {},
}

pub(crate) fn app_route(page: PageKind) -> AppRoute {
    match page {
        PageKind::Home => AppRoute::Home {},
        PageKind::Demos => AppRoute::Demos {},
        PageKind::DioxusDemo => AppRoute::DioxusDemo {},
        PageKind::TerminalDemo => AppRoute::TerminalDemo {},
    }
}

fn route_element(page: PageKind) -> Element {
    rsx! {
        StayhydatedProjectPageMetadata {
            project: PROJECT,
            page_title: page.title(),
            description: page.description(),
        }
        {pages::route_content(page)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_kinds_map_to_their_app_routes() {
        assert_eq!(app_route(PageKind::Home).to_string(), "/");
        assert_eq!(app_route(PageKind::Demos).to_string(), "/demos/");
        assert_eq!(
            app_route(PageKind::DioxusDemo).to_string(),
            "/demos/dioxus/"
        );
        assert_eq!(
            app_route(PageKind::TerminalDemo).to_string(),
            "/demos/terminal/"
        );
    }
}

#[component]
fn HomeRoute() -> Element {
    route_element(PageKind::Home)
}

#[component]
fn DemosRoute() -> Element {
    route_element(PageKind::Demos)
}

#[component]
fn DioxusDemoRoute() -> Element {
    route_element(PageKind::DioxusDemo)
}

#[component]
fn TerminalDemoRoute() -> Element {
    route_element(PageKind::TerminalDemo)
}
