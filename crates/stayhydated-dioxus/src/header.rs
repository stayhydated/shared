use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    Href, ProjectIdentity, ProjectNavConfig, ProjectNavItem, ProjectNavLabels,
    ProjectNavigationHeader,
};
use strum::{Display, IntoStaticStr};

use crate::Project;

#[derive(Clone, Copy, Debug, Display, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str)]
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
}

impl HeaderMessage {
    pub const fn as_str(self) -> &'static str {
        self.into_str()
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
    pub project: ProjectIdentity,
    pub home: NavigationTarget<R>,
    pub demos: NavigationTarget<R>,
    pub book: Href,
    pub docs: Href,
    pub source: Href,
    pub labels: ProjectNavLabels,
    pub active: ProjectNavItem,
}

#[bon::bon]
impl<R> StayhydatedProjectHeaderConfig<R> {
    #[builder]
    pub fn new(
        #[builder(start_fn)] project: Project,
        #[builder(default = Href::new(project.href()), into)] project_href: Href,
        home: NavigationTarget<R>,
        demos: NavigationTarget<R>,
        #[builder(default = project.book_href(), into)] book: Href,
        #[builder(default = Href::new(project.rustdoc_href()), into)] docs: Href,
        #[builder(default = Href::new(project.source_href()), into)] source: Href,
        #[builder(default = stayhydated_header_labels())] labels: ProjectNavLabels,
        #[builder(default = ProjectNavItem::Home)] active: ProjectNavItem,
    ) -> Self {
        Self {
            project: project.identity_with_href(project_href),
            home,
            demos,
            book,
            docs,
            source,
            labels,
            active,
        }
    }

    pub fn into_nav_config(self) -> ProjectNavConfig<R> {
        ProjectNavConfig::builder()
            .project(self.project)
            .home(self.home)
            .demos(self.demos)
            .book(self.book)
            .docs(self.docs)
            .source(self.source)
            .labels(self.labels)
            .active(self.active)
            .build()
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
    use stayhydated_dioxus_core::DisplayText;
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
    }

    #[test]
    fn config_uses_project_registry_defaults() {
        let config = StayhydatedProjectHeaderConfig::<()>::builder(Project::Koruma)
            .home(NavigationTarget::External("/".to_owned()))
            .demos(NavigationTarget::External("/demos/".to_owned()))
            .build();

        assert_eq!(config.project.name.as_str(), "koruma");
        assert_eq!(config.project.href.as_str(), "/koruma/");
        assert_eq!(
            config.project.description.as_ref().map(DisplayText::as_str),
            Some("Rust validation")
        );
        assert_eq!(config.source.as_str(), Project::Koruma.source_href());
        assert_eq!(config.docs.as_str(), Project::Koruma.rustdoc_href());
        assert_eq!(config.book.as_str(), Project::Koruma.book_href().as_str());
        assert_eq!(config.labels.book.as_str(), "Book");
        assert_eq!(config.labels.docs.as_str(), "Docs");

        let nav = config.into_nav_config();
        assert_eq!(nav.book.as_str(), Project::Koruma.book_href().as_str());
        assert_eq!(nav.docs.as_str(), Project::Koruma.rustdoc_href());
        assert_eq!(nav.project.href.as_str(), "/koruma/");
        assert_eq!(nav.active, ProjectNavItem::Home);
    }
}
