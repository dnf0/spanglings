use crate::core::reference::get_reference_card;
use crate::engine::validator::ValidationResult;
use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Main vertical layout: Header, Main Area, Footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main Workspace
            Constraint::Length(3), // Footer
        ])
        .split(size);

    draw_header(frame, app, main_chunks[0]);
    draw_workspace(frame, app, main_chunks[1]);
    draw_footer(frame, app, main_chunks[2]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22), // App Title
            Constraint::Min(20),    // Exercise Info
            Constraint::Length(24), // Counter & Status
        ])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));
    let title_text = Paragraph::new(Line::from(vec![Span::styled(
        " 🇪🇸 SPANGLINGS ",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(title_block)
    .alignment(Alignment::Center);
    frame.render_widget(title_text, header_chunks[0]);

    let (topic_title, level_str) = match app.current_exercise() {
        Some(ex) => (
            format!(" {} - {}", ex.id, ex.title),
            format!("[{}]", ex.level),
        ),
        None => (" No Exercises Found".to_string(), "[-]".to_string()),
    };

    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));
    let info_text = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {} ", level_str),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            topic_title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(info_block);
    frame.render_widget(info_text, header_chunks[1]);

    let (counter_str, status_span) = if app.exercises.is_empty() {
        (
            " 0 / 0 ".to_string(),
            Span::styled(" EMPTY ", Style::default().fg(Color::DarkGray)),
        )
    } else {
        let is_done = app.current_exercise().is_some_and(|e| e.is_done);
        let status = if is_done {
            Span::styled(
                " ✓ DONE ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                " ● PENDING ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        };
        (
            format!(" {} / {} ", app.current_index + 1, app.exercises.len()),
            status,
        )
    };

    let status_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));
    let status_text = Paragraph::new(Line::from(vec![
        Span::styled(counter_str, Style::default().fg(Color::Cyan)),
        status_span,
    ]))
    .block(status_block)
    .alignment(Alignment::Right);
    frame.render_widget(status_text, header_chunks[2]);
}

