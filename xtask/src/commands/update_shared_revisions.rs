use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context as _, bail};
use cargo_metadata::MetadataCommand;
use toml_edit::{DocumentMut, Item, TableLike, Value, visit_mut};

const SHARED_BRANCH: &str = "master";
const SHARED_GIT_URL: &str = "https://github.com/stayhydated/shared";

#[derive(Clone, Debug, Eq, PartialEq)]
struct CommitSha(String);

impl CommitSha {
    fn parse(value: impl Into<String>) -> anyhow::Result<Self> {
        let value = value.into();
        if value.len() != 40 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            bail!("invalid full commit SHA `{value}`");
        }
        Ok(Self(value.to_ascii_lowercase()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn short(&self) -> &str {
        &self.0[..7]
    }
}

impl std::fmt::Display for CommitSha {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UpdateReport {
    source_sha: CommitSha,
    packages: Vec<String>,
    matched_revisions: usize,
    changed_revisions: usize,
    changed_manifests: usize,
}

impl UpdateReport {
    fn changed(&self) -> bool {
        self.changed_revisions > 0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct UpdateStats {
    matches: usize,
    changes: usize,
}

impl std::ops::AddAssign for UpdateStats {
    fn add_assign(&mut self, other: Self) {
        self.matches += other.matches;
        self.changes += other.changes;
    }
}

#[derive(Debug)]
struct ManifestUpdate {
    path: PathBuf,
    rendered: String,
    stats: UpdateStats,
}

pub fn run(workspace_root: &Path) -> anyhow::Result<()> {
    let report = update_shared_revisions(workspace_root)?;
    if report.changed() {
        update_cargo_lockfile(workspace_root, &report.packages)?;
    }
    print_report(&report);
    write_github_files(&report)
}

fn update_shared_revisions(workspace_root: &Path) -> anyhow::Result<UpdateReport> {
    let packages = discover_shared_packages(workspace_root)?;
    let source_sha = resolve_source_sha(workspace_root)?;
    let manifests = discover_cargo_manifests(workspace_root)?;
    let (planned_updates, stats) = plan_manifest_updates(&manifests, &source_sha)?;
    let changed_manifests = write_manifest_updates(&planned_updates)?;

    Ok(UpdateReport {
        source_sha,
        packages,
        matched_revisions: stats.matches,
        changed_revisions: stats.changes,
        changed_manifests,
    })
}

fn discover_shared_packages(workspace_root: &Path) -> anyhow::Result<Vec<String>> {
    let metadata = MetadataCommand::new()
        .current_dir(workspace_root)
        .no_deps()
        .exec()
        .context("failed to read downstream Cargo metadata")?;
    let packages = metadata
        .workspace_packages()
        .into_iter()
        .flat_map(|package| &package.dependencies)
        .filter(|dependency| {
            dependency
                .source
                .as_ref()
                .is_some_and(|source| is_shared_git_source(&source.repr))
        })
        .map(|dependency| dependency.name.to_string())
        .collect::<BTreeSet<_>>();
    Ok(packages.into_iter().collect())
}

fn update_cargo_lockfile(workspace_root: &Path, packages: &[String]) -> anyhow::Result<()> {
    let mut command = cargo_lockfile_command(workspace_root, packages);
    let status = command
        .status()
        .context("failed to run Cargo while updating the downstream lockfile")?;
    if !status.success() {
        bail!("failed to update the downstream Cargo lockfile");
    }
    Ok(())
}

fn cargo_lockfile_command(workspace_root: &Path, packages: &[String]) -> Command {
    let mut command = Command::new("cargo");
    command.current_dir(workspace_root);
    if packages.is_empty() {
        command
            .args(["metadata", "--format-version", "1"])
            .stdout(Stdio::null());
    } else {
        command.arg("update");
        for package in packages {
            command.args(["--package", package]);
        }
    }
    command
}

fn resolve_source_sha(workspace_root: &Path) -> anyhow::Result<CommitSha> {
    let output = Command::new("git")
        .arg("-C")
        .arg(workspace_root)
        .args([
            "ls-remote",
            "--refs",
            SHARED_GIT_URL,
            &format!("refs/heads/{SHARED_BRANCH}"),
        ])
        .output()
        .context("failed to run git ls-remote")?;
    if !output.status.success() {
        bail!(
            "failed to resolve stayhydated/shared@{SHARED_BRANCH}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let stdout = String::from_utf8(output.stdout).context("git ls-remote returned non-UTF-8")?;
    let mut fields = stdout.split_whitespace();
    let Some(sha) = fields.next() else {
        bail!("could not resolve stayhydated/shared@{SHARED_BRANCH}");
    };
    let Some(reference) = fields.next() else {
        bail!("git ls-remote returned an incomplete result");
    };
    if fields.next().is_some() || reference != format!("refs/heads/{SHARED_BRANCH}") {
        bail!("git ls-remote returned an unexpected result");
    }
    CommitSha::parse(sha)
}

fn discover_cargo_manifests(workspace_root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(workspace_root)
        .args(["ls-files", "-z", "--", "Cargo.toml", ":(glob)**/Cargo.toml"])
        .output()
        .context("failed to list tracked Cargo manifests")?;
    if !output.status.success() {
        bail!(
            "failed to list tracked Cargo manifests: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let stdout = String::from_utf8(output.stdout).context("git ls-files returned non-UTF-8")?;
    let mut manifests = stdout
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(|path| workspace_root.join(path))
        .collect::<Vec<_>>();
    manifests.sort();
    manifests.dedup();
    Ok(manifests)
}

fn plan_manifest_updates(
    manifests: &[PathBuf],
    source_sha: &CommitSha,
) -> anyhow::Result<(Vec<ManifestUpdate>, UpdateStats)> {
    let mut planned_updates = Vec::new();
    let mut total_stats = UpdateStats::default();

    for manifest in manifests {
        let original = fs::read_to_string(manifest)
            .with_context(|| format!("failed to read {}", manifest.display()))?;
        let (rendered, stats) = update_manifest_text(&original, source_sha)
            .with_context(|| format!("failed to update {}", manifest.display()))?;
        total_stats += stats;
        if stats.matches > 0 {
            planned_updates.push(ManifestUpdate {
                path: manifest.clone(),
                rendered,
                stats,
            });
        }
    }

    if total_stats.matches == 0 {
        bail!("no Cargo dependencies sourced from stayhydated/shared were found");
    }
    Ok((planned_updates, total_stats))
}

fn write_manifest_updates(planned_updates: &[ManifestUpdate]) -> anyhow::Result<usize> {
    let mut changed_manifests = 0;
    for update in planned_updates {
        if update.stats.changes == 0 {
            continue;
        }
        fs::write(&update.path, &update.rendered)
            .with_context(|| format!("failed to write {}", update.path.display()))?;
        changed_manifests += 1;
    }
    Ok(changed_manifests)
}

fn update_manifest_text(
    text: &str,
    source_sha: &CommitSha,
) -> anyhow::Result<(String, UpdateStats)> {
    let mut document = text
        .parse::<DocumentMut>()
        .context("failed to parse TOML")?;
    let mut updater = RevisionUpdater::new(source_sha);
    visit_mut::VisitMut::visit_document_mut(&mut updater, &mut document);
    let stats = updater.finish()?;
    Ok((document.to_string(), stats))
}

struct RevisionUpdater<'a> {
    source_sha: &'a CommitSha,
    stats: UpdateStats,
    error: Option<anyhow::Error>,
}

impl<'a> RevisionUpdater<'a> {
    fn new(source_sha: &'a CommitSha) -> Self {
        Self {
            source_sha,
            stats: UpdateStats::default(),
            error: None,
        }
    }

    fn update_dependency(&mut self, table: &mut dyn TableLike) -> anyhow::Result<()> {
        let Some(git_url) = table.get("git").and_then(Item::as_str) else {
            return Ok(());
        };
        if !is_shared_git_url(git_url) {
            return Ok(());
        }
        let git_url = git_url.to_owned();

        let Some(revision) = table.get_mut("rev") else {
            bail!("dependency using {git_url} has no rev");
        };
        let Some(current_revision) = revision.as_str().map(str::to_owned) else {
            bail!("dependency using {git_url} has a non-string rev");
        };
        CommitSha::parse(&current_revision).context("dependency has an invalid rev")?;

        let changed = !current_revision.eq_ignore_ascii_case(self.source_sha.as_str());
        if changed {
            replace_string_value(revision, self.source_sha.as_str())?;
        }
        self.stats.matches += 1;
        self.stats.changes += usize::from(changed);
        Ok(())
    }

    fn finish(self) -> anyhow::Result<UpdateStats> {
        self.error.map_or(Ok(self.stats), Err)
    }
}

impl visit_mut::VisitMut for RevisionUpdater<'_> {
    fn visit_table_like_mut(&mut self, table: &mut dyn TableLike) {
        if self.error.is_some() {
            return;
        }
        if let Err(error) = self.update_dependency(table) {
            self.error = Some(error);
            return;
        }
        visit_mut::visit_table_like_mut(self, table);
    }
}

fn replace_string_value(item: &mut Item, replacement: &str) -> anyhow::Result<()> {
    let Some(value) = item.as_value_mut() else {
        bail!("revision is not a TOML value");
    };
    if !matches!(value, Value::String(_)) {
        bail!("revision is not a TOML string");
    }
    let decor = value.decor().clone();
    *value = Value::from(replacement);
    *value.decor_mut() = decor;
    Ok(())
}

fn is_shared_git_url(value: &str) -> bool {
    value
        .trim_end_matches('/')
        .strip_suffix(".git")
        .unwrap_or_else(|| value.trim_end_matches('/'))
        == SHARED_GIT_URL
}

fn is_shared_git_source(value: &str) -> bool {
    let value = value.strip_prefix("git+").unwrap_or(value);
    let repository = value.split(['?', '#']).next().unwrap_or(value);
    is_shared_git_url(repository)
}

fn print_report(report: &UpdateReport) {
    if report.changed() {
        println!(
            "Updated {} shared revisions in {} Cargo manifests to {}.",
            report.changed_revisions, report.changed_manifests, report.source_sha
        );
    } else {
        println!(
            "All {} shared revisions already point to {}.",
            report.matched_revisions, report.source_sha
        );
    }
}

fn write_github_files(report: &UpdateReport) -> anyhow::Result<()> {
    if let Some(path) = std::env::var_os("GITHUB_OUTPUT") {
        append_lines(
            Path::new(&path),
            &[
                format!("changed={}", report.changed()),
                format!("source_sha={}", report.source_sha),
                format!("source_short_sha={}", report.source_sha.short()),
            ],
        )?;
    }

    if let Some(path) = std::env::var_os("GITHUB_STEP_SUMMARY") {
        let summary = if report.changed() {
            format!(
                "Updated {} shared revisions in {} Cargo manifests to {}.",
                report.changed_revisions, report.changed_manifests, report.source_sha
            )
        } else {
            format!(
                "All {} shared revisions already point to {}.",
                report.matched_revisions, report.source_sha
            )
        };
        append_lines(Path::new(&path), &[summary])?;
    }
    Ok(())
}

fn append_lines(path: &Path, lines: &[String]) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to open {}", path.display()))?;
    for line in lines {
        writeln!(file, "{line}").with_context(|| format!("failed to write {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OLD_SHA: &str = "1111111111111111111111111111111111111111";
    const NEW_SHA: &str = "2222222222222222222222222222222222222222";
    const OTHER_SHA: &str = "3333333333333333333333333333333333333333";

    fn new_sha() -> CommitSha {
        CommitSha::parse(NEW_SHA).expect("fixture SHA should parse")
    }

    #[test]
    fn discovers_real_package_names_for_renamed_shared_dependencies() {
        let directory = tempfile::tempdir().expect("temporary directory should be created");
        let source_directory = directory.path().join("src");
        fs::create_dir(&source_directory).expect("source directory should be created");
        fs::write(
            directory.path().join("Cargo.toml"),
            format!(
                r#"[package]
name = "fixture"
version = "0.1.0"
edition = "2024"

[dependencies]
site = {{ package = "stayhydated-site", git = "{SHARED_GIT_URL}", rev = "{OLD_SHA}" }}
direct = {{ package = "stayhydated-dioxus", git = "{SHARED_GIT_URL}.git", rev = "{OLD_SHA}" }}
other = {{ git = "https://github.com/example/other", rev = "{OTHER_SHA}" }}
"#
            ),
        )
        .expect("fixture manifest should be written");
        fs::write(source_directory.join("lib.rs"), "").expect("fixture source should be written");

        let packages = discover_shared_packages(directory.path())
            .expect("shared package names should be discovered");

        assert_eq!(packages, ["stayhydated-dioxus", "stayhydated-site"]);
    }

    #[test]
    fn cargo_update_targets_only_discovered_shared_packages() {
        let packages = vec![
            "stayhydated-dioxus".to_owned(),
            "stayhydated-site".to_owned(),
        ];
        let command = cargo_lockfile_command(Path::new("workspace"), &packages);
        let arguments = command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            arguments,
            [
                "update",
                "--package",
                "stayhydated-dioxus",
                "--package",
                "stayhydated-site",
            ]
        );
        assert_eq!(command.get_current_dir(), Some(Path::new("workspace")));
    }

    #[test]
    fn cargo_metadata_refreshes_lockfile_without_resolved_shared_packages() {
        let command = cargo_lockfile_command(Path::new("workspace"), &[]);
        let arguments = command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(arguments, ["metadata", "--format-version", "1"]);
    }

    #[test]
    fn updates_inline_multiline_and_expanded_dependencies() {
        let original = format!(
            r#"[workspace.dependencies]
inline = {{ git = "https://github.com/stayhydated/shared", rev = "{OLD_SHA}" }}
multiline = {{
  rev = "{OLD_SHA}",
  features = ["one"],
  git = "https://github.com/stayhydated/shared.git",
}}
other = {{ git = "https://github.com/example/other", rev = "{OTHER_SHA}" }}

[dependencies.expanded]
git = "https://github.com/stayhydated/shared"
rev = "{OLD_SHA}"
"#
        );

        let (rendered, stats) =
            update_manifest_text(&original, &new_sha()).expect("manifest should update");

        assert_eq!(stats.matches, 3);
        assert_eq!(stats.changes, 3);
        assert_eq!(rendered, original.replace(OLD_SHA, NEW_SHA));
        assert!(rendered.contains(OTHER_SHA));
    }

    #[test]
    fn noop_preserves_manifest_exactly() {
        let original = format!(
            "[dependencies]\nshared = {{ git = \"https://github.com/stayhydated/shared\", rev = \"{NEW_SHA}\" }}\n"
        );

        let (rendered, stats) =
            update_manifest_text(&original, &new_sha()).expect("manifest should parse");

        assert_eq!(stats.matches, 1);
        assert_eq!(stats.changes, 0);
        assert_eq!(rendered, original);
    }

    #[test]
    fn rejects_matching_dependency_without_rev() {
        let original =
            "[dependencies]\nshared = { git = \"https://github.com/stayhydated/shared\" }\n";

        let error =
            update_manifest_text(original, &new_sha()).expect_err("missing rev should fail");

        assert!(error.to_string().contains("has no rev"));
    }

    #[test]
    fn rejects_non_commit_rev() {
        let original = "[dependencies]\nshared = { git = \"https://github.com/stayhydated/shared\", rev = \"master\" }\n";

        let error = update_manifest_text(original, &new_sha()).expect_err("branch rev should fail");

        assert!(error.to_string().contains("invalid rev"));
    }

    #[test]
    fn planning_is_atomic_when_a_later_manifest_is_invalid() {
        let directory = tempfile::tempdir().expect("temporary directory should be created");
        let valid = directory.path().join("valid.toml");
        let invalid = directory.path().join("invalid.toml");
        let valid_text = format!(
            "[dependencies]\nshared = {{ git = \"https://github.com/stayhydated/shared\", rev = \"{OLD_SHA}\" }}\n"
        );
        fs::write(&valid, &valid_text).expect("valid fixture should be written");
        fs::write(
            &invalid,
            "[dependencies]\nshared = { git = \"https://github.com/stayhydated/shared\" }\n",
        )
        .expect("invalid fixture should be written");

        let error = plan_manifest_updates(&[valid.clone(), invalid], &new_sha())
            .expect_err("invalid later manifest should fail planning");

        assert!(error.to_string().contains("failed to update"));
        assert_eq!(
            fs::read_to_string(valid).expect("valid fixture should remain readable"),
            valid_text
        );
    }

    #[test]
    fn commit_type_rejects_non_full_sha() {
        assert!(CommitSha::parse("short").is_err());
    }
}
