use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCopy, LdCopyCheck},
};
use dioxus_primitives::{
    ContentSide,
    tooltip::{Tooltip, TooltipContent, TooltipTrigger},
};
use stayhydated_dioxus_core::{DisplayText, Href, ProjectPageMetadata};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Project {
    Koruma,
    EsFluent,
    SumNumbersAi,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProjectMetadata {
    name: &'static str,
    description: &'static str,
    href: &'static str,
    site_url: &'static str,
    rustdoc_href: &'static str,
    source_href: &'static str,
    skill_command: &'static str,
}

macro_rules! published_project_metadata {
    ($slug:literal, $description:literal) => {
        ProjectMetadata {
            name: $slug,
            description: $description,
            href: concat!("/", $slug, "/"),
            site_url: concat!("https://stayhydated.github.io/", $slug, "/"),
            rustdoc_href: concat!("https://docs.rs/", $slug, "/"),
            source_href: concat!("https://github.com/stayhydated/", $slug),
            skill_command: concat!("npx skills add stayhydated/", $slug),
        }
    };
}

const DISABLED_PROJECT_HREF: &str = "about:blank";
const KORUMA_METADATA: ProjectMetadata = published_project_metadata!("koruma", "Rust validation");
const ES_FLUENT_METADATA: ProjectMetadata =
    published_project_metadata!("es-fluent", "Rust localization");
const SUM_NUMBERS_AI_METADATA: ProjectMetadata = ProjectMetadata {
    rustdoc_href: DISABLED_PROJECT_HREF,
    source_href: DISABLED_PROJECT_HREF,
    ..published_project_metadata!("sum-numbers-ai", "An auditable AI addition API")
};

impl Project {
    const fn metadata(self) -> ProjectMetadata {
        match self {
            Self::Koruma => KORUMA_METADATA,
            Self::EsFluent => ES_FLUENT_METADATA,
            Self::SumNumbersAi => SUM_NUMBERS_AI_METADATA,
        }
    }

    pub const fn as_str(self) -> &'static str {
        self.metadata().name
    }

    pub const fn site_url(self) -> &'static str {
        self.metadata().site_url
    }

    pub(crate) const fn description(self) -> &'static str {
        self.metadata().description
    }

    pub(crate) const fn source_href(self) -> &'static str {
        self.metadata().source_href
    }

    pub(crate) const fn rustdoc_href(self) -> &'static str {
        self.metadata().rustdoc_href
    }

    pub(crate) const fn skill_command(self) -> &'static str {
        self.metadata().skill_command
    }

    pub(crate) fn book_href(self) -> Href {
        Href::new(format!("{}book/", self.metadata().href))
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

#[cfg(target_arch = "wasm32")]
fn copy_text_to_clipboard(value: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_text_to_clipboard(_value: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_clipboard_helper_is_a_safe_noop() {
        copy_text_to_clipboard("npx skills add stayhydated/koruma");
    }

    #[test]
    fn project_metadata_exposes_current_site_destinations() {
        assert_eq!(Project::Koruma.as_str(), "koruma");
        assert_eq!(Project::Koruma.description(), "Rust validation");
        assert_eq!(
            Project::Koruma.site_url(),
            "https://stayhydated.github.io/koruma/"
        );
        assert_eq!(Project::Koruma.rustdoc_href(), "https://docs.rs/koruma/");
        assert_eq!(
            Project::EsFluent.source_href(),
            "https://github.com/stayhydated/es-fluent"
        );
        assert_eq!(Project::EsFluent.book_href().as_str(), "/es-fluent/book/");
        assert_eq!(Project::SumNumbersAi.rustdoc_href(), DISABLED_PROJECT_HREF);
        assert_eq!(Project::SumNumbersAi.source_href(), DISABLED_PROJECT_HREF);
        assert_eq!(
            Project::Koruma.skill_command(),
            "npx skills add stayhydated/koruma"
        );
    }
}
