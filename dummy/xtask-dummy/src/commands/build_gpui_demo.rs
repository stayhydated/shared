use stayhydated_xtask::trunk::TrunkDemoBuildConfig;

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    stayhydated_xtask::trunk::build(
        &TrunkDemoBuildConfig::builder()
            .workspace_root(workspace_root)
            .example_dir("dummy/gpui-demo")
            .output_dir("dummy/web-dummy/public/gpui-demo")
            .example_name("gpui-demo")
            .required_marker("sum-numbers-ai-gpui-demo")
            .toolchain("nightly")
            .loader_destination("dummy/scripts/trunk-loader.js")
            .build(),
    )
}
