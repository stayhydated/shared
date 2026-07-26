use crate::site::constants::SITE_URL;
use stayhydated_site::routing::SiteUrl;

pub(crate) fn render_sitemap() -> String {
    let paths = crate::site::routing::all_routes()
        .into_iter()
        .map(|route| route.path())
        .chain([
            stayhydated_site::routing::Href::new("/bevy-demo/"),
            stayhydated_site::routing::Href::new("/gpui-demo/"),
        ])
        .collect::<Vec<_>>();

    stayhydated_site::sitemap::render_project(&SiteUrl::new(SITE_URL), paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sitemap_includes_both_static_wasm_demos() {
        let sitemap = render_sitemap();

        assert!(
            sitemap.contains("<loc>https://stayhydated.github.io/sum-numbers-ai/bevy-demo/</loc>")
        );
        assert!(
            sitemap.contains("<loc>https://stayhydated.github.io/sum-numbers-ai/gpui-demo/</loc>")
        );
    }
}
