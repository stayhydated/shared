use crate::InlineStyle;

fn reveal_style(delay_ms: u64, distance_px: f32) -> InlineStyle {
    InlineStyle::new(format!(
        "--motion-delay: {delay_ms}ms; --motion-distance: {distance_px}px;"
    ))
}

pub fn page_entry_reveal_style() -> InlineStyle {
    reveal_style(0, 24.0)
}

pub fn surface_reveal_style() -> InlineStyle {
    reveal_style(90, 18.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_presets_keep_staggered_css_variables() {
        assert_eq!(
            page_entry_reveal_style().as_str(),
            "--motion-delay: 0ms; --motion-distance: 24px;"
        );
        assert_eq!(
            surface_reveal_style().as_str(),
            "--motion-delay: 90ms; --motion-distance: 18px;"
        );
    }
}
