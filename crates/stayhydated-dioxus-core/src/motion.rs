use dioxus::prelude::*;

use crate::InlineStyle;

pub fn use_reveal_style(delay_ms: u64, distance_px: f32) -> InlineStyle {
    InlineStyle::new(format!(
        "--motion-delay: {delay_ms}ms; --motion-distance: {distance_px:.2}px;"
    ))
}

pub fn hero_reveal_style() -> InlineStyle {
    use_reveal_style(0, 24.0)
}

pub fn page_entry_reveal_style() -> InlineStyle {
    hero_reveal_style()
}

pub fn surface_reveal_style() -> InlineStyle {
    use_reveal_style(90, 18.0)
}

pub fn feature_card_reveal_style(index: usize) -> InlineStyle {
    use_reveal_style(160 + (index as u64 * 70), 16.0)
}

pub fn contribute_reveal_style() -> InlineStyle {
    use_reveal_style(370, 16.0)
}

#[component]
pub fn MotionReveal(children: Element, #[props(default, into)] style: InlineStyle) -> Element {
    let style = style.into_string();
    rsx! {
        div { class: "motion-reveal", style, {children} }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_presets_match_project_site_timing() {
        assert_eq!(
            hero_reveal_style().as_str(),
            "--motion-delay: 0ms; --motion-distance: 24.00px;"
        );
        assert_eq!(
            surface_reveal_style().as_str(),
            "--motion-delay: 90ms; --motion-distance: 18.00px;"
        );
        assert_eq!(
            feature_card_reveal_style(2).as_str(),
            "--motion-delay: 300ms; --motion-distance: 16.00px;"
        );
        assert_eq!(
            contribute_reveal_style().as_str(),
            "--motion-delay: 370ms; --motion-distance: 16.00px;"
        );
    }
}
