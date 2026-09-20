---
name: use-stayhydated-github-pages
description: Adopt, update, or review Rust/Dioxus GitHub Pages sites that consume stayhydated/shared. Covers shared revision pins, project shells, route manifests, generated outputs, static preview, and reusable workflows.
---

# Use stayhydated GitHub Pages

Maintain the consumer's site against its pinned shared API. For a review,
report findings and use read-only checks; apply changes when the user requests
adoption, updates, or fixes.

## Establish the source of truth

- Read the consumer's repository guidance, manifests, `justfile`, web and xtask
  sources, and relevant workflows.
- Read shared source at the consumer's full pinned SHA. For a revision update,
  compare it with the intended target revision before changing the consumer.
- Identify the project slug, canonical URL, Dioxus base path, routes, generated
  outputs, optional browser demos, and default branch.

## Choose the application shape

| Consumer need | Shared application |
| --- | --- |
| One portal route | `StayhydatedSinglePageProjectApp` with `ProjectSite` |
| Portal with one embedded static demo | `StayhydatedEmbeddedDemoProjectApp` and its manifest preset |
| Consumer-defined routes | `StayhydatedProjectApp<R>` with a consumer-owned `Routable` enum |

For a new adoption or full-site review, read
[references/adoption-patterns.md](references/adoption-patterns.md). For a narrow
change, read its matching section for dependency pins, application setup,
assets, build tasks, demos, preview, deployment, or revision automation.

A revision-only update should synchronize the shared pins and affected
lockfile packages, adapt any changed APIs, and run the consumer audit.

## Apply the ownership contract

Shared owns generic components, shells, styles, routing helpers, static
assembly, and reusable workflows. The consumer owns identity, URLs,
destinations, routes, project-specific CSS, demo inputs, and build sequencing.

- Pin `stayhydated-dioxus`, `stayhydated-site`, and `stayhydated-xtask` to one
  full shared SHA in workspace dependencies. Keep any directly used
  `stayhydated-dioxus-core` dependency on that revision too.
- Define consumer-owned `Project` and `ProjectSite` values. Configure a Skills
  command or demo path only when its destination exists.
- Keep the web package featureless, enable Dioxus Web directly, and launch with
  `stayhydated_site::launch(web::App)`.
- Keep application routes root-relative. Dioxus configuration and shared
  routing helpers apply the GitHub Pages project prefix.
- Export one `SiteRouteManifest` for fallback generation and the sitemap.
  Build every static output it declares; the project presets include book and
  LLM destinations.
- Use shared components and bundled styles directly. Put only project-specific
  CSS in the consumer, and configure its stylesheet explicitly.
- Assemble the standard layout with `WebBuildConfig::github_pages`, adding
  explicit assets and demos only when the consumer owns them.
- Keep `web-build`, `web`, and `web-preview` distinct. Build prerequisites before
  final assembly, and regenerate output through its owning task.
- Use the reusable Pages and revision-update workflows for the standard
  integration. Install Trunk or nightly only for demos that require them.

## Validate the consumer

Select checks from the reference's
[validation checklist](references/adoption-patterns.md#validation-checklist)
that fit the requested work. Run [the consumer audit](scripts/audit_consumer.py)
with `--expected-shared-revision`. Add `--dist` and `--site-url` to inspect an
assembled artifact, and `--project-style-input` for a tracked consumer stylesheet.

The audit checks the standard `web/` and `xtask/` layout, shared pins, reusable
workflows, and declared sitemap destinations. It cannot infer omitted static
paths or prove browser behavior. Keep consumer manifest tests for promised
outputs and inspect direct navigation under the configured base path when
routing or assembly changes.

Report checks that ran, failures, and remaining gaps. For changes to shared
itself, start with the affected crates and dummy integration before validating
consumers.
