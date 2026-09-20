# stayhydated-site

[![Codecov: stayhydated-site][codecov-badge]][codecov]

`stayhydated-site` provides routing and generated-site primitives for Dioxus Web
application authors. It supplies the browser launch boundary, base-path-aware
href types, route manifests, sitemap rendering, and route-cache helpers used
during static-site assembly.

Application crates normally consume these types through `stayhydated-dioxus`;
repository xtasks pass the resulting `SiteRouteManifest` to
`stayhydated-xtask`.

## Example

Application routes receive fallback pages during assembly. Static paths name
outputs produced by another build step. Both appear in the sitemap:

```rust
use stayhydated_site::{SiteRouteManifest, routing::SiteUrl};

let manifest = SiteRouteManifest::new(
    SiteUrl::new("https://example.test/project/"),
    ["/", "/demos/"],
)
.with_static_paths(["/gpui-demo/"]);

assert!(manifest.sitemap_xml().contains(
    "https://example.test/project/gpui-demo/"
));
```

Keep manifest paths relative to the site root. The canonical `SiteUrl` supplies
the project prefix for sitemap URLs; the `routing` module resolves navigation
and asset hrefs against the active Dioxus base path.

[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-site
[codecov]: https://app.codecov.io/github/stayhydated/shared
