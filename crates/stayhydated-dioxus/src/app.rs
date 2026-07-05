use std::marker::PhantomData;

use dioxus::{document, prelude::*};
use stayhydated_dioxus_core::{Href, ShaderBackground, SharedStyles};

pub fn stayhydated_asset_href(base_href: impl AsRef<str>, asset_path: impl AsRef<str>) -> String {
    let base_href = base_href.as_ref();
    let asset_path = asset_path.as_ref().trim_start_matches('/');

    if base_href.is_empty() {
        asset_path.to_string()
    } else {
        let base_href = base_href.trim_end_matches('/');
        if base_href.is_empty() {
            return format!("/{asset_path}");
        }
        format!("{base_href}/{asset_path}")
    }
}

#[component]
pub fn StayhydatedDocumentAssets(
    #[props(into)] base_href: Href,
    #[props(default = Href::new("assets/site.css"), into)] site_stylesheet_path: Href,
    #[props(default = Href::new("dx-components-theme.css"), into)] components_theme_path: Href,
) -> Element {
    let stylesheet_href = stayhydated_asset_href(&base_href, &site_stylesheet_path);
    let components_theme_href = stayhydated_asset_href(&base_href, &components_theme_path);

    rsx! {
        SharedStyles {}
        document::Stylesheet { href: stylesheet_href }
        document::Stylesheet { href: components_theme_href }
        ShaderBackground {}
    }
}

#[component]
pub fn StayhydatedDioxusApp(#[props(into)] base_href: Href, children: Element) -> Element {
    rsx! {
        StayhydatedDocumentAssets { base_href }
        {children}
    }
}

#[derive(Clone, Eq, PartialEq, Props)]
pub struct StayhydatedRouterAppProps<R>
where
    R: Routable + Clone + PartialEq + 'static,
{
    #[props(into)]
    pub base_href: Href,
    #[props(default)]
    route: PhantomData<R>,
}

#[allow(non_snake_case)]
pub fn StayhydatedRouterApp<R>(props: StayhydatedRouterAppProps<R>) -> Element
where
    R: Routable + Clone + PartialEq + 'static,
{
    rsx! {
        StayhydatedDioxusApp { base_href: props.base_href,
            Router::<R> {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_href_joins_root_base() {
        assert_eq!(
            stayhydated_asset_href("/", "assets/site.css"),
            "/assets/site.css"
        );
    }

    #[test]
    fn asset_href_joins_project_base() {
        assert_eq!(
            stayhydated_asset_href("/koruma/", "assets/site.css"),
            "/koruma/assets/site.css"
        );
    }

    #[test]
    fn asset_href_accepts_leading_asset_slash() {
        assert_eq!(
            stayhydated_asset_href("/es-fluent/", "/dx-components-theme.css"),
            "/es-fluent/dx-components-theme.css"
        );
    }
}
