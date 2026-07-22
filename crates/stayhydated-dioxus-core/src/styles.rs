use dioxus::prelude::*;

pub const DX_COMPONENTS_THEME_FILE_NAME: &str = "dx-components-theme.css";
pub const DX_COMPONENTS_THEME_CSS: &str = include_str!("dx-components-theme.css");

#[component]
pub fn SharedStyles() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./theme.css") }
        document::Stylesheet { href: asset!("./layout.css") }
        document::Stylesheet { href: asset!("./cards.css") }
        document::Stylesheet { href: asset!("./motion.css") }
        document::Stylesheet { href: asset!("./demo.css") }
        document::Stylesheet { href: asset!("./portal.css") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_dx_components_theme_exposes_expected_file() {
        assert_eq!(DX_COMPONENTS_THEME_FILE_NAME, "dx-components-theme.css");
        assert!(DX_COMPONENTS_THEME_CSS.contains("--primary-color"));
        assert!(DX_COMPONENTS_THEME_CSS.contains("--focused-border-color"));
    }
}
