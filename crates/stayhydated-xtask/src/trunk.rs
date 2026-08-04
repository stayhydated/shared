use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, bail};
use bon::Builder;
use path_slash::PathExt as _;

/// File name used for the loader beside a generated Trunk index.
pub const TRUNK_DEMO_LOADER_FILE_NAME: &str = "trunk-loader.js";
/// Shared JavaScript initializer for Trunk-built WebAssembly demos.
pub const WASM_DEMO_LOADER_JS: &str = include_str!("trunk_loader.js");

const TRUNK_DEMO_STAGING_DIR: &str = "stayhydated-trunk";

/// A Trunk `copy-dir` declaration rendered into a generated demo page.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrunkDemoCopyDir {
    source: PathBuf,
    target: PathBuf,
}

impl TrunkDemoCopyDir {
    pub fn new(source: impl Into<PathBuf>, target: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

/// Standard fullscreen HTML inputs for a Trunk-built WebAssembly demo.
#[derive(Builder, Clone, Debug, Eq, PartialEq)]
pub struct TrunkDemoPageConfig {
    #[builder(into)]
    title: String,
    #[builder(into)]
    demo_name: String,
    #[builder(with = |value: impl Into<String>| value.into())]
    bootstrap_module: Option<String>,
    #[builder(with = |value: impl Into<String>| value.into())]
    bootstrap_export: Option<String>,
    #[builder(default)]
    copy_dirs: Vec<TrunkDemoCopyDir>,
    #[builder(with = |value: impl Into<String>| value.into())]
    canvas_id: Option<String>,
}

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
    /// Stages `index.html` and an adjacent shared loader before Trunk runs.
    pub generated_page: Option<TrunkDemoPageConfig>,
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

    fn generated_input_dir(&self) -> PathBuf {
        self.workspace_root
            .join("target")
            .join(TRUNK_DEMO_STAGING_DIR)
            .join(&self.example_name)
    }
}

pub fn build(config: &TrunkDemoBuildConfig) -> anyhow::Result<()> {
    validate_config(config)?;

    let example_dir = config.example_dir();
    if !example_dir.is_dir() {
        bail!(
            "Trunk demo directory does not exist at {}",
            example_dir.display()
        );
    }
    let example_dir = fs::canonicalize(&example_dir)
        .with_context(|| format!("failed to resolve {}", example_dir.display()))?;

    let index = if let Some(page) = &config.generated_page {
        write_generated_page_inputs(&config.generated_input_dir(), &example_dir, page)?
    } else if let Some(destination) = &config.loader_destination {
        write_loader(&config.resolve(destination))?;
        example_dir.join("index.html")
    } else {
        example_dir.join("index.html")
    };
    if !index.is_file() {
        bail!("Trunk demo build requires {}", index.display());
    }

    let output_dir = config.output_dir();
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
        .arg(&index)
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
    if config.generated_page.is_some() && config.loader_destination.is_some() {
        bail!("Trunk generated page and loader destination are mutually exclusive");
    }
    if config.generated_page.is_some()
        && (matches!(config.example_name.as_str(), "." | "..")
            || config
                .example_name
                .chars()
                .any(|character| matches!(character, '/' | '\\')))
    {
        bail!("Trunk generated page example name must be a single path segment");
    }
    if let Some(page) = &config.generated_page {
        validate_page_config(page)?;
    }
    Ok(())
}

fn validate_page_config(config: &TrunkDemoPageConfig) -> anyhow::Result<()> {
    if config.title.trim().is_empty() {
        bail!("Trunk demo page title cannot be empty");
    }
    if config.demo_name.trim().is_empty() {
        bail!("Trunk demo loader name cannot be empty");
    }
    if config
        .bootstrap_module
        .as_deref()
        .is_some_and(|module| module.trim().is_empty())
    {
        bail!("Trunk demo bootstrap module cannot be empty");
    }
    if config
        .bootstrap_export
        .as_deref()
        .is_some_and(|export| export.trim().is_empty())
    {
        bail!("Trunk demo bootstrap export cannot be empty");
    }
    if config.bootstrap_export.is_some() && config.bootstrap_module.is_none() {
        bail!("Trunk demo bootstrap export requires a bootstrap module");
    }
    if config
        .canvas_id
        .as_deref()
        .is_some_and(|canvas_id| canvas_id.trim().is_empty())
    {
        bail!("Trunk demo canvas ID cannot be empty");
    }
    for copy_dir in &config.copy_dirs {
        if copy_dir.source.as_os_str().is_empty() {
            bail!("Trunk demo copy directory source cannot be empty");
        }
        if copy_dir.target.as_os_str().is_empty() {
            bail!("Trunk demo copy directory target cannot be empty");
        }
    }
    Ok(())
}

fn render_index_html(
    config: &TrunkDemoPageConfig,
    example_dir: &Path,
    input_dir: &Path,
) -> anyhow::Result<String> {
    validate_page_config(config)?;

    let manifest = staged_relative_path(input_dir, &example_dir.join("Cargo.toml"))?;
    let mut html = String::from(concat!(
        "<!doctype html>\n",
        "<!-- Generated by stayhydated-xtask. -->\n",
        "<html lang=\"en\">\n",
        "  <head>\n",
        "    <meta charset=\"utf-8\" />\n",
        "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n",
        "    <title>",
    ));
    push_html_escaped(&mut html, &config.title);
    html.push_str(concat!(
        "</title>\n",
        "    <base data-trunk-public-url />\n",
        "    <meta\n",
        "      data-wasm-demo-config\n",
        "      data-wasm-demo-name=\"",
    ));
    push_html_escaped(&mut html, &config.demo_name);
    html.push_str("\"\n");
    if let Some(module) = &config.bootstrap_module {
        html.push_str("      data-wasm-bootstrap-module=\"");
        push_html_escaped(&mut html, module);
        html.push_str("\"\n");
    }
    if let Some(export) = &config.bootstrap_export {
        html.push_str("      data-wasm-bootstrap-export=\"");
        push_html_escaped(&mut html, export);
        html.push_str("\"\n");
    }
    html.push_str("    />\n");
    for copy_dir in &config.copy_dirs {
        let source = resolve_path(example_dir, &copy_dir.source);
        let source = staged_relative_path(input_dir, &source)?;
        html.push_str("    <link data-trunk rel=\"copy-dir\" href=\"");
        push_html_path(&mut html, &source)?;
        html.push_str("\" data-target-path=\"");
        push_html_path(&mut html, &copy_dir.target)?;
        html.push_str("\" />\n");
    }
    html.push_str("    <link data-trunk rel=\"rust\" href=\"");
    push_html_path(&mut html, &manifest)?;
    html.push_str("\" data-initializer=\"");
    html.push_str(TRUNK_DEMO_LOADER_FILE_NAME);
    html.push_str(concat!(
        "\" />\n",
        "    <style>\n",
        "      html,\n",
        "      body {\n",
        "        width: 100%;\n",
        "        height: 100%;\n",
        "        margin: 0;\n",
        "        overflow: hidden;\n",
        "        background: #000;\n",
        "      }\n",
    ));
    if config.canvas_id.is_some() {
        html.push_str(concat!(
            "\n",
            "      canvas {\n",
            "        width: 100% !important;\n",
            "        height: 100% !important;\n",
            "      }\n",
        ));
    }
    html.push_str("    </style>\n  </head>\n");
    if let Some(canvas_id) = &config.canvas_id {
        html.push_str("  <body>\n    <canvas id=\"");
        push_html_escaped(&mut html, canvas_id);
        html.push_str("\"></canvas>\n  </body>\n");
    } else {
        html.push_str("  <body></body>\n");
    }
    html.push_str("</html>\n");

    Ok(html)
}

fn push_html_escaped(html: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            '"' => html.push_str("&quot;"),
            '\'' => html.push_str("&#39;"),
            character => html.push(character),
        }
    }
}

