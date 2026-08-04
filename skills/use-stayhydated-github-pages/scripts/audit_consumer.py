#!/usr/bin/env python3

import argparse
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from collections.abc import Mapping
from pathlib import Path
from typing import TypeAlias, cast
from urllib.parse import ParseResult, urlparse

import tomllib

Table: TypeAlias = Mapping[str, object]

SHARED_PACKAGES = (
    "stayhydated-dioxus",
    "stayhydated-site",
    "stayhydated-xtask",
)
REQUIRED_DIST_FILES = (
    ".nojekyll",
    "404.html",
    "index.html",
    "sitemap.xml",
)
REMOVED_THEME_PATH = Path("web/public/dx-components-theme.css")
FULL_SHA = re.compile(r"^[0-9a-f]{40}$")


class AuditError(Exception):
    """A consumer configuration or artifact violates the shared site contract."""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Audit a stayhydated/shared GitHub Pages consumer."
    )
    parser.add_argument("consumer", type=Path, help="Path to the consumer repository.")
    parser.add_argument(
        "--dist",
        action="store_true",
        help="Also validate the assembled web/dist artifact and sitemap routes.",
    )
    parser.add_argument(
        "--project-style-input",
        type=Path,
        help=(
            "Optional consumer-relative tracked stylesheet source expected at "
            "web/dist/assets/site.css."
        ),
    )
    parser.add_argument(
        "--site-url",
        help=(
            "Canonical project URL, required with --dist "
            "(for example, https://example.github.io/project/)."
        ),
    )
    parser.add_argument(
        "--expected-shared-revision",
        help="Full shared SHA that every pinned dependency must resolve to.",
    )
    return parser.parse_args()


def as_table(value: object, context: str) -> Table:
    if not isinstance(value, dict):
        raise AuditError(f"{context} must be a table")
    return cast(dict[str, object], value)


def require_table(table: Table, key: str, context: str) -> Table:
    if key not in table:
        raise AuditError(f"{context}.{key} is required")
    return as_table(table[key], f"{context}.{key}")


def optional_table(table: Table, key: str, context: str) -> Table | None:
    value = table.get(key)
    if value is None:
        return None
    return as_table(value, f"{context}.{key}")


def require_string(table: Table, key: str, context: str) -> str:
    value = table.get(key)
    if not isinstance(value, str) or not value.strip():
        raise AuditError(f"{context}.{key} must be a non-empty string")
    return value


def require_table_list(table: Table, key: str, context: str) -> tuple[Table, ...]:
    value = table.get(key)
    if not isinstance(value, list):
        raise AuditError(f"{context}.{key} must be an array of tables")

    entries = []
    for index, entry in enumerate(value):
        entries.append(as_table(entry, f"{context}.{key}[{index}]"))
    return tuple(entries)


def read_toml(path: Path) -> Table:
    try:
        parsed = tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise AuditError(f"failed to read {path}: {error}") from error
    return as_table(parsed, str(path))


def workspace_shared_revision(root: Path, cargo: Table) -> str:
    workspace = require_table(cargo, "workspace", "Cargo.toml")
    dependencies = require_table(workspace, "dependencies", "Cargo.toml.workspace")
    revisions = []

    for package in SHARED_PACKAGES:
        dependency = require_table(
            dependencies,
            package,
            "Cargo.toml.workspace.dependencies",
        )
        dependency_context = f"Cargo.toml.workspace.dependencies.{package}"
        if (
            require_string(dependency, "git", dependency_context)
            != "https://github.com/stayhydated/shared"
        ):
            raise AuditError(
                f"{package} must use https://github.com/stayhydated/shared"
            )

        revision = require_string(dependency, "rev", dependency_context)
        if not FULL_SHA.fullmatch(revision):
            raise AuditError(f"{package} must use a full 40-character shared revision")
        revisions.append(revision)

    if len(set(revisions)) != 1:
        raise AuditError(
            "all stayhydated/shared workspace dependencies must use one revision"
        )

    revision = revisions[0]
    cargo_lock = read_toml(root / "Cargo.lock")
    packages = require_table_list(cargo_lock, "package", "Cargo.lock")
    for package_name in SHARED_PACKAGES:
        matching = [
            package for package in packages if package.get("name") == package_name
        ]
        if len(matching) != 1:
            raise AuditError(f"Cargo.lock must contain one {package_name} package")

        source = require_string(
            matching[0],
            "source",
            f"Cargo.lock package {package_name}",
        )
        if not source.startswith("git+https://github.com/stayhydated/shared"):
            raise AuditError(
                f"Cargo.lock {package_name} must resolve from stayhydated/shared"
            )
        if not source.endswith(f"#{revision}"):
            raise AuditError(
                f"Cargo.lock {package_name} does not resolve to {revision}"
            )

    return revision


