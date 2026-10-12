# stayhydated-dioxus

[![Codecov: stayhydated-dioxus][codecov-badge]][codecov]

`stayhydated-dioxus` provides configured Dioxus application shells for authors
of stayhydated project sites. Consumers supply their identity and destinations
through `Project` and `ProjectSite`, then choose a single-page portal, an
embedded-demo portal, or a custom `Routable` application.

Depend on this package directly for configured shells and reexported common UI
components. Use [`stayhydated-dioxus-core`][core] directly for core-only APIs.

Requires Rust 1.99 or newer and Dioxus 0.7. This is an unpublished Git-sourced
package; keep directly used shared packages on the same revision.

## Overview

| Site shape | Application component |
| --- | --- |
| One portal route | `StayhydatedSinglePageProjectApp` |
| Portal with one embedded static demo | `StayhydatedEmbeddedDemoProjectApp` |
| Consumer-defined routes | `StayhydatedProjectApp<R>` |

Create one `ProjectSite` for the application's identity, URLs, version, and
optional demo or stylesheet. Export its route manifest for the build task so
the rendered site, fallback pages, and sitemap use the same destinations.

The crate re-exports the common components from `stayhydated-dioxus-core`. The
[consumer adoption skill][adoption] covers revision synchronization, routing,
build assembly, deployment, and integration audits.

## Example

Construct consumer-owned configuration once and pass it to the selected shell.
The following self-contained example uses the public builder; the
[dummy site's configuration][dummy-site] shows its application context.

```rust
use stayhydated_dioxus::{Project, ProjectSite};

fn main() {
    let site = ProjectSite::builder()
        .project(Project::new("example", "Example project"))
        .site_url("https://example.test/example/")
        .rustdoc_url("https://docs.example.test/example/")
        .source_url("https://code.example.test/example")
        .version(env!("CARGO_PKG_VERSION"))
        .site_stylesheet_path("assets/site.css")
        .build();

    assert_eq!(site.project().as_str(), "example");
    assert_eq!(site.site_stylesheet_path(), Some("assets/site.css"));
}
```

The shell uses this identity and explicitly configured stylesheet. The
[configuration tests][tests] cover the same builder and destination behavior.

[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/SKILL.md
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-dioxus
[codecov]: https://app.codecov.io/github/stayhydated/shared
[core]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-dioxus-core/README.md
[dummy-site]: https://github.com/stayhydated/shared/blob/master/dummy/web-dummy/src/site/constants.rs
[tests]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-dioxus/src/project_site.rs
