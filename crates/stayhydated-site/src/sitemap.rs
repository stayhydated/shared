use std::fmt::Write as _;

use crate::routing::{Href, SiteUrl};

pub const PROJECT_STATIC_PATHS: [&str; 3] = ["/book/", "/llms.txt", "/llms-full.txt"];

pub fn project_static_paths() -> impl Iterator<Item = Href> {
    PROJECT_STATIC_PATHS.into_iter().map(Href::new)
}

pub fn render_project<I, P>(site_url: &SiteUrl, route_paths: I) -> String
where
    I: IntoIterator<Item = P>,
    P: AsRef<str>,
{
    let paths = route_paths
        .into_iter()
        .map(|path| Href::new(path.as_ref()))
        .chain(project_static_paths());

    render(site_url, paths)
}

pub fn render<I, P>(site_url: &SiteUrl, paths: I) -> String
where
    I: IntoIterator<Item = P>,
    P: AsRef<str>,
{
    let mut entries = String::new();

    for path in paths {
        let url = absolute_url(site_url, path.as_ref());
        let _ = writeln!(entries, "  <url><loc>{url}</loc></url>");
    }

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{entries}</urlset>\n"
    )
}

fn absolute_url(site_url: &SiteUrl, path: &str) -> String {
    let base_url = site_url.as_str();
    let path = path.trim_start_matches('/');

    if path.is_empty() {
        base_url.to_owned()
    } else {
        format!("{base_url}{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_root_and_nested_paths() {
        let sitemap = render(
            &SiteUrl::new("https://example.test/project"),
            ["/", "/demos/", "llms.txt"],
        );

        assert!(sitemap.contains("<loc>https://example.test/project/</loc>"));
        assert!(sitemap.contains("<loc>https://example.test/project/demos/</loc>"));
        assert!(sitemap.contains("<loc>https://example.test/project/llms.txt</loc>"));
    }

    #[test]
    fn project_sitemap_includes_standard_static_outputs() {
        let sitemap = render_project(&SiteUrl::new("https://example.test/project"), ["/"]);

        assert!(sitemap.contains("<loc>https://example.test/project/book/</loc>"));
        assert!(sitemap.contains("<loc>https://example.test/project/llms.txt</loc>"));
        assert!(sitemap.contains("<loc>https://example.test/project/llms-full.txt</loc>"));
    }
}
