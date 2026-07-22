use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    DisplayText, Href, PortalAccent, PortalDestination, ProjectPortal, ProjectPortalShell,
};

use crate::{Project, projects::ProjectSkillsCopyButton};

fn project_portal_skills(project: Project) -> Element {
    rsx! {
        div { class: "portal-skills-copy",
            span { class: "portal-skills-label", "Skills" }
            ProjectSkillsCopyButton { project }
        }
    }
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
            title_extra: Some(project_portal_skills(project)),
            {children}
        }
    }
}

/// Stayhydated project portal with the standard docs, book, demos, and source destinations.
#[component]
pub fn StayhydatedProjectPortal<R: Routable + Clone + PartialEq + 'static>(
    project: Project,
    #[props(into)] version: DisplayText,
    home: NavigationTarget<R>,
    docs: Option<Href>,
    book: Option<Href>,
    demos: NavigationTarget<R>,
    source: Option<Href>,
) -> Element {
    let shader_id_prefix = format!("{}-portal", project.as_str());
    let docs = docs.unwrap_or_else(|| Href::new(project.rustdoc_href()));
    let book = book.unwrap_or_else(|| project.book_href());
    let source = source.unwrap_or_else(|| Href::new(project.source_href()));
    let destinations = vec![
        PortalDestination::href(docs, "Docs", PortalAccent::Yellow),
        PortalDestination::href(book, "Book", PortalAccent::Cyan),
        PortalDestination::new(demos, "Demos", PortalAccent::Magenta),
        PortalDestination::href(source, "Git", PortalAccent::White),
    ];

    rsx! {
        ProjectPortal::<R> {
            project_name: project.as_str(),
            version,
            tagline: project.description(),
            home,
            destinations,
            shader_id_prefix,
            title_extra: Some(project_portal_skills(project)),
        }
    }
}
