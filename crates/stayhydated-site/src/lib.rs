use dioxus::prelude::Element;

mod manifest;
pub mod route_cache;
pub mod routing;
pub mod sitemap;

pub use manifest::SiteRouteManifest;

/// Launches the repository's browser-only Dioxus application.
pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
