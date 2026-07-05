use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    ButtonRouteLink, ButtonVariant, CssClass, DemoCard, DemoCardGrid, DisplayText, ExternalNavLink,
    FeatureCard, FooterPanel, GridColumns, HeaderNav, HeroSidePanel, Href, InlineStyle, LinkTarget,
    NavLink, ProjectHeader, ProjectHero, ProjectHomeShell, ProjectId, ProjectMark,
    ProjectNavConfig, ProjectNavHeader, ProjectNavItem, ProjectNavLabels, ProjectNavigationHeader,
    ProjectOption, ProjectPageMetadata, ProjectPageShell, ProjectSurfaceSection,
    project_document_title,
};

#[derive(Clone, Debug, PartialEq, Routable)]
enum AppRoute {
    #[route("/", HomeRoute)]
    Home {},
    #[route("/demos/", DemosRoute)]
    Demos {},
}

#[component]
fn HomeRoute() -> Element {
    rsx! {}
}

#[component]
fn DemosRoute() -> Element {
    rsx! {}
}

fn app() -> Element {
    rsx! {
        ProjectHomeShell {
            header: rsx! {
                ProjectHeader {
                    project: example_project(),
                    HeaderNav {
                        NavLink::<AppRoute> {
                            target: LinkTarget::route(AppRoute::Home {}),
                            label: "Home",
                            active: true,
                        }
                        ExternalNavLink {
                            href: "https://github.com/stayhydated/stayhydated",
                            label: "Source",
                        }
                    }
                }
            },
            footer: rsx! {
                FooterPanel {
                    div { class: "footer-section", "Footer" }
                }
            },
            ProjectPageMetadata {
                site_name: "example-project",
                page_title: "Home",
                description: "Example project home page.",
            }
            ProjectHero {
                eyebrow: "Project",
                title: "Project home",
                body: "Shared page shell and hero layout.",
                style: "--reveal-delay: 0ms; --reveal-distance: 24px;",
                side: Some(rsx! {
                    HeroSidePanel { class: "hero-panel",
                        h2 { class: "panel-label", "Highlights" }
                    }
                }),
                actions: Some(rsx! {
                    ButtonRouteLink::<AppRoute> {
                        target: LinkTarget::route(AppRoute::Demos {}),
                        label: "View demos",
                        variant: ButtonVariant::Secondary,
                    }
                }),
            }
            ProjectSurfaceSection {
                label: Some("Workflow"),
                title: "Standard section",
                lead: Some("Shared surface copy."),
                extra_class: "project-directory-section",
                heading_extra_class: "directory-heading",
                content_class: "project-list",
                FeatureCard {
                    label: "one",
                    title: "First card",
                    body: "First card body.",
                }
            }
        }
    }
}

fn project_nav_app() -> Element {
    rsx! {
        ProjectNavHeader::<AppRoute> {
            project: example_project(),
            home: LinkTarget::route(AppRoute::Home {}),
            demos: LinkTarget::route(AppRoute::Demos {}),
            book: "/project/book/",
            source: "https://github.com/stayhydated/example-project",
            labels: ProjectNavLabels::new("Home", "Demos", "Book", "Source"),
            active: ProjectNavItem::Home,
        }
    }
}

fn project_navigation_header_app() -> Element {
    let nav = ProjectNavConfig::new(
        example_project(),
        LinkTarget::route(AppRoute::Home {}),
        LinkTarget::route(AppRoute::Demos {}),
        "/project/book/",
        "https://github.com/stayhydated/example-project",
        ProjectNavLabels::new("Home", "Demos", "Book", "Source"),
        ProjectNavItem::Demos,
    );

    rsx! {
        ProjectNavigationHeader::<AppRoute> { nav }
    }
}

fn page_shell_app() -> Element {
    rsx! {
        ProjectPageShell {
            header: rsx!(header { class: "page-header", "Header" }),
            main_extra_class: "project-directory",
            DemoCardGrid::<AppRoute> {
                cards: vec![
                    DemoCard::route(
                        AppRoute::Demos {},
                        "demo",
                        "Demos",
                        "Shared card grid.",
                        "Open demo",
                    ),
                    DemoCard::href(
                        "https://github.com/stayhydated/stayhydated",
                        "source",
                        "Source",
                        "External card target.",
                        "Open source",
                    ),
                ],
                columns: GridColumns::Two,
                extra_class: "motion-reveal",
            }
        }
    }
}

fn newtype_conversions() {
    let _: DisplayText = "Label".into();
    let _: Href = "/demos/".into();
    let _: CssClass = "motion-reveal".into();
    let _: InlineStyle = "--reveal-delay: 0ms;".into();
    let project_id = ProjectId::new("example-project");
    assert_eq!(project_id.as_str(), "example-project");
    assert!(ProjectNavItem::Home.is_home());
    assert!(ProjectNavItem::Demos.is_demos());
    assert_eq!(
        project_document_title("example-project", "Home"),
        "example-project | Home"
    );
}

fn example_project() -> ProjectOption {
    ProjectOption::with_description(
        ProjectId::new("example-project"),
        ProjectMark::new("EP"),
        DisplayText::new("example-project"),
        DisplayText::new("Example project"),
        Href::new("/project/"),
    )
}

fn main() {
    let _ = app;
    let _ = project_nav_app;
    let _ = project_navigation_header_app;
    let _ = page_shell_app;
    let _ = newtype_conversions;
}
