# stayhydated-site

[![Codecov: stayhydated-site][codecov-badge]][codecov]

`stayhydated-site` provides routing and generated-site primitives for Dioxus Web
application authors. It supplies the browser launch boundary, base-path-aware
href types, route manifests, sitemap rendering, and route-cache helpers used
during static-site assembly.

Depend on this package directly for browser launch, custom routing, or an
exported `SiteRouteManifest`. [`stayhydated-dioxus`][dioxus] supplies configured
UI shells; consumer xtasks pass the resulting manifest to
[`stayhydated-xtask`][xtask].

Requires Rust 1.99 or newer and Dioxus 0.7. This is an unpublished Git-sourced
package; keep directly used shared packages on the same revision.

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

See the [manifest API][manifest] for detailed routing and sitemap semantics.

Route-cache cleanup requires a generated-cache marker. Its direct-directory
API accepts only top-level directory names; use the [route-cache API][route-cache]
for cleanup from nested route outputs. For a marked cache, invalid directory
names return an error before generated pages are removed.

[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-site
[codecov]: https://app.codecov.io/github/stayhydated/shared
[dioxus]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-dioxus/README.md
[xtask]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-xtask/README.md
[manifest]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-site/src/manifest.rs
[route-cache]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-site/src/route_cache.rs
