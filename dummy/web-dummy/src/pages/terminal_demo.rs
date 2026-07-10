use crate::components::{FooterPanel, PageHeader};
use crate::site::routing::PageKind;
use crate::terminal;
use dioxus::prelude::*;
use stayhydated_dioxus::{
    PageTitleBand, ProjectPageShell, ProjectSurfaceSection, surface_reveal_style,
};

#[component]
pub(crate) fn TerminalDemoPage() -> Element {
    let terminal_style = surface_reveal_style();

    use_effect(move || {
        terminal::launch_terminal_demo();
    });

    rsx! {
        ProjectPageShell {
            header: rsx!(PageHeader { current_page: PageKind::TerminalDemo }),
            footer: Some(rsx!(FooterPanel {})),
            PageTitleBand {
                label: "Operator CLI",
                title: "A terminal view of the same request contract",
                lead: "Type commands such as [1,2,3], sum [4, 5, 6], or help. The terminal renders the provider-style evidence a support or platform team would inspect.",
            }
            ProjectSurfaceSection {
                label: "Ratzilla terminal",
                title: "Command-line workflow",
                lead: "The CLI accepts bracket-list workloads and returns request identity, verification state, provider metadata, and trace events from the local crate.",
                content_class: "sum-terminal-grid",
                style: terminal_style,
                div {
                    class: "sum-terminal-focus-shell",
                    onkeydown: move |event: KeyboardEvent| {
                        trap_terminal_keydown(event);
                    },
                    div {
                        id: terminal::TERMINAL_MOUNT_ID,
                        class: "sum-ratzilla-terminal",
                        role: "img",
                        aria_label: "Terminal rendering of the sum-numbers-ai API response",
                    }
                }
            }
        }
    }
}

fn trap_terminal_keydown(event: KeyboardEvent) {
    event.prevent_default();
    event.stop_propagation();
}
