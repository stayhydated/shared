use proptest::prelude::*;
use quick_xml::{Reader, events::Event};
use stayhydated_site::{
    SiteRouteManifest,
    route_cache::generated_top_level_dirs,
    routing::{
        BaseHref, BasePath, OutputDir, RoutePath, SiteUrl, href, normalized_path_segments,
        site_root_prefix,
    },
    sitemap,
};

fn segment() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-z][a-z0-9_-]{0,12}",
        Just("café".to_owned()),
        Just("文".to_owned())
    ]
}

fn path_from_segments(segments: &[String], slash_count: usize) -> String {
    let slash = "/".repeat(slash_count);
    format!("{slash}{}{slash}", segments.join(&slash))
}

// IDs carry the model: membership is tracked independently of the implementation's
// string comparisons, and each expected path is constructed from its first ID.
fn first_paths(ids: &[u8]) -> Vec<String> {
    let mut present = [false; 16];
    let mut paths = Vec::new();
    for &id in ids {
        if !present[usize::from(id)] {
            present[usize::from(id)] = true;
            paths.push(format!("/route-{id}/"));
        }
    }
    paths
}

fn sitemap_locations(xml: &str) -> Vec<String> {
    let mut reader = Reader::from_str(xml);
    let mut locations = Vec::new();
    loop {
        match reader
            .read_event()
            .expect("generated sitemap should be valid XML")
        {
            Event::Start(start) if start.name().as_ref() == "loc" => {
                let text = reader.read_text(start.name()).expect("location end tag");
                locations.push(
                    quick_xml::escape::unescape(&text.xml10_content())
                        .expect("location entities")
                        .into_owned(),
                );
            },
            Event::Eof => return locations,
            _ => {},
        }
    }
}

