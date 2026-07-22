use stayhydated_xtask::llms::LlmsConfig;

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    let output_dir = workspace_root.join("dummy/web-dummy/public");
    let base_url = web_dummy::SITE_URL.trim_end_matches('/');

    stayhydated_xtask::llms::build(
        LlmsConfig::builder()
            .book_root(&workspace_root.join("dummy/book-dummy"))
            .llms_path(&output_dir.join("llms.txt"))
            .llms_full_path(&output_dir.join("llms-full.txt"))
            .llms_markdown_dir(&output_dir.join("llms"))
            .base_url(base_url)
            .markdown_dir_name("llms")
            .build(),
    )
}