fn push_html_path(html: &mut String, path: &Path) -> anyhow::Result<()> {
    let path = path
        .to_slash()
        .with_context(|| format!("Trunk demo path is not valid UTF-8: {}", path.display()))?;
    push_html_escaped(html, &path);
    Ok(())
}

fn resolve_path(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        base.join(path)
    }
}

fn staged_relative_path(input_dir: &Path, path: &Path) -> anyhow::Result<PathBuf> {
    pathdiff::diff_paths(path, input_dir).with_context(|| {
        format!(
            "failed to make Trunk demo path {} relative to {}",
            path.display(),
            input_dir.display()
        )
    })
}

fn write_generated_page_inputs(
    input_dir: &Path,
    example_dir: &Path,
    config: &TrunkDemoPageConfig,
) -> anyhow::Result<PathBuf> {
    let manifest = example_dir.join("Cargo.toml");
    if !manifest.is_file() {
        bail!(
            "Trunk generated page requires a Cargo manifest at {}",
            manifest.display()
        );
    }

    fs::create_dir_all(input_dir)
        .with_context(|| format!("failed to create {}", input_dir.display()))?;
    let input_dir = fs::canonicalize(input_dir)
        .with_context(|| format!("failed to resolve {}", input_dir.display()))?;
    let index = input_dir.join("index.html");
    let loader = input_dir.join(TRUNK_DEMO_LOADER_FILE_NAME);
    write_loader(&loader)?;
    fs::write(&index, render_index_html(config, example_dir, &input_dir)?)
        .with_context(|| format!("failed to write {}", index.display()))?;
    Ok(index)
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

    #[test]
    fn generated_page_renders_the_complete_trunk_contract() {
        let config = TrunkDemoPageConfig::builder()
            .title("Configured demo")
            .demo_name("Demo & loader")
            .bootstrap_module("./bootstrap.js")
            .bootstrap_export("run")
            .copy_dirs(vec![TrunkDemoCopyDir::new("assets", "public/assets")])
            .canvas_id("demo-canvas")
            .build();

        let html = render_index_html(
            &config,
            Path::new("workspace/examples/demo"),
            Path::new("workspace/target/stayhydated-trunk/demo"),
        )
        .expect("page should render");

        assert_eq!(
            html,
            concat!(
                "<!doctype html>\n",
                "<!-- Generated by stayhydated-xtask. -->\n",
                "<html lang=\"en\">\n",
                "  <head>\n",
                "    <meta charset=\"utf-8\" />\n",
                "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n",
                "    <title>Configured demo</title>\n",
                "    <base data-trunk-public-url />\n",
                "    <meta\n",
                "      data-wasm-demo-config\n",
                "      data-wasm-demo-name=\"Demo &amp; loader\"\n",
                "      data-wasm-bootstrap-module=\"./bootstrap.js\"\n",
                "      data-wasm-bootstrap-export=\"run\"\n",
                "    />\n",
                "    <link data-trunk rel=\"copy-dir\" href=\"../../../examples/demo/assets\" ",
                "data-target-path=\"public/assets\" />\n",
                "    <link data-trunk rel=\"rust\" href=\"../../../examples/demo/Cargo.toml\" ",
                "data-initializer=\"trunk-loader.js\" />\n",
                "    <style>\n",
                "      html,\n",
                "      body {\n",
                "        width: 100%;\n",
                "        height: 100%;\n",
                "        margin: 0;\n",
                "        overflow: hidden;\n",
                "        background: #000;\n",
                "      }\n",
                "\n",
                "      canvas {\n",
                "        width: 100% !important;\n",
                "        height: 100% !important;\n",
                "      }\n",
                "    </style>\n",
                "  </head>\n",
                "  <body>\n",
                "    <canvas id=\"demo-canvas\"></canvas>\n",
                "  </body>\n",
                "</html>\n",
            )
        );
    }

    #[test]
    fn generated_page_escapes_html_values() {
        let config = TrunkDemoPageConfig::builder()
            .title("<demo> & \"page\"")
            .demo_name("'loader'")
            .canvas_id("canvas&one")
            .build();

        let html = render_index_html(
            &config,
            Path::new("workspace/examples/demo&one"),
            Path::new("workspace/target/stayhydated-trunk/demo"),
        )
        .expect("page should render");

        assert!(html.contains("<title>&lt;demo&gt; &amp; &quot;page&quot;</title>"));
        assert!(html.contains("data-wasm-demo-name=\"&#39;loader&#39;\""));
        assert!(html.contains("href=\"../../../examples/demo&amp;one/Cargo.toml\""));
        assert!(html.contains("<canvas id=\"canvas&amp;one\"></canvas>"));
    }

    #[test]
    fn generated_page_requires_a_module_for_its_bootstrap_export() {
        let config = TrunkDemoPageConfig::builder()
            .title("Demo")
            .demo_name("Demo")
            .bootstrap_export("run")
            .build();

        let error = render_index_html(
            &config,
            Path::new("workspace/examples/demo"),
            Path::new("workspace/target/stayhydated-trunk/demo"),
        )
        .expect_err("bootstrap export should require a module");

        assert_eq!(
            error.to_string(),
            "Trunk demo bootstrap export requires a bootstrap module"
        );
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
    fn write_fake_trunk(path: &Path) {
        write_executable(
            path,
            "#!/bin/sh\nset -eu\ntarget=''\ndist=''\nif [ \"$1\" = 'build' ]; then\n  shift\n  target=\"$1\"\n  shift\nfi\nwhile [ \"$#\" -gt 0 ]; do\n  if [ \"$1\" = '--dist' ]; then shift; dist=\"$1\"; fi\n  shift\ndone\ntest -n \"$target\"\nmkdir -p \"$dist\"\ncp \"$target\" \"$dist/index.html\"\nprintf 'app' > \"$dist/demo.js\"\nprintf 'prefix-demo-marker-suffix' > \"$dist/demo.wasm\"\n",
        );
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
        write_fake_trunk(&fake_trunk);
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

    #[cfg(unix)]
    #[test]
    fn build_materializes_shared_page_inputs_before_running_trunk() {
        let temp = tempfile::tempdir().expect("temporary directory should be created");
        let example_dir = temp.path().join("examples/demo");
        fs::create_dir_all(&example_dir).expect("example directory should be created");
        write(
            &example_dir.join("Cargo.toml"),
            "[package]\nname = \"demo\"\n",
        );
        let fake_trunk = temp.path().join("fake-trunk");
        write_fake_trunk(&fake_trunk);
        let page = TrunkDemoPageConfig::builder()
            .title("Shared demo")
            .demo_name("Shared loader")
            .canvas_id("demo")
            .build();
        let config = TrunkDemoBuildConfig::builder()
            .workspace_root(temp.path())
            .example_dir("examples/demo")
            .output_dir("web/public/demo")
            .example_name("demo")
            .required_marker("demo-marker")
            .program(fake_trunk)
            .generated_page(page.clone())
            .build();

        build(&config).expect("Trunk demo should build");

        let staged_dir = temp.path().join("target/stayhydated-trunk/demo");
        let example_dir =
            fs::canonicalize(example_dir).expect("example directory should be resolvable");
        let staged_dir =
            fs::canonicalize(staged_dir).expect("staging directory should be resolvable");
        let expected_index =
            render_index_html(&page, &example_dir, &staged_dir).expect("page should render");
        assert!(!example_dir.join("index.html").exists());
        assert!(!example_dir.join(TRUNK_DEMO_LOADER_FILE_NAME).exists());
        assert_eq!(
            fs::read_to_string(staged_dir.join("index.html"))
                .expect("generated index should be readable"),
            expected_index
        );
        assert_eq!(
            fs::read_to_string(staged_dir.join(TRUNK_DEMO_LOADER_FILE_NAME))
                .expect("generated loader should be readable"),
            WASM_DEMO_LOADER_JS
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("web/public/demo/index.html"))
                .expect("built index should be readable"),
            expected_index
        );
    }

    #[test]
    fn generated_page_example_name_must_be_a_path_segment() {
        let page = TrunkDemoPageConfig::builder()
            .title("Shared demo")
            .demo_name("Shared loader")
            .build();
        let config = TrunkDemoBuildConfig::builder()
            .workspace_root("workspace")
            .example_dir("examples/demo")
            .output_dir("web/public/demo")
            .example_name("../demo")
            .required_marker("demo-marker")
            .generated_page(page)
            .build();

        let error = validate_config(&config).expect_err("path traversal should be rejected");

        assert_eq!(
            error.to_string(),
            "Trunk generated page example name must be a single path segment"
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
