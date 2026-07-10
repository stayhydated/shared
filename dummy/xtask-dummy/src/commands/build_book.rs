pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;

    stayhydated_xtask::book::build(
        &workspace_root.join("dummy/book-dummy"),
        &workspace_root
            .join("dummy/web-dummy")
            .join("public")
            .join("book"),
    )
}
