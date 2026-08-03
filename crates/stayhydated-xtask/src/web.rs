use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context as _, bail};
use bon::Builder;
use stayhydated_site::SiteRouteManifest;
use strum::IntoStaticStr;
use walkdir::WalkDir;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageName(String);

impl PackageName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

pub struct CopyPath {
    pub source: PathBuf,
    pub destination: PathBuf,
}

pub struct WriteFile {
    pub destination: PathBuf,
    pub contents: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DioxusBuildCommand {
    program: PathBuf,
    package: Option<PackageName>,
    base_path: Option<String>,
    platform: DioxusPlatform,
    ssg: bool,
    release: bool,
    debug_symbols: bool,
    force_sequential: bool,
}

impl DioxusBuildCommand {
    pub fn web_static_site(package: Option<PackageName>) -> Self {
        Self {
            program: PathBuf::from("dx"),
            package,
            base_path: None,
            platform: DioxusPlatform::Web,
            ssg: true,
            release: true,
            debug_symbols: false,
            force_sequential: true,
        }
    }

    pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
        self.base_path = Some(base_path.into());
        self
    }

    pub fn argv(&self) -> Vec<String> {
        let mut args = vec!["build".to_owned()];
        if let Some(package) = &self.package {
            args.push("--package".to_owned());
            args.push(package.0.clone());
        }
        args.extend([
            "--platform".to_owned(),
            self.platform.as_arg().to_owned(),
            "--ssg".to_owned(),
            "--release".to_owned(),
            "--debug-symbols".to_owned(),
            self.debug_symbols.to_string(),
            "--force-sequential".to_owned(),
            self.force_sequential.to_string(),
        ]);
        if let Some(base_path) = &self.base_path {
            args.push("--base-path".to_owned());
            args.push(base_path.clone());
        }

        args
    }

    fn display(&self) -> String {
        format!("{} {}", self.program.display(), self.argv().join(" "))
    }
}

#[derive(Clone, Copy, Debug, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
enum DioxusPlatform {
    Web,
}

impl DioxusPlatform {
    fn as_arg(self) -> &'static str {
        self.into_str()
    }
}

#[derive(Builder)]
#[builder(
    builder_type = WebBuildConfigBonBuilder,
    on(PathBuf, into),
    on(String, into)
)]
pub struct WebBuildConfig {
    pub command_current_dir: PathBuf,
    pub dioxus_command: DioxusBuildCommand,
    pub dx_public_dir: PathBuf,
    pub dist_dir: PathBuf,
    pub copy_dirs: Vec<CopyPath>,
    pub copy_files: Vec<CopyPath>,
    pub write_files: Vec<WriteFile>,
    pub route_manifest: Option<SiteRouteManifest>,
    pub write_404_from_index: bool,
}

pub struct WebBuildConfigBuilder {
    workspace_root: PathBuf,
    command_current_dir: Option<PathBuf>,
    package: Option<PackageName>,
    extra_copy_dirs: Vec<CopyPath>,
    extra_copy_files: Vec<CopyPath>,
    public_assets_dir: Option<PathBuf>,
    route_manifest: Option<SiteRouteManifest>,
}

impl WebBuildConfig {
    pub fn github_pages(workspace_root: impl Into<PathBuf>) -> WebBuildConfigBuilder {
        WebBuildConfigBuilder {
            workspace_root: workspace_root.into(),
            command_current_dir: None,
            package: None,
            extra_copy_dirs: Vec::new(),
            extra_copy_files: Vec::new(),
            public_assets_dir: Some(PathBuf::from("web/public/assets")),
            route_manifest: None,
        }
    }
}

