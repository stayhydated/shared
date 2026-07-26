use dioxus::prelude::*;
use strum::IntoStaticStr;

use crate::{CssClass, DisplayText, ShaderBackground};

/// Saturated RGB-edge accent applied to a project demo card.
#[derive(Clone, Copy, Debug, Eq, IntoStaticStr, PartialEq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum DemoCardAccent {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl DemoCardAccent {
    /// Stable color token used by the shared card and WebAssembly loader styles.
    pub const fn token(self) -> &'static str {
        self.into_str()
    }
}

#[component]
pub fn CodeBlock(
    #[props(into)] code: DisplayText,
    #[props(default = CssClass::new("code-sample"), into)] class: CssClass,
) -> Element {
    let class = class.into_string();
    rsx! {
        pre { class, code { "{code}" } }
    }
}

/// Shader-backed navigation card for a project demo.
#[component]
pub fn DemoCard<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    accent: DemoCardAccent,
    #[props(into)] title: DisplayText,
    #[props(into)] shader_id: String,
    #[props(default)] time_offset: f32,
) -> Element {
    let aria_label = format!("Open {title}");
    let class = format!("demo-card demo-card-accent-{}", accent.token());

    match target {
        NavigationTarget::Internal(route) if try_router().is_some() => {
            rsx! {
                Link {
                    class,
                    to: route,
                    aria_label,
                    DemoCardContents { title, shader_id, time_offset }
                }
            }
        },
        NavigationTarget::Internal(route) => {
            rsx! {
                a {
                    class,
                    href: route.to_string(),
                    aria_label,
                    DemoCardContents { title, shader_id, time_offset }
                }
            }
        },
        NavigationTarget::External(href) => {
            rsx! {
                a {
                    class,
                    href,
                    aria_label,
                    DemoCardContents { title, shader_id, time_offset }
                }
            }
        },
    }
}

#[component]
fn DemoCardContents(title: DisplayText, shader_id: String, time_offset: f32) -> Element {
    rsx! {
        ShaderBackground {
            canvas_id: shader_id,
            extra_class: CssClass::new("demo-card-shader"),
            time_offset,
        }
        span { class: "demo-card-tint", aria_hidden: "true" }
        h2 { class: "demo-card-title", "{title}" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_card_accents_cover_every_saturated_rgb_edge() {
        let accents = [
            (DemoCardAccent::Red, "red", "rgb(255 0 0)"),
            (DemoCardAccent::Yellow, "yellow", "rgb(255 255 0)"),
            (DemoCardAccent::Green, "green", "rgb(0 255 0)"),
            (DemoCardAccent::Cyan, "cyan", "rgb(0 255 255)"),
            (DemoCardAccent::Blue, "blue", "rgb(0 0 255)"),
            (DemoCardAccent::Magenta, "magenta", "rgb(255 0 255)"),
        ];
        let stylesheet = include_str!("cards.css");

        for (accent, token, color) in accents {
            assert_eq!(accent.token(), token);
            assert!(stylesheet.contains(&format!(".demo-card-accent-{token}")));
            assert!(stylesheet.contains(color));
        }
    }
}
