use derive_more::Display;
use dioxus::prelude::*;
use stayhydated_dioxus_core::{
    CssClass, DisplayText, ExternalTextLink, FooterPanel, Href, OptionalDisplayText,
    ProjectId as CoreProjectId, ProjectMark, ProjectOption, ProjectPageMetadata,
};
use strum::Display as StrumDisplay;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum Project {
    #[display("koruma")]
    Koruma,
    #[display("es-fluent")]
    EsFluent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, StrumDisplay)]
pub enum ProjectMessage {
    #[strum(to_string = "Rust validation")]
    KorumaDescription,
    #[strum(to_string = "Rust localization")]
    EsFluentDescription,
}

impl ProjectMessage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KorumaDescription => "Rust validation",
            Self::EsFluentDescription => "Rust localization",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProjectMetadata {
    mark: &'static str,
    description: &'static str,
    href: &'static str,
    site_url: &'static str,
    source_href: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectPackage {
    name: &'static str,
    crates_href: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectSupportLink {
    label: &'static str,
    href: &'static str,
}

impl ProjectPackage {
    pub const KORUMA: Self = Self::new("koruma", "https://crates.io/crates/koruma");
    pub const KORUMA_COLLECTION: Self = Self::new(
        "koruma-collection",
        "https://crates.io/crates/koruma-collection",
    );
    pub const ES_FLUENT: Self = Self::new("es-fluent", "https://crates.io/crates/es-fluent");
    pub const ES_FLUENT_MANAGER_DIOXUS: Self = Self::new(
        "es-fluent-manager-dioxus",
        "https://crates.io/crates/es-fluent-manager-dioxus",
    );

    pub const fn new(name: &'static str, crates_href: &'static str) -> Self {
        Self { name, crates_href }
    }

    pub const fn name(self) -> &'static str {
        self.name
    }

    pub const fn crates_href(self) -> &'static str {
        self.crates_href
    }

    pub fn support_links(self) -> &'static [ProjectSupportLink] {
        if self == Self::KORUMA_COLLECTION {
            &KORUMA_COLLECTION_SUPPORT_LINKS
        } else {
            &[]
        }
    }
}

impl ProjectSupportLink {
    pub const KORUMA_COLLECTION_CROWDIN: Self =
        Self::new("Crowdin", "https://crowdin.com/project/koruma-collection");

    pub const fn new(label: &'static str, href: &'static str) -> Self {
        Self { label, href }
    }

    pub const fn label(self) -> &'static str {
        self.label
    }

    pub const fn href(self) -> &'static str {
        self.href
    }
}

pub const PROJECT_FLUENT_URL: &str = "https://projectfluent.org/";

static KORUMA_PACKAGES: [ProjectPackage; 2] =
    [ProjectPackage::KORUMA, ProjectPackage::KORUMA_COLLECTION];
static ES_FLUENT_PACKAGES: [ProjectPackage; 2] = [
    ProjectPackage::ES_FLUENT,
    ProjectPackage::ES_FLUENT_MANAGER_DIOXUS,
];
static ES_FLUENT_FOOTER_PACKAGES: [ProjectPackage; 1] = [ProjectPackage::ES_FLUENT];
static KORUMA_COLLECTION_SUPPORT_LINKS: [ProjectSupportLink; 1] =
    [ProjectSupportLink::KORUMA_COLLECTION_CROWDIN];
static KORUMA_SUPPORT_LINKS: [ProjectSupportLink; 1] =
    [ProjectSupportLink::KORUMA_COLLECTION_CROWDIN];

impl Project {
    pub const ALL: [Self; 2] = [Self::Koruma, Self::EsFluent];

    const fn metadata(self) -> ProjectMetadata {
        match self {
            Self::Koruma => ProjectMetadata {
                mark: "K",
                description: "Rust validation",
                href: "/koruma/",
                site_url: "https://stayhydated.github.io/koruma/",
                source_href: "https://github.com/stayhydated/koruma",
            },
            Self::EsFluent => ProjectMetadata {
                mark: "EF",
                description: "Rust localization",
                href: "/es-fluent/",
                site_url: "https://stayhydated.github.io/es-fluent/",
                source_href: "https://github.com/stayhydated/es-fluent",
            },
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Koruma => "koruma",
            Self::EsFluent => "es-fluent",
        }
    }

