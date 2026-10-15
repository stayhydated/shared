fn main() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    stayhydated_xtask::book::build(
        &workspace_root.join("book"),
        &workspace_root.join("target/shared-user-book"),
    )
}
