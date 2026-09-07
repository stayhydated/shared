#[cfg(target_family = "wasm")]
use std::{borrow::Cow, cell::RefCell};

use gpui_kit::component::{
    ActiveTheme as _, Root, Theme, ThemeMode,
    button::Button,
    input::{Input, InputEvent, InputState},
    v_flex,
};
use gpui_kit::prelude::*;
use gpui_kit::{App, Context, Entity, Subscription, Window, WindowOptions};
#[cfg(not(target_family = "wasm"))]
use gpui_kit::{Bounds, WindowBounds};
use sum_numbers_ai_dummy::{MAX_DEMO_INPUTS, SumRequest, sum_with_request};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

const DEMO_MARKER: &str = "sum-numbers-ai-gpui-demo";
const DEFAULT_INPUTS: [&str; MAX_DEMO_INPUTS] = ["8", "13", "21"];

struct SumDemo {
    inputs: [Entity<InputState>; MAX_DEMO_INPUTS],
    _subscriptions: Vec<Subscription>,
}

impl SumDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let inputs = DEFAULT_INPUTS.map(|value| {
            cx.new(|cx| {
                InputState::new(window, cx)
                    .default_value(value)
                    .validate(|value, _| value.parse::<i64>().is_ok())
            })
        });
        let subscriptions = inputs
            .iter()
            .map(|input| cx.subscribe_in(input, window, Self::on_input))
            .collect();

        Self {
            inputs,
            _subscriptions: subscriptions,
        }
    }

    fn on_input(
        &mut self,
        _state: &Entity<InputState>,
        event: &InputEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::Change) {
            cx.notify();
        }
    }

    fn reset(
        &mut self,
        _event: &gpui_kit::ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for (input, value) in self.inputs.iter().zip(DEFAULT_INPUTS) {
            input.update(cx, |input, cx| input.set_value(value, window, cx));
        }
        cx.notify();
    }

    fn status(&self, cx: &App) -> String {
        let values = self
            .inputs
            .iter()
            .map(|input| input.read(cx).value().parse::<i64>())
            .collect::<Result<Vec<_>, _>>();

        match values {
            Ok(values) => {
                let response = sum_with_request(&SumRequest::new(values));
                format!(
                    "Total {} · {} operands · verified {}",
                    response.sum,
                    response.numbers.len(),
                    response.verified
                )
            },
            Err(_) => "Review the three integer inputs".to_owned(),
        }
    }
}

impl Render for SumDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input_rows = self
            .inputs
            .iter()
            .enumerate()
            .map(|(index, input)| {
                v_flex()
                    .gap_1()
                    .child(format!("Operand {}", index + 1))
                    .child(Input::new(input))
            })
            .collect::<Vec<_>>();

        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .p_6()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                v_flex()
                    .w_full()
                    .max_w(gpui_kit::rems(35.))
                    .gap_4()
                    .p_6()
                    .rounded(cx.theme().radius_lg)
                    .border_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().group_box)
                    .text_color(cx.theme().group_box_foreground)
                    .child(
                        gpui_kit::div()
                            .text_2xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .child(DEMO_MARKER),
                    )
                    .child(
                        gpui_kit::div()
                            .text_color(cx.theme().muted_foreground)
                            .child("Three GPUI Kit inputs share the verified Rust sum contract."),
                    )
                    .children(input_rows)
                    .child(
                        Button::new("reset-inputs")
                            .label("Reset to 8 + 13 + 21")
                            .on_click(cx.listener(Self::reset)),
                    )
                    .child(
                        gpui_kit::div()
                            .text_lg()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .child(self.status(cx)),
                    ),
            )
    }
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets::new(""))
        .run(launch);
}

#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(target_family = "wasm")]
thread_local! {
    static APPLICATION: RefCell<Option<gpui_kit::ApplicationHandle>> = const { RefCell::new(None) };
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    gpui_kit::platform::web_init();
    let app =
        gpui_kit::platform::single_threaded_web().with_assets(gpui_kit::assets::Assets::new(""));
    APPLICATION.with(|application| {
        *application.borrow_mut() = Some(app.run_embedded(launch));
    });
    Ok(())
}

fn launch(cx: &mut App) {
    gpui_kit::init(cx);

    #[cfg(target_family = "wasm")]
    {
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(ttf_inter::REGULAR)])
            .expect("the GPUI demo font should load");
    }

    Theme::change(ThemeMode::Dark, None, cx);
    #[cfg(target_family = "wasm")]
    {
        Theme::global_mut(cx).font_family = "Inter".into();
        Theme::sync_base(cx);
    }

    #[cfg(not(target_family = "wasm"))]
    let options = {
        let bounds = Bounds::centered(
            None,
            gpui_kit::size(gpui_kit::px(820.), gpui_kit::px(620.)),
            cx,
        );
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        }
    };
    #[cfg(target_family = "wasm")]
    let options = WindowOptions::default();

    cx.open_window(options, |window, cx| {
        let view = cx.new(|cx| SumDemo::new(window, cx));
        cx.new(|cx| Root::new(view, window, cx).bg(cx.theme().background))
    })
    .expect("the GPUI demo window should open");
    cx.activate(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpui_demo_has_exactly_three_default_inputs() {
        assert_eq!(DEFAULT_INPUTS.len(), MAX_DEMO_INPUTS);
        assert_eq!(
            DEFAULT_INPUTS
                .into_iter()
                .map(|value| value.parse::<i64>().expect("default should parse"))
                .sum::<i64>(),
            42
        );
    }
}
