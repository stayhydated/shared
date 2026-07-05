use dioxus::prelude::*;

use crate::{
    CssClass, DisplayText, Href, InlineStyle,
    layout::{ButtonLink, ButtonVariant, GridColumns, GridSection, HeaderNav, ProjectHeader},
    projects::ProjectOption,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkTarget<R> {
    Route(R),
    Href(Href),
}

impl<R> LinkTarget<R> {
    pub fn route(route: R) -> Self {
        Self::Route(route)
    }

    pub fn href(href: impl Into<Href>) -> Self {
        Self::Href(href.into())
    }
}

impl<R: Routable> LinkTarget<R> {
    fn key(&self) -> String {
        match self {
            Self::Route(route) => route.to_string(),
            Self::Href(href) => href.to_string(),
        }
    }
}

#[component]
pub fn RouteLink<R: Routable + Clone + PartialEq + 'static>(
    target: LinkTarget<R>,
    #[props(into)] class: CssClass,
    #[props(into)] label: DisplayText,
) -> Element {
    let class = class.into_string();

    match target {
        LinkTarget::Route(route) if try_router().is_some() => {
            rsx! {
                Link {
                    class,
                    to: route,
                    "{label}"
                }
            }
        },
        LinkTarget::Route(route) => {
            let href = route.to_string();
            rsx! {
                a {
                    class,
                    href,
                    "{label}"
                }
            }
        },
        LinkTarget::Href(href) => {
            rsx! {
                a {
                    class,
                    href: href.as_str(),
                    "{label}"
                }
            }
        },
    }
}

#[component]
pub fn NavLink<R: Routable + Clone + PartialEq + 'static>(
    target: LinkTarget<R>,
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
    pub source: DisplayText,
}

impl ProjectNavLabels {
    pub fn new(
        home: impl Into<DisplayText>,
        demos: impl Into<DisplayText>,
        book: impl Into<DisplayText>,
        source: impl Into<DisplayText>,
    ) -> Self {
        Self {
            home: home.into(),
            demos: demos.into(),
            book: book.into(),
            source: source.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectNavConfig<R> {
    pub project: ProjectOption,
    pub project_options: Vec<ProjectOption>,
    pub project_label: DisplayText,
    pub project_list_label: DisplayText,
    pub home: LinkTarget<R>,
    pub demos: LinkTarget<R>,
    pub book: Href,
    pub source: Href,
    pub labels: ProjectNavLabels,
    pub active: ProjectNavItem,
}

impl<R> ProjectNavConfig<R> {
    pub fn new(
        project: ProjectOption,
        home: LinkTarget<R>,
        demos: LinkTarget<R>,
        book: impl Into<Href>,
        source: impl Into<Href>,
        labels: ProjectNavLabels,
        active: ProjectNavItem,
    ) -> Self {
        Self {
            project,
            project_options: Vec::new(),
            project_label: DisplayText::new("Project selector"),
            project_list_label: DisplayText::new("Projects"),
            home,
            demos,
            book: book.into(),
            source: source.into(),
            labels,
            active,
        }
    }

    pub fn with_labels(mut self, labels: ProjectNavLabels) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_active(mut self, active: ProjectNavItem) -> Self {
        self.active = active;
        self
    }

    pub fn with_project_options(mut self, project_options: Vec<ProjectOption>) -> Self {
        self.project_options = project_options;
        self
    }

    pub fn with_project_labels(
        mut self,
        label: impl Into<DisplayText>,
        list_label: impl Into<DisplayText>,
    ) -> Self {
        self.project_label = label.into();
        self.project_list_label = list_label.into();
        self
    }
}

#[component]
pub fn ProjectNav<R: Routable + Clone + PartialEq + 'static>(
    home: LinkTarget<R>,
    demos: LinkTarget<R>,
    #[props(into)] book: Href,
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
                target: LinkTarget::href(book),
                label: labels.book,
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
            project_options: nav.project_options,
            project_label: nav.project_label,
            project_list_label: nav.project_list_label,
            home: nav.home,
            demos: nav.demos,
            book: nav.book,
            source: nav.source,
            labels: nav.labels,
            active: nav.active,
            {children}
        }
    }
}

#[component]
pub fn ProjectNavHeader<R: Routable + Clone + PartialEq + 'static>(
    project: ProjectOption,
    #[props(default)] project_options: Vec<ProjectOption>,
    #[props(default = DisplayText::new("Project selector"), into)] project_label: DisplayText,
    #[props(default = DisplayText::new("Projects"), into)] project_list_label: DisplayText,
    home: LinkTarget<R>,
    demos: LinkTarget<R>,
    #[props(into)] book: Href,
    #[props(into)] source: Href,
    labels: ProjectNavLabels,
    active: ProjectNavItem,
    children: Element,
) -> Element {
    rsx! {
        ProjectHeader {
            project,
            project_options,
            project_label,
            project_list_label,
            ProjectNav::<R> {
                home,
                demos,
                book,
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
    target: LinkTarget<R>,
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
    target: LinkTarget<R>,
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
    demos: LinkTarget<R>,
    #[props(into)] primary_label: DisplayText,
    #[props(into)] secondary_label: DisplayText,
) -> Element {
    rsx! {
        ButtonLink {
            href: book,
            label: primary_label,
        }
        ButtonRouteLink::<R> {
            target: demos,
            label: secondary_label,
            variant: ButtonVariant::Secondary,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoCard<R> {
    pub target: LinkTarget<R>,
    pub label: DisplayText,
    pub title: DisplayText,
    pub body: DisplayText,
    pub action: DisplayText,
    pub body_class: Option<CssClass>,
}

impl<R> DemoCard<R> {
    pub fn new(
        target: LinkTarget<R>,
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
        Self::new(LinkTarget::route(route), label, title, body, action)
    }

    pub fn href(
        href: impl Into<Href>,
        label: impl Into<DisplayText>,
        title: impl Into<DisplayText>,
        body: impl Into<DisplayText>,
        action: impl Into<DisplayText>,
    ) -> Self {
        Self::new(LinkTarget::href(href), label, title, body, action)
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
                    let key = card.target.key();

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
pub fn RouteCardLink<R: Routable + Clone + PartialEq + 'static>(
    target: LinkTarget<R>,
    #[props(into)] label: DisplayText,
    #[props(into)] title: DisplayText,
    #[props(into)] body: DisplayText,
    #[props(into)] body_class: CssClass,
    #[props(into)] action: DisplayText,
) -> Element {
    let body_class = body_class.into_string();

    match target {
        LinkTarget::Route(route) if try_router().is_some() => {
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
        LinkTarget::Route(route) => {
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
        LinkTarget::Href(href) => {
            rsx! {
                a {
                    class: "demo-card",
                    href: href.as_str(),
                    div { class: "card-label", "{label}" }
                    h2 { "{title}" }
                    p { class: body_class, "{body}" }
                    span { class: "card-link", "{action}" }
                }
            }
        },
    }
}
