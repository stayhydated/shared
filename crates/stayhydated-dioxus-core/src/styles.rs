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

    #[test]
    fn portal_header_constrains_and_wraps_project_copy() {
        let portal_css = include_str!("portal.css");
        let title_copy_rule = rule(portal_css, ".portal-title-copy");
        let version_rule = rule(portal_css, ".portal-version");
        let tagline_rule = rule(portal_css, ".portal-header p");

        assert!(title_copy_rule.contains("flex: 1 1 auto"));
        assert!(title_copy_rule.contains("min-width: 0"));
        assert!(version_rule.contains("flex: 0 0 auto"));
        assert!(version_rule.contains("white-space: nowrap"));
        assert!(tagline_rule.contains("width: 100%"));
        assert!(tagline_rule.contains("min-width: 0"));
        assert!(tagline_rule.contains("overflow-wrap: anywhere"));
        assert!(tagline_rule.contains("white-space: normal"));
    }

    fn rule<'a>(css: &'a str, selector: &str) -> &'a str {
        css.split_once(&format!("{selector} {{"))
            .and_then(|(_, declarations)| declarations.split_once('}'))
            .map(|(declarations, _)| declarations)
            .unwrap_or_else(|| panic!("missing CSS rule for {selector}"))
    }
}
