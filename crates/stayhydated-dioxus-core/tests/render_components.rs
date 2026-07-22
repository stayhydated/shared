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
}

#[test]
fn renders_metadata_code_and_fullscreen_frame() {
    let html = render(rsx! {
        ProjectPageMetadata {
            site_name: "shared",
            page_title: "Components",
            description: "Shared project components",
        }
        SharedStyles {}
        CodeBlock { code: "cargo test", class: "command" }
        FullscreenDemoFrame {
            src: "/demo/",
            title: "Demo frame",
            allowfullscreen: true,
        }
    });

    assert!(html.contains("cargo test"));
    assert!(html.contains("fullscreen-demo-frame"));
    assert!(html.contains("src=\"/demo/\""));
}

#[test]
fn renders_project_portal_shell_and_destinations() {
    let shell_html = render(rsx! {
        ProjectPortalShell::<TestRoute> {
            project_name: "shared",
            version: "0.1.0",
            tagline: "Shared project components",
            home: NavigationTarget::Internal(TestRoute::Home {}),
            title_extra: Some(rsx! { span { class: "title-extra", "Extra" } }),
            section { class: "portal-content", "Portal content" }
        }
    });

    assert!(shell_html.contains("portal-title-copy"));
    assert!(shell_html.contains("title-extra"));
    assert!(shell_html.contains("portal-content"));
    assert!(shell_html.contains("aria-label=\"Home\""));
    assert!(!shell_html.contains("project-portal is-root"));

    let portal_html = render(rsx! {
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

    assert!(portal_html.contains("project-portal is-root"));
    assert!(portal_html.contains("id=\"shared-portal-0\""));
    assert!(portal_html.contains("id=\"shared-portal-1\""));
    assert_eq!(
        portal_html
            .matches("data-shader-background=\"loading\"")
            .count(),
        2
    );
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
fn public_newtypes_preserve_values() {
    let text = DisplayText::new("text");
    assert_eq!(<DisplayText as AsRef<str>>::as_ref(&text), "text");
    assert!(!text.is_empty());
    assert_eq!(text.into_string(), "text");

    let href = Href::new("/path/");
    assert_eq!(href.as_str(), "/path/");
    assert_eq!(href.into_string(), "/path/");

    let class = CssClass::new("extra");
    assert_eq!(class.as_str(), "extra");
    assert!(!class.is_empty());
    assert_eq!(class.into_string(), "extra");

    let style = InlineStyle::new("--delay: 0ms;");
    assert_eq!(style.as_str(), "--delay: 0ms;");
    assert_eq!(style.into_string(), "--delay: 0ms;");
    assert_eq!(project_document_title("", "Page"), "Page");
}
