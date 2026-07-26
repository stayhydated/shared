use gpui::prelude::*;
use gpui::{
    App, Application, Bounds, Context, Entity, Subscription, Window, WindowBounds, WindowOptions,
};
use gpui_component::{
    Root, Theme, ThemeMode,
    button::Button,
    input::{Input, InputEvent, InputState},
    v_flex,
};
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

    fn reset(&mut self, _event: &gpui::ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
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
            .bg(gpui::rgb(0x000000))
            .text_color(gpui::rgb(0xf5f5f5))
            .child(
                v_flex()
                    .w(gpui::px(560.))
                    .gap_4()
                    .p_6()
                    .rounded_xl()
                    .border_1()
                    .border_color(gpui::rgb(0x2a2a2a))
                    .bg(gpui::rgb(0x0a0a0a))
                    .child(
                        gpui::div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(DEMO_MARKER),
                    )
                    .child(
                        gpui::div().text_color(gpui::rgb(0xa3a3a3)).child(
                            "Three gpui-component inputs share the verified Rust sum contract.",
                        ),
                    )
                    .children(input_rows)
                    .child(
                        Button::new("reset-inputs")
                            .label("Reset to 8 + 13 + 21")
                            .on_click(cx.listener(Self::reset)),
                    )
                    .child(
                        gpui::div()
                            .text_lg()
                            .text_color(gpui::rgb(0xb6ff00))
                            .child(self.status(cx)),
                    ),
            )
    }
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_with_app(gpui_platform::application());
}

#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    gpui_platform::web_init();
    let app = keep_web_application_alive(gpui_platform::single_threaded_web());
    run_with_app(app);
    Ok(())
}

#[cfg(target_family = "wasm")]
fn keep_web_application_alive(app: Application) -> Application {
    struct WasmApplication(std::rc::Rc<gpui::AppCell>);

    // SAFETY: GPUI's web application must outlive the wasm entry point. The
    // wrapper exposes the application cell so one strong reference can be
    // intentionally retained for the browser process lifetime.
    unsafe {
        let wasm_app = std::mem::transmute::<Application, WasmApplication>(app);
        std::mem::forget(wasm_app.0.clone());
        std::mem::transmute::<WasmApplication, Application>(wasm_app)
    }
}

fn run_with_app(app: Application) {
    app.run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);
        let bounds = Bounds::centered(None, gpui::size(gpui::px(820.), gpui::px(620.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| SumDemo::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .expect("the GPUI demo window should open");
        cx.activate(true);
    });
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
