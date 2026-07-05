use bon::Builder;
use derive_more::{AsRef, Display, From};
use dioxus::{
    prelude::*,
    router::{NavigationTarget, RouterContext},
};

use crate::{DisplayText, Href, select};

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

    fn text_value(&self) -> String {
        match &self.description {
            Some(description) if !description.is_empty() => format!("{} {description}", self.name),
            _ => self.name.to_string(),
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
    rsx! {
        ProjectSelect {
            selected,
            projects,
            label,
            list_label,
        }
    }
}

#[component]
pub fn ProjectSelect(
    selected: ReadSignal<ProjectOption>,
    projects: Vec<ProjectOption>,
    #[props(default = DisplayText::new("Project"), into)] label: DisplayText,
    #[props(default = DisplayText::new("Projects"), into)] list_label: DisplayText,
) -> Element {
    let selected_value = use_memo(move || Some(selected().id));
    let selected_project = selected();
    let projects = selected_project_options(selected_project.clone(), projects);
    let duplicate_id = duplicate_project_id(&projects);
    let label = label.into_string();
    let list_label = list_label.into_string();
    let selected_id = selected_project.id.clone();
    let selected_id_for_change = selected_id.clone();
    let router = try_router();

    if let Some(duplicate_id) = duplicate_id {
        return rsx! {
            div { class: "project-switcher-error",
                "duplicate project id: {duplicate_id:?}"
            }
        };
    }

    let trigger_project = projects
        .iter()
        .find(|project| project.id == selected_id)
        .cloned()
        .unwrap_or_else(|| selected_project.clone());
    let projects_for_change = projects
        .iter()
        .map(|project| {
            (
                project.id.clone(),
                NavigationTarget::from(project.href.as_str()),
            )
        })
        .collect::<Vec<_>>();
    let on_value_change = move |next_project_id: Option<ProjectId>| {
        let Some(next_project_id) = next_project_id else {
            return;
        };

        if next_project_id == selected_id_for_change {
            return;
        }

        let Some((_, target)) = projects_for_change
            .iter()
            .find(|(project_id, _)| *project_id == next_project_id)
            .cloned()
        else {
            return;
        };

        navigate_to_project(target, router);
    };

    rsx! {
        div { class: "project-switcher",
            select::Select::<ProjectId> {
                value: Some(selected_value.into()),
                on_value_change,
                select::SelectTrigger {
                    aria_label: label,
                    ProjectLockup {
                        project: trigger_project,
                    }
                }
                select::SelectList {
                    aria_label: list_label,
                    for (index, project) in projects.iter().enumerate() {
                        {
                            let active = project.id == selected_id;
                            let option_class = if active {
                                "project-select-option is-active".to_string()
                            } else {
                                "project-select-option".to_string()
                            };
                            rsx! {
                                select::SelectOption::<ProjectId> {
                                    key: "{project.id:?}",
                                    index,
                                    value: project.id.clone(),
                                    text_value: Some(project.text_value()),
                                    class: Some(option_class),
                                    ProjectLockup {
                                        project: project.clone(),
                                        compact: true,
                                    }
                                    if active {
                                        select::SelectItemIndicator {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn selected_project_options(
    selected: ProjectOption,
    mut projects: Vec<ProjectOption>,
) -> Vec<ProjectOption> {
    if let Some(project) = projects
        .iter_mut()
        .find(|project| project.id == selected.id)
    {
        *project = selected;
    } else {
        projects.insert(0, selected);
    }

    projects
}

fn duplicate_project_id(projects: &[ProjectOption]) -> Option<ProjectId> {
    let mut seen = Vec::with_capacity(projects.len());

    for project in projects {
        if seen.contains(&project.id) {
            return Some(project.id.clone());
        }

        seen.push(project.id.clone());
    }

    None
}

fn navigate_to_project(target: NavigationTarget, router: Option<RouterContext>) {
    if let Some(router) = router {
        let _ = router.push(target);
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

    #[test]
    fn selected_project_replaces_matching_project_option() {
        let selected = ProjectOption::with_description(
            ProjectId::new("koruma"),
            ProjectMark::new("K"),
            DisplayText::new("koruma"),
            DisplayText::new("Localized validation"),
            Href::new("/koruma/"),
        );

        let projects = selected_project_options(
            selected.clone(),
            vec![
                ProjectOption::with_description("stayhydated", "SH", "stayhydated", "Home", "/"),
                ProjectOption::with_description(
                    "koruma",
                    "K",
                    "koruma",
                    "Rust validation",
                    "/koruma/",
                ),
            ],
        );

        let koruma = projects
            .iter()
            .find(|project| project.id.as_str() == "koruma")
            .expect("koruma option");
        assert_eq!(koruma, &selected);
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn selected_project_is_inserted_when_missing_from_options() {
        let selected = ProjectOption::with_description(
            ProjectId::new("es-fluent"),
            ProjectMark::new("EF"),
            DisplayText::new("es-fluent"),
            DisplayText::new("Rust localization"),
            Href::new("/es-fluent/"),
        );
        let projects = vec![ProjectOption::with_description(
            "stayhydated",
            "SH",
            "stayhydated",
            "Home",
            "/",
        )];

        let projects = selected_project_options(selected.clone(), projects);

        assert_eq!(projects.first(), Some(&selected));
        assert_eq!(projects.len(), 2);
    }
}
