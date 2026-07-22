use stayhydated_dioxus::Project;

pub(crate) const PROJECT: Project = Project::SumNumbersAi;
pub const SITE_URL: &str = PROJECT.site_url();
pub(crate) const SOURCE_URL: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/tree/master/dummy/sum-numbers-ai-dummy"
);
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
