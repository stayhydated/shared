use bevy::{
    input_focus::{AutoFocus, tab_navigation::TabIndex},
    prelude::*,
    text::{EditableText, EditableTextFilter, TextCursorStyle},
    window::{WindowPlugin, WindowResolution},
};
use sum_numbers_ai_dummy::{MAX_DEMO_INPUTS, SumRequest, sum_with_request};

const DEMO_MARKER: &str = "sum-numbers-ai-bevy-demo";
const DEFAULT_INPUTS: [i64; MAX_DEMO_INPUTS] = [8, 13, 21];
const BACKGROUND: Color = Color::BLACK;
const PANEL: Color = Color::srgb_u8(10, 10, 10);
const FIELD: Color = Color::srgb_u8(17, 17, 17);
const BORDER: Color = Color::srgb_u8(42, 42, 42);
const MUTED_TEXT: Color = Color::srgb_u8(163, 163, 163);
const CYAN: Color = Color::srgb_u8(34, 211, 238);
const LIME: Color = Color::srgb_u8(182, 255, 0);

#[derive(Component)]
struct OperandInput(usize);

#[derive(Component)]
struct TotalOutput;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-demo".to_owned()),
                fit_canvas_to_parent: true,
                resolution: WindowResolution::new(960, 640),
                title: "sum-numbers-ai · Bevy UI".to_owned(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update_total)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: px(24).all(),
            ..default()
        })
        .insert(BackgroundColor(BACKGROUND))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: percent(100),
                    max_width: px(620),
                    padding: px(28).all(),
                    border: px(1).all(),
                    border_radius: BorderRadius::all(px(18)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(16),
                    ..default()
                },
                BackgroundColor(PANEL),
                BorderColor::all(BORDER),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new(DEMO_MARKER),
                    TextFont::from_font_size(34.),
                    TextColor(Color::WHITE),
                ));
                panel.spawn((
                    Text::new("Three Bevy UI inputs share the same verified Rust sum contract."),
                    TextFont::from_font_size(17.),
                    TextColor(MUTED_TEXT),
                ));

                for (index, value) in DEFAULT_INPUTS.into_iter().enumerate() {
                    let mut input = panel.spawn((
                        Node {
                            width: percent(100),
                            height: px(52),
                            padding: px(12).all(),
                            border: px(1).all(),
                            border_radius: BorderRadius::all(px(10)),
                            ..default()
                        },
                        EditableText::new(value.to_string()),
                        EditableTextFilter::new(|character| {
                            character.is_ascii_digit() || character == '-'
                        }),
                        TextCursorStyle {
                            color: LIME,
                            ..default()
                        },
                        TextFont::from_font_size(22.),
                        TextColor(Color::WHITE),
                        BackgroundColor(FIELD),
                        BorderColor::all(CYAN),
                        OperandInput(index),
                        TabIndex(index as i32),
                    ));
                    if index == 0 {
                        input.insert(AutoFocus);
                    }
                }

                panel.spawn((
                    Text::new(result_text(&DEFAULT_INPUTS)),
                    TextFont::from_font_size(24.),
                    TextColor(LIME),
                    TotalOutput,
                ));
            });
        });
}

fn update_total(
    inputs: Query<(&OperandInput, &EditableText)>,
    mut output: Single<&mut Text, With<TotalOutput>>,
) {
    let mut values = [None; MAX_DEMO_INPUTS];
    for (index, input) in &inputs {
        values[index.0] = input.value().to_string().parse::<i64>().ok();
    }

    let next = values
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .map(|values| result_text(&values))
        .unwrap_or_else(|| "Review the three integer inputs".to_owned());
    if output.0 != next {
        output.0 = next;
    }
}

fn result_text(values: &[i64]) -> String {
    let response = sum_with_request(&SumRequest::new(values.iter().copied()));
    format!(
        "Total {} · {} operands · verified {}",
        response.sum,
        response.numbers.len(),
        response.verified
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bevy_inputs_match_the_demo_limit_and_sum_to_42() {
        assert_eq!(DEFAULT_INPUTS.len(), MAX_DEMO_INPUTS);
        assert!(result_text(&DEFAULT_INPUTS).contains("Total 42"));
    }
}
