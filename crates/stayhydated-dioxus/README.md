# stayhydated-dioxus

[![Codecov: stayhydated-dioxus][codecov-badge]][codecov]

`stayhydated-dioxus` provides configured Dioxus application shells for authors
of stayhydated project sites. Consumers supply their identity and destinations
through `Project` and `ProjectSite`, then choose a single-page portal, an
embedded-demo portal, or a custom `Routable` application.

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

[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/SKILL.md
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-dioxus
[codecov]: https://app.codecov.io/github/stayhydated/shared
