use dioxus::prelude::*;

use crate::{CssClass, classes};

const DEFAULT_SHADER_BACKGROUND_CANVAS_ID: &str = "shader-background-canvas";
const SHADER_BACKGROUND_CLASS: &str = "shader-background";
const SHADER_BACKGROUND_PREPAINT_STYLE: &str =
    "html, body, #main { background: #000; }\n#main { min-height: 100vh; }";
const SHADER_BACKGROUND_CANVAS_PREPAINT_STYLE: &str = "background-color: #000;";

#[component]
pub(crate) fn ShaderBackground(
    #[props(default = DEFAULT_SHADER_BACKGROUND_CANVAS_ID.to_owned())] canvas_id: String,
    #[props(default)] extra_class: CssClass,
    #[props(default = 0.08)] grid_opacity: f32,
    #[props(default)] time_offset: f32,
) -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        let canvas_id = canvas_id.clone();
        let mut renderer_handle =
            use_signal(|| None::<crate::shader_background_renderer::ShaderBackgroundHandle>);

        use_effect(move || {
            renderer_handle.set(Some(crate::shader_background_renderer::start(
                canvas_id.clone(),
                grid_opacity,
                time_offset,
            )));
        });
        dioxus::core::use_drop(move || {
            renderer_handle.write().take();
        });
    }

    let class = classes::join(SHADER_BACKGROUND_CLASS, &extra_class);

    rsx! {
        document::Style { "{SHADER_BACKGROUND_PREPAINT_STYLE}" }
        document::Stylesheet { href: asset!("./shader_background.css") }
        canvas {
            id: canvas_id,
            class,
            style: SHADER_BACKGROUND_CANVAS_PREPAINT_STYLE,
            "data-shader-background": "loading",
            aria_hidden: "true",
            tabindex: "-1",
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shader_module_parses_and_validates() {
        let source = include_str!("shader_background.wgsl");
        let module = naga::front::wgsl::parse_str(source).expect("shader source should parse");

        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .expect("shader module should validate");
    }
}
