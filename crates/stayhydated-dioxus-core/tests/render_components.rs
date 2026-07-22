use dioxus::prelude::*;
use stayhydated_dioxus_core::*;

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

fn render(element: Element) -> String {
    dioxus::ssr::render_element(element)
}

fn project() -> ProjectIdentity {
    ProjectIdentity::with_description("shared", "Shared crates", "/shared/")
}

#[component]
fn PrimitiveApp() -> Element {
    rsx! {
        Tabs {
            default_value: "overview",
            orientation: TabsOrientation::Vertical,
            TabList {
                TabTrigger {
                    value: "overview",
                    index: 0usize,
                    extra_class: "first-tab",
                    "Overview"
                }
            }
            TabContent {
                value: "overview",
                index: 0usize,
                extra_class: "overview-content",
                "Overview content"
            }
        }
        select::Select::<String> {
            default_value: String::from("one"),
            select::SelectTrigger {
                select::SelectValue { placeholder: "Choose one" }
            }
            select::SelectList {
                select::SelectGroup {
                    select::SelectGroupLabel { "Options" }
                    select::SelectOption::<String> {
                        index: 0usize,
                        value: String::from("one"),
                        text_value: String::from("One"),
                        "One"
                        select::SelectItemIndicator {}
                    }
                }
            }
        }
        select::SelectMulti::<String> {
            default_values: vec![String::from("one")],
            select::SelectList {
                select::SelectOption::<String> {
                    index: 0usize,
                    value: String::from("one"),
                    text_value: String::from("One"),
                    "One"
                }
            }
        }
    }
}

#[test]
fn renders_card_grid_and_shell_components() {
    let cards = vec![
        DemoCard::route(
            TestRoute::Demos {},
            "demo",
            "Demos",
            "Browse the demos",
            "View demos",
        )
        .with_body_class("route-card-body"),
        DemoCard::href(
            "https://example.com/source",
            "source",
            "Source",
            "Browse the source",
            "View source",
        ),
    ];

    let html = render(rsx! {
        ProjectPageShell {
            extra_class: "outer",
            main_extra_class: "inner",
            header: rsx! { header { "Header" } },
            footer: rsx! { FooterPanel { div { "Footer" } } },
            FeatureCard {
                label: "one",
                title: "First feature",
                body: "Feature body",
                style: "--motion-delay: 10ms;",
            }
            CodeBlock { code: "cargo test", class: "command" }
            DemoCardGrid::<TestRoute> {
                cards,
                columns: GridColumns::Two,
                extra_class: "demo-grid",
                body_class: "default-card-body",
            }
        }
    });

    assert!(html.contains("page-shell outer"));
    assert!(html.contains("grid columns-2 demo-grid"));
    assert!(html.contains("route-card-body"));
    assert!(html.contains("default-card-body"));
    assert!(html.contains("https://example.com/source"));
    assert!(html.contains("cargo test"));
}

#[test]
fn renders_project_header_navigation_and_hero_components() {
    let labels = ProjectNavLabels::new("Home", "Demos", "Book", "Docs", "Source");
    let nav = ProjectNavConfig::builder()
        .project(project())
        .home(NavigationTarget::Internal(TestRoute::Home {}))
        .demos(NavigationTarget::Internal(TestRoute::Demos {}))
        .book(Href::new("/shared/book/"))
        .docs(Href::new("https://docs.rs/shared"))
        .source(Href::new("https://example.com/source"))
        .labels(labels.clone())
        .active(ProjectNavItem::Demos)
        .build()
        .with_labels(labels)
        .with_active(ProjectNavItem::Home);

    let html = render(rsx! {
        ProjectHomeShell {
            header: rsx! {
                ProjectNavigationHeader::<TestRoute> { nav,
                    ExternalTextLink {
                        href: "https://example.com/help",
                        label: "Help",
                        class: "help-link",
                    }
                }
            },
            footer: rsx! { footer { "Footer" } },
            ProjectHero {
                eyebrow: "Workspace",
                title: "Shared Rust crates",
                body: "Build project sites",
                extra_class: "home-hero",
                actions: rsx! {
                    ProjectHeroActions::<TestRoute> {
                        book: "/shared/book/",
                        docs: "https://docs.rs/shared",
                        demos: NavigationTarget::Internal(TestRoute::Demos {}),
                    }
                },
                side: rsx! {
                    HeroListPanel {
                        label: "Workflow",
                        label_heading: true,
                        kind: HeroPanelListKind::Ordered,
                        items: vec![
                            HeroPanelItem::new("Build", "Compile the workspace"),
                            HeroPanelItem::new("Test", "Run focused tests"),
                        ],
                    }
                },
            }
            ProjectSurfaceSection {
                label: "Libraries",
                title: "Published crates",
                lead: "Reusable building blocks",
                extra_class: "surface",
                heading_extra_class: "surface-heading",
                content_class: "surface-content",
                style: "--motion-delay: 90ms;",
                div { "Crates" }
            }
        }
    });

    assert!(html.contains("href=\"/shared/\""));
    assert!(html.contains("Shared crates"));
    assert!(html.contains("Shared Rust crates"));
    assert!(html.contains("<ol"));
    assert!(html.contains("Read the book"));
    assert!(html.contains("help-link"));
    assert!(html.contains("surface-content"));
}

