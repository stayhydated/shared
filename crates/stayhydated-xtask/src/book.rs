use std::fs;
use std::path::Path;

use mdbook_driver::MDBook;

pub fn build(book_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    println!("Building mdBook to {}", output_dir.display());

    let mut book = MDBook::load(book_dir)?;
    book.config.build.build_dir = output_dir.to_path_buf();
    book.build()?;

    let gitignore_path = output_dir.join(".gitignore");
    fs::write(&gitignore_path, "*")?;

    println!("mdBook built successfully");
    Ok(())
}

pub fn build_workspace_book(workspace_root: &Path) -> anyhow::Result<()> {
    build(
        &workspace_root.join("book"),
        &workspace_root.join("web").join("public").join("book"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_book(book_dir: &Path) {
        fs::create_dir_all(book_dir.join("src")).expect("book source should be created");
        fs::write(
            book_dir.join("book.toml"),
            "[book]\ntitle = \"Test book\"\n",
        )
        .expect("book config should be written");
        fs::write(
            book_dir.join("src/SUMMARY.md"),
            "# Summary\n\n- [Introduction](introduction.md)\n",
        )
        .expect("summary should be written");
        fs::write(
            book_dir.join("src/introduction.md"),
            "# Introduction\n\nHello.\n",
        )
        .expect("chapter should be written");
    }

    #[test]
    fn builds_book_output_and_workspace_layout() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        write_book(&temp.path().join("book"));

        build_workspace_book(temp.path()).expect("workspace book should build");

        let output = temp.path().join("web/public/book");
        assert!(output.join("index.html").is_file());
        assert_eq!(
            fs::read_to_string(output.join(".gitignore"))
                .expect("generated gitignore should be readable"),
            "*"
        );
    }
}
