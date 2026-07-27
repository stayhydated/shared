#!/usr/bin/env python3

import argparse
import json
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
SHARED_DEVELOPMENT_INPUTS = (
    "web/public/.nojekyll",
    "web/public/dx-components-theme.css",
)
DEFAULT_PROJECT_STYLE_INPUT = Path("web/public/assets/site.css")
REQUIRED_DIST_FILES = (
    ".nojekyll",
    "404.html",
    "assets/site.css",
    "dx-components-theme.css",
    "index.html",
    "sitemap.xml",
)
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
        default=DEFAULT_PROJECT_STYLE_INPUT,
        type=Path,
        help=(
            "Consumer-relative tracked stylesheet source copied to "
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


def read_json(path: Path) -> Table:
    try:
        parsed: object = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
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
    package_name = require_string(
        read_json(root / "package.json"),
        "name",
        "package.json",
    )
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

    configured_names = (package_name, application_name, base_path)
    normalized = [name.strip().strip("/") for name in configured_names]
    if len(set(normalized)) != 1:
        raise AuditError(
            "package.json name and Dioxus application name/base_path "
            "must use one project slug"
        )

    workspace = require_table(cargo, "workspace", "Cargo.toml")
    workspace_package = optional_table(workspace, "package", "Cargo.toml.workspace")
    if workspace_package is not None and "repository" in workspace_package:
        repository = require_string(
            workspace_package,
            "repository",
            "Cargo.toml.workspace.package",
        )
        repository_slug = urlparse(repository).path.rstrip("/").rsplit("/", 1)[-1]
        repository_slug = repository_slug.removesuffix(".git")
        if repository_slug and repository_slug != normalized[0]:
            raise AuditError(
                f"workspace repository slug {repository_slug!r} "
                f"differs from {normalized[0]!r}"
            )

    return normalized[0]


def consumer_relative_path(path: Path, option: str) -> str:
    if path.is_absolute() or path == Path(".") or ".." in path.parts:
        raise AuditError(
            f"{option} must be a consumer-relative path without parent traversal"
        )
    return path.as_posix()


def check_tracked_inputs(root: Path, project_style_input: Path) -> None:
    tracked_inputs = (
        *SHARED_DEVELOPMENT_INPUTS,
        consumer_relative_path(project_style_input, "--project-style-input"),
    )
    try:
        result = subprocess.run(
            [
                "git",
                "-C",
                str(root),
                "ls-files",
                "--error-unmatch",
                *tracked_inputs,
            ],
            capture_output=True,
            check=False,
            text=True,
        )
    except OSError as error:
        raise AuditError(f"failed to inspect tracked inputs: {error}") from error
    if result.returncode != 0:
        raise AuditError(
            "tracked local-development inputs are incomplete: "
            + ", ".join(tracked_inputs)
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


def check_dist(root: Path, slug: str, canonical_site_url: str) -> None:
    site_url = parse_site_url(canonical_site_url, slug)
    dist = root / "web" / "dist"
    for relative in REQUIRED_DIST_FILES:
        if not (dist / relative).is_file():
            raise AuditError(f"missing assembled output web/dist/{relative}")

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
        slug = project_slug(root, cargo)
        check_tracked_inputs(root, args.project_style_input)
        if args.dist:
            if args.site_url is None:
                raise AuditError("--site-url is required with --dist")
            check_dist(root, slug, args.site_url)
    except AuditError as error:
        print(f"consumer audit failed: {error}", file=sys.stderr)
        return 1

    scope = "source and dist" if args.dist else "source"
    print(f"consumer audit passed ({scope}): slug={slug} shared={revision}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