#[test]
fn renders_remaining_layout_metadata_and_portal_components() {
    let html = render(rsx! {
        ProjectPageMetadata {
            site_name: "shared",
            page_title: "Components",
            description: "Shared project components",
        }
        SharedStyles {}
        HeroListPanel {
            label: "Items",
            items: vec![HeroPanelItem::new("One", "First")],
        }
        ContributePanelShell {
            style: "--motion-delay: 370ms;",
            "Contribute"
        }
        FullscreenDemoFrame {
            src: "/demo/",
            title: "Demo frame",
            allowfullscreen: true,
        }
        FullscreenDemoPage::<TestRoute> {
            back_target: NavigationTarget::Internal(TestRoute::Demos {}),
            back_label: "Back",
            src: "/other-demo/",
            title: "Other demo",
            allowfullscreen: false,
        }
        ProjectPortalShell::<TestRoute> {
            project_name: "shared",
            version: "0.1.0",
            tagline: "Shared project components",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            title_extra: Some(rsx! { span { class: "title-extra", "Extra" } }),
            section { class: "portal-content", "Portal content" }
        }
    });

    assert!(html.contains("<ul"));
    assert!(html.contains("fullscreen-demo-frame"));
    assert!(html.contains("portal-title-copy"));
    assert!(html.contains("title-extra"));
    assert!(html.contains("portal-content"));
    assert!(html.contains("aria-label=\"Home\""));
    assert!(!html.contains("portal-telemetry"));
    assert!(!html.contains("project-portal is-root"));
}

#[test]
fn project_portal_renders_unique_shader_canvases() {
    let html = render(rsx! {
        ProjectPortal::<TestRoute> {
            project_name: "shared",
            version: "0.1.0",
            tagline: "Shared project components",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            shader_id_prefix: "shared-portal",
            destinations: vec![
                PortalDestination::href("/book/", "Book", PortalAccent::Yellow),
                PortalDestination::route(TestRoute::Demos {}, "Demos", PortalAccent::Cyan),
            ],
        }
    });

    assert!(html.contains("id=\"shared-portal-0\""));
    assert!(html.contains("id=\"shared-portal-1\""));
    assert_eq!(
        html.matches("data-shader-background=\"loading\"").count(),
        2
    );
    assert_eq!(html.matches("portal-destination-shader").count(), 2);
}

#[test]
fn renders_tab_and_select_primitives() {
    let html = render(rsx! { PrimitiveApp {} });

    assert!(html.contains("collection-module-tabs"));
    assert!(html.contains("first-tab"));
    assert!(html.contains("dx-select"));
    assert!(html.contains("Choose one"));
}

#[test]
fn newtypes_and_constructor_variants_preserve_values() {
    let text = DisplayText::new("text");
    assert_eq!(<DisplayText as AsRef<str>>::as_ref(&text), "text");
    assert!(!text.is_empty());
    assert_eq!(text.into_string(), "text");

    assert!(OptionalDisplayText::default().into_option().is_none());
    assert_eq!(
        OptionalDisplayText::from(Some(String::from("optional")))
            .into_option()
            .map(DisplayText::into_string),
        Some(String::from("optional"))
    );
    assert_eq!(
        OptionalDisplayText::from(Some("borrowed"))
            .into_option()
            .map(DisplayText::into_string),
        Some(String::from("borrowed"))
    );
    assert!(
        OptionalDisplayText::from(None::<String>)
            .into_option()
            .is_none()
    );
    assert!(
        OptionalDisplayText::from(None::<&str>)
            .into_option()
            .is_none()
    );
    assert_eq!(
        OptionalDisplayText::from(DisplayText::new("display text"))
            .into_option()
            .map(DisplayText::into_string),
        Some(String::from("display text"))
    );
    assert_eq!(
        OptionalDisplayText::from(String::from("owned"))
            .into_option()
            .map(DisplayText::into_string),
        Some(String::from("owned"))
    );
    assert_eq!(
        OptionalDisplayText::from("borrowed directly")
            .into_option()
            .map(DisplayText::into_string),
        Some(String::from("borrowed directly"))
    );

    let href = Href::new("/path/");
    assert_eq!(<Href as AsRef<str>>::as_ref(&href), "/path/");
    assert_eq!(href.into_string(), "/path/");

    let class = CssClass::new("class");
    assert_eq!(<CssClass as AsRef<str>>::as_ref(&class), "class");
    assert!(!class.is_empty());
    assert!(CssClass::new("  ").is_empty());
    assert_eq!(class.as_str(), "class");
    assert_eq!(class.into_string(), "class");

    let style = InlineStyle::new("color: red;");
    assert_eq!(<InlineStyle as AsRef<str>>::as_ref(&style), "color: red;");
    assert_eq!(style.into_string(), "color: red;");

    let project = ProjectIdentity::new("Project", "/project/");
    assert!(project.description.is_none());
    assert_eq!(project.href.as_str(), "/project/");

    assert!(ProjectNavItem::Home.is_home());
    assert!(!ProjectNavItem::Home.is_demos());
    assert!(ProjectNavItem::Demos.is_demos());
    assert!(!ProjectNavItem::Demos.is_home());

    assert_eq!(project_document_title("", ""), "");
    assert_eq!(project_document_title("", "Page"), "Page");
    assert_eq!(page_entry_reveal_style(), hero_reveal_style());
}
