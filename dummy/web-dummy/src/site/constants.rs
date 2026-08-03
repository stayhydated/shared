use stayhydated_dioxus::{Project, ProjectSite};

pub(crate) const PROJECT: Project = Project::new("sum-numbers-ai", "An auditable AI addition API")
    .with_skill_command("npx skills add stayhydated/sum-numbers-ai");
pub const SITE_URL: &str = "https://stayhydated.github.io/sum-numbers-ai/";
pub(crate) const SOURCE_URL: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/tree/master/dummy/sum-numbers-ai-dummy"
);
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn site() -> ProjectSite {
    ProjectSite::builder()
        .project(PROJECT)
        .site_url(SITE_URL)
        .rustdoc_url("about:blank")
        .source_url(SOURCE_URL)
        .version(VERSION)
        .site_stylesheet_path("assets/site.css")
        .build()
}
