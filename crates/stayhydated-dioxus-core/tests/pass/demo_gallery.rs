use dioxus::prelude::*;
use stayhydated_dioxus_core::{DemoGallery, DemoGalleryColumns, DemoGalleryItem};

#[derive(Clone, Debug, PartialEq, Routable)]
enum AppRoute {
    #[route("/", Home)]
    Home {},
}

#[component]
fn Home() -> Element {
    rsx! {}
}

fn gallery() -> Element {
    rsx! {
        DemoGallery::<AppRoute> {
            columns: DemoGalleryColumns::Three,
            items: vec![
                DemoGalleryItem::route(AppRoute::Home {}, "Home", "home-shader"),
                DemoGalleryItem::href("/demo/", "Demo", "demo-shader"),
            ],
        }
    }
}

fn main() {
    let _ = gallery;
}
