use dioxus::prelude::*;
use strum::IntoStaticStr;

use crate::{CssClass, DisplayText, Href, ShaderBackground, motion::page_entry_reveal_style};

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
    const PALETTE: [Self; 6] = [
        Self::Red,
        Self::Yellow,
        Self::Green,
        Self::Cyan,
        Self::Blue,
        Self::Magenta,
    ];

    /// Stable color token used by the shared card and WebAssembly loader styles.
    pub const fn token(self) -> &'static str {
        self.into_str()
    }

    /// Selects an accent for one position in a gallery of `total` cards.
    ///
    /// Galleries up to the palette size are spread evenly across the available
    /// accents. Larger galleries cycle the palette. Positions beyond `total`
    /// wrap, and a zero `total` falls back to cycling the full palette.
    pub const fn for_position(position: usize, total: usize) -> Self {
        let palette_len = Self::PALETTE.len();
        let position = if total == 0 {
            position
        } else {
            position % total
        };
        let palette_index = if total == 0 || total > palette_len {
            position % palette_len
        } else {
            position * palette_len / total
        };

        Self::PALETTE[palette_index]
    }
}

/// Responsive column count used by a [`DemoGallery`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DemoGalleryColumns {
    #[default]
    Two,
    Three,
}

impl DemoGalleryColumns {
    const fn class(self) -> &'static str {
        match self {
            Self::Two => "columns-2",
            Self::Three => "columns-3",
        }
    }
}

/// One destination rendered by a [`DemoGallery`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoGalleryItem<R> {
    target: NavigationTarget<R>,
    title: DisplayText,
    shader_id: String,
}

impl<R> DemoGalleryItem<R> {
    pub fn new(
        target: NavigationTarget<R>,
        title: impl Into<DisplayText>,
        shader_id: impl Into<String>,
    ) -> Self {
        Self {
            target,
            title: title.into(),
            shader_id: shader_id.into(),
        }
    }

    pub fn route(route: R, title: impl Into<DisplayText>, shader_id: impl Into<String>) -> Self {
        Self::new(NavigationTarget::Internal(route), title, shader_id)
    }

    pub fn href(
        href: impl Into<Href>,
        title: impl Into<DisplayText>,
        shader_id: impl Into<String>,
    ) -> Self {
        Self::new(
            NavigationTarget::External(href.into().into_string()),
            title,
            shader_id,
        )
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

/// Shared reveal layout for a collection of shader-backed demo cards.
#[component]
pub fn DemoGallery<R: Routable + Clone + PartialEq + 'static>(
    items: Vec<DemoGalleryItem<R>>,
    #[props(default)] columns: DemoGalleryColumns,
) -> Element {
    let item_count = items.len();
    let grid_class = format!("grid {} demo-example-cards motion-reveal", columns.class());
    let reveal_style = page_entry_reveal_style().into_string();

    rsx! {
        div { class: "demo-page demo-gallery",
            section { class: grid_class, style: reveal_style,
                for (position, item) in items.into_iter().enumerate() {
                    DemoCard::<R> {
                        key: "{item.shader_id}",
                        target: item.target,
                        accent: DemoCardAccent::for_position(position, item_count),
                        title: item.title,
                        shader_id: item.shader_id,
                        time_offset: position as f32 * 13.0,
                    }
                }
            }
        }
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

    #[test]
    fn gallery_positions_spread_across_the_palette() {
        let accents = |total| {
            (0..total)
                .map(|position| DemoCardAccent::for_position(position, total))
                .collect::<Vec<_>>()
        };

        assert_eq!(accents(1), [DemoCardAccent::Red]);
        assert_eq!(accents(2), [DemoCardAccent::Red, DemoCardAccent::Cyan]);
        assert_eq!(
            accents(3),
            [
                DemoCardAccent::Red,
                DemoCardAccent::Green,
                DemoCardAccent::Blue,
            ]
        );
        assert_eq!(
            accents(4),
            [
                DemoCardAccent::Red,
                DemoCardAccent::Yellow,
                DemoCardAccent::Cyan,
                DemoCardAccent::Blue,
            ]
        );
        assert_eq!(
            accents(6),
            [
                DemoCardAccent::Red,
                DemoCardAccent::Yellow,
                DemoCardAccent::Green,
                DemoCardAccent::Cyan,
                DemoCardAccent::Blue,
                DemoCardAccent::Magenta,
            ]
        );
    }

    #[test]
    fn gallery_positions_wrap_for_large_empty_and_out_of_range_inputs() {
        assert_eq!(DemoCardAccent::for_position(6, 7), DemoCardAccent::Red);
        assert_eq!(DemoCardAccent::for_position(7, 7), DemoCardAccent::Red);
        assert_eq!(DemoCardAccent::for_position(2, 0), DemoCardAccent::Green);
        assert_eq!(DemoCardAccent::for_position(4, 2), DemoCardAccent::Red);
    }

    #[test]
    fn gallery_columns_have_stable_layout_classes() {
        assert_eq!(DemoGalleryColumns::default().class(), "columns-2");
        assert_eq!(DemoGalleryColumns::Three.class(), "columns-3");
    }
}
