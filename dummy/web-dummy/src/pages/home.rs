use dioxus::prelude::*;
use stayhydated_dioxus::{Href, NavigationTarget, StayhydatedProjectPortal};

use crate::site::{
    constants::{PROJECT, SOURCE_URL, VERSION},
    routing::PageKind,
};

#[component]
pub(crate) fn HomePage() -> Element {
    rsx! {
        StayhydatedProjectPortal::<crate::site::routing::AppRoute> {
            project: PROJECT,
            version: VERSION,
            home: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Home)),
            docs: Href::new(
                crate::site::routing::book_page_href("api-contract.html").into_string(),
            ),
            book: Href::new(crate::site::routing::book_href().into_string()),
            demos: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Demos)),
            source: Href::new(SOURCE_URL),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_page_keeps_the_project_navigation() {
        let html = dioxus::ssr::render_element(rsx! { HomePage {} });

        assert!(html.contains("project-portal is-root"));
        assert!(html.contains("portal-header"));
        assert!(html.contains("portal-destinations"));
        assert!(html.contains(r#"href="/book/api-contract.html""#));
        assert!(html.contains(SOURCE_URL));
    }
}
