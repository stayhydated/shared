use stayhydated_xtask::trunk::{TrunkDemoBuildConfig, TrunkDemoPageConfig};

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    stayhydated_xtask::trunk::build(
        &TrunkDemoBuildConfig::builder()
            .workspace_root(workspace_root)
            .example_dir("dummy/bevy-demo")
            .output_dir("dummy/web-dummy/public/bevy-demo")
            .example_name("bevy-demo")
            .required_marker("sum-numbers-ai-bevy-demo")
            .generated_page(
                TrunkDemoPageConfig::builder()
                    .title("sum-numbers-ai Bevy UI demo")
                    .demo_name("Bevy UI")
                    .canvas_id("bevy-demo")
                    .build(),
            )
            .build(),
    )
}
