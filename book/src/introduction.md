# Choose shared packages

Use the shared crates to build a Dioxus repository site while keeping its
identity, content, routes, and build sequence in your own repository. This
guide covers the four public Git-sourced packages in one workflow.

Requires Rust 1.99 or newer. The UI and browser boundary use Dioxus 0.7.
These packages are unpublished: pin every directly used shared package to the
same full Git commit in your workspace dependencies. Inherit those dependencies
in the web and build-task packages. Update the consumer lockfile when changing
the pin; builds use `--locked`.

| Package | Choose it when |
| --- | --- |
| `stayhydated-dioxus` | You need a configured project shell, portal, or application router. |
| `stayhydated-dioxus-core` | You need individual components or core-only APIs instead of a configured shell. |
| `stayhydated-site` | You need browser launch, base-path navigation, a route manifest, or sitemap and cache helpers. |
| `stayhydated-xtask` | Your own build task needs documentation, site assembly, browser-demo, or preview helpers. |

The configured facade reexports common core components. Add a direct core
dependency only when you use its APIs directly. None of these four packages
exposes package-level Cargo features; choose the package and Dioxus configuration
appropriate to your application.

Start by [configuring the application](application.md), then export its
[route manifest](routes.md) for the [build task](build.md). Shared supplies the
reusable behavior; you supply real project URLs and generate every destination
that your site advertises.

The [consumer adoption reference][adoption] provides dependency and workflow
templates. The [dummy consumer][dummy] demonstrates the integration in this
workspace. Its separate book describes a maintainer test fixture; it is not the
user guide for these packages.

To render this guide from a shared checkout, run `just book-build` at the
repository root. It uses the existing Rust mdBook helper and writes
`target/shared-user-book/index.html`; no standalone mdBook executable is needed.

[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/references/adoption-patterns.md
[dummy]: https://github.com/stayhydated/shared/tree/master/dummy
