use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCopy, LdCopyCheck},
};
use dioxus_primitives::{
    ContentSide,
    tooltip::{Tooltip, TooltipContent, TooltipTrigger},
};
use stayhydated_dioxus_core::{
    CssClass, DisplayText, ExternalTextLink, FooterPanel, Href, OptionalDisplayText,
    ProjectId as CoreProjectId, ProjectMark, ProjectOption, ProjectPageMetadata,
};
use strum::{Display, IntoStaticStr};

#[derive(Clone, Copy, Debug, Display, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Project {
    Koruma,
    EsFluent,
    SumNumbersAi,
}

#[derive(Clone, Copy, Debug, Display, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str)]
pub enum ProjectMessage {
    #[strum(to_string = "Rust validation")]
    KorumaDescription,
    #[strum(to_string = "Rust localization")]
    EsFluentDescription,
    #[strum(to_string = "AI-assisted arithmetic")]
    SumNumbersAiDescription,
}

impl ProjectMessage {
    pub const fn as_str(self) -> &'static str {
        self.into_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProjectMetadata {
    mark: &'static str,
    description: &'static str,
    href: &'static str,
    site_url: &'static str,
    rustdoc_href: &'static str,
    source_href: &'static str,
    skill_command: &'static str,
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
    pub const SUM_NUMBERS_AI_DUMMY: Self = Self::new("sum-numbers-ai-dummy", DISABLED_PROJECT_HREF);

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
pub const DISABLED_PROJECT_HREF: &str = "about:blank";

static KORUMA_PACKAGES: [ProjectPackage; 2] =
    [ProjectPackage::KORUMA, ProjectPackage::KORUMA_COLLECTION];
static ES_FLUENT_PACKAGES: [ProjectPackage; 2] = [
    ProjectPackage::ES_FLUENT,
    ProjectPackage::ES_FLUENT_MANAGER_DIOXUS,
];
static SUM_NUMBERS_AI_PACKAGES: [ProjectPackage; 1] = [ProjectPackage::SUM_NUMBERS_AI_DUMMY];
static KORUMA_COLLECTION_SUPPORT_LINKS: [ProjectSupportLink; 1] =
    [ProjectSupportLink::KORUMA_COLLECTION_CROWDIN];
static KORUMA_SUPPORT_LINKS: [ProjectSupportLink; 1] =
    [ProjectSupportLink::KORUMA_COLLECTION_CROWDIN];

impl Project {
    pub const ALL: [Self; 3] = [Self::Koruma, Self::EsFluent, Self::SumNumbersAi];

    const fn metadata(self) -> ProjectMetadata {
        match self {
            Self::Koruma => ProjectMetadata {
                mark: "K",
                description: "Rust validation",
                href: "/koruma/",
                site_url: "https://stayhydated.github.io/koruma/",
                rustdoc_href: "https://docs.rs/koruma/",
                source_href: "https://github.com/stayhydated/koruma",
                skill_command: "npx skills add stayhydated/koruma",
            },
            Self::EsFluent => ProjectMetadata {
                mark: "EF",
                description: "Rust localization",
                href: "/es-fluent/",
                site_url: "https://stayhydated.github.io/es-fluent/",
                rustdoc_href: "https://docs.rs/es-fluent/",
                source_href: "https://github.com/stayhydated/es-fluent",
                skill_command: "npx skills add stayhydated/es-fluent",
            },
            Self::SumNumbersAi => ProjectMetadata {
                mark: "SN",
                description: "AI-assisted arithmetic",
                href: "/sum-numbers-ai/",
                site_url: "https://stayhydated.github.io/sum-numbers-ai/",
                rustdoc_href: DISABLED_PROJECT_HREF,
                source_href: DISABLED_PROJECT_HREF,
                skill_command: "npx skills add stayhydated/sum-numbers-ai",
            },
        }
    }

    pub const fn as_str(self) -> &'static str {
        self.into_str()
    }

    pub const fn href(self) -> &'static str {
        self.metadata().href
    }