def project_slug(root: Path, cargo: Table) -> str:
    dioxus = read_toml(root / "web" / "Dioxus.toml")
    application = require_table(dioxus, "application", "web/Dioxus.toml")
    application_name = require_string(
        application,
        "name",
        "web/Dioxus.toml.application",
    )
    web = require_table(dioxus, "web", "web/Dioxus.toml")
    app = require_table(web, "app", "web/Dioxus.toml.web")
    base_path = require_string(app, "base_path", "web/Dioxus.toml.web.app")

    configured_names = (application_name, base_path)
    normalized = [name.strip().strip("/") for name in configured_names]
    if any(not name for name in normalized):
        raise AuditError(
            "Dioxus application name and base_path must use a non-empty project slug"
        )
    if len(set(normalized)) != 1:
        raise AuditError(
            "Dioxus application name and base_path must use one project slug"
        )

    workspace = require_table(cargo, "workspace", "Cargo.toml")
    workspace_package = optional_table(workspace, "package", "Cargo.toml.workspace")
    root_package = optional_table(cargo, "package", "Cargo.toml")
    repository_sources = (
        (workspace_package, "Cargo.toml.workspace.package"),
        (root_package, "Cargo.toml.package"),
    )
    for package, context in repository_sources:
        if package is None or "repository" not in package:
            continue
        repository = require_string(
            package,
            "repository",
            context,
        )
        repository_slug = urlparse(repository).path.rstrip("/").rsplit("/", 1)[-1]
        repository_slug = repository_slug.removesuffix(".git")
        if repository_slug and repository_slug != normalized[0]:
            raise AuditError(
                f"{context} repository slug {repository_slug!r} differs from "
                f"{normalized[0]!r}"
            )

    return normalized[0]


def consumer_relative_path(path: Path, option: str) -> str:
    if path.is_absolute() or path == Path(".") or ".." in path.parts:
        raise AuditError(
            f"{option} must be a consumer-relative path without parent traversal"
        )
    return path.as_posix()


def check_tracked_style(root: Path, project_style_input: Path | None) -> None:
    if project_style_input is None:
        default_style = root / "web" / "public" / "assets" / "site.css"
        if default_style.exists():
            raise AuditError(
                "web/public/assets/site.css exists; pass --project-style-input "
                "when it contains project-specific CSS or remove it"
            )
        return

    tracked_input = consumer_relative_path(
        project_style_input,
        "--project-style-input",
    )
    try:
        result = subprocess.run(
            [
                "git",
                "-C",
                str(root),
                "ls-files",
                "--error-unmatch",
                tracked_input,
            ],
            capture_output=True,
            check=False,
            text=True,
        )
    except OSError as error:
        raise AuditError(f"failed to inspect tracked inputs: {error}") from error
    if result.returncode != 0:
        raise AuditError(f"project stylesheet is not tracked: {tracked_input}")


def dependency_features(dependency: object, context: str) -> tuple[str, ...]:
    table = as_table(dependency, context)
    value = table.get("features", [])
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        raise AuditError(f"{context}.features must be an array of strings")
    return tuple(cast(list[str], value))


