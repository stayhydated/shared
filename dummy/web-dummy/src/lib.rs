#[cfg(any(target_arch = "wasm32", test))]
mod cli;
mod pages;
mod site;
mod terminal;

pub use site::app::App;
pub use site::constants::SITE_URL;

pub fn route_manifest() -> stayhydated_site::SiteRouteManifest {
    site::constants::site()
        .route_manifest::<site::routing::AppRoute>()
        .with_static_paths(["/bevy-demo/", "/gpui-demo/"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_manifest_includes_both_static_wasm_demos() {
        let manifest = route_manifest();
        let sitemap = manifest.sitemap_xml();

        assert_eq!(
            manifest
                .application_paths()
                .iter()
                .map(|path| path.as_str())
                .collect::<Vec<_>>(),
            ["/", "/demos/", "/demos/dioxus/", "/demos/terminal/"]
        );

        assert!(
            sitemap.contains("<loc>https://stayhydated.github.io/sum-numbers-ai/bevy-demo/</loc>")
        );
        assert!(
            sitemap.contains("<loc>https://stayhydated.github.io/sum-numbers-ai/gpui-demo/</loc>")
        );
    }
}
