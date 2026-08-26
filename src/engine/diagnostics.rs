use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: String,
    pub title: String,
    pub file_path: String,
    pub line_number: usize,
    pub user_snippet: String,
    pub message: String,
    pub note: Option<String>,
    pub help: Option<String>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn format_terminal(&self) -> String {
        let err_code = format!("error[{}]", self.code).red().bold();
        let title_label = self.title.bold();
        let arrow_color = "-->".blue().bold();
        let pipe_color = "|".blue().bold();
        let line_str = format!("{}", self.line_number);
        let line_padding = " ".repeat(line_str.len());

        let underline_len = if self.user_snippet.is_empty() {
            1
        } else {
            self.user_snippet.len()
        };
        let underline = "^".repeat(underline_len).red().bold();

        let mut lines = Vec::new();
        lines.push(format!("{}: {}", err_code, title_label));
        lines.push(format!(
            "{} {}:{}:{}",
            " ".repeat(line_str.len() + 1),
            arrow_color,
            self.file_path,
            self.line_number
        ));
        lines.push(format!("{} {}", line_padding, pipe_color));
        lines.push(format!(
            " {} {} {}",
            line_str, pipe_color, self.user_snippet
        ));
        lines.push(format!(
            "{} {} {} {}",
            line_padding, pipe_color, underline, self.message
        ));

        if let Some(ref note) = self.note {
            lines.push(format!(
                "{} {} {}: {}",
                line_padding,
                "=".blue().bold(),
                "note".bold(),
                note
            ));
        }
        if let Some(ref help) = self.help {
            lines.push(format!(
                "{} {} {}: {}",
                line_padding,
                "=".blue().bold(),
                "help".bold(),
                help
            ));
        }
        if let Some(ref hint) = self.hint {
            lines.push(format!(
                "{} {} {}: {}",
                line_padding,
                "=".blue().bold(),
                "hint".bold(),
                hint
            ));
        }

        lines.join("\n")
    }
}