def check_web_source_contract(root: Path) -> None:
    web_cargo = read_toml(root / "web" / "Cargo.toml")
    if "features" in web_cargo:
        raise AuditError("web/Cargo.toml must not define a feature matrix")

    dependencies = require_table(web_cargo, "dependencies", "web/Cargo.toml")
    dioxus_features = dependency_features(
        dependencies.get("dioxus"),
        "web/Cargo.toml.dependencies.dioxus",
    )
    if "web" not in dioxus_features:
        raise AuditError("web/Cargo.toml must enable the Dioxus `web` feature")
    if "ssr" in dioxus_features:
        raise AuditError("web/Cargo.toml must not enable Dioxus SSR")

    dev_dependencies = optional_table(
        web_cargo,
        "dev-dependencies",
        "web/Cargo.toml",
    )
    if dev_dependencies is not None and "dioxus" in dev_dependencies:
        dev_features = dependency_features(
            dev_dependencies["dioxus"],
            "web/Cargo.toml.dev-dependencies.dioxus",
        )
        if "ssr" in dev_features:
            raise AuditError("web tests must not enable Dioxus SSR")

    try:
        main_source = (root / "web" / "src" / "main.rs").read_text(encoding="utf-8")
        library_source = (root / "web" / "src" / "lib.rs").read_text(encoding="utf-8")
        build_source = (
            root / "xtask" / "src" / "commands" / "build_web.rs"
        ).read_text(encoding="utf-8")
    except OSError as error:
        raise AuditError(f"failed to read web source contract: {error}") from error

    if "cfg(feature" in main_source or "SiteApp" in main_source:
        raise AuditError("web/src/main.rs must use the unconditional launch API")
    if "stayhydated_site::launch(" not in main_source:
        raise AuditError("web/src/main.rs must call stayhydated_site::launch")
    if "route_manifest" not in library_source:
        raise AuditError("web/src/lib.rs must export a route manifest")
    if "route_manifest(web::route_manifest())" not in build_source:
        raise AuditError("xtask build_web must pass the web route manifest")
    for removed in ("route_fallback_paths(", "sitemap_xml("):
        if removed in build_source:
            raise AuditError(f"xtask build_web still uses removed API {removed}")

    if (root / REMOVED_THEME_PATH).exists():
        raise AuditError(f"remove bundled theme copy {REMOVED_THEME_PATH}")


def check_workflows(root: Path) -> None:
    try:
        pages = (root / ".github/workflows/gh-pages.yml").read_text(encoding="utf-8")
        revisions = (
            root / ".github/workflows/update-shared-revisions.yml"
        ).read_text(encoding="utf-8")
    except OSError as error:
        raise AuditError(f"failed to read shared workflows: {error}") from error

    if "stayhydated/shared/.github/workflows/deploy-pages.yml@" not in pages:
        raise AuditError("gh-pages workflow must call the shared Pages workflow")
    if "stayhydated/shared/.github/workflows/update-shared-revisions.yml@" not in revisions:
        raise AuditError("shared revision workflow must call the reusable updater")


def just_recipe(justfile: str, name: str) -> tuple[tuple[str, ...], tuple[str, ...]]:
    header_match = re.search(rf"(?m)^{re.escape(name)}:([^\n]*)\n", justfile)
    if header_match is None:
        raise AuditError(f"justfile must define a {name} recipe")

    dependencies = tuple(header_match.group(1).split())
    body = []
    for line in justfile[header_match.end() :].splitlines():
        if not line.strip():
            continue
        if line[0].isspace():
            body.append(line.strip())
            continue
        break
    return dependencies, tuple(body)


def check_justfile(root: Path) -> None:
    try:
        justfile = (root / "justfile").read_text(encoding="utf-8")
    except OSError as error:
        raise AuditError(f"failed to read {root / 'justfile'}: {error}") from error

    _, web_build_body = just_recipe(justfile, "web-build")
    if "cargo xtask build web" not in web_build_body:
        raise AuditError("justfile web-build must run `cargo xtask build web`")

    web_dependencies, web_body = just_recipe(justfile, "web")
    if web_dependencies != ("web-build",) or web_body != ("dx serve --package web",):
        raise AuditError(
            "justfile web must depend on web-build and run `dx serve --package web`"
        )

    preview_dependencies, preview_body = just_recipe(justfile, "web-preview")
    if preview_dependencies != ("web-build",) or preview_body != (
        "cargo xtask preview web",
    ):
        raise AuditError(
            "justfile web-preview must depend on web-build and run "
            "`cargo xtask preview web`"
        )


