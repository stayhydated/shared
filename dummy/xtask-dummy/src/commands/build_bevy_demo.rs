use stayhydated_xtask::trunk::TrunkDemoBuildConfig;

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    stayhydated_xtask::trunk::build(
        &TrunkDemoBuildConfig::builder()
            .workspace_root(workspace_root)
            .example_dir("dummy/bevy-demo")
            .output_dir("dummy/web-dummy/public/bevy-demo")
            .example_name("bevy-demo")
            .required_marker("sum-numbers-ai-bevy-demo")
            .loader_destination("dummy/scripts/trunk-loader.js")
            .build(),
    )
}
