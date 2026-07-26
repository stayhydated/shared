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
use stayhydated_site::Project;

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
        assert_eq!(Project::SumNumbersAi.rustdoc_href(), "about:blank");
        assert_eq!(Project::SumNumbersAi.source_href(), "about:blank");
        assert_eq!(
            Project::Koruma.skill_command(),
            "npx skills add stayhydated/koruma"
        );
        assert_eq!(Project::GpuiForm.demo_path(), Some("gpui-demo/"));
    }
}
