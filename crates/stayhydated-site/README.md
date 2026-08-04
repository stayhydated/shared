# stayhydated-site

Routing and generated-site primitives for Dioxus Web applications. The crate
provides the browser launch boundary, base-path-aware href types, route
manifests, sitemap rendering, and route-cache helpers used during static-site
assembly.

Application crates normally consume these types through `stayhydated-dioxus`;
repository xtasks pass the resulting `SiteRouteManifest` to
`stayhydated-xtask`.
