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

#[test]
fn renders_project_metadata_and_responsive_portal_destinations() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPageMetadata {
            project: Project::SumNumbersAi,
            page_title: "Home",
            description: "Sum numbers project home",
        }
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
    assert!(html.contains("An auditable AI addition API"));
}

#[test]
fn project_portal_uses_registry_defaults() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortal::<TestRoute> {
            project: Project::Koruma,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            demos: NavigationTarget::Internal(TestRoute::Demos {}),
        }
    });

    assert!(html.contains("https://docs.rs/koruma/"));
    assert!(html.contains("/koruma/book/"));
    assert!(html.contains("https://github.com/stayhydated/koruma"));
    assert_eq!(Project::Koruma.as_str(), "koruma");
    assert_eq!(
        Project::EsFluent.site_url(),
        "https://stayhydated.github.io/es-fluent/"
    );
}

#[test]
fn project_portal_shell_keeps_only_the_shared_heading() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortalShell::<TestRoute> {
            project: Project::SumNumbersAi,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            section { class: "example-cards", "Examples" }
        }
    });

    assert!(html.contains("portal-header"));
    assert!(html.contains("portal-skills-copy"));
    assert!(html.contains("example-cards"));
    assert!(!html.contains("portal-destinations"));
    assert!(!html.contains("project-portal is-root"));
}
