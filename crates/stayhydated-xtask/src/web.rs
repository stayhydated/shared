use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context as _, bail};
use bon::Builder;
use strum::IntoStaticStr;
use walkdir::WalkDir;

pub const DX_COMPONENTS_THEME_FILE_NAME: &str = "dx-components-theme.css";
pub const DX_COMPONENTS_THEME_CSS: &str =
    include_str!("../../stayhydated-dioxus-core/src/dx-components-theme.css");

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
    package: Option<PackageName>,
    platform: DioxusPlatform,
    ssg: bool,
    release: bool,
    debug_symbols: bool,
    force_sequential: bool,
}

impl DioxusBuildCommand {
    pub fn web_static_site(package: Option<PackageName>) -> Self {
        Self {
            package,
            platform: DioxusPlatform::Web,
            ssg: true,
            release: true,
            debug_symbols: false,
            force_sequential: true,
        }
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

        args
    }

    fn display(&self) -> String {
        format!("dx {}", self.argv().join(" "))
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
    pub route_fallback_paths: Vec<String>,
    pub write_404_from_index: bool,
    pub sitemap_xml: Option<String>,
}

pub struct WebBuildConfigBuilder {
    workspace_root: PathBuf,
    command_current_dir: Option<PathBuf>,
    package: Option<PackageName>,
    extra_copy_dirs: Vec<CopyPath>,
    extra_copy_files: Vec<CopyPath>,
    public_assets_dir: Option<PathBuf>,
    route_fallback_paths: Vec<String>,
    sitemap_xml: Option<String>,
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
            route_fallback_paths: Vec::new(),
            sitemap_xml: None,
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

    pub fn sitemap_xml(mut self, sitemap_xml: impl Into<String>) -> Self {
        self.sitemap_xml = Some(sitemap_xml.into());
        self
    }

    pub fn route_fallback_paths<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        self.route_fallback_paths
            .extend(paths.into_iter().map(|path| path.as_ref().to_owned()));
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
                source: self.workspace_root.join("web/public/.nojekyll"),
                destination: dist_dir.join(".nojekyll"),
            },
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
            destination: dist_dir.join(DX_COMPONENTS_THEME_FILE_NAME),
            contents: DX_COMPONENTS_THEME_CSS,
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
            .route_fallback_paths(self.route_fallback_paths)
            .write_404_from_index(true)
            .maybe_sitemap_xml(self.sitemap_xml)
            .build()
    }

    fn dist_dir(&self) -> PathBuf {
        self.workspace_root.join("web/dist")
    }
}

pub fn build(config: WebBuildConfig) -> anyhow::Result<()> {
    if config.dx_public_dir.exists() {
        fs::remove_dir_all(&config.dx_public_dir).with_context(|| {
            format!(
                "failed to clear generated Dioxus output at {}",
                config.dx_public_dir.display()
            )
        })?;
    }

    let dioxus_args = config.dioxus_command.argv();
    let dioxus_command_display = config.dioxus_command.display();
    let status = Command::new("dx")
        .current_dir(&config.command_current_dir)
        .args(&dioxus_args)
        .status()
        .with_context(|| format!("failed to run `{dioxus_command_display}` for the docs site"))?;

    if !status.success() {
        bail!("`{dioxus_command_display}` failed with status {status}");
    }

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

    write_route_fallbacks(&config.dist_dir, &config.route_fallback_paths)?;

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

    if let Some(sitemap_xml) = config.sitemap_xml {
        fs::write(config.dist_dir.join("sitemap.xml"), sitemap_xml).with_context(|| {
            format!(
                "failed to write {}",
                config.dist_dir.join("sitemap.xml").display()
            )
        })?;
    }

    Ok(())
}

fn write_route_fallbacks(dist_dir: &std::path::Path, paths: &[String]) -> anyhow::Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    let root_index = dist_dir.join("index.html");
    if !root_index.is_file() {
        bail!("expected root route fallback at {}", root_index.display());
    }

    for path in paths {
        let Some(relative_index) = route_fallback_path(path) else {
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
    fn github_pages_config_writes_shared_dx_components_theme() {
        let config = WebBuildConfig::github_pages("/workspace")
            .package("web")
            .build();
        let theme = config
            .write_files
            .iter()
            .find(|file| file.destination.ends_with(DX_COMPONENTS_THEME_FILE_NAME))
            .expect("shared theme should be written");

        assert_eq!(
            theme.destination,
            PathBuf::from("/workspace/web/dist").join(DX_COMPONENTS_THEME_FILE_NAME)
        );
        assert!(theme.contents.contains("--primary-color"));
    }

    #[test]
    fn github_pages_config_tracks_route_fallback_paths() {
        let config = WebBuildConfig::github_pages("/workspace")
            .route_fallback_paths(["/", "/demos/", "/zh/demos/"])
            .build();

        assert_eq!(config.route_fallback_paths, ["/", "/demos/", "/zh/demos/"]);
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
    }
}
