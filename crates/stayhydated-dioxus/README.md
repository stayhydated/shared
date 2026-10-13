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

Export the site's route manifest for the build task so fallback pages and the
sitemap use the same destinations.

Use the [consumer adoption skill][adoption] for route manifests, build assembly,
and deployment.

The `select` module includes `SearchableSelect<T>` for filtering typed options
through a search bar. See the [core component guide][core] for its language
selector example.

## Example

Construct consumer-owned configuration once and pass it to the selected shell.

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

[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/SKILL.md
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-dioxus
[codecov]: https://app.codecov.io/github/stayhydated/shared
[core]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-dioxus-core/README.md
