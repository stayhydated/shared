# Configure the application

Use `stayhydated-dioxus` to put your project's identity and destinations into a
shared application shell. Use `stayhydated-dioxus-core` when composing individual
components in your own shell.

## Choose a shell

Create a `Project` with your name and description, then build a `ProjectSite`
with the canonical site URL, API documentation URL, source URL, and version.
Add a skill command or demo path only when its destination exists. Set
`site_stylesheet_path` only for a real consumer-owned stylesheet.

| Site shape | Component | Exported manifest |
| --- | --- | --- |
| One portal route | `StayhydatedSinglePageProjectApp` | `single_page_route_manifest()` |
| Portal with one embedded static demo | `StayhydatedEmbeddedDemoProjectApp` | `embedded_demo_route_manifest()` |
| Your own `Routable` enum | `StayhydatedProjectApp<R>` | `route_manifest::<R>()` |

Pass the same `ProjectSite` configuration to the selected shell and manifest
method. The embedded-demo preset creates an application route at `/demo/` and
uses `demo_path` for the separately built iframe artifact. Custom routes need
your own page content and metadata.

The consumer's web binary launches its exported application through
`stayhydated_site::launch`. Enable Dioxus Web in that package and configure the
Dioxus base path for the hosted project prefix. Keep manifest paths independent
of that prefix; see [route ownership](routes.md).

The [dummy configuration][configuration] and [application][application] show a
complete multi-route consumer. The [adoption reference][adoption] contains the
single-page and embedded-demo forms.

## Compose core components

Core provides `CodeBlock`, `DemoGallery`, tabs, selects, landing and portal
components, `FullscreenDemoFrame`, metadata, motion styles, and
`ShaderBackground`. Configured shells include shared styles automatically.
When using core directly, render `SharedStyles` once at the application root.

Give each `ShaderBackground` a distinct `canvas_id`. Its browser renderer is
compiled only for WebAssembly; native markup tests do not prove GPU or browser
behavior.

Use `select::SearchableSelect<T>` for typed options with searchable labels.
Supply `SelectItem<T>` entries and an accessible input name. Use a controlled
selection signal when your application owns the selected value. Disabled items
remain visible but cannot be selected. The [language selector][select-example]
shows the complete component shape.

## Verify the result

Check the configured identity and destinations before rendering. In a browser,
verify the title, portal links, shared theme, keyboard interaction, and direct
navigation under the project's base path. Test consumer-owned configuration and
manifests directly; shared render and compile-pass tests cover the reusable
component surfaces.

[configuration]: https://github.com/stayhydated/shared/blob/master/dummy/web-dummy/src/site/constants.rs
[application]: https://github.com/stayhydated/shared/blob/master/dummy/web-dummy/src/site/app.rs
[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/references/adoption-patterns.md
[select-example]: https://github.com/stayhydated/shared/blob/master/crates/stayhydated-dioxus-core/tests/pass/searchable_language_select.rs