impl WebBuildConfigBuilder {
    pub fn command_current_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.command_current_dir = Some(path.into());
        self
    }

    pub fn package(mut self, package: impl Into<String>) -> Self {
        self.package = Some(PackageName::new(package));
        self
    }

    pub fn no_public_assets_dir(mut self) -> Self {
        self.public_assets_dir = None;
        self
    }

    pub fn public_assets_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.public_assets_dir = Some(path.into());
        self
    }

    pub fn extra_dir(
        mut self,
        source: impl Into<PathBuf>,
        destination: impl Into<PathBuf>,
    ) -> Self {
        self.extra_copy_dirs.push(CopyPath {
            source: self.workspace_root.join(source.into()),
            destination: self.dist_dir().join(destination.into()),
        });
        self
    }

    pub fn extra_file(
        mut self,
        source: impl Into<PathBuf>,
        destination: impl Into<PathBuf>,
    ) -> Self {
        self.extra_copy_files.push(CopyPath {
            source: self.workspace_root.join(source.into()),
            destination: self.dist_dir().join(destination.into()),
        });
        self
    }

    pub fn route_manifest(mut self, route_manifest: SiteRouteManifest) -> Self {
        self.route_manifest = Some(route_manifest);
        self
    }

    pub fn build(self) -> WebBuildConfig {
        let dist_dir = self.dist_dir();
        let dioxus_command = DioxusBuildCommand::web_static_site(self.package);

        let mut copy_dirs = Vec::new();
        if let Some(public_assets_dir) = self.public_assets_dir {
            copy_dirs.push(CopyPath {
                source: self.workspace_root.join(public_assets_dir),
                destination: dist_dir.join("assets"),
            });
        }
        copy_dirs.extend([
            CopyPath {
                source: self.workspace_root.join("web/public/book"),
                destination: dist_dir.join("book"),
            },
            CopyPath {
                source: self.workspace_root.join("web/public/llms"),
                destination: dist_dir.join("llms"),
            },
        ]);
        copy_dirs.extend(self.extra_copy_dirs);

        let mut copy_files = vec![
            CopyPath {
                source: self.workspace_root.join("web/public/llms.txt"),
                destination: dist_dir.join("llms.txt"),
            },
            CopyPath {
                source: self.workspace_root.join("web/public/llms-full.txt"),
                destination: dist_dir.join("llms-full.txt"),
            },
        ];
        copy_files.extend(self.extra_copy_files);

        let write_files = vec![WriteFile {
            destination: dist_dir.join(".nojekyll"),
            contents: "",
        }];

        WebBuildConfig::builder()
            .command_current_dir(
                self.command_current_dir
                    .unwrap_or_else(|| self.workspace_root.clone()),
            )
            .dioxus_command(dioxus_command)
            .dx_public_dir(self.workspace_root.join("target/dx/web/release/web/public"))
            .dist_dir(dist_dir)
            .copy_dirs(copy_dirs)
            .copy_files(copy_files)
            .write_files(write_files)
            .maybe_route_manifest(self.route_manifest)
            .write_404_from_index(true)
            .build()
    }

    fn dist_dir(&self) -> PathBuf {
        self.workspace_root.join("web/dist")
    }
}

pub fn build(config: WebBuildConfig) -> anyhow::Result<()> {
    build_with(config, run_dioxus_build)
}

