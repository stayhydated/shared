---
name: use-stayhydated-github-pages
description: "Use when adding, migrating, reviewing, or updating a GitHub Pages site that consumes the stayhydated/shared base, including the current Koruma and es-fluent site patterns. Covers synchronized shared-crate revisions, consumer-owned project configuration, Dioxus app and page components, base-path-safe routing and assets, route fallbacks and sitemaps, default and custom xtask assembly, books and browser demos, Bun static preview, Pages workflows, and shared-revision updates."
---

# Use stayhydated GitHub Pages

## Scope boundary

Use this skill for application-level adoption of the
`stayhydated/shared` repository in a stayhydated project site.

Treat the shared repository as the source of truth for:

- common Dioxus shells, landing pages, cards, navigation, and styles;
- base-path, route, sitemap, and static-output types;
- GitHub Pages build, fallback, preview, book, llms.txt, and Trunk helpers;
- reusable Pages and shared-revision workflows.

Keep the project name, tagline, canonical URL, docs/book/source/demo
destinations, Skills command, landing theme, and other project identity in the
consumer repository.

Read the shared repository's own `AGENTS.md` before maintaining those
implementations. Read the consumer repository's guidance before changing its
site.

## Current consumer baseline

Use Koruma and es-fluent as the active consumer evidence:

- both pin `stayhydated-dioxus`, `stayhydated-site`, and
  `stayhydated-xtask` to one full shared commit;
- both launch a Dioxus `web` package through `SiteApp`, render
  `StayhydatedRouterApp`, derive route metadata once, and export route fallback
  paths plus sitemap XML;
- both build book, llms.txt, and final Pages output through xtask;
- both preview `web/dist` under the repository base path with
  `web/preview.ts` and Bun;
- both own an explicit `.github/workflows/gh-pages.yml` and consume the shared
  revision-update workflow.

Preserve the evidenced variant: Koruma uses the default public-assets build,
while es-fluent assembles localization assets and Bevy/GPUI demos explicitly.
Do not turn either variant into a requirement for the other.

## Core workflow

1. Inspect the consumer before editing:
   - locate `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `justfile`, `web/`,
     `xtask/`, and `.github/workflows/`;
   - identify its project slug, default branch, Dioxus package name, static
     routes, generated directories, books, llms.txt outputs, and browser demos;
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
   - define the canonical site URL and docs, book, source, and demo destinations
     beside the consumer's routing constants;
   - pass theme and landing links explicitly where the landing component is
     used.
4. Pin `stayhydated-dioxus`, `stayhydated-site`, and
   `stayhydated-xtask` to the same full shared SHA in workspace dependencies.
   Regenerate `Cargo.lock` with package-scoped `cargo update` and review solver
   churn.
5. Wire the web crate around shared types:
   - launch with `stayhydated_site::SiteApp`;
   - derive the runtime base href from `dioxus::cli_config::base_path()`;
   - render through `StayhydatedRouterApp`;
   - use `StayhydatedProjectPageMetadata` for every route;
   - pass the consumer-owned `Project` to page and portal wrappers;
   - pass docs, book, source, demo, and landing configuration explicitly;
   - export project route paths and sitemap XML for the build task.
6. Assemble Pages through
   `stayhydated_xtask::web::WebBuildConfig::github_pages`:
   - pass every client-side route as a fallback;
   - write the sitemap;
   - keep the default public-assets pipeline when it fits;
   - use explicit assets and demo inputs when the consumer already owns them;
   - keep `.nojekyll`, `404.html`, and the shared component theme owned by the
     final artifact builder.
7. Preserve the consumer's preview and deployment topology unless the task
   explicitly requests a migration:
   - current consumers use Bun to preview `web/dist`;
   - current consumers own explicit Pages build and deploy jobs;
   - install Trunk and nightly only for browser demos that need them.
8. Build and inspect the actual `web/dist` artifact under the project base path
   before calling the migration complete.

## Ownership rules

- Keep generic visual behavior and CSS in shared. Keep only project-specific
  layout/content CSS in the consumer.
- Use shared components directly. Do not retain downstream wrappers or copied
  helpers once shared exposes the required API.
- Use `DemoCardAccent::for_position(position, total)` for ordinary galleries.
  Use an explicit accent only when color has project-specific semantic meaning.
- Test project-owned routes, content, metadata, and integration. Do not repeat
  generic shared component-rendering tests downstream.
- Treat `web/dist`, generated books, llms.txt trees, and built wasm demos as
  generated outputs. Rebuild them through the owning task.
- Preserve the checked-in `web/public/.nojekyll` and
  `web/public/dx-components-theme.css` inputs used by current local-development
  flows; `WebBuildConfig` writes the authoritative copies into `web/dist`.
- Keep root-relative application routes separate from the GitHub Pages project
  prefix. Let Dioxus CLI config and shared routing types apply the base path.
- Use `NavigationTarget::Internal` for typed Dioxus routes and
  `NavigationTarget::External` or typed `Href` values for external/static
  destinations.
- Preserve the consumer's existing localization manager, route organization,
  asset pipeline, preview implementation, deployment workflow, and command
  naming where they do not conflict with the shared contract.

## Reference selection

Read [references/adoption-patterns.md](references/adoption-patterns.md) when
implementing or reviewing an adoption. It contains the evidenced Cargo, Dioxus,
routing, xtask, preview, and workflow shapes.

Prefer current local source at the pinned shared revision over the reference
when signatures differ.

## Validation

Use repository recipes where available, then validate the affected surfaces:

1. Format Rust and repository-owned config.
2. Run the workspace check and warning-denied Clippy command.
3. Compile or test the web package with its supported feature combinations.
4. Build books, llms.txt outputs, browser demos, and the final web artifact
   through xtask.
5. Confirm `web/dist` contains:
   - `index.html`, `404.html`, and `.nojekyll`;
   - `dx-components-theme.css` and the project stylesheet;
   - an `index.html` fallback for every exported route;
   - `sitemap.xml`;
   - requested book, llms.txt, and demo outputs.
6. Preview with the project base path and exercise direct navigation to nested
   routes.
7. Run `git diff --check`, review generated diffs, and verify all shared Cargo
   sources resolve to one requested SHA.

When shared itself changed, validate the focused shared crates and dummy site
before validating consumers. Do not migrate preview or deployment while
updating a shared revision unless that migration is part of the request.
