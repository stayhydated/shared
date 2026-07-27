use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCopy, LdCopyCheck},
};
use dioxus_primitives::{
    ContentSide,
    tooltip::{Tooltip, TooltipContent, TooltipTrigger},
};
use stayhydated_dioxus_core::{DisplayText, ProjectPageMetadata};

/// Consumer-owned identity used by the shared project-site wrappers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Project {
    name: &'static str,
    description: &'static str,
    skill_command: Option<&'static str>,
}

impl Project {
    /// Creates a project identity with no Skills command.
    pub const fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            skill_command: None,
        }
    }

    /// Adds the command shown by the portal's Skills copy button.
    pub const fn with_skill_command(mut self, skill_command: &'static str) -> Self {
        self.skill_command = Some(skill_command);
        self
    }

    pub const fn as_str(self) -> &'static str {
        self.name
    }

    pub const fn description(self) -> &'static str {
        self.description
    }

    pub const fn skill_command(self) -> Option<&'static str> {
        self.skill_command
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
pub(crate) fn ProjectSkillsCopyButton(command: &'static str) -> Element {
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
        copy_text_to_clipboard("npx skills add example/project");
    }

    #[test]
    fn project_identity_uses_consumer_owned_values() {
        const PROJECT: Project = Project::new("example-project", "An example project.")
            .with_skill_command("npx skills add example/project");

        assert_eq!(PROJECT.as_str(), "example-project");
        assert_eq!(PROJECT.description(), "An example project.");
        assert_eq!(
            PROJECT.skill_command(),
            Some("npx skills add example/project")
        );
    }

    #[test]
    fn project_identity_does_not_require_a_skills_command() {
        const PROJECT: Project = Project::new("example-project", "An example project.");

        assert_eq!(PROJECT.skill_command(), None);
    }
}
