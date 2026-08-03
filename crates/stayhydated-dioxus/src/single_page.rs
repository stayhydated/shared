use dioxus::prelude::*;
use stayhydated_dioxus_core::NavigationTarget;
use stayhydated_site::SiteRouteManifest;

use crate::{
    ProjectSite, StayhydatedProjectApp, StayhydatedProjectPageMetadata,
    StayhydatedProjectSitePortal,
};

impl ProjectSite {
    /// Builds the SSG route and sitemap contract for the single-page preset.
    pub fn single_page_route_manifest(self) -> SiteRouteManifest {
        self.route_manifest::<SinglePageProjectRoute>()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Routable)]
enum SinglePageProjectRoute {
    #[route("/", SinglePageProjectHomeRoute)]
    Home {},
}

/// Browser application for a project whose only Dioxus route is its portal.
#[component]
pub fn StayhydatedSinglePageProjectApp(site: ProjectSite) -> Element {
    rsx! {
        StayhydatedProjectApp::<SinglePageProjectRoute> { site }
    }
}

#[component]
fn SinglePageProjectHomeRoute() -> Element {
    let site = use_context::<ProjectSite>();
    let demos = site
        .demo_href()
        .map(|href| NavigationTarget::External(href.into_string()));

    rsx! {
        StayhydatedProjectPageMetadata {
            project: site.project(),
            page_title: "Home",
            description: site.project().description(),
        }
        StayhydatedProjectSitePortal::<SinglePageProjectRoute> {
            site,
            home: NavigationTarget::Internal(SinglePageProjectRoute::Home {}),
            demos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_page_manifest_includes_an_optional_static_demo() {
        let site = ProjectSite::builder()
            .project(crate::Project::new("example", "Example project"))
            .site_url("https://example.test/example/")
            .rustdoc_url("https://docs.example.test/example/")
            .source_url("https://code.example.test/example")
            .version("0.1.0")
            .demo_path("gpui-demo")
            .build();

        let manifest = site.single_page_route_manifest();
        assert_eq!(
            manifest.application_paths(),
            [stayhydated_dioxus_core::Href::new("/")]
        );
        assert!(
            manifest
                .static_paths()
                .contains(&stayhydated_dioxus_core::Href::new("/gpui-demo/"))
        );
    }
}
