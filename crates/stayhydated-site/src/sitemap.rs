use std::fmt::Write as _;

use crate::routing::SiteUrl;

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
}
