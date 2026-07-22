use crate::{pages, site::constants::PROJECT};
use dioxus::cli_config;
use dioxus::prelude::*;
use stayhydated_dioxus::StayhydatedProjectPageMetadata;
use stayhydated_site::routing::{BaseHref, BasePath, Href, RoutePath};

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
    pub(crate) fn all() -> [Self; 4] {
        [
            Self::Home,
            Self::Demos,
            Self::DioxusDemo,
            Self::TerminalDemo,
        ]
    }

    const fn metadata(self) -> PageMetadata {
        match self {
            Self::Home => PageMetadata {
                title: "Home",
                description: "A production-shaped documentation and demo target for an AI-assisted sum API.",
            },
            Self::Demos => PageMetadata {
                title: "Demos",
                description: "Dioxus and terminal clients for inspecting the sum-numbers-ai API contract.",
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SiteRoute {
    pub(crate) page: PageKind,
}

impl SiteRoute {
    pub(crate) const fn new(page: PageKind) -> Self {
        Self { page }
    }

    pub(crate) fn path(self) -> Href {
        Href::new(app_route(self.page).to_string())
    }
}

pub(crate) fn all_routes() -> Vec<SiteRoute> {
    PageKind::all().into_iter().map(SiteRoute::new).collect()
}

pub(crate) fn app_base_href() -> BaseHref {
    let base_path = cli_config::base_path();
    let base_path = base_path.as_deref().map(BasePath::new);
    stayhydated_site::routing::base_href(base_path.as_ref())
}

pub(crate) fn book_href() -> Href {
    stayhydated_site::routing::href(&app_base_href(), &RoutePath::new("book"))
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

fn route_element(route: SiteRoute) -> Element {
    rsx! {
        StayhydatedProjectPageMetadata {
            project: PROJECT,
            page_title: route.page.title(),
            description: route.page.description(),
        }
        {pages::route_content(route)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_routes_use_app_route_paths() {
        assert_eq!(SiteRoute::new(PageKind::Home).path().as_str(), "/");
        assert_eq!(SiteRoute::new(PageKind::Demos).path().as_str(), "/demos/");
        assert_eq!(
            SiteRoute::new(PageKind::DioxusDemo).path().as_str(),
            "/demos/dioxus/"
        );
        assert_eq!(
            SiteRoute::new(PageKind::TerminalDemo).path().as_str(),
            "/demos/terminal/"
        );
    }
}

#[component]
fn HomeRoute() -> Element {
    route_element(SiteRoute::new(PageKind::Home))
}

#[component]
fn DemosRoute() -> Element {
    route_element(SiteRoute::new(PageKind::Demos))
}

#[component]
fn DioxusDemoRoute() -> Element {
    route_element(SiteRoute::new(PageKind::DioxusDemo))
}

#[component]
fn TerminalDemoRoute() -> Element {
    route_element(SiteRoute::new(PageKind::TerminalDemo))
}
