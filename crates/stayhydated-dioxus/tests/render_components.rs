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
fn renders_router_app_at_root_route() {
    let html = dioxus::ssr::render_element(rsx! {
        StayhydatedRouterApp::<TestRoute> { base_href: "/" }
    });

    assert!(html.contains("shader-background-canvas"));
}

#[component]
fn ProjectApp() -> Element {
    let config = StayhydatedProjectHeaderConfig::new(
        Project::Koruma,
        "/koruma/",
        LinkTarget::route(TestRoute::Home {}),
        LinkTarget::route(TestRoute::Demos {}),
        "/koruma/book/",
        stayhydated_header_labels(),
        ProjectNavItem::Home,
    )
    .with_project_options(stayhydated_project_options())
    .with_source("https://example.com/source")
    .with_docs("https://example.com/docs")
    .with_project_labels("Choose project", "Available projects")
    .with_labels(stayhydated_header_labels_with(|message| {
        format!("Project {message}")
    }))
    .with_active(ProjectNavItem::Demos);

    rsx! {
        StayhydatedDioxusApp { base_href: "/koruma/",
            StayhydatedProjectHeader::<TestRoute> { config,
                span { "Header child" }
            }
            ProjectSwitcher {
                project: Project::EsFluent,
                href: "/es-fluent/",
                label: "Switch project",
                list_label: "Project list",
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
fn renders_project_app_assets_navigation_resources_and_footer() {
    let html = dioxus::ssr::render_element(rsx! { ProjectApp {} });

    assert!(html.contains("shader-background-canvas"));
    assert!(html.contains("Choose project"));
    assert!(html.contains("Project Home"));
    assert!(html.contains("translate-link"));
    assert!(html.contains("Project Fluent"));
    assert!(html.contains("es-fluent crate"));
    assert!(html.contains("Resources"));
    assert!(html.contains("/koruma/llms-full.txt"));
}

#[test]
fn project_registry_constructors_cover_custom_and_disabled_metadata() {
    let custom = Project::SumNumbersAi.option_with("Arithmetic", "/arithmetic/");
    assert_eq!(custom.name.as_str(), "Arithmetic");
    assert!(custom.description.is_none());

    let localized = Project::EsFluent
        .option_with_message_href("/localized/", |message| format!("Localized {message}"));
    assert_eq!(
        localized.description.as_ref().map(DisplayText::as_str),
        Some("Localized Rust localization")
    );

    let converted: CoreProjectId = Project::Koruma.into();
    assert_eq!(converted.as_str(), "koruma");
    let converted: ProjectOption = Project::Koruma.into();
    assert_eq!(converted.href.as_str(), "/koruma/");

    assert_eq!(HeaderMessage::Language.as_str(), "Language");
    assert_eq!(
        ProjectMessage::SumNumbersAiDescription.as_str(),
        "AI-assisted arithmetic"
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
