use std::fmt;

use clap::{Parser, Subcommand, error::ErrorKind};
use sum_numbers_ai_dummy::{SumRequest, sum_with_request};

const PROGRAM_NAME: &str = "sum-numbers-ai";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalCommandOutput {
    pub status: TerminalCommandStatus,
    pub lines: Vec<String>,
}

impl TerminalCommandOutput {
    fn success(lines: impl IntoIterator<Item = String>) -> Self {
        Self {
            status: TerminalCommandStatus::Success,
            lines: lines.into_iter().collect(),
        }
    }

    fn error(lines: impl IntoIterator<Item = String>) -> Self {
        Self {
            status: TerminalCommandStatus::Error,
            lines: lines.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalCommandStatus {
    Success,
    Error,
}

#[derive(Debug, Parser)]
#[command(
    name = PROGRAM_NAME,
    about = "Inspect local sum-numbers-ai request contracts.",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
struct TerminalCli {
    #[command(subcommand)]
    command: TerminalCommand,
}

#[derive(Debug, Subcommand)]
enum TerminalCommand {
    /// Run a JSON-style integer workload through the provider contract.
    #[command(alias = "run")]
    Sum {
        /// Input workload, for example [1,2,3].
        numbers: String,
    },
}

pub fn run_terminal_command(input: &str) -> TerminalCommandOutput {
    let trimmed = input.trim();
    let argv = command_argv(trimmed);

    match TerminalCli::try_parse_from(argv) {
        Ok(cli) => match cli.command {
            TerminalCommand::Sum { numbers } => run_sum_command(&numbers),
        },
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            TerminalCommandOutput::success(error.to_string().lines().map(str::to_owned))
        },
        Err(error) => TerminalCommandOutput::error(error.to_string().lines().map(str::to_owned)),
    }
}

fn command_argv(input: &str) -> Vec<String> {
    if input.eq_ignore_ascii_case("help") {
        return vec![PROGRAM_NAME.to_owned(), "--help".to_owned()];
    }

    if input.starts_with('[') {
        return vec![PROGRAM_NAME.to_owned(), "sum".to_owned(), input.to_owned()];
    }

    if let Some(numbers) = input.strip_prefix("sum ") {
        return vec![
            PROGRAM_NAME.to_owned(),
            "sum".to_owned(),
            numbers.trim().to_owned(),
        ];
    }

    if let Some(numbers) = input.strip_prefix("run ") {
        return vec![
            PROGRAM_NAME.to_owned(),
            "run".to_owned(),
            numbers.trim().to_owned(),
        ];
    }

    let mut argv = vec![PROGRAM_NAME.to_owned()];
    argv.extend(input.split_whitespace().map(str::to_owned));
    argv
}

fn run_sum_command(numbers: &str) -> TerminalCommandOutput {
    match parse_number_list(numbers) {
        Ok(numbers) => {
            let request = SumRequest::new(numbers);
            let response = sum_with_request(&request);
            let mut lines = vec![
                format!("request_id {}", response.request_id),
                format!("numbers [{}]", format_numbers(&response.numbers)),
                format!("sum {}", response.sum),
                format!("verified {}", response.verified),
                format!("model {}", response.provider.model),
                format!("latency_ms {}", response.provider.latency_ms),
                "trace".to_owned(),
            ];
            lines.extend(
                response
                    .trace
                    .iter()
                    .map(|event| format!("  {} {}", event.code, event.message)),
            );
            TerminalCommandOutput::success(lines)
        },
        Err(error) => TerminalCommandOutput::error([
            format!("error: {error}"),
            "usage: [1,2,3] or sum [1,2,3]".to_owned(),
        ]),
    }
}

fn parse_number_list(input: &str) -> Result<Vec<i64>, NumberListError> {
    let trimmed = input.trim();
    let Some(inner) = trimmed
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    else {
        return Err(NumberListError::MissingBrackets);
    };

    if inner.trim().is_empty() {
        return Err(NumberListError::EmptyList);
    }

    inner
        .split(',')
        .map(|part| {
            let value = part.trim();
            if value.is_empty() {
                return Err(NumberListError::EmptyValue);
            }
            value
                .parse::<i64>()
                .map_err(|_| NumberListError::InvalidNumber(value.to_owned()))
        })
        .collect()
}

fn format_numbers(numbers: &[i64]) -> String {
    numbers
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NumberListError {
    MissingBrackets,
    EmptyList,
    EmptyValue,
    InvalidNumber(String),
}

impl fmt::Display for NumberListError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBrackets => write!(formatter, "numbers must be wrapped in []"),
            Self::EmptyList => write!(formatter, "number list must contain at least one value"),
            Self::EmptyValue => write!(formatter, "number list contains an empty value"),
            Self::InvalidNumber(value) => write!(formatter, "`{value}` is not a valid i64"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_list_runs_sum_command() {
        let output = run_terminal_command("[1,2,3]");

        assert_eq!(output.status, TerminalCommandStatus::Success);
        assert!(output.lines.iter().any(|line| line == "sum 6"));
        assert!(output.lines.iter().any(|line| line == "numbers [1, 2, 3]"));
    }

    #[test]
    fn sum_command_accepts_spaces_inside_list() {
        let output = run_terminal_command("sum [1, 2, 3]");

        assert_eq!(output.status, TerminalCommandStatus::Success);
        assert!(output.lines.iter().any(|line| line == "sum 6"));
    }

    #[test]
    fn help_renders_clap_help() {
        let output = run_terminal_command("help");

        assert_eq!(output.status, TerminalCommandStatus::Success);
        assert!(output.lines.iter().any(|line| line.contains("Usage:")));
        assert!(output.lines.iter().any(|line| line.contains("Commands:")));
    }

    #[test]
    fn invalid_list_returns_error() {
        let output = run_terminal_command("[1,nope,3]");

        assert_eq!(output.status, TerminalCommandStatus::Error);
        assert!(
            output
                .lines
                .iter()
                .any(|line| line.contains("not a valid i64"))
        );
    }
}