    pub const fn href(self) -> &'static str {
        self.metadata().href
    }

    pub const fn source_href(self) -> &'static str {
        self.metadata().source_href
    }

    pub const fn site_url(self) -> &'static str {
        self.metadata().site_url
    }

    pub const fn primary_package(self) -> ProjectPackage {
        match self {
            Self::Koruma => ProjectPackage::KORUMA,
            Self::EsFluent => ProjectPackage::ES_FLUENT,
        }
    }

    pub const fn packages(self) -> &'static [ProjectPackage] {
        match self {
            Self::Koruma => &KORUMA_PACKAGES,
            Self::EsFluent => &ES_FLUENT_PACKAGES,
        }
    }

    pub const fn footer_packages(self) -> &'static [ProjectPackage] {
        match self {
            Self::Koruma => &KORUMA_PACKAGES,
            Self::EsFluent => &ES_FLUENT_FOOTER_PACKAGES,
        }
    }

    pub fn package_footer_links(self) -> Vec<ProjectPackageFooterLink> {
        self.footer_packages()
            .iter()
            .copied()
            .map(ProjectPackageFooterLink::from)
            .collect()
    }

    pub const fn support_links(self) -> &'static [ProjectSupportLink] {
        match self {
            Self::Koruma => &KORUMA_SUPPORT_LINKS,
            Self::EsFluent => &[],
        }
    }

    pub const fn description_message(self) -> ProjectMessage {
        match self {
            Self::Koruma => ProjectMessage::KorumaDescription,
            Self::EsFluent => ProjectMessage::EsFluentDescription,
        }
    }

    pub fn option(self) -> ProjectOption {
        let metadata = self.metadata();
        self.option_with_description(self.as_str(), metadata.description, metadata.href)
    }

    pub fn option_with_href(self, href: impl Into<Href>) -> ProjectOption {
        let metadata = self.metadata();
        self.option_with_description(self.as_str(), metadata.description, href)
    }

    pub fn option_with(self, name: impl Into<DisplayText>, href: impl Into<Href>) -> ProjectOption {
        self.option_with_optional_description(name.into(), None, href.into())
    }

    pub fn option_with_description(
        self,
        name: impl Into<DisplayText>,
        description: impl Into<DisplayText>,
        href: impl Into<Href>,
    ) -> ProjectOption {
        self.option_with_optional_description(name.into(), Some(description.into()), href.into())
    }

    pub fn option_with_message_href(
        self,
        href: impl Into<Href>,
        mut message_text: impl FnMut(ProjectMessage) -> String,
    ) -> ProjectOption {
        self.option_with_description(
            self.as_str(),
            message_text(self.description_message()),
            href,
        )
    }

    fn option_with_optional_description(
        self,
        name: DisplayText,
        description: Option<DisplayText>,
        href: Href,
    ) -> ProjectOption {
        let metadata = self.metadata();
        ProjectOption {
            id: self.into(),
            mark: ProjectMark::new(metadata.mark),
            name,
            description,
            href,
        }
    }
}

impl From<Project> for CoreProjectId {
    fn from(project: Project) -> Self {
        Self::new(project.as_str())
    }
}

impl From<Project> for ProjectOption {
    fn from(project: Project) -> Self {
        project.option()
    }
}

pub fn stayhydated_project_options() -> Vec<ProjectOption> {
    Project::ALL.into_iter().map(Project::option).collect()
}

