use dioxus::prelude::*;
use stayhydated_dioxus::{NavigationTarget, StayhydatedProjectPortalShell};

use crate::{
    site::{
        constants::{PROJECT, VERSION},
        routing::PageKind,
    },
    terminal,
};

#[component]
pub(crate) fn TerminalDemoPage() -> Element {
    use_effect(move || {
        terminal::launch_terminal_demo();
    });

    rsx! {
        StayhydatedProjectPortalShell {
            project: PROJECT,
            version: VERSION,
            home: NavigationTarget::Internal(crate::site::routing::app_route(PageKind::Home)),
            div { class: "demo-page sum-terminal-demo",
                div {
                    id: terminal::TERMINAL_MOUNT_ID,
                    class: "sum-ratzilla-terminal",
                    role: "img",
                    aria_label: "Terminal rendering of the sum-numbers-ai API response",
                    onkeydown: move |event: KeyboardEvent| {
                        trap_terminal_keydown(event);
                    },
                }
            }
        }
    }
}

fn trap_terminal_keydown(event: KeyboardEvent) {
    event.prevent_default();
    event.stop_propagation();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_demo_expands_the_terminal_inside_the_portal() {
        let html = dioxus::ssr::render_element(rsx! { TerminalDemoPage {} });

        assert!(html.contains("demo-page sum-terminal-demo"));
        assert!(html.contains("class=\"sum-ratzilla-terminal\""));
        assert!(html.contains(terminal::TERMINAL_MOUNT_ID));
        assert!(!html.contains("sum-terminal-grid"));
        assert!(!html.contains("sum-terminal-focus-shell"));
        assert!(html.contains("class=\"project-portal\""));
        assert!(html.contains("portal-header"));
        assert!(html.contains("portal-skills-copy"));
        assert!(!html.contains("project-portal is-root"));
        assert!(!html.contains("page-header"));
        assert!(!html.contains("page-title-band"));
        assert!(!html.contains("project-surface-header"));
        assert!(!html.contains("site-footer"));
    }
}
