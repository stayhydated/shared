use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::tabs;

use crate::{CssClass, classes};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl TabsOrientation {
    fn is_horizontal(self) -> bool {
        matches!(self, Self::Horizontal)
    }
}

#[component]
pub fn Tabs(
    children: Element,
    #[props(default)] default_value: String,
    #[props(default)] on_value_change: Callback<String>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] orientation: ReadSignal<TabsOrientation>,
    #[props(default = true)] roving_loop: ReadSignal<bool>,
) -> Element {
    let base = attributes!(div {
        class: "collection-module-tabs"
    });
    let horizontal = use_memo(move || orientation().is_horizontal());

    rsx! {
        tabs::Tabs {
            default_value,
            on_value_change,
            disabled,
            horizontal,
            roving_loop,
            attributes: base,
            {children}
        }
    }
}

#[component]
pub fn TabList(children: Element) -> Element {
    let base = attributes!(div {
        class: "collection-module-tab-list"
    });

    rsx! {
        tabs::TabList { attributes: base, {children} }
    }
}

#[component]
pub fn TabTrigger(
    value: String,
    index: ReadSignal<usize>,
    children: Element,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] id: Option<String>,
    #[props(default, into)] extra_class: CssClass,
) -> Element {
    let base = attributes!(button {
        class: classes::join("collection-module-tab", &extra_class)
    });

    rsx! {
        tabs::TabTrigger {
            class: None,
            id,
            value,
            index,
            disabled,
            attributes: base,
            {children}
        }
    }
}

#[component]
pub fn TabContent(
    value: String,
    index: ReadSignal<usize>,
    children: Element,
    #[props(default)] id: ReadSignal<Option<String>>,
    #[props(default, into)] extra_class: CssClass,
) -> Element {
    let base = attributes!(div {
        class: classes::join("collection-module-content", &extra_class)
    });

    rsx! {
        tabs::TabContent {
            class: None,
            value,
            id,
            index,
            attributes: base,
            {children}
        }
    }
}
