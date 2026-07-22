use dioxus::prelude::*;

use crate::{DisplayText, Href};

/// Project branding rendered by shared navigation components.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectIdentity {
    pub name: DisplayText,
    pub description: Option<DisplayText>,
    pub href: Href,
}

impl ProjectIdentity {
    pub fn new(name: impl Into<DisplayText>, href: impl Into<Href>) -> Self {
        Self::with_optional_description(name, None, href)
    }

    pub fn with_description(
        name: impl Into<DisplayText>,
        description: impl Into<DisplayText>,
        href: impl Into<Href>,
    ) -> Self {
        Self::with_optional_description(name, Some(description.into()), href)
    }

    pub fn with_optional_description(
        name: impl Into<DisplayText>,
        description: Option<DisplayText>,
        href: impl Into<Href>,
    ) -> Self {
        Self {
            name: name.into(),
            description,
            href: href.into(),
        }
    }
}

#[component]
pub fn ProjectLockup(project: ProjectIdentity, #[props(default)] compact: bool) -> Element {
    let class = if compact {
        "project-lockup is-compact"
    } else {
        "project-lockup"
    };

    rsx! {
        a { class, href: project.href.as_str(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_identity_preserves_supplied_branding() {
        let project = ProjectIdentity::with_description("shared", "Shared crates", "/shared/");

        assert_eq!(project.name.as_str(), "shared");
        assert_eq!(
            project.description.as_ref().map(DisplayText::as_str),
            Some("Shared crates")
        );
        assert_eq!(project.href.as_str(), "/shared/");
    }
}
