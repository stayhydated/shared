use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    DisplayText, Href, LinkTarget, ProjectNavConfig, ProjectNavItem, ProjectNavLabels,
    ProjectNavigationHeader, ProjectOption,
};
use strum::Display;

use crate::{Project, stayhydated_project_options};

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum HeaderMessage {
    #[strum(to_string = "Home")]
    NavHome,
    #[strum(to_string = "Demos")]
    NavDemos,
    #[strum(to_string = "Book")]
    NavBook,
    #[strum(to_string = "Docs")]
    NavDocs,
    #[strum(to_string = "Source")]
    NavSource,
    #[strum(to_string = "Project selector")]
    ProjectSelector,
    #[strum(to_string = "Projects")]
    Projects,
    #[strum(to_string = "Language")]
    Language,
}

impl HeaderMessage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NavHome => "Home",
            Self::NavDemos => "Demos",
            Self::NavBook => "Book",
            Self::NavDocs => "Docs",
            Self::NavSource => "Source",
            Self::ProjectSelector => "Project selector",
            Self::Projects => "Projects",
            Self::Language => "Language",
        }
    }
}

pub fn stayhydated_header_labels() -> ProjectNavLabels {
    stayhydated_header_labels_with(|message| message.to_string())
}

pub fn stayhydated_header_labels_with(
    mut label_for: impl FnMut(HeaderMessage) -> String,
) -> ProjectNavLabels {
    ProjectNavLabels::new(
        label_for(HeaderMessage::NavHome),
        label_for(HeaderMessage::NavDemos),
        label_for(HeaderMessage::NavBook),
        label_for(HeaderMessage::NavDocs),
        label_for(HeaderMessage::NavSource),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StayhydatedProjectHeaderConfig<R> {
    pub project: ProjectOption,
    pub project_options: Vec<ProjectOption>,
    pub project_label: DisplayText,
    pub project_list_label: DisplayText,
    pub home: LinkTarget<R>,
    pub demos: LinkTarget<R>,
    pub book: Href,
    pub docs: Href,
    pub source: Href,
    pub labels: ProjectNavLabels,
    pub active: ProjectNavItem,
}

impl<R> StayhydatedProjectHeaderConfig<R> {
    pub fn new(
        project: Project,
        project_href: impl Into<Href>,
        home: LinkTarget<R>,
        demos: LinkTarget<R>,
        book: impl Into<Href>,
        labels: ProjectNavLabels,
        active: ProjectNavItem,
    ) -> Self {
        Self {
            project: project.option_with_href(project_href),
            project_options: stayhydated_project_options(),
            project_label: DisplayText::new(HeaderMessage::ProjectSelector.as_str()),
            project_list_label: DisplayText::new(HeaderMessage::Projects.as_str()),
            home,
            demos,
            book: book.into(),
            docs: Href::new(project.rustdoc_href()),
            source: Href::new(project.source_href()),
            labels,
            active,
        }
    }

    pub fn with_project_options(mut self, project_options: Vec<ProjectOption>) -> Self {
        self.project_options = project_options;
        self
    }

    pub fn with_source(mut self, source: impl Into<Href>) -> Self {
        self.source = source.into();
        self
    }

    pub fn with_docs(mut self, docs: impl Into<Href>) -> Self {
        self.docs = docs.into();
        self
    }

    pub fn with_project_labels(
        mut self,
        label: impl Into<DisplayText>,
        list_label: impl Into<DisplayText>,
    ) -> Self {
        self.project_label = label.into();
        self.project_list_label = list_label.into();
        self
    }

    pub fn with_labels(mut self, labels: ProjectNavLabels) -> Self {
        self.labels = labels;
        self
    }

    pub fn with_active(mut self, active: ProjectNavItem) -> Self {
        self.active = active;
        self
    }

    pub fn into_nav_config(self) -> ProjectNavConfig<R> {
        ProjectNavConfig::new(
            self.project,
            self.home,
            self.demos,
            self.book,
            self.docs,
            self.source,
            self.labels,
            self.active,
        )
        .with_project_options(self.project_options)
        .with_project_labels(self.project_label, self.project_list_label)
    }
}

#[component]
pub fn StayhydatedProjectHeader<R: Routable + Clone + PartialEq + 'static>(
    config: StayhydatedProjectHeaderConfig<R>,
    children: Element,
) -> Element {
    let nav = config.into_nav_config();

    rsx! {
        ProjectNavigationHeader::<R> {
            nav,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn header_labels_use_shared_book_slot() {
        let labels = stayhydated_header_labels_with(|message| format!("{message:?}"));

        assert_eq!(labels.home.as_str(), "NavHome");
        assert_eq!(labels.demos.as_str(), "NavDemos");
        assert_eq!(labels.book.as_str(), "NavBook");
        assert_eq!(labels.docs.as_str(), "NavDocs");
        assert_eq!(labels.source.as_str(), "NavSource");
    }

    #[test]
    fn header_message_display_is_english() {
        assert_eq!(HeaderMessage::NavHome.to_string(), "Home");
        assert_eq!(HeaderMessage::NavDocs.to_string(), "Docs");
        assert_eq!(
            HeaderMessage::ProjectSelector.to_string(),
            "Project selector"
        );
    }

    #[test]
    fn config_uses_project_registry_defaults() {
        let config = StayhydatedProjectHeaderConfig::<()>::new(
            Project::Koruma,
            "/koruma/",
            LinkTarget::href("/"),
            LinkTarget::href("/demos/"),
            "/book/",
            stayhydated_header_labels_with(|message| format!("{message:?}")),
            ProjectNavItem::Home,
        );

        assert_eq!(config.project.id.as_str(), "koruma");
        assert_eq!(config.project.href.as_str(), "/koruma/");
        assert_eq!(
            config.project.description.as_ref().map(DisplayText::as_str),
            Some("Rust validation")
        );
        assert_eq!(config.project_options.len(), Project::ALL.len());
        assert_eq!(config.project_label.as_str(), "Project selector");
        assert_eq!(config.project_list_label.as_str(), "Projects");
        assert_eq!(config.source.as_str(), Project::Koruma.source_href());
        assert_eq!(config.docs.as_str(), Project::Koruma.rustdoc_href());
        assert_eq!(config.labels.book.as_str(), "NavBook");
        assert_eq!(config.labels.docs.as_str(), "NavDocs");

        let nav = config.into_nav_config();
        assert_eq!(nav.book.as_str(), "/book/");
        assert_eq!(nav.docs.as_str(), Project::Koruma.rustdoc_href());
        assert_eq!(nav.project_label.as_str(), "Project selector");
        assert_eq!(nav.project_list_label.as_str(), "Projects");
        assert_eq!(nav.active, ProjectNavItem::Home);
    }
}
