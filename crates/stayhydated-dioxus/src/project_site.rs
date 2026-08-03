use std::marker::PhantomData;

use bon::Builder;
use dioxus::prelude::*;
use stayhydated_dioxus_core::{Href, NavigationTarget};
use stayhydated_site::{
    SiteRouteManifest,
    routing::{RoutePath, SiteUrl, dioxus_base_href, href},
};

use crate::{Project, StayhydatedProjectPortal, StayhydatedRouterApp};

/// Consumer-owned configuration shared by single-page and multi-route project sites.
#[derive(Builder, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectSite {
    project: Project,
    site_url: &'static str,
    rustdoc_url: &'static str,
    source_url: &'static str,
    version: &'static str,
    #[builder(default = "book")]
    book_path: &'static str,
    demo_path: Option<&'static str>,
    site_stylesheet_path: Option<&'static str>,
}

impl ProjectSite {
    pub const fn project(self) -> Project {
        self.project
    }

    pub const fn site_url(self) -> &'static str {
        self.site_url
    }

    pub const fn rustdoc_url(self) -> &'static str {
        self.rustdoc_url
    }

    pub const fn source_url(self) -> &'static str {
        self.source_url
    }

    pub const fn version(self) -> &'static str {
        self.version
    }

    pub const fn book_path(self) -> &'static str {
        self.book_path
    }

    pub const fn demo_path(self) -> Option<&'static str> {
        self.demo_path
    }

    pub const fn site_stylesheet_path(self) -> Option<&'static str> {
        self.site_stylesheet_path
    }

    /// Resolves a project-relative path against the active Dioxus base path.
    pub fn static_href(self, path: impl AsRef<str>) -> Href {
        href(&dioxus_base_href(), &RoutePath::new(path))
    }

    /// Resolves the configured book destination against the active Dioxus base path.
    pub fn book_href(self) -> Href {
        self.static_href(self.book_path)
    }

    /// Resolves the optional static demo destination against the active Dioxus base path.
    pub fn demo_href(self) -> Option<Href> {
        self.demo_path.map(|path| self.static_href(path))
    }

    /// Builds the standard project manifest from the statically enumerable routes in `R`.
    pub fn route_manifest<R>(self) -> SiteRouteManifest
    where
        R: Routable,
    {
        let manifest = SiteRouteManifest::project_from_routable::<R>(SiteUrl::new(self.site_url));

        match self.demo_path {
            Some(path) => manifest.with_static_paths([SiteRouteManifest::rooted_path(path)]),
            None => manifest,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Props)]
pub struct StayhydatedProjectAppProps<R>
where
    R: Routable + Clone + PartialEq + 'static,
{
    pub site: ProjectSite,
    #[props(default)]
    route: PhantomData<R>,
}

/// Configured browser application for a consumer-owned project site.
#[allow(non_snake_case)]
pub fn StayhydatedProjectApp<R>(props: StayhydatedProjectAppProps<R>) -> Element
where
    R: Routable + Clone + PartialEq + 'static,
{
    use_context_provider(|| props.site);
    let site_stylesheet_path = props.site.site_stylesheet_path.map(Href::new);

    rsx! {
        StayhydatedRouterApp::<R> { site_stylesheet_path }
    }
}

/// Configured project portal using docs, book, source, and version from `site`.
#[component]
pub fn StayhydatedProjectSitePortal<R: Routable + Clone + PartialEq + 'static>(
    site: ProjectSite,
    home: NavigationTarget<R>,
    demos: Option<NavigationTarget<R>>,
) -> Element {
    rsx! {
        StayhydatedProjectPortal::<R> {
            project: site.project,
            version: site.version,
            home,
            docs: Href::new(site.rustdoc_url),
            book: site.book_href(),
            demos,
            source: Href::new(site.source_url),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_site_keeps_consumer_owned_values_and_default_book_path() {
        let project = Project::new("example", "Example project");
        let site = ProjectSite::builder()
            .project(project)
            .site_url("https://example.test/example/")
            .rustdoc_url("https://docs.example.test/example/")
            .source_url("https://code.example.test/example")
            .version("0.1.0")
            .site_stylesheet_path("assets/site.css")
            .build();

        assert_eq!(site.project(), project);
        assert_eq!(site.site_url(), "https://example.test/example/");
        assert_eq!(site.rustdoc_url(), "https://docs.example.test/example/");
        assert_eq!(site.source_url(), "https://code.example.test/example");
        assert_eq!(site.version(), "0.1.0");
        assert_eq!(site.book_path(), "book");
        assert_eq!(site.demo_path(), None);
        assert_eq!(site.site_stylesheet_path(), Some("assets/site.css"));
    }

    #[test]
    fn project_site_resolves_book_and_demo_paths_from_one_configuration() {
        let site = ProjectSite::builder()
            .project(Project::new("example", "Example project"))
            .site_url("https://example.test/example/")
            .rustdoc_url("https://docs.example.test/example/")
            .source_url("https://code.example.test/example")
            .version("0.1.0")
            .book_path("guide")
            .demo_path("gpui-demo")
            .build();

        assert_eq!(site.book_href().as_str(), "/guide/");
        assert_eq!(
            site.demo_href().as_ref().map(Href::as_str),
            Some("/gpui-demo/")
        );
        assert_eq!(site.static_href("assets").as_str(), "/assets/");
    }
}
