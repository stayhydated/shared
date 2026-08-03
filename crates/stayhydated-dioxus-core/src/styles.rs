use dioxus::prelude::*;

#[component]
pub fn SharedStyles() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./theme.css") }
        document::Stylesheet { href: asset!("./layout.css") }
        document::Stylesheet { href: asset!("./cards.css") }
        document::Stylesheet { href: asset!("./motion.css") }
        document::Stylesheet { href: asset!("./demo.css") }
        document::Stylesheet { href: asset!("./portal.css") }
        document::Stylesheet { href: asset!("./dx-components-theme.css") }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shared_dx_components_theme_exposes_expected_tokens() {
        let theme = include_str!("dx-components-theme.css");

        assert!(theme.contains("--primary-color"));
        assert!(theme.contains("--focused-border-color"));
    }

    #[test]
    fn skills_command_tooltip_uses_a_yellow_border() {
        let layout_css = include_str!("layout.css");

        assert!(layout_css.contains("border: 1px solid rgba(255, 255, 0, 0.72)"));
        assert!(layout_css.contains("0 0 18px rgba(255, 255, 0, 0.34)"));
        assert!(layout_css.contains("0 0 44px rgba(255, 255, 0, 0.16)"));
        assert!(!layout_css.contains("rgba(255, 0, 230, 0.58)"));
    }
}
