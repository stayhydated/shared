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

Keep the project name, tagline, canonical URL, docs/book/source/demo
destinations, Skills command, landing theme, and other project identity in the
consumer repository.

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
   - use explicit assets and demo inputs when the consumer already owns them.
7. Preserve the consumer's preview and deployment topology unless the task
   explicitly requests a migration:
   - do not introduce non-Cargo package manifests, package-manager commands,
     or JavaScript or TypeScript tooling solely for the Pages site;
   - use an existing non-JavaScript static preview when available, otherwise
     validate the assembled artifact through the consumer audit;
   - install Trunk and nightly only for browser demos that need them.
8. Build and inspect the actual `web/dist` artifact under the project base path
   before calling the site work complete.

## Ownership rules

- Keep generic visual behavior and CSS in shared. Keep only project-specific
  layout/content CSS in the consumer.
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