proptest! {
    #[test]
    fn segment_normalization_removes_one_complete_prefix(
        base in prop::collection::vec(segment(), 1..5),
        suffix in prop::collection::vec(segment(), 0..8),
        slash_count in 1usize..5,
    ) {
        let base_path = BasePath::new(base.join("/"));
        let mut full = base.clone();
        full.extend(suffix.iter().cloned());
        let path = path_from_segments(&full, slash_count);
        let expected = suffix.iter().map(String::as_str).collect::<Vec<_>>();
        prop_assert_eq!(normalized_path_segments(&path, Some(&base_path)), expected);
        prop_assert_eq!(
            normalized_path_segments(&path, None),
            full.iter().map(String::as_str).collect::<Vec<_>>()
        );

        let partial = &base[..base.len() - 1];
        let partial_path = path_from_segments(partial, slash_count);
        prop_assert_eq!(
            normalized_path_segments(&partial_path, Some(&base_path)),
            partial.iter().map(String::as_str).collect::<Vec<_>>()
        );

        let mut mismatch = full;
        mismatch[0].push_str("-other");
        let mismatch_path = path_from_segments(&mismatch, slash_count);
        prop_assert_eq!(
            normalized_path_segments(&mismatch_path, Some(&base_path)),
            mismatch.iter().map(String::as_str).collect::<Vec<_>>()
        );
    }

    #[test]
    fn hrefs_and_output_depth_follow_the_segment_specification(
        base in prop::collection::vec(segment(), 0..5),
        route in prop::collection::vec(segment(), 0..8),
        outer_slashes in 1usize..5,
    ) {
        let raw_base = format!("{}{}{}", "/".repeat(outer_slashes), base.join("/"), "/".repeat(outer_slashes));
        let base_href = BaseHref::from_base_path(Some(&BasePath::new(raw_base)));
        let raw_route = format!("{}{}{}", "/".repeat(outer_slashes), route.join("/"), "/".repeat(outer_slashes));
        let route_path = RoutePath::new(raw_route);
        let joined = base.iter().chain(&route).map(String::as_str).collect::<Vec<_>>();
        let expected_href = if joined.is_empty() { "/".to_owned() } else { format!("/{}/", joined.join("/")) };
        let resolved_href = href(&base_href, &route_path);
        prop_assert_eq!(resolved_href.as_str(), expected_href);
        let expected_prefix = if route.is_empty() { "./".to_owned() } else { "../".repeat(route.len()) };
        prop_assert_eq!(site_root_prefix(&OutputDir::new(route.join("/"))), expected_prefix);
    }

    #[test]
    fn manifests_keep_first_occurrence_order_in_each_path_category(
        application_ids in prop::collection::vec(0u8..16, 0..64),
        static_ids in prop::collection::vec(0u8..16, 0..64),
    ) {
        let expected_apps = first_paths(&application_ids);
        let expected_static = first_paths(&static_ids);
        let static_paths = static_ids.iter().map(|id| format!("/route-{id}/")).collect::<Vec<_>>();
        let manifest = SiteRouteManifest::new(
            SiteUrl::new("https://example.test/project"),
            application_ids.iter().map(|id| format!("/route-{id}/")),
        ).with_static_paths(static_paths.iter().cloned()).with_static_paths(static_paths);
        prop_assert_eq!(
            manifest.application_paths().iter().map(|path| path.as_str()).collect::<Vec<_>>(),
            expected_apps.iter().map(String::as_str).collect::<Vec<_>>()
        );
        prop_assert_eq!(
            manifest.static_paths().iter().map(|path| path.as_str()).collect::<Vec<_>>(),
            expected_static.iter().map(String::as_str).collect::<Vec<_>>()
        );
        let expected_urls = expected_apps.iter().chain(&expected_static)
            .map(|path| format!("https://example.test/project{}", path)).collect::<Vec<_>>();
        prop_assert_eq!(sitemap_locations(&manifest.sitemap_xml()), expected_urls);
    }

    #[test]
    fn generated_top_level_dirs_follow_first_top_level_ids(
        outputs in prop::collection::vec((0u8..16, prop::collection::vec(segment(), 0..4)), 0..64),
        slash_count in 1usize..5,
    ) {
        let ids = outputs.iter().map(|(id, _)| *id).collect::<Vec<_>>();
        let expected = first_paths(&ids).into_iter()
            .map(|path| path.trim_matches('/').to_owned()).collect::<Vec<_>>();
        let rendered = outputs.iter().map(|(id, children)| {
            let mut segments = vec![format!("route-{id}")];
            segments.extend(children.iter().cloned());
            path_from_segments(&segments, slash_count)
        }).collect::<Vec<_>>();
        let inputs = ["", "/"].into_iter().chain(rendered.iter().map(String::as_str));
        prop_assert_eq!(generated_top_level_dirs(inputs), expected);
    }

    #[test]
    fn sitemap_preserves_xml_metacharacters_and_path_order(
        paths in prop::collection::vec(prop::collection::vec(
            prop_oneof![Just('&'), Just('<'), Just('>'), Just('\''), Just('"'), Just('é'), Just('文'), proptest::char::range('a', 'z')],
            0..48,
        ), 0..24),
        leading_slashes in 0usize..5,
    ) {
        let texts = paths.iter().map(|chars| chars.iter().collect::<String>()).collect::<Vec<_>>();
        let rendered = texts.iter().map(|text| format!("{}{text}", "/".repeat(leading_slashes))).collect::<Vec<_>>();
        let expected = texts.iter().map(|text| format!("https://example.test/project/{text}")).collect::<Vec<_>>();
        prop_assert_eq!(sitemap_locations(&sitemap::render(&SiteUrl::new("https://example.test/project"), &rendered)), expected);
    }
}

#[test]
fn repeated_base_is_removed_only_once() {
    assert_eq!(
        normalized_path_segments(
            "/repo/docs/repo/docs/guide/",
            Some(&BasePath::new("repo/docs"))
        ),
        ["repo", "docs", "guide"]
    );
}

#[test]
fn sitemap_escapes_existing_entity_spelling_without_decoding_it_twice() {
    let xml = sitemap::render(&SiteUrl::new("https://example.test/"), ["a&amp;b", "a&b"]);
    assert_eq!(
        sitemap_locations(&xml),
        ["https://example.test/a&amp;b", "https://example.test/a&b"]
    );
}
