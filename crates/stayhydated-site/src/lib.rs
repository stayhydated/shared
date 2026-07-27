#[cfg(feature = "web")]
use bon::Builder;
#[cfg(feature = "web")]
use dioxus::prelude::Element;

pub mod route_cache;
pub mod routing;
pub mod sitemap;

#[cfg(feature = "web")]
#[derive(Builder, Clone, Copy)]
pub struct SiteApp {
    app: fn() -> Element,
}

#[cfg(feature = "web")]
pub fn launch(site_app: SiteApp) {
    dioxus::launch(site_app.app);
}
