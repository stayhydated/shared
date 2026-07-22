use dioxus::prelude::*;

use crate::{CssClass, DisplayText};

#[component]
pub(crate) fn RouteLink<R: Routable + Clone + PartialEq + 'static>(
    target: NavigationTarget<R>,
    #[props(into)] class: CssClass,
    #[props(into)] label: DisplayText,
    #[props(into)] aria_label: DisplayText,
) -> Element {
    let class = class.into_string();
    let aria_label = aria_label.into_string();

    match target {
        NavigationTarget::Internal(route) if try_router().is_some() => {
            rsx! {
                Link {
                    class,
                    to: route,
                    aria_label,
                    "{label}"
                }
            }
        },
        NavigationTarget::Internal(route) => {
            let href = route.to_string();
            rsx! {
                a {
                    class,
                    href,
                    aria_label,
                    "{label}"
                }
            }
        },
        NavigationTarget::External(href) => {
            rsx! {
                a {
                    class,
                    href,
                    aria_label,
                    "{label}"
                }
            }
        },
    }
}
