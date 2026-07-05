use dioxus::prelude::*;

use crate::{CssClass, classes};

pub const DEFAULT_SHADER_BACKGROUND_CANVAS_ID: &str = "shader-background-canvas";
const SHADER_BACKGROUND_CLASS: &str = "shader-background";
const SHADER_BACKGROUND_PREPAINT_STYLE: &str =
    "html, body, #main { background: #000; }\n#main { min-height: 100vh; }";
const SHADER_BACKGROUND_CANVAS_PREPAINT_STYLE: &str = "background-color: #000;";

#[component]
pub fn ShaderBackground(
    #[props(default = DEFAULT_SHADER_BACKGROUND_CANVAS_ID.to_owned())] canvas_id: String,
    #[props(default)] extra_class: CssClass,
    #[props(default = 0.08)] grid_opacity: f32,
) -> Element {
    let _ = grid_opacity;

    #[cfg(target_arch = "wasm32")]
    {
        let canvas_id = canvas_id.clone();
        let mut renderer_handle =
            use_signal(|| None::<crate::shader_background_renderer::ShaderBackgroundHandle>);

        use_effect(move || {
            if renderer_handle.peek().is_none() {
                renderer_handle.set(Some(crate::shader_background_renderer::start(
                    canvas_id.clone(),
                    grid_opacity,
                )));
            }
        });
        dioxus::core::use_drop(move || {
            if let Some(handle) = renderer_handle.write().take() {
                handle.stop();
            }
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
            aria_hidden: "true",
            tabindex: "-1",
        }
    }
}
