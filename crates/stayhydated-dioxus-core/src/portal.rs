use dioxus::prelude::*;
use strum::IntoStaticStr;

use crate::{CssClass, DisplayText, links::RouteLink, shader_background::ShaderBackground};

/// Color treatment for a destination in a project portal.
#[derive(Clone, Copy, Debug, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum PortalAccent {
    Yellow,
    Cyan,
    Magenta,
    White,
}

impl PortalAccent {
    fn class(self) -> &'static str {
        self.into_str()
    }
}

/// One full-panel destination in a [`ProjectPortal`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortalDestination<R> {
    pub target: NavigationTarget<R>,
    pub label: DisplayText,
    pub accent: PortalAccent,
}

impl<R> PortalDestination<R> {
    pub fn new(
        target: NavigationTarget<R>,
        label: impl Into<DisplayText>,
        accent: PortalAccent,
    ) -> Self {
        Self {
            target,
            label: label.into(),
            accent,
        }
    }

    pub fn route(route: R, label: impl Into<DisplayText>, accent: PortalAccent) -> Self {
        Self::new(NavigationTarget::Internal(route), label, accent)
    }

    pub fn href(
        href: impl Into<crate::Href>,
        label: impl Into<DisplayText>,
        accent: PortalAccent,
    ) -> Self {
        let href = href.into();
        Self::new(
            NavigationTarget::External(href.into_string()),
            label,
            accent,
        )
    }
}

/// Responsive full-viewport project navigation inspired by technical control panels.
#[component]
pub fn ProjectPortal<R: Routable + Clone + PartialEq + 'static>(
    #[props(into)] project_name: DisplayText,
    #[props(into)] version: DisplayText,
    #[props(into)] tagline: DisplayText,
    home: NavigationTarget<R>,
    destinations: Vec<PortalDestination<R>>,
    #[props(default = String::from("project-portal"))] shader_id_prefix: String,
    #[props(default)] title_extra: Option<Element>,
) -> Element {
    let destination_count = destinations.len();
    let destinations_label = format!("{project_name} destinations");

    rsx! {
        ProjectPortalShell {
            project_name,
            version,
            tagline,
            home,
            animate_header: true,
            title_extra,
            nav {
                class: "portal-destinations",
                aria_label: destinations_label,
                style: "--portal-destination-count: {destination_count};",
                for (index, destination) in destinations.into_iter().enumerate() {
                    {
                        let key = destination.target.to_string();
                        let shader_id = format!("{shader_id_prefix}-{index}");
                        let phase = index as f32 * 13.0;

                        rsx! {
                            PortalDestinationLink::<R> {
                                key: "{key}",
                                destination,
                                shader_id,
                                phase,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Shared full-viewport portal frame and project heading.
#[component]
pub fn ProjectPortalShell<R: Routable + Clone + PartialEq + 'static>(
    #[props(into)] project_name: DisplayText,
    #[props(into)] version: DisplayText,
    #[props(into)] tagline: DisplayText,
    home: NavigationTarget<R>,
    children: Element,
    #[props(default)] title_extra: Option<Element>,
    #[props(default)] animate_header: bool,
) -> Element {
    let class = if animate_header {
        "project-portal is-root"
    } else {
        "project-portal"
    };

    rsx! {
        main { class,
            div { class: "portal-frame", aria_hidden: "true" }
            header { class: "portal-header",
                div { class: "portal-title-row",
                    RouteLink::<R> {
                        target: home,
                        class: "portal-slashes portal-home-link",
                        label: "//",
                        aria_label: "Home",
                    }
                    div { class: "portal-title-copy",
                        h1 { "{project_name}" }
                        if let Some(title_extra) = title_extra {
                            {title_extra}
                        }
                    }
                    span { class: "portal-version", "v{version}" }
                }
                p { "{tagline}" }
            }
            {children}
        }
    }
}

#[component]
fn PortalDestinationLink<R: Routable + Clone + PartialEq + 'static>(
    destination: PortalDestination<R>,
    shader_id: String,
    phase: f32,
) -> Element {
    let accent = destination.accent.class();
    let class = format!("portal-destination portal-accent-{accent}");
    let aria_label = format!("Open {}", destination.label);

    match destination.target {
        NavigationTarget::Internal(route) if try_router().is_some() => {
            rsx! {
                Link {
                    class,
                    to: route,
                    aria_label,
                    PortalDestinationContents {
                        label: destination.label,
                        shader_id,
                        phase,
                    }
                }
            }
        },
        NavigationTarget::Internal(route) => rsx! {
            a {
                class,
                href: route.to_string(),
                aria_label,
                PortalDestinationContents {
                    label: destination.label,
                    shader_id,
                    phase,
                }
            }
        },
        NavigationTarget::External(href) => rsx! {
            a {
                class,
                href,
                aria_label,
                PortalDestinationContents {
                    label: destination.label,
                    shader_id,
                    phase,
                }
            }
        },
    }
}

#[component]
fn PortalDestinationContents(label: DisplayText, shader_id: String, phase: f32) -> Element {
    rsx! {
        span { class: "portal-destination-surface",
            ShaderBackground {
                canvas_id: shader_id,
                extra_class: CssClass::new("portal-destination-shader"),
                time_offset: phase,
            }
            span { class: "portal-destination-tint", aria_hidden: "true" }
            span { class: "portal-destination-grid", aria_hidden: "true" }
            span { class: "portal-destination-corners", aria_hidden: "true" }
            span { class: "portal-destination-label", "{label}" }
        }
    }
}
