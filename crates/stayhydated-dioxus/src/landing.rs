use dioxus::prelude::*;
use stayhydated_dioxus_core::{LandingLink, LandingTheme, ProjectLanding};

use crate::Project;

fn landing_theme(project: Project) -> LandingTheme {
    match project {
        Project::GpuiForm => LandingTheme::Blue,
        Project::GpuiTable => LandingTheme::Green,
        Project::GpuiStorybook => LandingTheme::Rose,
        Project::GpuiEsFluent => LandingTheme::Purple,
        Project::FrameCapture => LandingTheme::Amber,
        Project::Koruma | Project::EsFluent | Project::SumNumbersAi => LandingTheme::Cyan,
    }
}

fn landing_links(project: Project) -> Vec<LandingLink> {
    let mut links = vec![LandingLink::new("book/", "Read the book")];
    if let Some(demo_path) = project.demo_path() {
        links.push(LandingLink::new(demo_path, "Open the demo"));
    }
    links.extend([
        LandingLink::new(project.rustdoc_href(), "Rust API docs"),
        LandingLink::new(project.source_href(), "Source"),
    ]);
    links
}

#[component]
pub fn StayhydatedProjectLanding(project: Project) -> Element {
    let eyebrow = format!("stayhydated / {}", project.category());

    rsx! {
        ProjectLanding {
            project_name: project.display_name(),
            tagline: project.description(),
            eyebrow,
            links: landing_links(project),
            theme: landing_theme(project),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_landing_uses_project_destinations_and_theme() {
        let html = dioxus::ssr::render_element(rsx! {
            StayhydatedProjectLanding { project: Project::GpuiTable }
        });

        assert!(html.contains("project-landing--green"));
        assert!(html.contains("stayhydated / Rust UI"));
        assert!(html.contains("href=\"gpui-demo/\""));
        assert!(html.contains("https://docs.rs/gpui-table/"));
    }

    #[test]
    fn project_without_demo_omits_demo_destination() {
        let html = dioxus::ssr::render_element(rsx! {
            StayhydatedProjectLanding { project: Project::FrameCapture }
        });

        assert!(!html.contains("Open the demo"));
        assert!(html.contains("project-landing--amber"));
    }
}
