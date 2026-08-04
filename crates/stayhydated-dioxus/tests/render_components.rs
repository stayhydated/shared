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

const TEST_PROJECT: Project = Project::new("example-project", "An example project.")
    .with_skill_command("npx skills add example/project");

fn test_site() -> ProjectSite {
    ProjectSite::builder()
        .project(TEST_PROJECT)
        .site_url("https://example.test/example-project/")
        .rustdoc_url("https://docs.example/project/")
        .source_url("https://code.example/project")
        .version("0.1.0")
        .build()
}

#[test]
fn router_app_keeps_shader_local_to_rendered_surfaces() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedRouterApp::<TestRoute> {}
    });

    assert!(!html.contains("shader-background-canvas"));
}

#[test]
fn renders_project_metadata_and_responsive_portal_destinations() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPageMetadata {
            project: TEST_PROJECT,
            page_title: "Home",
            description: "Example project home",
        }
        StayhydatedProjectPortal::<TestRoute> {
            project: TEST_PROJECT,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            docs: Href::new("https://docs.example/project/"),
            book: Href::new("/example-project/book/"),
            demos: NavigationTarget::Internal(TestRoute::Demos {}),
            source: Href::new("https://code.example/project"),
        }
    });

    assert!(html.contains("project-portal is-root"));
    assert!(html.contains("href=\"/\""));
    assert!(html.contains("aria-label=\"Home\""));
    assert!(html.contains("example-project-portal-0"));
    assert!(html.contains("example-project-portal-3"));
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
    assert!(html.contains("An example project."));
}

#[test]
fn project_portal_uses_consumer_destinations() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortal::<TestRoute> {
            project: TEST_PROJECT,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            docs: Href::new("https://docs.example/project/"),
            book: Href::new("/example-project/book/"),
            demos: NavigationTarget::Internal(TestRoute::Demos {}),
            source: Href::new("https://code.example/project"),
        }
    });

    assert!(html.contains("https://docs.example/project/"));
    assert!(html.contains("/example-project/book/"));
    assert!(html.contains("https://code.example/project"));
}

#[test]
fn project_site_portal_uses_the_shared_configuration() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectSitePortal::<TestRoute> {
            site: test_site(),
            home: NavigationTarget::Internal(TestRoute::Home {}),
            demos: NavigationTarget::Internal(TestRoute::Demos {}),
        }
    });

    assert!(html.contains("https://docs.example/project/"));
    assert!(html.contains("href=\"/book/\""));
    assert!(html.contains("https://code.example/project"));
    assert!(html.contains("Demos"));
}

#[test]
fn project_portal_omits_the_demo_destination_when_not_configured() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortal::<TestRoute> {
            project: TEST_PROJECT,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            docs: Href::new("https://docs.example/project/"),
            book: Href::new("/example-project/book/"),
            source: Href::new("https://code.example/project"),
        }
    });

    assert!(html.contains("Docs"));
    assert!(html.contains("Book"));
    assert!(html.contains("Git"));
    assert!(!html.contains("Demos"));
    assert_eq!(html.matches("class=\"portal-destination ").count(), 3);
    assert!(html.contains("example-project-portal-2"));
    assert!(!html.contains("example-project-portal-3"));
}

#[test]
fn project_portal_shell_keeps_only_the_shared_heading() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortalShell::<TestRoute> {
            project: TEST_PROJECT,
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

#[test]
fn project_portal_shell_omits_skills_for_projects_without_a_command() {
    const PROJECT_WITHOUT_SKILLS: Project = Project::new("example-project", "An example project.");
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedProjectPortalShell::<TestRoute> {
            project: PROJECT_WITHOUT_SKILLS,
            version: "0.1.0",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            section { "Examples" }
        }
    });

    assert!(!html.contains("portal-skills-copy"));
    assert!(!html.contains("Copy skills command"));
}
