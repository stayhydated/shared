use dioxus::prelude::*;
use strum::IntoStaticStr;

use crate::{DisplayText, Href};

/// Color treatment for a compact project landing page.
#[derive(Clone, Copy, Debug, Default, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum LandingTheme {
    #[default]
    Blue,
    Green,
    Rose,
    Purple,
    Amber,
    Cyan,
}

impl LandingTheme {
    fn class(self) -> &'static str {
        self.into_str()
    }
}

/// One destination exposed by a [`ProjectLanding`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LandingLink {
    pub href: Href,
    pub label: DisplayText,
}

impl LandingLink {
    pub fn new(href: impl Into<Href>, label: impl Into<DisplayText>) -> Self {
        Self {
            href: href.into(),
            label: label.into(),
        }
    }
}

/// Compact, responsive landing page for a repository project.
#[component]
pub fn ProjectLanding(
    #[props(into)] project_name: DisplayText,
    #[props(into)] tagline: DisplayText,
    #[props(into)] eyebrow: DisplayText,
    links: Vec<LandingLink>,
    #[props(default)] theme: LandingTheme,
) -> Element {
    let class = format!("project-landing project-landing--{}", theme.class());

    rsx! {
        document::Stylesheet { href: asset!("./landing.css") }
        main { class,
            p { class: "project-landing__eyebrow", "{eyebrow}" }
            h1 { "{project_name}" }
            p { class: "project-landing__tagline", "{tagline}" }
            nav { aria_label: "Project resources",
                for link in links {
                    a {
                        href: link.href.as_str(),
                        "{link.label}"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn landing_renders_theme_and_links() {
        let html = dioxus::ssr::render_element(rsx! {
            ProjectLanding {
                project_name: "Example",
                tagline: "A useful project.",
                eyebrow: "stayhydated / Rust",
                theme: LandingTheme::Green,
                links: vec![LandingLink::new("book/", "Read the book")],
            }
        });

        assert!(html.contains("project-landing--green"));
        assert!(html.contains("href=\"book/\""));
        assert!(html.contains("A useful project."));
    }
}
