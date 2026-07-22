use dioxus::prelude::*;
use stayhydated_dioxus::*;

#[derive(Clone, Debug, PartialEq, Routable)]
enum TestRoute {
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

#[test]
fn router_app_keeps_shader_local_to_rendered_surfaces() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedRouterApp::<TestRoute> { base_href: "/" }
    });

    assert!(!html.contains("shader-background-canvas"));
}

#[component]
fn ProjectApp() -> Element {
    let config = StayhydatedProjectHeaderConfig::builder(Project::Koruma)
        .home(NavigationTarget::Internal(TestRoute::Home {}))
        .demos(NavigationTarget::Internal(TestRoute::Demos {}))
        .source("https://example.com/source")
        .docs("https://example.com/docs")
        .labels(stayhydated_header_labels_with(|message| {
            format!("Project {message}")
        }))
        .active(ProjectNavItem::Demos)
        .build();

    rsx! {
        StayhydatedDioxusApp { base_href: "/koruma/",
            StayhydatedProjectHeader::<TestRoute> { config,
                span { "Header child" }
            }
            StayhydatedProjectPageMetadata {
                project: Project::Koruma,
                page_title: "Home",
                description: "Koruma project home",
            }
            ProjectSupportTextLink {
                link: ProjectSupportLink::KORUMA_COLLECTION_CROWDIN,
            }
            ProjectSupportTextLink {
                link: ProjectSupportLink::KORUMA_COLLECTION_CROWDIN,
                label: "Translate",
                class: "translate-link",
            }
            ProjectSourceTextLink { project: Project::Koruma }
            ProjectSourceTextLink {
                project: Project::EsFluent,
                label: "Repository",
            }
            ProjectFluentTextLink {}
            ProjectFluentTextLink { label: "Fluent" }
            ProjectPackageTextLink { package: ProjectPackage::KORUMA }
            ProjectPackageTextLink {
                package: ProjectPackage::ES_FLUENT,
                label: "es-fluent crate",
            }
            ProjectFooterPanelForProject { project: Project::Koruma }
        }
    }
}

#[test]
fn renders_project_app_navigation_resources_and_footer() {
    let html = dioxus::ssr::render_element(rsx! { ProjectApp {} });

    assert!(!html.contains("shader-background-canvas"));
    assert!(html.contains("href=\"/koruma/\""));
    assert!(html.contains("Rust validation"));
    assert!(html.contains("Project Home"));
    assert!(html.contains("translate-link"));
    assert!(html.contains("Project Fluent"));
    assert!(html.contains("es-fluent crate"));
    assert!(html.contains("Resources"));
    assert!(html.contains("/koruma/llms-full.txt"));
}

#[test]
fn renders_responsive_project_portal_destinations() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortal::<TestRoute> {
            project: Project::SumNumbersAi,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            docs: Href::new("/sum-numbers-ai/book/api-contract.html"),
            book: Href::new("/sum-numbers-ai/book/"),
            demos: NavigationTarget::Internal(TestRoute::Demos {}),
            source: Href::new("https://github.com/stayhydated/shared"),
        }
    });

    assert!(html.contains("project-portal is-root"));
    assert!(html.contains("href=\"/\""));
    assert!(html.contains("aria-label=\"Home\""));
    assert!(html.contains("sum-numbers-ai-portal-0"));
    assert!(html.contains("sum-numbers-ai-portal-3"));
    assert!(html.contains("portal-accent-yellow"));
    assert!(html.contains("portal-accent-cyan"));
    assert!(html.contains("portal-accent-magenta"));
    assert!(html.contains("portal-accent-white"));
    assert!(html.contains("Docs"));
    assert!(html.contains("Book"));
    assert!(html.contains("Demos"));
    assert!(html.contains("Git"));
    assert!(html.contains("portal-skills-copy"));
    assert!(html.contains("Copy skills command"));
    assert!(html.contains(Project::SumNumbersAi.description()));

    let defaults_html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortal::<TestRoute> {
            project: Project::Koruma,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            demos: NavigationTarget::Internal(TestRoute::Demos {}),
        }
    });

    assert!(defaults_html.contains(Project::Koruma.rustdoc_href()));
    assert!(defaults_html.contains(Project::Koruma.book_href().as_str()));
    assert!(defaults_html.contains(Project::Koruma.source_href()));

    let shell_html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortalShell::<TestRoute> {
            project: Project::SumNumbersAi,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            section { class: "example-cards", "Examples" }
        }
    });

    assert!(shell_html.contains("portal-header"));
    assert!(shell_html.contains("portal-skills-copy"));
    assert!(shell_html.contains("example-cards"));
    assert!(!shell_html.contains("portal-destinations"));
    assert!(!shell_html.contains("project-portal is-root"));
}

#[test]
fn project_registry_identity_covers_custom_and_disabled_metadata() {
    let identity = Project::EsFluent.identity_with_href("/localized/");
    assert_eq!(identity.name.as_str(), "es-fluent");
    assert_eq!(
        identity.description.as_ref().map(DisplayText::as_str),
        Some("Rust localization")
    );
    assert_eq!(identity.href.as_str(), "/localized/");
    assert_eq!(
        Project::SumNumbersAi.description(),
        "An auditable AI addition API"
    );
    let package = ProjectPackage::new("custom", "https://example.com/custom");
    assert_eq!(package.name(), "custom");
    assert_eq!(package.crates_href(), "https://example.com/custom");
    let support = ProjectSupportLink::new("Support", "https://example.com/support");
    assert_eq!(support.label(), "Support");
    assert_eq!(support.href(), "https://example.com/support");
    assert_eq!(Project::Koruma.primary_package(), ProjectPackage::KORUMA);
    assert_eq!(
        Project::SumNumbersAi.primary_package().name(),
        "sum-numbers-ai-dummy"
    );
    assert!(Project::EsFluent.support_links().is_empty());
    assert!(ProjectPackage::KORUMA.support_links().is_empty());
    assert_eq!(
        ProjectPackage::KORUMA_COLLECTION.support_links(),
        &[ProjectSupportLink::KORUMA_COLLECTION_CROWDIN]
    );
}
