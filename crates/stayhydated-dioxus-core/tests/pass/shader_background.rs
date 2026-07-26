use dioxus::prelude::*;
use stayhydated_dioxus_core::ShaderBackground;

fn background() -> Element {
    rsx! {
        ShaderBackground {
            canvas_id: "example-background",
            extra_class: "example-surface",
            time_offset: 13.0,
        }
    }
}

fn main() {
    let _ = background;
}
