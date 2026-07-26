use crate::routing::{Href, SiteUrl};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Project {
    GpuiForm,
    GpuiTable,
    GpuiStorybook,
    GpuiEsFluent,
    Koruma,
    EsFluent,
    FrameCapture,
    SumNumbersAi,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProjectMetadata {
    slug: &'static str,
    display_name: &'static str,
    description: &'static str,
    category: &'static str,
    site_url: &'static str,
    rustdoc_href: &'static str,
    source_href: &'static str,
    skill_command: &'static str,
    demo_path: Option<&'static str>,
}

macro_rules! published_project_metadata {
    (
        $slug:literal,
        $display_name:literal,
        $description:literal,
        $category:literal,
        $demo_path:expr
    ) => {
        ProjectMetadata {
            slug: $slug,
            display_name: $display_name,
            description: $description,
            category: $category,
            site_url: concat!("https://stayhydated.github.io/", $slug, "/"),
            rustdoc_href: concat!("https://docs.rs/", $slug, "/"),
            source_href: concat!("https://github.com/stayhydated/", $slug),
            skill_command: concat!("npx skills add stayhydated/", $slug),
            demo_path: $demo_path,
        }
    };
}

const DISABLED_PROJECT_HREF: &str = "about:blank";

const GPUI_FORM_METADATA: ProjectMetadata = published_project_metadata!(
    "gpui-form",
    "gpui-form",
    "Derive typed, component-backed GPUI forms from application models.",
    "Rust UI",
    Some("gpui-demo/")
);
const GPUI_TABLE_METADATA: ProjectMetadata = published_project_metadata!(
    "gpui-table",
    "gpui-table",
    "Derive typed GPUI tables, filters, and query contracts from row models.",
    "Rust UI",
    Some("gpui-demo/")
);
const GPUI_STORYBOOK_METADATA: ProjectMetadata = published_project_metadata!(
    "gpui-storybook",
    "GPUI Storybook",
    "Discover, render, and automate component stories for GPUI applications.",
    "Rust UI",
    Some("gpui-demo/")
);
const GPUI_ES_FLUENT_METADATA: ProjectMetadata = published_project_metadata!(
    "gpui-es-fluent",
    "gpui-es-fluent",
    "Typed es-fluent localization stored in GPUI global state.",
    "Rust UI",
    Some("gpui-demo/")
);
const KORUMA_METADATA: ProjectMetadata = published_project_metadata!(
    "koruma",
    "koruma",
    "Rust validation",
    "Rust libraries",
    Some("demos/")
);
const ES_FLUENT_METADATA: ProjectMetadata = published_project_metadata!(
    "es-fluent",
    "es-fluent",
    "Rust localization",
    "Rust libraries",
    Some("demos/")
);
const FRAME_CAPTURE_METADATA: ProjectMetadata = published_project_metadata!(
    "frame-capture",
    "frame-capture",
    "Typed routes, scenarios, sizes, and output paths for deterministic captures.",
    "Visual tooling",
    None
);
const SUM_NUMBERS_AI_METADATA: ProjectMetadata = ProjectMetadata {
    rustdoc_href: DISABLED_PROJECT_HREF,
    source_href: DISABLED_PROJECT_HREF,
    ..published_project_metadata!(
        "sum-numbers-ai",
        "sum-numbers-ai",
        "An auditable AI addition API",
        "AI tooling",
        Some("demos/")
    )
};

impl Project {
    const fn metadata(self) -> ProjectMetadata {
        match self {
            Self::GpuiForm => GPUI_FORM_METADATA,
            Self::GpuiTable => GPUI_TABLE_METADATA,
            Self::GpuiStorybook => GPUI_STORYBOOK_METADATA,
            Self::GpuiEsFluent => GPUI_ES_FLUENT_METADATA,
            Self::Koruma => KORUMA_METADATA,
            Self::EsFluent => ES_FLUENT_METADATA,
            Self::FrameCapture => FRAME_CAPTURE_METADATA,
            Self::SumNumbersAi => SUM_NUMBERS_AI_METADATA,
        }
    }

    pub const fn as_str(self) -> &'static str {
        self.metadata().slug
    }

    pub const fn display_name(self) -> &'static str {
        self.metadata().display_name
    }

    pub const fn description(self) -> &'static str {
        self.metadata().description
    }

    pub const fn category(self) -> &'static str {
        self.metadata().category
    }

    pub const fn site_url(self) -> &'static str {
        self.metadata().site_url
    }

    pub fn typed_site_url(self) -> SiteUrl {
        SiteUrl::new(self.site_url())
    }

    pub const fn rustdoc_href(self) -> &'static str {
        self.metadata().rustdoc_href
    }

    pub const fn source_href(self) -> &'static str {
        self.metadata().source_href
    }

    pub const fn skill_command(self) -> &'static str {
        self.metadata().skill_command
    }

    pub const fn demo_path(self) -> Option<&'static str> {
        self.metadata().demo_path
    }

    pub fn book_href(self) -> Href {
        Href::new(format!("/{}/book/", self.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_projects_expose_consistent_destinations() {
        assert_eq!(Project::GpuiForm.as_str(), "gpui-form");
        assert_eq!(Project::GpuiStorybook.display_name(), "GPUI Storybook");
        assert_eq!(Project::FrameCapture.category(), "Visual tooling");
        assert_eq!(
            Project::GpuiTable.site_url(),
            "https://stayhydated.github.io/gpui-table/"
        );
        assert_eq!(
            Project::GpuiEsFluent.rustdoc_href(),
            "https://docs.rs/gpui-es-fluent/"
        );
        assert_eq!(
            Project::Koruma.source_href(),
            "https://github.com/stayhydated/koruma"
        );
        assert_eq!(
            Project::EsFluent.skill_command(),
            "npx skills add stayhydated/es-fluent"
        );
        assert_eq!(Project::GpuiForm.demo_path(), Some("gpui-demo/"));
        assert_eq!(Project::FrameCapture.demo_path(), None);
        assert_eq!(Project::Koruma.book_href().as_str(), "/koruma/book/");
    }

    #[test]
    fn unpublished_dummy_destinations_are_disabled() {
        assert_eq!(Project::SumNumbersAi.rustdoc_href(), DISABLED_PROJECT_HREF);
        assert_eq!(Project::SumNumbersAi.source_href(), DISABLED_PROJECT_HREF);
    }
}
