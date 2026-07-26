use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, bail};
use bon::Builder;

pub const WASM_DEMO_LOADER_JS: &str = include_str!("trunk_loader.js");

#[derive(Builder, Clone, Debug)]
pub struct TrunkDemoBuildConfig {
    #[builder(into)]
    pub workspace_root: PathBuf,
    #[builder(into)]
    pub example_dir: PathBuf,
    #[builder(into)]
    pub output_dir: PathBuf,
    #[builder(into)]
    pub example_name: String,
    #[builder(into)]
    pub required_marker: String,
    #[builder(default = PathBuf::from("trunk"))]
    pub program: PathBuf,
    #[builder(with = |value: impl Into<String>| value.into())]
    pub toolchain: Option<String>,
    #[builder(default = true)]
    pub require_javascript: bool,
    #[builder(with = |value: impl Into<PathBuf>| value.into())]
    pub loader_destination: Option<PathBuf>,
}

impl TrunkDemoBuildConfig {
    fn resolve(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_owned()
        } else {
            self.workspace_root.join(path)
        }
    }

    fn example_dir(&self) -> PathBuf {
        self.resolve(&self.example_dir)
    }

    fn output_dir(&self) -> PathBuf {
        self.resolve(&self.output_dir)
    }
}

pub fn build(config: &TrunkDemoBuildConfig) -> anyhow::Result<()> {
    validate_config(config)?;

    if let Some(destination) = &config.loader_destination {
        write_loader(&config.resolve(destination))?;
    }

    let example_dir = config.example_dir();
    let output_dir = config.output_dir();
    let index = example_dir.join("index.html");
    if !index.is_file() {
        bail!("Trunk demo build requires {}", index.display());
    }

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)
            .with_context(|| format!("failed to clean {}", output_dir.display()))?;
    }
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create {}", output_dir.display()))?;

    let mut command = Command::new(&config.program);
    command
        .current_dir(&example_dir)
        .env_remove("NO_COLOR")
        .arg("build")
        .arg("index.html")
        .arg("--example")
        .arg(&config.example_name)
        .args([
            "--release",
            "--no-default-features",
            "--no-sri",
            "--public-url",
            "./",
        ])
        .arg("--dist")
        .arg(&output_dir);
    if let Some(toolchain) = &config.toolchain {
        command.env("RUSTUP_TOOLCHAIN", toolchain);
    }

    let status = command
        .status()
        .with_context(|| format!("failed to run Trunk for example `{}`", config.example_name))?;
    if !status.success() {
        let toolchain_hint = config
            .toolchain
            .as_deref()
            .map(|toolchain| format!(" with Rust toolchain `{toolchain}`"))
            .unwrap_or_default();
        bail!(
            "Trunk failed for example `{}`{toolchain_hint} with status {status}",
            config.example_name
        );
    }

    verify_output(config, &output_dir)?;
    fs::write(output_dir.join(".gitignore"), "*\n")
        .with_context(|| format!("failed to write {}/.gitignore", output_dir.display()))
}

fn validate_config(config: &TrunkDemoBuildConfig) -> anyhow::Result<()> {
    if config.example_name.trim().is_empty() {
        bail!("Trunk example name cannot be empty");
    }
    if config.required_marker.is_empty() {
        bail!("Trunk wasm marker cannot be empty");
    }
    Ok(())
}

fn write_loader(destination: &Path) -> anyhow::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(destination, WASM_DEMO_LOADER_JS)
        .with_context(|| format!("failed to write {}", destination.display()))
}

