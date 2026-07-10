#[cfg(target_arch = "wasm32")]
use crate::cli::{TerminalCommandOutput, TerminalCommandStatus, run_terminal_command};

pub const TERMINAL_MOUNT_ID: &str = "sum-ratzilla-terminal";

#[cfg(target_arch = "wasm32")]
pub fn launch_terminal_demo() {
    if terminal_is_mounted() {
        return;
    }

    if let Err(error) = start_terminal() {
        web_sys::console::error_1(&format!("failed to start Ratzilla demo: {error}").into());
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn launch_terminal_demo() {}

#[cfg(target_arch = "wasm32")]
fn terminal_is_mounted() -> bool {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(TERMINAL_MOUNT_ID))
        .is_some_and(|element| element.child_element_count() > 0)
}

#[cfg(target_arch = "wasm32")]
fn start_terminal() -> Result<(), String> {
    use std::{cell::RefCell, rc::Rc};

    use ratzilla::ratatui::backend::Backend as _;
    use ratzilla::ratatui::layout::Rect;
    use ratzilla::ratatui::{Terminal, TerminalOptions, Viewport};
    use ratzilla::{DomBackend, WebRenderer as _};

    let mut backend =
        DomBackend::new_by_id(TERMINAL_MOUNT_ID).map_err(|error| error.to_string())?;
    let size = backend
        .window_size()
        .map_err(|error| error.to_string())?
        .columns_rows;
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 0, size.width, size.height)),
        },
    )
    .map_err(|error| error.to_string())?;

    let state = Rc::new(RefCell::new(TerminalState::new()));
    terminal
        .on_key_event({
            let state = state.clone();
            move |event| state.borrow_mut().handle_key(event)
        })
        .map_err(|error| error.to_string())?;

    terminal.draw_web(move |frame| {
        render_terminal(frame, &state.borrow());
    });

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct TerminalState {
    entries: Vec<TerminalEntry>,
    command_history: Vec<String>,
    history_cursor: Option<usize>,
    input: String,
}

#[cfg(target_arch = "wasm32")]
impl TerminalState {
    fn new() -> Self {
        Self {
            entries: ["help", "[1,2,3]"]
                .into_iter()
                .map(TerminalEntry::run)
                .collect(),
            command_history: vec!["help".to_owned(), "[1,2,3]".to_owned()],
            history_cursor: None,
            input: String::new(),
        }
    }

    fn handle_key(&mut self, event: ratzilla::event::KeyEvent) {
        use ratzilla::event::KeyCode;

        match event.code {
            KeyCode::Char(character) if !event.ctrl && !event.alt => {
                self.input.push(character);
                self.history_cursor = None;
            },
            KeyCode::Backspace => {
                self.input.pop();
                self.history_cursor = None;
            },
            KeyCode::Enter => self.submit_input(),
            KeyCode::Up => self.recall_previous(),
            KeyCode::Down => self.recall_next(),
            KeyCode::Esc => {
                self.input.clear();
                self.history_cursor = None;
            },
            _ => {},
        }
    }

    fn submit_input(&mut self) {
        let command = self.input.trim().to_owned();
        if command.is_empty() {
            return;
        }

        self.entries.push(TerminalEntry::run(command.clone()));
        self.command_history.push(command);
        self.history_cursor = None;
        self.input.clear();
    }

    fn recall_previous(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let index = match self.history_cursor {
            Some(0) => 0,
            Some(index) => index - 1,
            None => self.command_history.len() - 1,
        };
        self.history_cursor = Some(index);
        self.input = self.command_history[index].clone();
    }

    fn recall_next(&mut self) {
        let Some(index) = self.history_cursor else {
            return;
        };

        if index + 1 >= self.command_history.len() {
            self.history_cursor = None;
            self.input.clear();
        } else {
            let next = index + 1;
            self.history_cursor = Some(next);
            self.input = self.command_history[next].clone();
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct TerminalEntry {
    command: String,
    output: TerminalCommandOutput,
}

#[cfg(target_arch = "wasm32")]
impl TerminalEntry {
    fn run(command: impl Into<String>) -> Self {
        let command = command.into();
        let output = run_terminal_command(&command);
        Self { command, output }
    }
}

#[cfg(target_arch = "wasm32")]
fn render_terminal(frame: &mut ratzilla::ratatui::Frame<'_>, state: &TerminalState) {
    use ratzilla::ratatui::style::{Color, Modifier, Style};
    use ratzilla::ratatui::text::{Line, Span};
    use ratzilla::ratatui::widgets::{Block, Paragraph, Wrap};

    let prompt_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let command_style = Style::default().fg(Color::White);
    let success_style = Style::default().fg(Color::Green);
    let error_style = Style::default().fg(Color::Red);
    let muted_style = Style::default().fg(Color::DarkGray);

    let mut lines = Vec::new();
    for entry in &state.entries {
        lines.push(Line::from(vec![
            Span::styled("$ ", prompt_style),
            Span::styled(entry.command.clone(), command_style),
        ]));
        let output_style = match entry.output.status {
            TerminalCommandStatus::Success => success_style,
            TerminalCommandStatus::Error => error_style,
        };
        lines.extend(
            entry
                .output
                .lines
                .iter()
                .map(|line| Line::from(Span::styled(line.clone(), output_style))),
        );
        lines.push(Line::raw(""));
    }
    lines.push(Line::from(vec![
        Span::styled("$ ", prompt_style),
        Span::styled(state.input.clone(), command_style),
        Span::styled(" ", muted_style),
        Span::styled("█", prompt_style),
    ]));

    let max_lines = frame.area().height.saturating_sub(2) as usize;
    let start = lines.len().saturating_sub(max_lines);
    let visible = lines.into_iter().skip(start).collect::<Vec<_>>();

    frame.render_widget(
        Paragraph::new(visible)
            .block(
                Block::bordered()
                    .title("sum-numbers-ai ops")
                    .border_style(Color::Green),
            )
            .wrap(Wrap { trim: false }),
        frame.area(),
    );
}
