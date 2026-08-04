use crate::{
    routing::{BaseHref, Href, RoutePath, SiteUrl},
    sitemap,
};
use dioxus::prelude::Routable;

const PROJECT_STATIC_PATHS: [&str; 3] = ["/book/", "/llms.txt", "/llms-full.txt"];

/// Canonical application and static-output routes for one generated site.
///
/// Application routes receive fallback `index.html` files during assembly.
/// Both application and static paths are included in the sitemap.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteRouteManifest {
    site_url: SiteUrl,
    application_paths: Vec<Href>,
    static_paths: Vec<Href>,
}

impl SiteRouteManifest {
    /// Creates a manifest with no implied generated outputs.
    pub fn new<I, P>(site_url: SiteUrl, application_paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Href>,
    {
        Self {
            site_url,
            application_paths: collect_unique_paths(application_paths),
            static_paths: Vec::new(),
        }
    }

    /// Creates a project-site manifest including the standard book and LLM outputs.
    pub fn project<I, P>(site_url: SiteUrl, application_paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Href>,
    {
        Self::new(site_url, application_paths).with_static_paths(PROJECT_STATIC_PATHS)
    }

    /// Creates a project-site manifest from the statically enumerable routes in `R`.
    ///
    /// Dynamic routes are not returned by [`Routable::static_routes`] and must be
    /// added through [`Self::new`] or another explicit manifest source when they
    /// need generated fallback files.
    pub fn project_from_routable<R>(site_url: SiteUrl) -> Self
    where
        R: Routable,
    {
        Self::project(
            site_url,
            R::static_routes()
                .into_iter()
                .map(|route| Href::new(route.to_string())),
        )
    }

    /// Adds generated or copied static outputs to the sitemap.
    pub fn with_static_paths<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Href>,
    {
        for path in paths {
            push_unique_path(&mut self.static_paths, path.into());
        }
        self
    }

    /// Returns the canonical site URL.
    pub fn site_url(&self) -> &SiteUrl {
        &self.site_url
    }

    /// Returns routes that need generated fallback files.
    pub fn application_paths(&self) -> &[Href] {
        &self.application_paths
    }

    /// Returns copied or generated static-output paths.
    pub fn static_paths(&self) -> &[Href] {
        &self.static_paths
    }

    /// Renders the sitemap represented by this manifest.
    pub fn sitemap_xml(&self) -> String {
        sitemap::render(
            &self.site_url,
            self.application_paths.iter().chain(&self.static_paths),
        )
    }

    /// Creates a rooted path suitable for application or static manifest entries.
    pub fn rooted_path(path: impl AsRef<str>) -> Href {
        Href::from_route(&BaseHref::root(), &RoutePath::new(path))
    }
}

fn collect_unique_paths<I, P>(paths: I) -> Vec<Href>
where
    I: IntoIterator<Item = P>,
    P: Into<Href>,
{
    let mut collected = Vec::new();
    for path in paths {
        push_unique_path(&mut collected, path.into());
    }
    collected
}

fn push_unique_path(paths: &mut Vec<Href>, path: Href) {
    if !paths.iter().any(|existing| existing == &path) {
        paths.push(path);
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;

    use super::*;

    #[derive(Clone, Debug, PartialEq, Routable)]
    enum TestRoute {
        #[route("/", TestHome)]
        Home {},
        #[route("/guide/", TestGuide)]
        Guide {},
        #[route("/articles/:slug/", TestArticle)]
        Article { slug: String },
    }

    #[component]
    fn TestHome() -> Element {
        rsx! {}
    }

    #[component]
    fn TestGuide() -> Element {
        rsx! {}
    }

    #[component]
    fn TestArticle(slug: String) -> Element {
        rsx! { "{slug}" }
    }

    #[test]
    fn project_manifest_uses_every_statically_enumerable_routable_path() {
        let manifest = SiteRouteManifest::project_from_routable::<TestRoute>(SiteUrl::new(
            "https://example.test/project",
        ));

        assert_eq!(
            manifest.application_paths(),
            [Href::new("/"), Href::new("/guide/")]
        );
        assert!(
            !manifest
                .application_paths()
                .iter()
                .any(|path| path.as_str().contains("articles"))
        );
        assert_eq!(
            manifest.static_paths(),
            [
                Href::new("/book/"),
                Href::new("/llms.txt"),
                Href::new("/llms-full.txt"),
            ]
        );
    }

    #[test]
    fn project_manifest_drives_fallbacks_and_standard_sitemap_outputs() {
        let manifest = SiteRouteManifest::project(
            SiteUrl::new("https://example.test/project"),
            [Href::new("/"), Href::new("/demos/")],
        )
        .with_static_paths([Href::new("/gpui-demo/")]);

        assert_eq!(
            manifest.application_paths(),
            [Href::new("/"), Href::new("/demos/")]
        );
        assert_eq!(
            manifest.static_paths(),
            [
                Href::new("/book/"),
                Href::new("/llms.txt"),
                Href::new("/llms-full.txt"),
                Href::new("/gpui-demo/"),
            ]
        );

        let sitemap = manifest.sitemap_xml();
        for path in [
            "",
            "demos/",
            "book/",
            "llms.txt",
            "llms-full.txt",
            "gpui-demo/",
        ] {
            assert!(sitemap.contains(&format!("<loc>https://example.test/project/{path}</loc>")));
        }
    }

    #[test]
    fn manifest_deduplicates_routes_without_mixing_static_and_app_paths() {
        let manifest = SiteRouteManifest::new(
            SiteUrl::new("https://example.test/project"),
            [Href::new("/"), Href::new("/")],
        )
        .with_static_paths([Href::new("/asset/"), Href::new("/asset/")]);

        assert_eq!(manifest.application_paths(), [Href::new("/")]);
        assert_eq!(manifest.static_paths(), [Href::new("/asset/")]);
    }

    #[test]
    fn rooted_paths_are_normalized_for_site_manifests() {
        assert_eq!(SiteRouteManifest::rooted_path("").as_str(), "/");
        assert_eq!(
            SiteRouteManifest::rooted_path("/demos/").as_str(),
            "/demos/"
        );
    }
}