pub fn stayhydated_project_options_with(
    mut message_text: impl FnMut(ProjectMessage) -> String,
) -> Vec<ProjectOption> {
    Project::ALL
        .into_iter()
        .map(|project| {
            let href = project.href();
            project.option_with_message_href(href, &mut message_text)
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectPackageFooterLink {
    pub package: ProjectPackage,
    pub label: OptionalDisplayText,
    pub class: CssClass,
}

impl ProjectPackageFooterLink {
    pub fn new(package: ProjectPackage) -> Self {
        Self {
            package,
            label: OptionalDisplayText::default(),
            class: CssClass::default(),
        }
    }

    pub fn with_label(mut self, label: impl Into<OptionalDisplayText>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<CssClass>) -> Self {
        self.class = class.into();
        self
    }
}

impl From<ProjectPackage> for ProjectPackageFooterLink {
    fn from(package: ProjectPackage) -> Self {
        Self::new(package)
    }
}

#[component]
pub fn ProjectSwitcher(
    project: Project,
    #[props(into)] href: Href,
    #[props(default = DisplayText::new("Project selector"), into)] label: DisplayText,
    #[props(default = DisplayText::new("Projects"), into)] list_label: DisplayText,
) -> Element {
    let selected = project.option_with_href(href);

    rsx! {
        stayhydated_dioxus_core::ProjectSwitcher {
            selected,
            projects: stayhydated_project_options(),
            label,
            list_label,
        }
    }
}

#[component]
pub fn StayhydatedProjectPageMetadata(
    project: Project,
    #[props(into)] page_title: DisplayText,
    #[props(into)] description: DisplayText,
) -> Element {
    rsx! {
        ProjectPageMetadata {
            site_name: project.as_str(),
            page_title,
            description,
        }
    }
}

#[component]
pub fn ProjectSupportTextLink(
    link: ProjectSupportLink,
    #[props(default, into)] label: OptionalDisplayText,
    #[props(default, into)] class: CssClass,
) -> Element {
    let label = label
        .into_option()
        .unwrap_or_else(|| DisplayText::new(link.label()));

    rsx! {
        ExternalTextLink {
            class,
            href: link.href(),
            label,
        }
    }
}

#[component]
pub fn ProjectSourceTextLink(
    project: Project,
    #[props(default, into)] label: OptionalDisplayText,
    #[props(default, into)] class: CssClass,
) -> Element {
    let label = label
        .into_option()
        .unwrap_or_else(|| DisplayText::new("GitHub"));

    rsx! {
        ExternalTextLink {
            class,
            href: project.source_href(),
            label,
        }
    }
}

#[component]
pub fn ProjectFluentTextLink(
    #[props(default, into)] label: OptionalDisplayText,
    #[props(default, into)] class: CssClass,
) -> Element {
    let label = label
        .into_option()
        .unwrap_or_else(|| DisplayText::new("Project Fluent"));

    rsx! {
        ExternalTextLink {
            class,
            href: PROJECT_FLUENT_URL,
            label,
        }
    }
}

#[component]
pub fn ProjectPackageTextLink(
    package: ProjectPackage,
    #[props(default, into)] label: OptionalDisplayText,
    #[props(default, into)] class: CssClass,
) -> Element {
    let label = label
        .into_option()
        .unwrap_or_else(|| DisplayText::new(package.name()));

    rsx! {
        ExternalTextLink {
            class,
            href: package.crates_href(),
            label,
        }
    }
}

#[component]
pub fn ProjectFooterCratesSection(packages: Vec<ProjectPackageFooterLink>) -> Element {
    rsx! {
        section { class: "footer-section footer-crates-section",
            h2 { class: "footer-section-title", "Crates" }
            ul { class: "footer-link-list",
                for (index, package) in packages.into_iter().enumerate() {
                    li { key: "{index}",
                        ProjectPackageTextLink {
                            package: package.package,
                            label: package.label,
                            class: package.class,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProjectFooterPanel(packages: Vec<ProjectPackageFooterLink>) -> Element {
    rsx! {
        FooterPanel {
            ProjectFooterCratesSection { packages }
        }
    }
}

#[component]
pub fn ProjectFooterPanelForProject(project: Project) -> Element {
    rsx! {
        ProjectFooterPanel {
            packages: project.package_footer_links(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_project_options_include_all_projects() {
        let projects = stayhydated_project_options();

        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].id.as_str(), "koruma");
        assert_eq!(projects[1].id.as_str(), "es-fluent");
    }

    #[test]
    fn project_messages_display_as_english_descriptions() {
        assert_eq!(
            ProjectMessage::KorumaDescription.to_string(),
            "Rust validation"
        );
        assert_eq!(
            ProjectMessage::EsFluentDescription.to_string(),
            "Rust localization"
        );
    }

    #[test]
    fn message_project_options_use_static_project_names() {
        let projects = stayhydated_project_options_with(|message| format!("{message:?}"));

        assert_eq!(projects[0].name.as_str(), "koruma");
        assert_eq!(
            projects[0].description.as_ref().map(DisplayText::as_str),
            Some("KorumaDescription")
        );
    }

    #[test]
    fn project_metadata_exposes_site_and_package_urls() {
        assert_eq!(
            Project::Koruma.site_url(),
            "https://stayhydated.github.io/koruma/"
        );
        assert_eq!(
            Project::EsFluent.primary_package(),
            ProjectPackage::ES_FLUENT
        );
        assert_eq!(
            ProjectPackage::KORUMA_COLLECTION.crates_href(),
            "https://crates.io/crates/koruma-collection"
        );
    }

    #[test]
    fn project_package_sets_include_shared_public_packages() {
        assert_eq!(
            Project::Koruma.packages(),
            &[ProjectPackage::KORUMA, ProjectPackage::KORUMA_COLLECTION]
        );
        assert_eq!(
            Project::EsFluent.packages(),
            &[
                ProjectPackage::ES_FLUENT,
                ProjectPackage::ES_FLUENT_MANAGER_DIOXUS,
            ]
        );
    }

    #[test]
    fn project_package_footer_links_use_footer_packages() {
        assert_eq!(
            Project::Koruma
                .package_footer_links()
                .into_iter()
                .map(|link| link.package)
                .collect::<Vec<_>>(),
            vec![ProjectPackage::KORUMA, ProjectPackage::KORUMA_COLLECTION]
        );
        assert_eq!(
            Project::EsFluent
                .package_footer_links()
                .into_iter()
                .map(|link| link.package)
                .collect::<Vec<_>>(),
            vec![ProjectPackage::ES_FLUENT]
        );
    }

    #[test]
    fn project_support_links_include_shared_crowdin_project() {
        assert_eq!(
            ProjectPackage::KORUMA_COLLECTION.support_links(),
            &[ProjectSupportLink::KORUMA_COLLECTION_CROWDIN]
        );
        assert_eq!(
            Project::Koruma.support_links(),
            &[ProjectSupportLink::KORUMA_COLLECTION_CROWDIN]
        );
        assert_eq!(Project::EsFluent.support_links(), &[]);
        assert_eq!(
            ProjectSupportLink::KORUMA_COLLECTION_CROWDIN.href(),
            "https://crowdin.com/project/koruma-collection"
        );
        assert_eq!(
            ProjectSupportLink::KORUMA_COLLECTION_CROWDIN.label(),
            "Crowdin"
        );
        assert_eq!(PROJECT_FLUENT_URL, "https://projectfluent.org/");
        assert_eq!(
            Project::Koruma.source_href(),
            "https://github.com/stayhydated/koruma"
        );
    }

    #[test]
    fn package_footer_link_supports_custom_label_and_class() {
        let link = ProjectPackageFooterLink::new(ProjectPackage::ES_FLUENT_MANAGER_DIOXUS)
            .with_label("Dioxus manager")
            .with_class("footer-link");

        assert_eq!(link.package, ProjectPackage::ES_FLUENT_MANAGER_DIOXUS);
        assert_eq!(
            link.label.into_option().map(|label| label.into_string()),
            Some("Dioxus manager".to_string())
        );
        assert_eq!(link.class.as_str(), "footer-link");
    }
}
