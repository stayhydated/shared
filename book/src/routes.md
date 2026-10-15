# Define routes and outputs

Export one `SiteRouteManifest` from your web library and pass it to your build
task. This keeps application fallback pages and sitemap destinations aligned.
`stayhydated-site` owns the manifest, routing types, sitemap rendering, and
generated-route-cache helpers.

## Separate application and static paths

Application paths receive fallback `index.html` files during assembly. Static
paths describe files produced by other build steps. Both appear in the sitemap;
declaring a static path does not build or copy that output.

Use `SiteRouteManifest::new` when you need an explicit set with no implied static
outputs. Use `SiteRouteManifest::project` or a `ProjectSite` manifest preset
when you produce the standard `/book/`, `/llms.txt`, and `/llms-full.txt` outputs.
Add browser-demo destinations with `with_static_paths`.

The project manifest includes the standard book path even if a shell's
`book_path` is customized. For a different output layout, construct an explicit
manifest that matches the generated destinations. Do not assume changing a
navigation link also changes build outputs.

`project_from_routable` uses `Routable::static_routes()`. Dynamic route segments
are not enumerated; provide concrete dynamic paths explicitly when they need
generated fallbacks. Keep a consumer test for each promised static destination.

## Apply the base path once

Keep manifest entries site-root-relative, such as `/demos/`, without the
repository slug. `SiteUrl` supplies the canonical prefix for sitemap URLs.
The active Dioxus base path supplies the prefix for browser navigation and
assets through `dioxus_base_href`, `href`, and `asset_href`.

`RoutePath` and `BasePath` trim surrounding slashes. These constructors join
paths; they do not validate or canonicalize arbitrary URLs. `asset_href` keeps
file paths free of a trailing slash, while route hrefs end in `/`.

## Clean generated route caches

Use cache cleanup only for a directory that your build owns. Mark generated
content with `mark_generated_route_cache`; cleanup is a no-op when that marker
is absent. Preserve the marker check so ordinary source assets are not treated
as generated pages.

`cleanup_generated_route_cache` accepts direct top-level directory names and
ignores empty names. Once a marker is found, it validates all nonempty names
before removing index or fallback pages. Absolute, nested, traversal, and
backslash-separated names return `InvalidInput`. Use
`cleanup_generated_route_cache_for_outputs` when starting from nested route
outputs; it extracts their top-level directories first.

The additional-directory predicate chooses other direct child directories to
remove. Keep that predicate limited to generated content. These helpers are not
a sandbox for untrusted filesystem access; the caller owns the public directory
and marker name.

## Verify the result

Inspect the assembled sitemap and open every promised route directly under the
hosted prefix. A missing static destination means the build sequence or copy
configuration needs correction, even when its sitemap entry looks correct.
See [site assembly](build.md) for the output checks.
