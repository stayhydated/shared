use std::path::{Path, PathBuf};

use anyhow::Context as _;
use cargo_metadata::MetadataCommand;

pub mod book;
pub mod llms;
pub mod release;
pub mod web;

pub fn workspace_root_from_xtask_manifest() -> anyhow::Result<PathBuf> {
    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .context("failed to read cargo metadata from the current workspace")?;

    Ok(metadata.workspace_root.into_std_path_buf())
}

pub fn workspace_root_from_xtask_manifest_dir(manifest_dir: &Path) -> anyhow::Result<PathBuf> {
    manifest_dir
        .parent()
        .map(Path::to_path_buf)
        .context("failed to resolve workspace root from xtask manifest directory")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_workspace_roots_from_metadata_and_manifest_directory() {
        let metadata_root = workspace_root_from_xtask_manifest()
            .expect("current workspace metadata should resolve");
        let manifest_root = workspace_root_from_xtask_manifest_dir(&metadata_root.join("xtask"))
            .expect("xtask manifest parent should resolve");

        assert_eq!(metadata_root, manifest_root);
    }

    #[test]
    fn rejects_manifest_directory_without_parent() {
        let error = workspace_root_from_xtask_manifest_dir(Path::new("/"))
            .expect_err("filesystem root should not have a parent");

        assert_eq!(
            error.to_string(),
            "failed to resolve workspace root from xtask manifest directory"
        );
    }
}