fn draw_workspace(frame: &mut Frame, app: &App, area: Rect) {
    if app.exercises.is_empty() {
        let empty_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Welcome ");
        let msg = Paragraph::new(
            "No exercises found. Add markdown exercises to 'exercises/' directory to get started.",
        )
        .block(empty_block)
        .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let is_wide = area.width >= 90;

    if is_wide {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
            .split(area);

        draw_left_pane(frame, app, cols[0]);
        draw_right_pane(frame, app, cols[1]);
    } else {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        draw_left_pane(frame, app, rows[0]);
        draw_right_pane(frame, app, rows[1]);
    }
}

fn draw_left_pane(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(6)])
        .split(area);

    let Some(ex) = app.current_exercise() else {
        return;
    };

    // Prompt / Exercise Card
    let prompt_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(" Exercise: {} ({}) ", ex.title, ex.exercise_type))
        .border_style(Style::default().fg(Color::Blue));

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("Topic: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&ex.topic, Style::default().fg(Color::Cyan)),
        Span::styled("  |  Level: ", Style::default().fg(Color::DarkGray)),
        Span::styled(ex.level.to_string(), Style::default().fg(Color::Magenta)),
    ]));
    lines.push(Line::from(""));

    // Extract markdown description from in-memory raw_content
    if !ex.raw_content.is_empty() {
        let mut in_exercise_section = false;
        let mut context_lines = Vec::new();
        let mut exercise_lines = Vec::new();

        for line in ex.raw_content.lines() {
            if line.starts_with("<!--") {
                continue;
            }
            if line.starts_with("### Context") || line.starts_with("> **Grammar Rule**") {
                context_lines.push(line);
            } else if line.starts_with("### Exercise") {
                in_exercise_section = true;
            } else if in_exercise_section {
                if line.starts_with("<!--") {
                    break;
                }
                if !line.trim().is_empty() {
                    exercise_lines.push(line);
                }
            } else if line.starts_with('>') || line.starts_with("English:") {
                context_lines.push(line);
            }
        }

        if !context_lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "Context & Grammar:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            )));
            for cl in context_lines {
                lines.push(Line::from(Span::styled(
                    format!("  {}", cl),
                    Style::default().fg(Color::Gray),
                )));
            }
            lines.push(Line::from(""));
        }

        if !exercise_lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "Sentence:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::UNDERLINED),
            )));
            for el in exercise_lines {
                lines.push(Line::from(Span::styled(
                    format!("  {}", el),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
            }
        } else {
            lines.push(Line::from(Span::styled(
                format!("Solve: {}", ex.solution),
                Style::default().fg(Color::DarkGray),
            )));
        }
    } else {
        lines.push(Line::from(Span::styled(
            format!("Prompt: (from {})", ex.path.display()),
            Style::default().fg(Color::White),
        )));
    }

    let prompt_para = Paragraph::new(lines)
        .block(prompt_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(prompt_para, chunks[0]);

    // Input Card
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Your Answer (Type & Press Enter) ")
        .border_style(Style::default().fg(Color::Yellow));

    let total_chars = app.input_buffer.chars().count();
    let (before_cursor, cursor_char, remaining) = if app.cursor_position < total_chars {
        let byte_idx = app
            .input_buffer
            .char_indices()
            .nth(app.cursor_position)
            .map(|(idx, _)| idx)
            .unwrap_or(app.input_buffer.len());
        let (b, rest) = app.input_buffer.split_at(byte_idx);
        let mut chars = rest.chars();
        let c = chars.next().unwrap_or(' ');
        let rem = chars.as_str();
        (b, c, rem)
    } else {
        (app.input_buffer.as_str(), ' ', "")
    };

    let input_spans = vec![
        Span::styled(
            " > ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            before_cursor,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            cursor_char.to_string(),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            remaining,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let input_para = Paragraph::new(Line::from(input_spans))
        .block(input_block)
        .alignment(Alignment::Left);
    frame.render_widget(input_para, chunks[1]);
}

fn draw_right_pane(frame: &mut Frame, app: &App, area: Rect) {
    let Some(ex) = app.current_exercise() else {
        return;
    };

    if app.show_reference {
        // Grammar Cheat Sheet Card
        let card_content = get_reference_card(&ex.topic).unwrap_or(
            "No reference card available for this topic. Use '[E]' to view standard topics.",
        );

        let ref_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" Grammar Reference: {} ", ex.topic))
            .border_style(Style::default().fg(Color::Cyan));

        let lines: Vec<Line> = card_content
            .lines()
            .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::LightCyan))))
            .collect();

        let ref_para = Paragraph::new(lines)
            .block(ref_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(ref_para, area);
        return;
    }

    if app.show_hint {
        // Tiered Hints Card
        let hint_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Grammatical Hints ")
            .border_style(Style::default().fg(Color::Yellow));

        let mut lines = Vec::new();
        if ex.hints.is_empty() {
            lines.push(Line::from(Span::styled(
                "No hints specified for this exercise.",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (i, hint) in ex.hints.iter().enumerate() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!(" Tier {}: ", i + 1),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hint, Style::default().fg(Color::White)),
                ]));
                lines.push(Line::from(""));
            }
        }

        let hint_para = Paragraph::new(lines)
            .block(hint_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(hint_para, area);
        return;
    }

    // Default or Validation Result Card
    match &app.last_result {
        Some(ValidationResult::Passed { notice }) => {
            let pass_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Result: Passed! 🎉 ")
                .border_style(Style::default().fg(Color::Green));

            let mut lines = vec![
                Line::from(vec![
                    Span::styled(
                        " ✅ ¡Excelente! ",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("'{}' is correct.", ex.solution),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::from(""),
            ];

            if let Some(note) = notice {
                lines.push(Line::from(vec![
                    Span::styled(
                        " 💡 Accent Tip: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(note, Style::default().fg(Color::Yellow)),
                ]));
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(
                "Press [Tab] or [Ctrl-N] to advance to the next exercise.",
                Style::default().fg(Color::Cyan),
            )));

            let pass_para = Paragraph::new(lines)
                .block(pass_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(pass_para, area);
        }
        Some(ValidationResult::Failed { diagnostic, .. }) => {
            let fail_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(format!(" Diagnostic Error [{}] ", diagnostic.code))
                .border_style(Style::default().fg(Color::Red));

            let mut lines = vec![
                Line::from(vec![
                    Span::styled(
                        format!(" error[{}]: ", diagnostic.code),
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        &diagnostic.title,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Message: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&diagnostic.message, Style::default().fg(Color::Red)),
                ]),
            ];

            if let Some(help) = &diagnostic.help {
                lines.push(Line::from(vec![
                    Span::styled(
                        "  Help: ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(help, Style::default().fg(Color::Cyan)),
                ]));
            }

            if let Some(hint) = &diagnostic.hint {
                lines.push(Line::from(vec![
                    Span::styled(
                        "  Hint: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hint, Style::default().fg(Color::Yellow)),
                ]));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Press [Ctrl-H] / [F1] for hints or [Ctrl-E] / [F2] for grammar rules.",
                Style::default().fg(Color::DarkGray),
            )));

            let fail_para = Paragraph::new(lines)
                .block(fail_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(fail_para, area);
        }
        None => {
            let idle_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Guidance & Tips ")
                .border_style(Style::default().fg(Color::DarkGray));

            let lines = vec![
                Line::from(Span::styled(
                    "Spanish Conjugation & Grammar Assistant",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("• Type your answer in the prompt and press [Enter] to validate."),
                Line::from("• Conjugate irregular stems and apply mood triggers carefully."),
                Line::from("• Press [Ctrl-H] or [F1] to view progressive 3-tiered hints."),
                Line::from("• Press [Ctrl-E] or [F2] to read the in-terminal grammar cheat sheet."),
                Line::from("• Press [Tab] / [Ctrl-N] for Next, [BackTab] / [Ctrl-P] for Prev."),
                Line::from("• Press [Esc] or [Ctrl-C] to quit."),
            ];

            let idle_para = Paragraph::new(lines)
                .block(idle_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(idle_para, area);
        }
    }
}

fn draw_footer(frame: &mut Frame, _app: &App, area: Rect) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let shortcuts = Line::from(vec![
        Span::styled(
            " [Enter] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Submit  ", Style::default().fg(Color::White)),
        Span::styled(
            " [Ctrl-H / F1] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Hint  ", Style::default().fg(Color::White)),
        Span::styled(
            " [Ctrl-E / F2] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Explain  ", Style::default().fg(Color::White)),
        Span::styled(
            " [Tab / Ctrl-N] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Next  ", Style::default().fg(Color::White)),
        Span::styled(
            " [BackTab / Ctrl-P] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Prev  ", Style::default().fg(Color::White)),
        Span::styled(
            " [Ctrl-R] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Reset  ", Style::default().fg(Color::White)),
        Span::styled(
            " [Esc / Ctrl-C] ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Quit ", Style::default().fg(Color::White)),
    ]);

    let footer_para = Paragraph::new(shortcuts)
        .block(footer_block)
        .alignment(Alignment::Center);
    frame.render_widget(footer_para, area);
}