def parse_site_url(value: str, slug: str) -> ParseResult:
    site_url = urlparse(value)
    if site_url.scheme.lower() != "https" or not site_url.netloc:
        raise AuditError("--site-url must be an absolute HTTPS URL")
    if site_url.params or site_url.query or site_url.fragment:
        raise AuditError(
            "--site-url must not contain parameters, a query, or a fragment"
        )

    expected_path = f"/{slug}/"
    if site_url.path != expected_path:
        raise AuditError(f"--site-url path must be {expected_path}")
    return site_url


def check_dist(
    root: Path,
    slug: str,
    canonical_site_url: str,
    project_style_input: Path | None,
) -> None:
    site_url = parse_site_url(canonical_site_url, slug)
    dist = root / "web" / "dist"
    for relative in REQUIRED_DIST_FILES:
        if not (dist / relative).is_file():
            raise AuditError(f"missing assembled output web/dist/{relative}")
    project_style = dist / "assets/site.css"
    if project_style_input is not None and not project_style.is_file():
        raise AuditError("missing assembled project stylesheet web/dist/assets/site.css")
    if project_style_input is None and project_style.exists():
        raise AuditError("unexpected assembled project stylesheet web/dist/assets/site.css")
    if (dist / "dx-components-theme.css").exists():
        raise AuditError("web/dist must not contain a copied component theme")
    bundled_themes = tuple((dist / "assets").glob("dx-components-theme*.css"))
    if not any(path.is_file() for path in bundled_themes):
        raise AuditError("missing bundled Dioxus component theme under web/dist/assets")

    try:
        sitemap_root = ET.parse(dist / "sitemap.xml").getroot()
    except (OSError, ET.ParseError) as error:
        raise AuditError(f"failed to read web/dist/sitemap.xml: {error}") from error

    namespace = {"sitemap": "http://www.sitemaps.org/schemas/sitemap/0.9"}
    locations = sitemap_root.findall("sitemap:url/sitemap:loc", namespace)
    if not locations:
        raise AuditError("web/dist/sitemap.xml contains no URL locations")

    site_origin = (site_url.scheme.lower(), site_url.netloc.lower())
    site_prefix = site_url.path
    for location in locations:
        if not location.text:
            raise AuditError("web/dist/sitemap.xml contains an empty URL location")

        location_text = location.text.strip()
        location_url = urlparse(location_text)
        location_origin = (
            location_url.scheme.lower(),
            location_url.netloc.lower(),
        )
        if location_origin != site_origin:
            expected_origin = f"{site_url.scheme}://{site_url.netloc}"
            raise AuditError(
                f"sitemap URL has origin outside {expected_origin}: {location_text}"
            )

        path = location_url.path
        if not path.startswith(site_prefix):
            raise AuditError(f"sitemap URL is outside {site_prefix}: {location_text}")

        relative = path.removeprefix(site_prefix)
        output = dist / relative
        expected = output / "index.html" if path.endswith("/") else output
        if not expected.is_file():
            raise AuditError(
                f"sitemap URL {location_text} has no output at "
                f"{expected.relative_to(root)}"
            )


def main() -> int:
    args = parse_args()
    root = args.consumer.resolve()

    try:
        cargo = read_toml(root / "Cargo.toml")
        revision = workspace_shared_revision(root, cargo)
        if args.expected_shared_revision is not None:
            if not FULL_SHA.fullmatch(args.expected_shared_revision):
                raise AuditError(
                    "--expected-shared-revision must be a full 40-character SHA"
                )
            if revision != args.expected_shared_revision:
                raise AuditError(
                    f"shared revision is {revision}, expected "
                    f"{args.expected_shared_revision}"
                )
        slug = project_slug(root, cargo)
        check_tracked_style(root, args.project_style_input)
        check_web_source_contract(root)
        check_workflows(root)
        check_justfile(root)
        if args.dist:
            if args.site_url is None:
                raise AuditError("--site-url is required with --dist")
            check_dist(root, slug, args.site_url, args.project_style_input)
    except AuditError as error:
        print(f"consumer audit failed: {error}", file=sys.stderr)
        return 1

    scope = "source and dist" if args.dist else "source"
    print(f"consumer audit passed ({scope}): slug={slug} shared={revision}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
