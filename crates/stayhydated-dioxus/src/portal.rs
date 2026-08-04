use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    DisplayText, Href, PortalAccent, PortalDestination, ProjectPortal, ProjectPortalShell,
};

use crate::{Project, project::ProjectSkillsCopyButton};

fn project_portal_skills(project: Project) -> Option<Element> {
    project.skill_command().map(|command| {
        rsx! {
            div { class: "portal-skills-copy",
                span { class: "portal-skills-label", "Skills" }
                ProjectSkillsCopyButton { command }
            }
        }
    })
}

/// Stayhydated project portal frame with the shared project heading.
#[component]
pub fn StayhydatedProjectPortalShell<R: Routable + Clone + PartialEq + 'static>(
    project: Project,
    #[props(into)] version: DisplayText,
    home: NavigationTarget<R>,
    children: Element,
) -> Element {
    rsx! {
        ProjectPortalShell::<R> {
            project_name: project.as_str(),
            version,
            tagline: project.description(),
            home,
            title_extra: project_portal_skills(project),
            {children}
        }
    }
}

/// Stayhydated project portal with consumer-provided docs, book, source, and optional demos.
#[component]
pub fn StayhydatedProjectPortal<R: Routable + Clone + PartialEq + 'static>(
    project: Project,
    #[props(into)] version: DisplayText,
    home: NavigationTarget<R>,
    #[props(into)] docs: Href,
    #[props(into)] book: Href,
    demos: Option<NavigationTarget<R>>,
    #[props(into)] source: Href,
) -> Element {
    let shader_id_prefix = format!("{}-portal", project.as_str());
    let mut destinations = vec![
        PortalDestination::href(docs, "Docs", PortalAccent::Yellow),
        PortalDestination::href(book, "Book", PortalAccent::Cyan),
    ];
    if let Some(demos) = demos {
        destinations.push(PortalDestination::new(
            demos,
            "Demos",
            PortalAccent::Magenta,
        ));
    }
    destinations.push(PortalDestination::href(source, "Git", PortalAccent::White));

    rsx! {
        ProjectPortal::<R> {
            project_name: project.as_str(),
            version,
            tagline: project.description(),
            home,
            destinations,
            shader_id_prefix,
            title_extra: project_portal_skills(project),
        }
    }
}
