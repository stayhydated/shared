---
name: use-stayhydated-github-pages
description: "Use when adding, reviewing, migrating, or updating a GitHub Pages site built on stayhydated/shared, including shared revision pins, consumer-owned project configuration, Dioxus routing and assets, xtask assembly, books and browser demos, static preview, Pages workflows, and Koruma or es-fluent patterns."
---

# Use stayhydated GitHub Pages

## Scope boundary

Treat the shared repository as the source of truth for:

- common Dioxus shells, landing pages, cards, navigation, and styles;
- base-path, route, sitemap, and static-output types;
- GitHub Pages build, fallback, preview, book, llms.txt, and Trunk helpers;
- reusable Pages and shared-revision workflows.

Keep the project name, tagline, canonical URL, docs/book/source destinations,
optional demo destination, Skills command, landing theme, and other project
identity in the consumer repository.

Read the shared repository's own `AGENTS.md` before maintaining those
implementations. Read the consumer repository's guidance before changing its
site.

## Core workflow

1. Inspect the consumer before editing:
   - locate `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `justfile`, `web/`,
     `xtask/`, and `.github/workflows/`;
   - identify its project slug, Cargo repository metadata, Dioxus application
     name and base path, default branch, static routes, generated directories,
     books, llms.txt outputs, and browser demos;
   - preserve dirty worktree changes.
2. Resolve the shared source:
   - prefer the local `shared` checkout when available;
   - inspect its current commit and the exact requested revision;
   - compare the consumer's pinned revision with the target before changing
     call sites;
   - never infer an API from a newer local checkout when the consumer remains
     pinned to an older commit.
3. Define project identity and destinations in the consumer:
   - construct a local `stayhydated_dioxus::Project` value from the project
     name and tagline;
   - opt into the portal Skills control with `.with_skill_command(...)`;
   - define the canonical site URL and docs, book, and source destinations beside
     the consumer's routing constants, plus a demo destination when the project
     has demos;
   - pass theme and landing links explicitly where the landing component is
     used.
4. Pin `stayhydated-dioxus`, `stayhydated-site`, and
   `stayhydated-xtask` to the same full shared SHA in workspace dependencies.
   Regenerate `Cargo.lock` with package-scoped `cargo update` and review solver
   churn.
5. Wire the web crate around shared types:
   - keep the package featureless and enable Dioxus Web unconditionally;
   - launch directly with `stayhydated_site::launch(web::App)`;
   - define one consumer-owned `ProjectSite` for both one-route and multi-route
     sites;
   - use `StayhydatedSinglePageProjectApp` as the one-route preset;
   - use `StayhydatedProjectApp` for a custom `Routable` application;
   - use `StayhydatedProjectPageMetadata` for every route;
   - use `StayhydatedProjectSitePortal` when the configured docs, book, source,
     version, and project identity fit the home portal;
   - use `ProjectSite::static_href` for consumer-owned static destinations;
   - export one `SiteRouteManifest` for fallback and sitemap generation;
   - derive multi-route application paths from the app's `Routable` enum;
   - test consumer-owned configuration directly without enabling Dioxus SSR.
6. Assemble Pages through
   `stayhydated_xtask::web::WebBuildConfig::github_pages`:
   - pass the consumer's route manifest;
   - keep the default public-assets pipeline when it fits;
   - use explicit assets and demo inputs when the consumer already owns them.
7. Expose the Pages workflow through three root `justfile` recipes:
   - keep a dedicated `web-build` recipe that assembles every consumer-owned
     prerequisite and finishes with `cargo xtask build web`;
   - make `web: web-build` run `dx serve --package web`;
   - make `web-preview: web-build` run `cargo xtask preview web`;
   - wire the xtask preview command to the shared static preview helper with the
     consumer's `web/dist` directory and project base path.
8. Deploy through the shared reusable Pages workflow:
   - do not introduce consumer-owned non-Cargo package manifests,
     package-manager commands, or JavaScript or TypeScript tool configuration
     solely for the Pages site;
   - install Trunk and nightly only for browser demos that need them.
9. Build and inspect the actual `web/dist` artifact under the project base path
   before calling the site work complete.

## Ownership rules

- Keep generic visual behavior and CSS in shared. Keep only project-specific
  layout/content CSS in the consumer. The shared Dioxus component theme is a
  bundled asset; do not copy it into `web/public` or `web/dist`.
- Use shared components directly. Do not retain downstream wrappers or copied
  helpers once shared exposes the required API.
- Treat `web/dist`, generated books, llms.txt trees, and built wasm demos as
  generated outputs. Rebuild them through the owning task.
- Keep root-relative application routes separate from the GitHub Pages project
  prefix. Let Dioxus CLI config and shared routing types apply the base path.

## Reference selection

Read [references/adoption-patterns.md](references/adoption-patterns.md) for a
new adoption, a full-site review, or work that changes Cargo, Dioxus, routing,
xtask, preview, or workflow shapes. For a narrow shared-revision update, inspect
the pinned source and affected consumer surfaces first, then load the reference
only when a signature or site contract needs comparison.

Use locally available Koruma and es-fluent checkouts as consumer evidence, not
as sources for shared project metadata. Preserve their distinct build variants
instead of making either variant a requirement for the other.

Prefer current local source at the pinned shared revision over the reference
when signatures differ.

Follow the reference's validation checklist and run the bundled
[consumer audit](scripts/audit_consumer.py). When shared itself changed,
validate the focused shared crates and dummy site before validating consumers.
Pass the intended commit with `--expected-shared-revision`. The audit checks
structure and generated output; it does not replace a locked compile or
consumer tests that assert every expected static route is in the manifest.