fn verify_output(config: &TrunkDemoBuildConfig, output_dir: &Path) -> anyhow::Result<()> {
    let index = output_dir.join("index.html");
    if !index.is_file() {
        bail!("missing Trunk demo HTML output at {}", index.display());
    }

    let mut has_javascript = false;
    let mut has_wasm = false;
    let mut has_marker = false;

    for entry in fs::read_dir(output_dir)
        .with_context(|| format!("failed to read {}", output_dir.display()))?
    {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }

        match path.extension().and_then(|extension| extension.to_str()) {
            Some("js") => has_javascript = true,
            Some("wasm") => {
                has_wasm = true;
                let bytes = fs::read(&path)
                    .with_context(|| format!("failed to read {}", path.display()))?;
                has_marker |= bytes
                    .windows(config.required_marker.len())
                    .any(|window| window == config.required_marker.as_bytes());
            },
            _ => {},
        }
    }

    if config.require_javascript && !has_javascript {
        bail!(
            "missing Trunk demo JavaScript output in {}",
            output_dir.display()
        );
    }
    if !has_wasm {
        bail!("missing Trunk demo wasm output in {}", output_dir.display());
    }
    if !has_marker {
        bail!(
            "Trunk demo wasm output in {} is missing marker `{}`",
            output_dir.display(),
            config.required_marker
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_reads_configuration_from_persistent_markup() {
        assert!(
            WASM_DEMO_LOADER_JS
                .contains("const RUNTIME_CONFIG_SELECTOR = \"[data-wasm-demo-config]\";")
        );
        assert!(!WASM_DEMO_LOADER_JS.contains("link[data-trunk][data-initializer]"));
        assert!(!WASM_DEMO_LOADER_JS.contains("wasmDemoDescription"));
        assert!(!WASM_DEMO_LOADER_JS.contains("wasm-loader-copy"));
        assert!(!WASM_DEMO_LOADER_JS.contains("wasmDemoAccent"));
        assert!(!WASM_DEMO_LOADER_JS.contains("--wasm-loader-error"));
        assert!(WASM_DEMO_LOADER_JS.contains("--wasm-loader-accent: rgb(255 255 255)"));
    }

    fn write(path: &Path, contents: impl AsRef<[u8]>) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directory should be created");
        }
        fs::write(path, contents).expect("fixture should be written");
    }

    #[cfg(unix)]
    fn write_executable(path: &Path, contents: &str) {
        use std::os::unix::fs::PermissionsExt as _;

        write(path, contents);
        let mut permissions = fs::metadata(path)
            .expect("script metadata should be readable")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("script should be executable");
    }

    #[cfg(unix)]
    #[test]
    fn build_runs_trunk_and_verifies_generated_assets() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        write(
            &temp.path().join("examples/demo/index.html"),
            "<html></html>",
        );
        let fake_trunk = temp.path().join("fake-trunk");
        write_executable(
            &fake_trunk,
            "#!/bin/sh\nset -eu\ndist=''\nwhile [ \"$#\" -gt 0 ]; do\n  if [ \"$1\" = '--dist' ]; then shift; dist=\"$1\"; fi\n  shift\ndone\nmkdir -p \"$dist\"\nprintf '<html>built</html>' > \"$dist/index.html\"\nprintf 'app' > \"$dist/demo.js\"\nprintf 'prefix-demo-marker-suffix' > \"$dist/demo.wasm\"\n",
        );
        let config = TrunkDemoBuildConfig::builder()
            .workspace_root(temp.path())
            .example_dir("examples/demo")
            .output_dir("web/public/demo")
            .example_name("demo")
            .required_marker("demo-marker")
            .program(fake_trunk)
            .toolchain("nightly")
            .loader_destination("examples/scripts/trunk-loader.js")
            .build();

        build(&config).expect("Trunk demo should build");

        assert!(temp.path().join("web/public/demo/.gitignore").is_file());
        assert_eq!(
            fs::read_to_string(temp.path().join("examples/scripts/trunk-loader.js"))
                .expect("loader should be readable"),
            WASM_DEMO_LOADER_JS
        );
    }

    #[test]
    fn verifier_reports_missing_marker() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        let output = temp.path().join("output");
        write(&output.join("index.html"), "<html></html>");
        write(&output.join("demo.js"), "app");
        write(&output.join("demo.wasm"), "other");
        let config = TrunkDemoBuildConfig::builder()
            .workspace_root(temp.path())
            .example_dir("example")
            .output_dir("output")
            .example_name("demo")
            .required_marker("required")
            .build();

        let error = verify_output(&config, &output).expect_err("marker should be required");

        assert!(error.to_string().contains("missing marker `required`"));
    }
}
