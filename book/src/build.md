# Build and verify a site

Call `stayhydated-xtask` from your repository's own build task to generate
documentation, assemble the site, and preview the final artifact. Your task
chooses inputs, output paths, and build order.

## Prepare the build

Use the shared revision and Rust requirements described in
[package selection](introduction.md). For a web build, provide the Dioxus 0.7
CLI (`dx`), the WebAssembly target, an up-to-date consumer lockfile, and your
application's Dioxus configuration. Browser demos additionally need Trunk and
any toolchain their framework requires. Static preview uses Bun by default.

## Assemble the outputs

1. Build your book with `book::build`, supplying its source and output
   directories. `build_workspace_book` uses the standard `book/` source and
   `web/public/book` destination. Disable automatic missing-chapter creation
   when validating documentation with `MDBOOK_BUILD__CREATE_MISSING=false`.
2. Generate LLM text with `llms::build` or `build_workspace_llms` when your
   manifest declares those outputs. Use the same book source and canonical URL.
3. Build any browser demos with `trunk::build`. Supply the real demo input,
   output directory, and required marker; the helper verifies JavaScript, Wasm,
   and marker output. Do not require Trunk or nightly for a site without demos.
4. Construct `WebBuildConfig::github_pages` with the workspace root and web
   package, pass the exported route manifest, then call `web::build`. Add
   consumer-owned demo directories with `extra_dir` before final assembly.

The standard web helper runs a release Dioxus Web build with static-site
generation and `--locked`. It assembles `web/dist`, copies available book, LLM,
and public assets, writes application fallbacks and `404.html`, and renders the
manifest's sitemap. A declared output must exist before assembly.

The [dummy build task][dummy-build] demonstrates an explicit layout. For the
standard layout, use the [adoption reference][adoption] rather than copying the
dummy application's paths into your consumer.

## Preview and verify

Pass the assembled directory and project base path to
`preview::StaticSitePreviewConfig`, then call `preview::serve`. A preview of the
built artifact exercises copied files and fallbacks that a development server
alone does not verify.

Confirm that the artifact contains `index.html`, `404.html`, `.nojekyll`, and
`sitemap.xml`, plus every declared book, LLM, or browser-demo output. Open nested
routes directly, check shared styles, and follow static links from the portal.

For source and artifact checks, use the existing [consumer audit][audit] with
your pinned revision. The adoption reference documents its arguments and the
standard layout it checks. It cannot infer a destination omitted from your
manifest or prove browser behavior.

Use the reusable Pages workflow only after defining the consumer build command
and artifact path. Its default artifact is `web/dist`; optional demo tool
installation belongs to the consumer's declared requirements. Building this
guide locally does not deploy a site.

## Troubleshoot build boundaries

| Symptom | Action |
| --- | --- |
| Locked dependency resolution fails | Update and review the consumer lockfile for the selected shared revision. |
| `dx` or `trunk` cannot start | Make the required CLI available to the build task; Trunk is needed only for its demos. |
| A sitemap destination is missing | Build that output before assembly and check its copy destination. |
| Links contain the project prefix twice | Remove the prefix from manifest paths and retain it in site/Dioxus configuration. |
| Preview reports a missing directory | Build the assembled artifact first and pass its actual path. |

[dummy-build]: https://github.com/stayhydated/shared/blob/master/dummy/xtask-dummy/src/commands/build_web.rs
[adoption]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/references/adoption-patterns.md
[audit]: https://github.com/stayhydated/shared/blob/master/skills/use-stayhydated-github-pages/scripts/audit_consumer.py
