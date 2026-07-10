use stayhydated_xtask::llms::LlmsConfig;

const BASE_URL: &str = "https://stayhydated.github.io/sum-numbers-ai";

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    let output_dir = workspace_root.join("dummy/web-dummy/public");

    stayhydated_xtask::llms::build(
        LlmsConfig::builder()
            .book_root(&workspace_root.join("dummy/book-dummy"))
            .llms_path(&output_dir.join("llms.txt"))
            .llms_full_path(&output_dir.join("llms-full.txt"))
            .llms_markdown_dir(&output_dir.join("llms"))
            .base_url(BASE_URL)
            .markdown_dir_name("llms")
            .build(),
    )
}
