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
    ProjectIdentity, ProjectPageMetadata,
};
use strum::{Display, IntoStaticStr};

#[derive(Clone, Copy, Debug, Display, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Project {
    Koruma,
    EsFluent,
    SumNumbersAi,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProjectMetadata {
    description: &'static str,
    href: &'static str,
    site_url: &'static str,
    rustdoc_href: &'static str,
    source_href: &'static str,
    skill_command: &'static str,
}

macro_rules! project_metadata {
    ($slug:literal, $description:literal) => {
        ProjectMetadata {
            description: $description,
            href: concat!("/", $slug, "/"),
            site_url: concat!("https://stayhydated.github.io/", $slug, "/"),
            rustdoc_href: concat!("https://docs.rs/", $slug, "/"),
            source_href: concat!("https://github.com/stayhydated/", $slug),
            skill_command: concat!("npx skills add stayhydated/", $slug),
        }
    };
}

const KORUMA_METADATA: ProjectMetadata = project_metadata!("koruma", "Rust validation");
const ES_FLUENT_METADATA: ProjectMetadata = project_metadata!("es-fluent", "Rust localization");
const SUM_NUMBERS_AI_METADATA: ProjectMetadata =
    project_metadata!("sum-numbers-ai", "An auditable AI addition API");

#[bon::builder(const)]
const fn metadata_with(
    #[builder(start_fn)] defaults: ProjectMetadata,
    #[builder(default = defaults.description)] description: &'static str,
    #[builder(default = defaults.href)] href: &'static str,
    #[builder(default = defaults.site_url)] site_url: &'static str,
    #[builder(default = defaults.rustdoc_href)] rustdoc_href: &'static str,
    #[builder(default = defaults.source_href)] source_href: &'static str,
    #[builder(default = defaults.skill_command)] skill_command: &'static str,
) -> ProjectMetadata {
    let _ = defaults;

    ProjectMetadata {
        description,
        href,
        site_url,
        rustdoc_href,
        source_href,
        skill_command,
    }
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

macro_rules! published_package {
    ($name:literal) => {
        ProjectPackage::new($name, concat!("https://crates.io/crates/", $name))
    };
}

impl ProjectPackage {
    pub const KORUMA: Self = published_package!("koruma");
    pub const KORUMA_COLLECTION: Self = published_package!("koruma-collection");
    pub const ES_FLUENT: Self = published_package!("es-fluent");
    pub const ES_FLUENT_MANAGER_DIOXUS: Self = published_package!("es-fluent-manager-dioxus");
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

impl Project {
    const fn metadata(self) -> ProjectMetadata {
        match self {
            Self::Koruma => metadata_with(KORUMA_METADATA).call(),
            Self::EsFluent => metadata_with(ES_FLUENT_METADATA).call(),
            Self::SumNumbersAi => metadata_with(SUM_NUMBERS_AI_METADATA)
                .rustdoc_href(DISABLED_PROJECT_HREF)
                .source_href(DISABLED_PROJECT_HREF)
                .call(),
        }
    }

    pub const fn as_str(self) -> &'static str {
        self.into_str()
    }

    pub const fn href(self) -> &'static str {
        self.metadata().href
    }

    pub const fn description(self) -> &'static str {
        self.metadata().description
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

    pub fn book_href(self) -> Href {
        self.project_file_href("book/")
    }

    fn project_file_href(self, file_name: &str) -> Href {
        Href::new(format!("{}{file_name}", self.href()))
    }

    pub const fn primary_package(self) -> ProjectPackage {
        self.packages()[0]
    }

    /// Returns the project packages with the primary package first.
    pub const fn packages(self) -> &'static [ProjectPackage] {
        match self {
            Self::Koruma => &KORUMA_PACKAGES,
            Self::EsFluent => &ES_FLUENT_PACKAGES,
            Self::SumNumbersAi => &SUM_NUMBERS_AI_PACKAGES,
        }
    }

    pub const fn support_links(self) -> &'static [ProjectSupportLink] {
        match self {
            Self::Koruma => &KORUMA_COLLECTION_SUPPORT_LINKS,
            Self::EsFluent | Self::SumNumbersAi => &[],
        }
    }

    pub fn identity(self) -> ProjectIdentity {
        let metadata = self.metadata();
        self.identity_with_href(metadata.href)
    }

    pub fn identity_with_href(self, href: impl Into<Href>) -> ProjectIdentity {
        let metadata = self.metadata();
        ProjectIdentity::with_description(self.as_str(), metadata.description, href)
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
pub(crate) fn ProjectSkillsCopyButton(project: Project) -> Element {
    let command = project.skill_command();
    let mut copied = use_signal(|| false);
    let copy_label = if copied() {
        "Copied"
    } else {
        "Copy skills command"
    };

    rsx! {
        Tooltip {
            class: "skills-copy-tooltip",
            TooltipTrigger {
                as: move |trigger_attrs: Vec<Attribute>| {
                    rsx! {
                        button {
                            class: if copied() {
                                "skills-copy-button is-copied"
                            } else {
                                "skills-copy-button"
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
                                    class: "skills-copy-icon".to_string(),
                                    width: 16,
                                    height: 16,
                                    icon: LdCopyCheck,
                                }
                            } else {
                                Icon {
                                    class: "skills-copy-icon".to_string(),
                                    width: 16,
                                    height: 16,
                                    icon: LdCopy,
                                }
                            }
                            span { class: "skills-copy-status", "{copy_label}" }
                        }
                    }
                }
            }
            TooltipContent {
                side: ContentSide::Top,
                class: "skills-command-tooltip",
                code { "{command}" }
            }
        }
    }
}

#[component]
fn ProjectFooterSkillsSection(project: Project) -> Element {
    let llms_href = project.llms_href();
    let llms_full_href = project.llms_full_href();

    rsx! {
        section { class: "footer-section footer-resources-section",
            h2 { class: "footer-section-title", "Resources" }
            ul { class: "footer-resource-list",
                li {
                    div { class: "footer-resource-row footer-skill-resource",
                        span { class: "footer-link footer-skills-link", "Skills" }
                        ProjectSkillsCopyButton { project }
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
pub fn ProjectFooterPanelForProject(project: Project) -> Element {
    rsx! {
        FooterPanel {
            ProjectFooterSkillsSection { project }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_clipboard_helper_is_a_safe_noop() {
        copy_text_to_clipboard("npx skills add stayhydated/koruma");
    }

    #[test]
    fn project_identity_uses_registry_branding() {
        let project = Project::Koruma.identity();

        assert_eq!(project.name.as_str(), "koruma");
        assert_eq!(
            project.description.as_ref().map(DisplayText::as_str),
            Some("Rust validation")
        );
        assert_eq!(project.href.as_str(), "/koruma/");
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