fn build_with(
    config: WebBuildConfig,
    run_dioxus: impl FnOnce(&WebBuildConfig) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if config.dx_public_dir.exists() {
        fs::remove_dir_all(&config.dx_public_dir).with_context(|| {
            format!(
                "failed to clear generated Dioxus output at {}",
                config.dx_public_dir.display()
            )
        })?;
    }

    run_dioxus(&config)?;

    if !config.dx_public_dir.is_dir() {
        bail!(
            "expected Dioxus static output at {}",
            config.dx_public_dir.display()
        );
    }

    if config.dist_dir.exists() {
        fs::remove_dir_all(&config.dist_dir)
            .with_context(|| format!("failed to remove {}", config.dist_dir.display()))?;
    }
    fs::create_dir_all(&config.dist_dir)
        .with_context(|| format!("failed to create {}", config.dist_dir.display()))?;

    copy_directory(&config.dx_public_dir, &config.dist_dir)?;

    for copy in &config.copy_dirs {
        copy_directory(&copy.source, &copy.destination)?;
    }
    for copy in &config.copy_files {
        copy_file_if_present(&copy.source, &copy.destination)?;
    }
    for write in &config.write_files {
        write_file(&write.destination, write.contents)?;
    }

    if let Some(route_manifest) = &config.route_manifest {
        write_route_fallbacks(&config.dist_dir, route_manifest.application_paths())?;
    }

    if config.write_404_from_index {
        fs::copy(
            config.dist_dir.join("index.html"),
            config.dist_dir.join("404.html"),
        )
        .with_context(|| {
            format!(
                "failed to write {}",
                config.dist_dir.join("404.html").display()
            )
        })?;
    }

    if let Some(route_manifest) = &config.route_manifest {
        fs::write(
            config.dist_dir.join("sitemap.xml"),
            route_manifest.sitemap_xml(),
        )
        .with_context(|| {
            format!(
                "failed to write {}",
                config.dist_dir.join("sitemap.xml").display()
            )
        })?;
    }

    Ok(())
}

fn run_dioxus_build(config: &WebBuildConfig) -> anyhow::Result<()> {
    let dioxus_args = config.dioxus_command.argv();
    let dioxus_command_display = config.dioxus_command.display();
    let status = Command::new(&config.dioxus_command.program)
        .current_dir(&config.command_current_dir)
        .args(&dioxus_args)
        .status()
        .with_context(|| format!("failed to run `{dioxus_command_display}` for the docs site"))?;

    if !status.success() {
        bail!("`{dioxus_command_display}` failed with status {status}");
    }

    Ok(())
}

fn write_route_fallbacks<P>(dist_dir: &std::path::Path, paths: &[P]) -> anyhow::Result<()>
where
    P: AsRef<str>,
{
    if paths.is_empty() {
        return Ok(());
    }

    let root_index = dist_dir.join("index.html");
    if !root_index.is_file() {
        bail!("expected root route fallback at {}", root_index.display());
    }

    for path in paths {
        let Some(relative_index) = route_fallback_path(path.as_ref()) else {
            continue;
        };
        let destination = dist_dir.join(relative_index);
        if destination == root_index {
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::copy(&root_index, &destination).with_context(|| {
            format!(
                "failed to write route fallback {} from {}",
                destination.display(),
                root_index.display()
            )
        })?;
    }

    Ok(())
}

fn route_fallback_path(path: &str) -> Option<PathBuf> {
    let path = path.trim().trim_matches('/');
    if path.is_empty() {
        return None;
    }

    let mut relative = PathBuf::new();
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.contains('\\') {
            return None;
        }
        relative.push(segment);
    }
    relative.push("index.html");
    Some(relative)
}

fn write_file(destination: &std::path::Path, contents: &str) -> anyhow::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    fs::write(destination, contents)
        .with_context(|| format!("failed to write {}", destination.display()))?;

    Ok(())
}

fn copy_file_if_present(
    source: &std::path::Path,
    destination: &std::path::Path,
) -> anyhow::Result<()> {
    if !source.is_file() {
        return Ok(());
    }

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    fs::copy(source, destination).with_context(|| {
        format!(
            "failed to copy {} to {}",
            source.display(),
            destination.display()
        )
    })?;

    Ok(())
}

