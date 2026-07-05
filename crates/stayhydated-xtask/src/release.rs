use std::{
    collections::{HashMap, HashSet},
    io::{self, Write as _},
    path::Path,
    process::{Command, Output},
    thread,
    time::Duration,
};

use anyhow::{Context as _, bail};
use bon::Builder;
use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, Package, PackageId};

#[derive(Clone, Debug)]
struct ReleasePackage {
    name: PackageName,
    version: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PackageName(String);

impl PackageName {
    fn from_metadata(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    fn parse(value: impl Into<String>, label: &str) -> anyhow::Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            bail!("{label} package name cannot be empty");
        }

        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryName(String);

impl RegistryName {
    fn parse(value: impl Into<String>) -> anyhow::Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            bail!("registry name cannot be empty");
        }

        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseResumePoint(PackageName);

impl ReleaseResumePoint {
    fn parse(value: impl Into<String>) -> anyhow::Result<Self> {
        PackageName::parse(value, "release resume").map(Self)
    }

    fn package_name(&self) -> &PackageName {
        &self.0
    }
}

#[derive(Builder, Clone, Debug)]
pub struct PublishOptions {
    pub execute: bool,
    pub from: Option<ReleaseResumePoint>,
    pub registry: Option<RegistryName>,
    #[builder(default)]
    pub allow_dirty: bool,
    #[builder(default)]
    pub no_verify: bool,
    #[builder(default)]
    pub include_dev_deps: bool,
    #[builder(default)]
    pub skip_existing: bool,
    #[builder(default = 3)]
    pub retries: u32,
    #[builder(default = 20)]
    pub retry_delay_seconds: u64,
}

impl PublishOptions {
    pub fn new(execute: bool) -> Self {
        Self::builder().execute(execute).build()
    }

    pub fn resume_from(mut self, from: Option<String>) -> anyhow::Result<Self> {
        self.from = from.map(ReleaseResumePoint::parse).transpose()?;
        Ok(self)
    }

    pub fn registry(mut self, registry: Option<String>) -> anyhow::Result<Self> {
        self.registry = registry.map(RegistryName::parse).transpose()?;
        Ok(self)
    }

    pub fn allow_dirty(mut self, allow_dirty: bool) -> Self {
        self.allow_dirty = allow_dirty;
        self
    }

    pub fn no_verify(mut self, no_verify: bool) -> Self {
        self.no_verify = no_verify;
        self
    }

    pub fn include_dev_deps(mut self, include_dev_deps: bool) -> Self {
        self.include_dev_deps = include_dev_deps;
        self
    }

