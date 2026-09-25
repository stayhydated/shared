use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};

use crate::routing::SiteUrl;

const SITEMAP_NAMESPACE: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";

pub fn render<I, P>(site_url: &SiteUrl, paths: I) -> String
where
    I: IntoIterator<Item = P>,
    P: AsRef<str>,
{
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    write_event(
        &mut writer,
        Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)),
    );

    let mut urlset = BytesStart::new("urlset");
    urlset.push_attribute(("xmlns", SITEMAP_NAMESPACE));
    write_event(&mut writer, Event::Start(urlset));

    for path in paths {
        let url = absolute_url(site_url, path.as_ref());
        write_event(&mut writer, Event::Start(BytesStart::new("url")));
        write_event(&mut writer, Event::Start(BytesStart::new("loc")));
        write_event(&mut writer, Event::Text(BytesText::new(&url)));
        write_event(&mut writer, Event::End(BytesEnd::new("loc")));
        write_event(&mut writer, Event::End(BytesEnd::new("url")));
    }

    write_event(&mut writer, Event::End(BytesEnd::new("urlset")));

    String::from_utf8(writer.into_inner()).expect("the sitemap XML is always valid UTF-8")
}

fn write_event(writer: &mut Writer<Vec<u8>>, event: Event<'_>) {
    writer
        .write_event(event)
        .expect("writing the sitemap XML to an in-memory buffer cannot fail");
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
    fn escapes_xml_metacharacters_in_paths() {
        let sitemap = render(
            &SiteUrl::new("https://example.test/project"),
            ["a&b<c>d.txt"],
        );

        assert!(sitemap.contains("a&amp;b&lt;c&gt;d.txt"));
        assert!(!sitemap.contains("a&b<c>d.txt"));
    }
}