fn copy_directory(source: &std::path::Path, destination: &std::path::Path) -> anyhow::Result<()> {
    if !source.exists() {
        return Ok(());
    }

    for entry in WalkDir::new(source) {
        let entry = entry.with_context(|| format!("failed to walk {}", source.display()))?;
        let relative = entry
            .path()
            .strip_prefix(source)
            .with_context(|| format!("failed to strip prefix {}", source.display()))?;

        if relative.as_os_str().is_empty() {
            continue;
        }

        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("failed to create {}", target.display()))?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            fs::copy(entry.path(), &target).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    entry.path().display(),
                    target.display()
                )
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &std::path::Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directory should be created");
        }
        fs::write(path, contents).expect("fixture should be written");
    }

    #[test]
    fn dioxus_web_static_site_command_renders_expected_argv() {
        let command = DioxusBuildCommand::web_static_site(Some(PackageName::new("web")));

        assert_eq!(
            command.argv(),
            [
                "build",
                "--package",
                "web",
                "--platform",
                "web",
                "--ssg",
                "--release",
                "--debug-symbols",
                "false",
                "--force-sequential",
                "true",
            ]
        );
    }

    #[test]
    fn dioxus_web_static_site_command_accepts_base_path() {
        let command = DioxusBuildCommand::web_static_site(Some(PackageName::new("web")))
            .with_base_path("project");

        assert_eq!(
            command.argv(),
            [
                "build",
                "--package",
                "web",
                "--platform",
                "web",
                "--ssg",
                "--release",
                "--debug-symbols",
                "false",
                "--force-sequential",
                "true",
                "--base-path",
                "project",
            ]
        );
    }

    #[test]
    fn github_pages_config_uses_typed_dioxus_command() {
        let config = WebBuildConfig::github_pages("/workspace")
            .package("web")
            .build();

        assert_eq!(
            config.dioxus_command.argv(),
            DioxusBuildCommand::web_static_site(Some(PackageName::new("web"))).argv()
        );
    }

    #[test]
    fn github_pages_config_tracks_the_route_manifest() {
        let manifest = SiteRouteManifest::project(
            stayhydated_site::routing::SiteUrl::new("https://example.test/project"),
            ["/", "/demos/", "/zh/demos/"],
        );
        let config = WebBuildConfig::github_pages("/workspace")
            .route_manifest(manifest.clone())
            .build();

        assert_eq!(config.route_manifest, Some(manifest));
    }

    #[test]
    fn route_fallback_path_uses_route_index_files() {
        assert_eq!(route_fallback_path("/"), None);
        assert_eq!(
            route_fallback_path("/zh/demos/").as_deref(),
            Some(std::path::Path::new("zh/demos/index.html"))
        );
    }

    #[test]
    fn route_fallback_path_rejects_escaping_segments() {
        assert!(route_fallback_path("../secret").is_none());
        assert!(route_fallback_path("/zh/../secret/").is_none());
        assert!(route_fallback_path("/zh//secret/").is_none());
        assert!(route_fallback_path(r"/zh\secret/").is_none());
    }

    #[test]
    fn github_pages_builder_tracks_custom_inputs() {
        let manifest = SiteRouteManifest::new(
            stayhydated_site::routing::SiteUrl::new("https://example.test/project"),
            ["/"],
        );
        let config = WebBuildConfig::github_pages("/workspace")
            .command_current_dir("/workspace/web")
            .no_public_assets_dir()
            .public_assets_dir("public")
            .extra_dir("generated/book", "custom-book")
            .extra_file("generated/index.txt", "index.txt")
            .route_manifest(manifest.clone())
            .build();

        assert_eq!(config.command_current_dir, PathBuf::from("/workspace/web"));
        assert!(config.copy_dirs.iter().any(|copy| {
            copy.source == std::path::Path::new("/workspace/generated/book")
                && copy.destination == std::path::Path::new("/workspace/web/dist/custom-book")
        }));
        assert!(config.copy_files.iter().any(|copy| {
            copy.source == std::path::Path::new("/workspace/generated/index.txt")
                && copy.destination == std::path::Path::new("/workspace/web/dist/index.txt")
        }));
        assert_eq!(config.route_manifest, Some(manifest));
    }

    #[test]
    fn filesystem_helpers_copy_nested_outputs_and_fallbacks() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        let source = temp.path().join("source");
        let destination = temp.path().join("destination");
        write(&source.join("nested/file.txt"), "nested");

        copy_directory(&source, &destination).expect("directory should copy");
        assert_eq!(
            fs::read_to_string(destination.join("nested/file.txt"))
                .expect("copied file should be readable"),
            "nested"
        );
        copy_directory(&temp.path().join("missing"), &destination)
            .expect("missing directory should be ignored");

        let copied_file = temp.path().join("copied/deep/file.txt");
        copy_file_if_present(&source.join("nested/file.txt"), &copied_file)
            .expect("file should copy");
        assert_eq!(
            fs::read_to_string(&copied_file).expect("copied file should be readable"),
            "nested"
        );
        copy_file_if_present(&source.join("missing.txt"), &copied_file)
            .expect("missing file should be ignored");

        let generated = temp.path().join("generated/deep/file.txt");
        write_file(&generated, "generated").expect("file should be generated");
        assert_eq!(
            fs::read_to_string(&generated).expect("generated file should be readable"),
            "generated"
        );

        write(&destination.join("index.html"), "<main>Home</main>");
        write_route_fallbacks(
            &destination,
            &["/".to_owned(), "/demos/".to_owned(), "../escape".to_owned()],
        )
        .expect("route fallbacks should be generated");
        assert_eq!(
            fs::read_to_string(destination.join("demos/index.html"))
                .expect("fallback should be readable"),
            "<main>Home</main>"
        );
        write_route_fallbacks(&destination, &[] as &[String])
            .expect("empty fallback list should be a no-op");

        let missing_root = temp.path().join("missing-root");
        let error = write_route_fallbacks(&missing_root, &["/demos/".to_owned()])
            .expect_err("fallbacks require a root index");
        assert!(error.to_string().contains("expected root route fallback"));
    }

    #[test]
    fn build_with_runner_assembles_static_site() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        let public = temp.path().join("web/public");
        write(&public.join("assets/site.css"), "body {}");
        write(&public.join("book/index.html"), "Book");
        write(&public.join("llms/intro.md"), "Intro");
        write(&public.join("llms.txt"), "Index");
        write(&public.join("llms-full.txt"), "Full");

        let manifest = SiteRouteManifest::project(
            stayhydated_site::routing::SiteUrl::new("https://example.test/project"),
            ["/", "/demos/"],
        );
        let expected_sitemap = manifest.sitemap_xml();
        let config = WebBuildConfig::github_pages(temp.path())
            .route_manifest(manifest)
            .build();
        write(&config.dx_public_dir.join("stale.txt"), "stale");
        write(&config.dist_dir.join("stale.txt"), "stale");

        build_with(config, |config| {
            assert!(!config.dx_public_dir.exists());
            write(
                &config.dx_public_dir.join("index.html"),
                "<main>Home</main>",
            );
            write(&config.dx_public_dir.join("assets/app.js"), "app");
            Ok(())
        })
        .expect("static site should assemble");

        let dist = temp.path().join("web/dist");
        assert!(dist.join("index.html").is_file());
        assert!(dist.join(".nojekyll").is_file());
        assert!(dist.join("404.html").is_file());
        assert!(dist.join("demos/index.html").is_file());
        assert!(dist.join("assets/site.css").is_file());
        assert!(dist.join("book/index.html").is_file());
        assert!(dist.join("llms/intro.md").is_file());
        assert_eq!(
            fs::read_to_string(dist.join("sitemap.xml")).expect("sitemap should be readable"),
            expected_sitemap
        );
        assert!(!dist.join("stale.txt").exists());
    }

    #[test]
    fn build_with_runner_requires_generated_output() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        let config = WebBuildConfig::github_pages(temp.path()).build();

        let error = build_with(config, |_| Ok(()))
            .expect_err("runner must create the expected Dioxus output");

        assert!(error.to_string().contains("expected Dioxus static output"));
    }
}
