use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, bail};
use bon::Builder;

pub const STATIC_SITE_PREVIEW_JS: &str = include_str!("static_site_preview.js");

#[derive(Builder, Clone, Debug)]
pub struct StaticSitePreviewConfig {
    #[builder(into)]
    pub workspace_root: PathBuf,
    #[builder(into)]
    pub dist_dir: PathBuf,
    #[builder(into)]
    pub base_path: String,
    #[builder(default = String::from("Build the web site first."), into)]
    pub build_hint: String,
    #[builder(default = PathBuf::from("bun"))]
    pub program: PathBuf,
}

pub fn serve(config: &StaticSitePreviewConfig) -> anyhow::Result<()> {
    let dist_dir = resolve(&config.workspace_root, &config.dist_dir);
    if !dist_dir.is_dir() {
        bail!(
            "missing static site output at {}; {}",
            dist_dir.display(),
            config.build_hint
        );
    }

    let script = config
        .workspace_root
        .join("target/stayhydated/static-site-preview.js");
    write_script(&script)?;

    let status = Command::new(&config.program)
        .current_dir(&config.workspace_root)
        .arg("run")
        .arg(&script)
        .env("STAYHYDATED_PREVIEW_DIST", &dist_dir)
        .env(
            "STAYHYDATED_PREVIEW_BASE_PATH",
            normalize_base_path(&config.base_path),
        )
        .env("STAYHYDATED_PREVIEW_BUILD_HINT", &config.build_hint)
        .status()
        .context("failed to run the static site preview server")?;

    if !status.success() {
        bail!("static site preview server failed with status {status}");
    }
    Ok(())
}

fn resolve(workspace_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        workspace_root.join(path)
    }
}

fn normalize_base_path(value: &str) -> String {
    let trimmed = value.trim_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("/{trimmed}")
    }
}

fn write_script(destination: &Path) -> anyhow::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(destination, STATIC_SITE_PREVIEW_JS)
        .with_context(|| format!("failed to write {}", destination.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_paths_are_normalized_for_the_preview_server() {
        assert_eq!(normalize_base_path("project"), "/project");
        assert_eq!(normalize_base_path("/project/"), "/project");
        assert_eq!(normalize_base_path("/"), "");
    }

    #[test]
    fn preview_script_is_written_under_target() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        let script = temp
            .path()
            .join("target/stayhydated/static-site-preview.js");

        write_script(&script).expect("preview script should be written");

        assert_eq!(
            fs::read_to_string(script).expect("preview script should be readable"),
            STATIC_SITE_PREVIEW_JS
        );
    }
}
