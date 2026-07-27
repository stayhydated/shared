use dioxus::prelude::*;
use stayhydated_dioxus_core::{DisplayText, LandingLink, LandingTheme, ProjectLanding};

use crate::Project;

#[component]
pub fn StayhydatedProjectLanding(
    project: Project,
    #[props(into)] eyebrow: DisplayText,
    links: Vec<LandingLink>,
    #[props(default)] theme: LandingTheme,
) -> Element {
    rsx! {
        ProjectLanding {
            project_name: project.as_str(),
            tagline: project.description(),
            eyebrow,
            links,
            theme,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_landing_uses_consumer_destinations_and_theme() {
        const PROJECT: Project = Project::new("example-project", "An example project.");
        let html = dioxus::ssr::render_element(rsx! {
            StayhydatedProjectLanding {
                project: PROJECT,
                eyebrow: "Example organization / Rust",
                links: vec![
                    LandingLink::new("book/", "Read the book"),
                    LandingLink::new("https://docs.example/project/", "API docs"),
                ],
                theme: LandingTheme::Green,
            }
        });

        assert!(html.contains("project-landing--green"));
        assert!(html.contains("Example organization / Rust"));
        assert!(html.contains("href=\"book/\""));
        assert!(html.contains("https://docs.example/project/"));
    }

    #[test]
    fn project_landing_does_not_require_optional_destinations() {
        const PROJECT: Project = Project::new("example-project", "An example project.");
        let html = dioxus::ssr::render_element(rsx! {
            StayhydatedProjectLanding {
                project: PROJECT,
                eyebrow: "Example organization / Rust",
                links: vec![LandingLink::new("book/", "Read the book")],
            }
        });

        assert!(!html.contains("Open the demo"));
        assert!(html.contains("project-landing--blue"));
    }
}
