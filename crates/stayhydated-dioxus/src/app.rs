use std::marker::PhantomData;

use dioxus::{document, prelude::*};
use stayhydated_dioxus_core::{Href, SharedStyles};
use stayhydated_site::routing::{asset_href, dioxus_base_href};

#[component]
pub fn StayhydatedDocumentAssets(#[props(default)] site_stylesheet_path: Option<Href>) -> Element {
    let base_href = dioxus_base_href();
    let stylesheet_href =
        site_stylesheet_path.map(|path| asset_href(&base_href, path).into_string());

    rsx! {
        SharedStyles {}
        if let Some(stylesheet_href) = stylesheet_href {
            document::Stylesheet { href: stylesheet_href }
        }
    }
}

#[component]
pub fn StayhydatedDioxusApp(
    #[props(default)] site_stylesheet_path: Option<Href>,
    children: Element,
) -> Element {
    rsx! {
        StayhydatedDocumentAssets { site_stylesheet_path }
        {children}
    }
}

#[derive(Clone, Eq, PartialEq, Props)]
pub struct StayhydatedRouterAppProps<R>
where
    R: Routable + Clone + PartialEq + 'static,
{
    #[props(default)]
    pub site_stylesheet_path: Option<Href>,
    #[props(default)]
    route: PhantomData<R>,
}

#[allow(non_snake_case)]
pub fn StayhydatedRouterApp<R>(props: StayhydatedRouterAppProps<R>) -> Element
where
    R: Routable + Clone + PartialEq + 'static,
{
    rsx! {
        StayhydatedDioxusApp { site_stylesheet_path: props.site_stylesheet_path,
            Router::<R> {}
        }
    }
}
