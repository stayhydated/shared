use crate::pages;
use dioxus::cli_config;
use dioxus::prelude::*;
use stayhydated_dioxus::{Project, ProjectNavItem, StayhydatedProjectPageMetadata};
use stayhydated_site::routing::{BaseHref, BasePath, Href, RoutePath};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PageKind {
    Home,
    Demos,
    DioxusDemo,
    TerminalDemo,
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

    fn route(self) -> &'static str {
        match self {
            Self::Home => "",
            Self::Demos => "demos",
            Self::DioxusDemo => "demos/dioxus",
            Self::TerminalDemo => "demos/terminal",
        }
    }

    pub(crate) const fn project_nav_item(self) -> ProjectNavItem {
        match self {
            Self::Home => ProjectNavItem::Home,
            Self::Demos | Self::DioxusDemo | Self::TerminalDemo => ProjectNavItem::Demos,
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Demos => "Demos",
            Self::DioxusDemo => "Dioxus Demo",
            Self::TerminalDemo => "Terminal Demo",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Home => {
                "A production-shaped documentation and demo target for an AI-assisted sum API."
            },
            Self::Demos => {
                "Dioxus and terminal clients for inspecting the sum-numbers-ai API contract."
            },
            Self::DioxusDemo => {
                "A Dioxus console for request, response, and trace review in sum-numbers-ai."
            },
            Self::TerminalDemo => {
                "A Ratzilla operator CLI with a clap parser for sum-numbers-ai workloads."
            },
        }
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
        stayhydated_site::routing::href(&BaseHref::root(), &relative_path(self.page))
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

pub(crate) fn page_href(page: PageKind) -> Href {
    stayhydated_site::routing::href(&app_base_href(), &relative_path(page))
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

fn relative_path(page: PageKind) -> RoutePath {
    RoutePath::new(page.route())
}

fn route_element(route: SiteRoute) -> Element {
    rsx! {
        StayhydatedProjectPageMetadata {
            project: Project::SumNumbersAi,
            page_title: route.page.title(),
            description: route.page.description(),
        }
        {pages::route_content(route)}
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
