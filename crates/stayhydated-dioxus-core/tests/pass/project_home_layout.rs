use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    CssClass, DemoCard, DemoCardGrid, DisplayText, FeatureCard, FooterPanel, GridColumns,
    HeroListPanel, HeroPanelItem, Href, InlineStyle, NavigationTarget, ProjectHero,
    ProjectHeroActions, ProjectHomeShell, ProjectIdentity, ProjectNavConfig, ProjectNavItem,
    ProjectNavLabels, ProjectNavigationHeader, ProjectPageMetadata, ProjectPageShell,
    ProjectSurfaceSection, project_document_title,
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
    let nav = ProjectNavConfig::builder()
        .project(example_project())
        .home(NavigationTarget::Internal(AppRoute::Home {}))
        .demos(NavigationTarget::Internal(AppRoute::Demos {}))
        .book(Href::new("/project/book/"))
        .docs(Href::new("https://docs.rs/example-project/"))
        .source(Href::new("https://github.com/stayhydated/example-project"))
        .labels(ProjectNavLabels::new(
            "Home", "Demos", "Book", "Docs", "Source",
        ))
        .active(ProjectNavItem::Home)
        .build();

    rsx! {
        ProjectHomeShell {
            header: rsx! { ProjectNavigationHeader::<AppRoute> { nav } },
            footer: rsx! { FooterPanel { div { class: "footer-section", "Footer" } } },
            ProjectPageMetadata {
                site_name: "example-project",
                page_title: "Home",
                description: "Example project home page.",
            }
            ProjectHero {
                eyebrow: "Project",
                title: "Project home",
                body: "Shared page shell and hero layout.",
                style: "--motion-delay: 0ms; --motion-distance: 24px;",
                side: Some(rsx! {
                    HeroListPanel {
                        label: "Highlights",
                        items: vec![HeroPanelItem::new("Compose", "Reuse shared components.")],
                    }
                }),
                actions: Some(rsx! {
                    ProjectHeroActions::<AppRoute> {
                        book: "/project/book/",
                        docs: "https://docs.rs/example-project/",
                        demos: NavigationTarget::Internal(AppRoute::Demos {}),
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
    let _: InlineStyle = "--motion-delay: 0ms;".into();
    assert_eq!(example_project().name.as_str(), "example-project");
    assert!(ProjectNavItem::Home.is_home());
    assert!(ProjectNavItem::Demos.is_demos());
    assert_eq!(
        project_document_title("example-project", "Home"),
        "example-project | Home"
    );
}

fn example_project() -> ProjectIdentity {
    ProjectIdentity::with_description("example-project", "Example project", "/project/")
}

fn main() {
    let _ = app;
    let _ = page_shell_app;
    let _ = newtype_conversions;
}
