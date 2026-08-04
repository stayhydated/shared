use stayhydated_xtask::web::{
    CopyPath, DioxusBuildCommand, PackageName, WebBuildConfig, WriteFile,
};

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    let public_dir = workspace_root.join("dummy/web-dummy/public");
    let dist_dir = workspace_root.join("dummy/web-dummy/dist");

    stayhydated_xtask::web::build(
        WebBuildConfig::builder()
            .command_current_dir(workspace_root.clone())
            .dioxus_command(
                DioxusBuildCommand::web_static_site(Some(PackageName::new("web-dummy")))
                    .with_base_path("sum-numbers-ai"),
            )
            .dx_public_dir(workspace_root.join("target/dx/web-dummy/release/web/public"))
            .dist_dir(dist_dir.clone())
            .copy_dirs(vec![
                CopyPath {
                    source: public_dir.join("assets"),
                    destination: dist_dir.join("assets"),
                },
                CopyPath {
                    source: public_dir.join("book"),
                    destination: dist_dir.join("book"),
                },
                CopyPath {
                    source: public_dir.join("bevy-demo"),
                    destination: dist_dir.join("bevy-demo"),
                },
                CopyPath {
                    source: public_dir.join("gpui-demo"),
                    destination: dist_dir.join("gpui-demo"),
                },
                CopyPath {
                    source: public_dir.join("llms"),
                    destination: dist_dir.join("llms"),
                },
            ])
            .copy_files(vec![
                CopyPath {
                    source: public_dir.join("llms.txt"),
                    destination: dist_dir.join("llms.txt"),
                },
                CopyPath {
                    source: public_dir.join("llms-full.txt"),
                    destination: dist_dir.join("llms-full.txt"),
                },
            ])
            .write_files(vec![WriteFile {
                destination: dist_dir.join(".nojekyll"),
                contents: "",
            }])
            .route_manifest(web_dummy::route_manifest())
            .write_404_from_index(true)
            .build(),
    )
}