    pub const fn source_href(self) -> &'static str {
        self.metadata().source_href
    }

    pub const fn rustdoc_href(self) -> &'static str {
        self.metadata().rustdoc_href
    }

    pub const fn site_url(self) -> &'static str {
        self.metadata().site_url
    }

    pub const fn skill_command(self) -> &'static str {
        self.metadata().skill_command
    }

    pub fn llms_href(self) -> Href {
        self.project_file_href("llms.txt")
    }

    pub fn llms_full_href(self) -> Href {
        self.project_file_href("llms-full.txt")
    }

    fn project_file_href(self, file_name: &str) -> Href {
        Href::new(format!("{}{file_name}", self.href()))
    }

    pub const fn primary_package(self) -> ProjectPackage {
        match self {
            Self::Koruma => ProjectPackage::KORUMA,
            Self::EsFluent => ProjectPackage::ES_FLUENT,
            Self::SumNumbersAi => ProjectPackage::SUM_NUMBERS_AI_DUMMY,
        }
    }

    pub const fn packages(self) -> &'static [ProjectPackage] {
        match self {
            Self::Koruma => &KORUMA_PACKAGES,
            Self::EsFluent => &ES_FLUENT_PACKAGES,
            Self::SumNumbersAi => &SUM_NUMBERS_AI_PACKAGES,
        }
    }

    pub const fn support_links(self) -> &'static [ProjectSupportLink] {
        match self {
            Self::Koruma => &KORUMA_SUPPORT_LINKS,
            Self::EsFluent | Self::SumNumbersAi => &[],
        }
    }

    pub const fn description_message(self) -> ProjectMessage {
        match self {
            Self::Koruma => ProjectMessage::KorumaDescription,
            Self::EsFluent => ProjectMessage::EsFluentDescription,
            Self::SumNumbersAi => ProjectMessage::SumNumbersAiDescription,
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
pub fn ProjectFooterSkillsSection(project: Project) -> Element {
    let command = project.skill_command();
    let llms_href = project.llms_href();
    let llms_full_href = project.llms_full_href();
    let mut copied = use_signal(|| false);
    let copy_label = if copied() {
        "Copied"
    } else {
        "Copy skills command"
    };

    rsx! {
        section { class: "footer-section footer-resources-section",
            h2 { class: "footer-section-title", "Resources" }
            ul { class: "footer-resource-list",
                li {
                    div { class: "footer-resource-row footer-skill-resource",
                        span { class: "footer-link footer-skills-link", "Skills" }
                        Tooltip {
                            class: "footer-copy-tooltip",
                            TooltipTrigger {
                                as: move |trigger_attrs: Vec<Attribute>| {
                                    rsx! {
                                        button {
                                            class: if copied() {
                                                "footer-copy-button is-copied"
                                            } else {
                                                "footer-copy-button"
                                            },
                                            r#type: "button",
                                            "aria-label": copy_label,
                                            onclick: move |_| {
                                                copy_text_to_clipboard(command);
                                                copied.set(true);
                                            },
                                            ..trigger_attrs,
                                            if copied() {
                                                Icon {
                                                    class: "footer-copy-icon".to_string(),
                                                    width: 16,
                                                    height: 16,
                                                    icon: LdCopyCheck,
                                                }
                                            } else {
                                                Icon {
                                                    class: "footer-copy-icon".to_string(),
                                                    width: 16,
                                                    height: 16,
                                                    icon: LdCopy,
                                                }
                                            }
                                            span { class: "footer-copy-status", "{copy_label}" }
                                        }
                                    }
                                }
                            }
                            TooltipContent {
                                side: ContentSide::Top,
                                class: "footer-command-tooltip",
                                code { "{command}" }
                            }
                        }
                    }
                }
                li {
                    div { class: "footer-resource-row",
                        a {
                            class: "footer-link",
                            href: llms_href.as_str(),
                            "llms.txt"
                        }
                    }
                }
                li {
                    div { class: "footer-resource-row",
                        a {
                            class: "footer-link",
                            href: llms_full_href.as_str(),
                            "llms-full.txt"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn copy_text_to_clipboard(value: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_text_to_clipboard(_value: &str) {}

#[component]
pub fn ProjectFooterPanel(project: Project) -> Element {
    rsx! {
        FooterPanel {
            ProjectFooterSkillsSection {
                project,
            }
        }
    }
}

#[component]
pub fn ProjectFooterPanelForProject(project: Project) -> Element {
    rsx! {
        ProjectFooterPanel {
            project,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_project_options_include_all_projects() {
        let projects = stayhydated_project_options();

        assert_eq!(projects.len(), 3);
        assert_eq!(projects[0].id.as_str(), "koruma");
        assert_eq!(projects[1].id.as_str(), "es-fluent");
        assert_eq!(projects[2].id.as_str(), "sum-numbers-ai");
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
        assert_eq!(
            ProjectMessage::SumNumbersAiDescription.to_string(),
            "AI-assisted arithmetic"
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
        assert_eq!(Project::Koruma.rustdoc_href(), "https://docs.rs/koruma/");
        assert_eq!(
            Project::EsFluent.rustdoc_href(),
            "https://docs.rs/es-fluent/"
        );
        assert_eq!(Project::Koruma.llms_href().as_str(), "/koruma/llms.txt");
        assert_eq!(
            Project::EsFluent.llms_full_href().as_str(),
            "/es-fluent/llms-full.txt"
        );
        assert_eq!(
            Project::SumNumbersAi.site_url(),
            "https://stayhydated.github.io/sum-numbers-ai/"
        );
        assert_eq!(Project::SumNumbersAi.rustdoc_href(), DISABLED_PROJECT_HREF);
        assert_eq!(Project::SumNumbersAi.source_href(), DISABLED_PROJECT_HREF);
        assert_eq!(
            Project::Koruma.skill_command(),
            "npx skills add stayhydated/koruma"
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
        assert_eq!(
            Project::SumNumbersAi.packages(),
            &[ProjectPackage::SUM_NUMBERS_AI_DUMMY]
        );
        assert_eq!(
            Project::SumNumbersAi.primary_package(),
            ProjectPackage::SUM_NUMBERS_AI_DUMMY
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
        assert_eq!(Project::SumNumbersAi.support_links(), &[]);
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
}
