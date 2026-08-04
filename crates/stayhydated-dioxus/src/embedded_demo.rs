use dioxus::prelude::*;
use stayhydated_dioxus_core::{FullscreenDemoFrame, NavigationTarget};
use stayhydated_site::SiteRouteManifest;

use crate::{
    ProjectSite, StayhydatedProjectApp, StayhydatedProjectPageMetadata,
    StayhydatedProjectPortalShell, StayhydatedProjectSitePortal,
};

impl ProjectSite {
    /// Builds the SSG route and sitemap contract for a portal with an embedded demo page.
    ///
    /// The site must configure [`ProjectSite::demo_path`]. The raw demo remains a static
    /// output while `/demo/` keeps the shared project header around its frame.
    pub fn embedded_demo_route_manifest(self) -> SiteRouteManifest {
        assert!(
            self.demo_path().is_some(),
            "embedded demo sites require ProjectSite::demo_path"
        );
        self.route_manifest::<EmbeddedDemoProjectRoute>()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Routable)]
enum EmbeddedDemoProjectRoute {
    #[route("/", EmbeddedDemoProjectHomeRoute)]
    Home {},
    #[route("/demo/", EmbeddedDemoProjectDemoRoute)]
    Demo {},
}

/// Browser application for a project portal with one header-preserving demo route.
///
/// Configure the raw browser artifact with [`ProjectSite::demo_path`] and use
/// [`ProjectSite::embedded_demo_route_manifest`] when assembling the site.
#[component]
pub fn StayhydatedEmbeddedDemoProjectApp(site: ProjectSite) -> Element {
    assert!(
        site.demo_path().is_some(),
        "StayhydatedEmbeddedDemoProjectApp requires ProjectSite::demo_path"
    );

    rsx! {
        StayhydatedProjectApp::<EmbeddedDemoProjectRoute> { site }
    }
}

#[component]
fn EmbeddedDemoProjectHomeRoute() -> Element {
    let site = use_context::<ProjectSite>();

    rsx! {
        StayhydatedProjectPageMetadata {
            project: site.project(),
            page_title: "Home",
            description: site.project().description(),
        }
        StayhydatedProjectSitePortal::<EmbeddedDemoProjectRoute> {
            site,
            home: NavigationTarget::Internal(EmbeddedDemoProjectRoute::Home {}),
            demos: NavigationTarget::Internal(EmbeddedDemoProjectRoute::Demo {}),
        }
    }
}

#[component]
fn EmbeddedDemoProjectDemoRoute() -> Element {
    let site = use_context::<ProjectSite>();
    let demo_src = site
        .demo_href()
        .expect("StayhydatedEmbeddedDemoProjectApp requires ProjectSite::demo_path");
    let demo_title = format!("{} demo", site.project().as_str());

    rsx! {
        StayhydatedProjectPageMetadata {
            project: site.project(),
            page_title: "Demo",
            description: site.project().description(),
        }
        StayhydatedProjectPortalShell::<EmbeddedDemoProjectRoute> {
            project: site.project(),
            version: site.version(),
            home: NavigationTarget::Internal(EmbeddedDemoProjectRoute::Home {}),
            FullscreenDemoFrame {
                src: demo_src,
                title: demo_title,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn embedded_demo_site() -> ProjectSite {
        ProjectSite::builder()
            .project(crate::Project::new("example", "Example project"))
            .site_url("https://example.test/example/")
            .rustdoc_url("https://docs.example.test/example/")
            .source_url("https://code.example.test/example")
            .version("0.1.0")
            .demo_path("gpui-demo")
            .build()
    }

    #[test]
    fn embedded_demo_manifest_separates_the_frame_route_from_the_raw_demo() {
        let manifest = embedded_demo_site().embedded_demo_route_manifest();

        assert_eq!(
            manifest.application_paths(),
            [
                stayhydated_dioxus_core::Href::new("/"),
                stayhydated_dioxus_core::Href::new("/demo/"),
            ]
        );
        assert!(
            manifest
                .static_paths()
                .contains(&stayhydated_dioxus_core::Href::new("/gpui-demo/"))
        );
    }

    #[test]
    #[should_panic(expected = "embedded demo sites require ProjectSite::demo_path")]
    fn embedded_demo_manifest_requires_a_raw_demo_path() {
        ProjectSite::builder()
            .project(crate::Project::new("example", "Example project"))
            .site_url("https://example.test/example/")
            .rustdoc_url("https://docs.example.test/example/")
            .source_url("https://code.example.test/example")
            .version("0.1.0")
            .build()
            .embedded_demo_route_manifest();
    }
}
