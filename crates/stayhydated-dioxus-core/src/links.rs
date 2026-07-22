use bon::Builder;
use dioxus::prelude::*;

use crate::{
    CssClass, DisplayText, Href, OptionalDisplayText,
    layout::{ButtonLink, ButtonVariant, HeaderNav, ProjectHeader},
    projects::ProjectIdentity,
};

#[component]
pub fn RouteLink<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    #[props(into)] class: CssClass,
    #[props(into)] label: DisplayText,
    #[props(default, into)] aria_label: OptionalDisplayText,
) -> Element {
    let class = class.into_string();
    let aria_label = aria_label.into_option().map(DisplayText::into_string);

    match target {
        NavigationTarget::Internal(route) if try_router().is_some() => {
            rsx! {
                Link {
                    class,
                    to: route,
                    aria_label,
                    "{label}"
                }
            }
        },
        NavigationTarget::Internal(route) => {
            let href = route.to_string();
            rsx! {
                a {
                    class,
                    href,
                    aria_label,
                    "{label}"
                }
            }
        },
        NavigationTarget::External(href) => {
            rsx! {
                a {
                    class,
                    href,
                    aria_label,
                    "{label}"
                }
            }
        },
    }
}

#[component]
pub fn NavLink<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    #[props(into)] label: DisplayText,
    #[props(default)] active: bool,
) -> Element {
    rsx! {
        RouteLink {
            target,
            class: if active {
                CssClass::new("header-nav-item is-active")
            } else {
                CssClass::new("header-nav-item")
            },
            label,
        }
    }
}

#[component]
pub fn ExternalNavLink(#[props(into)] href: Href, #[props(into)] label: DisplayText) -> Element {
    rsx! {
        a {
            class: "header-nav-item",
            href: href.as_str(),
            target: "_blank",
            rel: "noreferrer",
            "{label}"
        }
    }
}

#[component]
pub fn ExternalTextLink(
    #[props(into)] href: Href,
    #[props(into)] label: DisplayText,
    #[props(default, into)] class: CssClass,
) -> Element {
    let class = class.into_string();

    rsx! {
        a {
            class,
            href: href.as_str(),
            target: "_blank",
            rel: "noreferrer",
            "{label}"
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectNavItem {
    Home,
    Demos,
}

impl ProjectNavItem {
    pub const fn is_home(self) -> bool {
        matches!(self, Self::Home)
    }

    pub const fn is_demos(self) -> bool {
        matches!(self, Self::Demos)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectNavLabels {
    pub home: DisplayText,
    pub demos: DisplayText,
    pub book: DisplayText,
    pub docs: DisplayText,
    pub source: DisplayText,
}

impl ProjectNavLabels {
    pub fn new(
        home: impl Into<DisplayText>,
        demos: impl Into<DisplayText>,
        book: impl Into<DisplayText>,
        docs: impl Into<DisplayText>,
        source: impl Into<DisplayText>,
    ) -> Self {
        Self {
            home: home.into(),
            demos: demos.into(),
            book: book.into(),
            docs: docs.into(),
            source: source.into(),
        }
    }
}

#[derive(Builder, Clone, Debug, Eq, PartialEq)]
pub struct ProjectNavConfig<R> {
    pub project: ProjectIdentity,
    pub home: NavigationTarget<R>,
    pub demos: NavigationTarget<R>,
    pub book: Href,
    pub docs: Href,
    pub source: Href,
    pub labels: ProjectNavLabels,
    pub active: ProjectNavItem,
}

impl<R> ProjectNavConfig<R> {
    pub fn with_labels(mut self, labels: ProjectNavLabels) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_active(mut self, active: ProjectNavItem) -> Self {
        self.active = active;
        self
    }
}

#[component]
pub fn ProjectNav<R: Routable + Clone + PartialEq + 'static>(
    home: NavigationTarget<R>,
    demos: NavigationTarget<R>,
    #[props(into)] book: Href,
    #[props(into)] docs: Href,
    #[props(into)] source: Href,
    labels: ProjectNavLabels,
    active: ProjectNavItem,
) -> Element {
    rsx! {
        HeaderNav {
            NavLink::<R> {
                target: home,
                label: labels.home,
                active: active == ProjectNavItem::Home,
            }
            NavLink::<R> {
                target: demos,
                label: labels.demos,
                active: active == ProjectNavItem::Demos,
            }
            NavLink::<R> {
                target: NavigationTarget::External(book.into_string()),
                label: labels.book,
            }
            ExternalNavLink {
                href: docs,
                label: labels.docs,
            }
            ExternalNavLink {
                href: source,
                label: labels.source,
            }
        }
    }
}

#[component]
pub fn ProjectNavigationHeader<R: Routable + Clone + PartialEq + 'static>(
    nav: ProjectNavConfig<R>,
    children: Element,
) -> Element {
    rsx! {
        ProjectNavHeader::<R> {
            project: nav.project,
            home: nav.home,
            demos: nav.demos,
            book: nav.book,
            docs: nav.docs,
            source: nav.source,
            labels: nav.labels,
            active: nav.active,
            {children}
        }
    }
}

#[component]
pub fn ProjectNavHeader<R: Routable + Clone + PartialEq + 'static>(
    project: ProjectIdentity,
    home: NavigationTarget<R>,
    demos: NavigationTarget<R>,
    #[props(into)] book: Href,
    #[props(into)] docs: Href,
    #[props(into)] source: Href,
    labels: ProjectNavLabels,
    active: ProjectNavItem,
    children: Element,
) -> Element {
    rsx! {
        ProjectHeader {
            project,
            ProjectNav::<R> {
                home,
                demos,
                book,
                docs,
                source,
                labels,
                active,
            }
            {children}
        }
    }
}

#[component]
pub fn BackLink<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    #[props(into)] label: DisplayText,
) -> Element {
    rsx! {
        RouteLink {
            target,
            class: CssClass::new("back-pill"),
            label,
        }
    }
}

#[component]
pub fn ButtonRouteLink<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    #[props(into)] label: DisplayText,
    #[props(default)] variant: ButtonVariant,
) -> Element {
    let variant = variant.class();

    rsx! {
        RouteLink {
            target,
            class: CssClass::new(format!("button-link {variant}")),
            label,
        }
    }
}

#[component]
pub fn ProjectHeroActions<R: Routable + Clone + PartialEq + 'static>(
    #[props(into)] book: Href,
    #[props(into)] docs: Href,
    demos: NavigationTarget<R>,
    #[props(default = DisplayText::new("Read the book"), into)] book_label: DisplayText,
    #[props(default = DisplayText::new("Read the docs"), into)] docs_label: DisplayText,
    #[props(default = DisplayText::new("View demos"), into)] demos_label: DisplayText,
) -> Element {
    rsx! {
        ButtonLink {
            href: book,
            label: book_label,
        }
        ButtonLink {
            href: docs,
            label: docs_label,
            variant: ButtonVariant::Secondary,
        }
        ButtonRouteLink::<R> {
            target: demos,
            label: demos_label,
            variant: ButtonVariant::Secondary,
        }
    }
}
