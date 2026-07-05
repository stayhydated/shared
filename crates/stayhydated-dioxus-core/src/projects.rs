use bon::Builder;
use derive_more::{AsRef, Display, From};
use dioxus::prelude::*;

use crate::{DisplayText, Href};

#[derive(AsRef, Clone, Debug, Display, Eq, From, PartialEq)]
#[as_ref(forward)]
#[from(String, &str)]
pub struct ProjectId(DisplayText);

impl ProjectId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(DisplayText::new(value))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(AsRef, Clone, Debug, Display, Eq, From, PartialEq)]
#[as_ref(forward)]
#[from(String, &str)]
pub struct ProjectMark(DisplayText);

impl ProjectMark {
    pub fn new(value: impl Into<String>) -> Self {
        Self(DisplayText::new(value))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Builder, Clone, Debug, Eq, PartialEq)]
pub struct ProjectOption {
    pub id: ProjectId,
    pub mark: ProjectMark,
    pub name: DisplayText,
    pub description: Option<DisplayText>,
    pub href: Href,
}

impl ProjectOption {
    pub fn new(
        id: impl Into<ProjectId>,
        mark: impl Into<ProjectMark>,
        name: impl Into<DisplayText>,
        href: impl Into<Href>,
    ) -> Self {
        Self::with_optional_description(id, mark, name, None, href)
    }

    pub fn with_description(
        id: impl Into<ProjectId>,
        mark: impl Into<ProjectMark>,
        name: impl Into<DisplayText>,
        description: impl Into<DisplayText>,
        href: impl Into<Href>,
    ) -> Self {
        Self::with_optional_description(id, mark, name, Some(description.into()), href)
    }

    pub fn with_optional_description(
        id: impl Into<ProjectId>,
        mark: impl Into<ProjectMark>,
        name: impl Into<DisplayText>,
        description: Option<DisplayText>,
        href: impl Into<Href>,
    ) -> Self {
        Self {
            id: id.into(),
            mark: mark.into(),
            name: name.into(),
            description,
            href: href.into(),
        }
    }
}

#[component]
pub fn ProjectLockup(project: ProjectOption, #[props(default)] compact: bool) -> Element {
    let class = if compact {
        "project-lockup is-compact"
    } else {
        "project-lockup"
    };

    rsx! {
        div { class,
            span { class: "brand-mark project-mark", "{project.mark}" }
            span { class: "brand-copy project-copy",
                if let Some(description) = project.description {
                    if !description.is_empty() {
                        span { class: "brand-kicker project-description", "{description}" }
                    }
                }
                span { class: "brand-title project-name", "{project.name}" }
            }
        }
    }
}

#[component]
pub fn ProjectSwitcher(
    selected: ReadSignal<ProjectOption>,
    projects: Vec<ProjectOption>,
    #[props(default = DisplayText::new("Project selector"), into)] label: DisplayText,
    #[props(default = DisplayText::new("Projects"), into)] list_label: DisplayText,
) -> Element {
    let _ = projects;
    let _ = list_label;
    let selected_project = selected();
    let label = label.into_string();

    rsx! {
        div {
            class: "project-switcher",
            "aria-label": label,
            ProjectLockup {
                project: selected_project,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_option_constructor_preserves_supplied_text() {
        let project = ProjectOption::with_description(
            ProjectId::new("stayhydated"),
            ProjectMark::new("SH"),
            DisplayText::new("stayhydated"),
            DisplayText::new("Home"),
            Href::new("/"),
        );

        assert_eq!(project.id.as_str(), "stayhydated");
        assert_eq!(project.mark.as_str(), "SH");
        assert_eq!(project.name.as_str(), "stayhydated");
        assert_eq!(
            project.description.as_ref().map(DisplayText::as_str),
            Some("Home")
        );
        assert_eq!(project.href.as_str(), "/");
    }
}