    pub fn skip_existing(mut self, skip_existing: bool) -> Self {
        self.skip_existing = skip_existing;
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    pub fn retry_delay_seconds(mut self, retry_delay_seconds: u64) -> Self {
        self.retry_delay_seconds = retry_delay_seconds;
        self
    }
}

pub fn plan(workspace_root: &Path) -> anyhow::Result<()> {
    let packages = release_order(workspace_root)?;
    print_order(&packages);
    Ok(())
}

pub fn publish(workspace_root: &Path, options: &PublishOptions) -> anyhow::Result<()> {
    let packages = release_order(workspace_root)?;
    let packages = packages_from(&packages, options.from.as_ref())?;

    print_order(packages);

    if !options.execute {
        println!();
        println!("No packages were uploaded. Add --execute to run:");
        for package in packages {
            println!("  {}", cargo_publish_command(package, options).display());
        }
        return Ok(());
    }

    if !options.include_dev_deps {
        ensure_cargo_hack()?;
    }

    for package in packages {
        publish_package(workspace_root, package, options)?;
    }

    Ok(())
}

fn release_order(workspace_root: &Path) -> anyhow::Result<Vec<ReleasePackage>> {
    let metadata = MetadataCommand::new()
        .manifest_path(workspace_root.join("Cargo.toml"))
        .exec()
        .context("failed to read cargo metadata")?;

    release_order_from_metadata(&metadata)
}

struct ReleaseDependencyGraph {
    package_by_id: HashMap<PackageId, ReleasePackage>,
    publishable_order: Vec<PackageId>,
    remaining_deps: HashMap<PackageId, HashSet<PackageId>>,
    dependents: HashMap<PackageId, Vec<PackageId>>,
    workspace_index: HashMap<PackageId, usize>,
}

impl ReleaseDependencyGraph {
    fn from_metadata(metadata: &Metadata) -> anyhow::Result<Self> {
        let metadata_package_by_id = metadata
            .packages
            .iter()
            .map(|package| (package.id.clone(), package))
            .collect::<HashMap<_, _>>();

        let publishable = metadata
            .workspace_members
            .iter()
            .filter_map(|id| metadata_package_by_id.get(id).copied())
            .filter(|package| is_publishable(package))
            .collect::<Vec<_>>();

        let publishable_ids = publishable
            .iter()
            .map(|package| package.id.clone())
            .collect::<HashSet<_>>();
        let package_name_to_id = publishable
            .iter()
            .map(|package| (package.name.to_string(), package.id.clone()))
            .collect::<HashMap<_, _>>();
        let workspace_index = publishable
            .iter()
            .enumerate()
            .map(|(index, package)| (package.id.clone(), index))
            .collect::<HashMap<_, _>>();

        let remaining_deps = publishable
            .iter()
            .map(|package| {
                let deps = package
                    .dependencies
                    .iter()
                    .filter(|dependency| !matches!(dependency.kind, DependencyKind::Development))
                    .filter_map(|dependency| package_name_to_id.get(&dependency.name.to_string()))
                    .filter(|dependency_id| publishable_ids.contains(*dependency_id))
                    .cloned()
                    .collect::<HashSet<_>>();
                (package.id.clone(), deps)
            })
            .collect::<HashMap<_, _>>();

        let mut dependents = HashMap::<PackageId, Vec<PackageId>>::new();
        for (package_id, deps) in &remaining_deps {
            for dep_id in deps {
                dependents
                    .entry(dep_id.clone())
                    .or_default()
                    .push(package_id.clone());
            }
        }

        let publishable_order = publishable
            .iter()
            .map(|package| package.id.clone())
            .collect::<Vec<_>>();
        let package_by_id = publishable
            .iter()
            .map(|package| {
                (
                    package.id.clone(),
                    ReleasePackage {
                        name: PackageName::from_metadata(package.name.to_string()),
                        version: package.version.to_string(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            package_by_id,
            publishable_order,
            remaining_deps,
            dependents,
            workspace_index,
        })
    }

    fn release_order(mut self) -> anyhow::Result<Vec<ReleasePackage>> {
        let mut ready = self
            .remaining_deps
            .iter()
            .filter_map(|(package_id, deps)| deps.is_empty().then_some(package_id.clone()))
            .collect::<Vec<_>>();
        sort_by_workspace_index(&mut ready, &self.workspace_index);

        let mut ordered_ids = HashSet::new();
        let mut ordered = Vec::new();
        while let Some(package_id) = ready.first().cloned() {
            ready.remove(0);

            let package = self
                .package_by_id
                .get(&package_id)
                .with_context(|| format!("metadata missing package {package_id}"))?
                .clone();
            ordered_ids.insert(package_id.clone());
            ordered.push(package);

            for dependent_id in self.dependents.get(&package_id).into_iter().flatten() {
                let deps = self.remaining_deps.get_mut(dependent_id).with_context(|| {
                    format!("metadata missing dependent package {dependent_id}")
                })?;
                deps.remove(&package_id);
                if deps.is_empty()
                    && !ordered_ids.contains(dependent_id)
                    && !ready.contains(dependent_id)
                {
                    ready.push(dependent_id.clone());
                }
            }
            sort_by_workspace_index(&mut ready, &self.workspace_index);
        }

        if ordered.len() != self.package_by_id.len() {
            bail!(
                "workspace publish dependencies contain a cycle: {}",
                self.blocked_packages(&ordered_ids).join("; ")
            );
        }

        Ok(ordered)
    }

    fn blocked_packages(&self, ordered_ids: &HashSet<PackageId>) -> Vec<String> {
        self.publishable_order
            .iter()
            .filter(|package_id| !ordered_ids.contains(*package_id))
            .filter_map(|package_id| {
                let package = self.package_by_id.get(package_id)?;
                let deps = self
                    .remaining_deps
                    .get(package_id)
                    .into_iter()
                    .flat_map(|deps| deps.iter())
                    .filter_map(|dep_id| self.package_by_id.get(dep_id))
                    .map(|dep| dep.name.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                Some(format!("{} waits on {}", package.name, deps))
            })
            .collect()
    }
}

fn release_order_from_metadata(metadata: &Metadata) -> anyhow::Result<Vec<ReleasePackage>> {
    ReleaseDependencyGraph::from_metadata(metadata)?.release_order()
}

fn packages_from<'a>(
    packages: &'a [ReleasePackage],
    from: Option<&ReleaseResumePoint>,
) -> anyhow::Result<&'a [ReleasePackage]> {
    let Some(from) = from else {
        return Ok(packages);
    };
    let from = from.package_name();

    let index = packages
        .iter()
        .position(|package| package.name == *from)
        .with_context(|| {
            let names = packages
                .iter()
                .map(|package| package.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            format!("unknown release package `{from}`; expected one of: {names}")
        })?;

    Ok(&packages[index..])
}

fn publish_package(
    workspace_root: &Path,
    package: &ReleasePackage,
    options: &PublishOptions,
) -> anyhow::Result<()> {
    let command = cargo_publish_command(package, options);
    for attempt in 0..=options.retries {
        if requires_clean_worktree_guard(options) {
            ensure_tracked_worktree_clean(workspace_root)?;
        }

        println!();
        println!("Running {}", command.display());

        let output = Command::new(command.program())
            .current_dir(workspace_root)
            .args(command.args())
            .output()
            .with_context(|| format!("failed to run {}", command.display()))?;

        print_output(&output)?;

        if output.status.success() {
            return Ok(());
        }

        if options.skip_existing && output_mentions_existing_upload(&output) {
            println!(
                "{} {} is already uploaded; continuing because --skip-existing was set",
                package.name, package.version
            );
            return Ok(());
        }

        if attempt == options.retries {
            bail!(
                "{} failed after {} attempt(s) with status {}",
                command.display(),
                attempt + 1,
                output.status
            );
        }

        println!(
            "Publish failed; retrying in {}s for crates.io index propagation",
            options.retry_delay_seconds
        );
        thread::sleep(Duration::from_secs(options.retry_delay_seconds));
    }

    Ok(())
}

fn ensure_cargo_hack() -> anyhow::Result<()> {
    let output = Command::new("cargo")
        .args(["hack", "--version"])
        .output()
        .context("failed to run `cargo hack --version`")?;

    if output.status.success() {
        return Ok(());
    }

    print_output(&output)?;
    bail!(
        "release publish requires cargo-hack; install it with `cargo install cargo-hack` or pass --include-dev-deps"
    );
}

fn ensure_tracked_worktree_clean(workspace_root: &Path) -> anyhow::Result<()> {
    let output = Command::new("git")
        .current_dir(workspace_root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .context("failed to inspect git working tree")?;

    if !output.status.success() {
        print_output(&output)?;
        bail!("failed to inspect git working tree before publishing");
    }

    if !output.stdout.is_empty() {
        let changes = String::from_utf8_lossy(&output.stdout);
        bail!(
            "release publish uses cargo-hack manifest rewrites and passes --allow-dirty through to cargo publish; commit or stash tracked changes first, or pass xtask's --allow-dirty to publish them anyway:\n{}",
            changes.trim_end()
        );
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CargoPublishCommand {
    program: String,
    args: Vec<String>,
}

impl CargoPublishCommand {
    fn new(package: &ReleasePackage, options: &PublishOptions) -> Self {
        let mut args = if options.include_dev_deps {
            vec![
                "publish".to_owned(),
                "-p".to_owned(),
                package.name.as_str().to_owned(),
            ]
        } else {
            vec![
                "hack".to_owned(),
                "--no-dev-deps".to_owned(),
                "publish".to_owned(),
                "-p".to_owned(),
                package.name.as_str().to_owned(),
            ]
        };

        if let Some(registry) = &options.registry {
            args.push("--registry".to_owned());
            args.push(registry.as_str().to_owned());
        }
        if cargo_publish_needs_allow_dirty(options) {
            args.push("--allow-dirty".to_owned());
        }
        if options.no_verify {
            args.push("--no-verify".to_owned());
        }

        Self {
            program: "cargo".to_owned(),
            args,
        }
    }

    fn program(&self) -> &str {
        &self.program
    }

    fn args(&self) -> &[String] {
        &self.args
    }

    fn argv(&self) -> Vec<String> {
        let mut argv = vec![self.program.clone()];
        argv.extend(self.args.clone());
        argv
    }

    fn display(&self) -> String {
        self.argv().join(" ")
    }
}

fn cargo_publish_command(
    package: &ReleasePackage,
    options: &PublishOptions,
) -> CargoPublishCommand {
    CargoPublishCommand::new(package, options)
}

fn cargo_publish_needs_allow_dirty(options: &PublishOptions) -> bool {
    options.allow_dirty || !options.include_dev_deps
}

fn requires_clean_worktree_guard(options: &PublishOptions) -> bool {
    !options.allow_dirty && !options.include_dev_deps
}

fn print_order(packages: &[ReleasePackage]) {
    println!("Release publish order:");
    for (index, package) in packages.iter().enumerate() {
        println!("{:>2}. {} {}", index + 1, package.name, package.version);
    }
    println!("Order is computed from non-dev workspace dependencies.");
}

fn print_output(output: &Output) -> anyhow::Result<()> {
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

fn output_mentions_existing_upload(output: &Output) -> bool {
    let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
    stderr.contains("already uploaded") || stderr.contains("already exists")
}

fn sort_by_workspace_index(
    package_ids: &mut [PackageId],
    workspace_index: &HashMap<PackageId, usize>,
) {
    package_ids.sort_by_key(|package_id| workspace_index.get(package_id).copied());
}

fn is_publishable(package: &Package) -> bool {
    package
        .publish
        .as_ref()
        .is_none_or(|registries| !registries.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(name: &str) -> ReleasePackage {
        ReleasePackage {
            name: PackageName::from_metadata(name),
            version: "0.1.0".to_owned(),
        }
    }

    fn options() -> PublishOptions {
        PublishOptions::builder().execute(false).build()
    }

    #[test]
    fn cargo_hack_publish_allows_its_temporary_manifest_edits() {
        let command = cargo_publish_command(&package("public-crate"), &options());

        assert_eq!(
            command.argv(),
            [
                "cargo",
                "hack",
                "--no-dev-deps",
                "publish",
                "-p",
                "public-crate",
                "--allow-dirty",
            ]
        );
    }

    #[test]
    fn cargo_hack_publish_guards_preexisting_dirty_changes_by_default() {
        assert!(requires_clean_worktree_guard(&options()));
    }

    #[test]
    fn explicit_dirty_publish_disables_the_clean_worktree_guard() {
        let mut options = options();
        options.allow_dirty = true;

        assert!(!requires_clean_worktree_guard(&options));
    }

    #[test]
    fn plain_cargo_publish_does_not_allow_dirty_by_default() {
        let mut options = options();
        options.include_dev_deps = true;

        let command = cargo_publish_command(&package("public-crate"), &options);

        assert_eq!(command.argv(), ["cargo", "publish", "-p", "public-crate"]);
    }

    #[test]
    fn registry_name_renders_into_publish_command() {
        let options = options()
            .registry(Some("private".to_owned()))
            .expect("registry should parse");
        let command = cargo_publish_command(&package("public-crate"), &options);

        assert_eq!(
            command.argv(),
            [
                "cargo",
                "hack",
                "--no-dev-deps",
                "publish",
                "-p",
                "public-crate",
                "--registry",
                "private",
                "--allow-dirty",
            ]
        );
    }

    #[test]
    fn empty_resume_package_name_is_precise_error() {
        let error = options()
            .resume_from(Some(" ".to_owned()))
            .expect_err("empty resume point should fail");

        assert_eq!(
            error.to_string(),
            "release resume package name cannot be empty"
        );
    }

    #[test]
    fn packages_from_reports_unknown_resume_point_with_expected_names() {
        let packages = [package("alpha"), package("beta")];
        let resume = ReleaseResumePoint::parse("missing").expect("resume point should parse");
        let error = packages_from(&packages, Some(&resume))
            .expect_err("unknown resume package should fail");

        assert_eq!(
            error.to_string(),
            "unknown release package `missing`; expected one of: alpha, beta"
        );
    }
}
