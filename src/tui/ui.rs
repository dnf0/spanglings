use crate::core::reference::get_reference_card;
use crate::engine::validator::ValidationResult;
use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
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

    if app.show_tour_welcome {
        let modal_area = centered_rect(65, 40, size);
        draw_tour_welcome_modal(frame, modal_area);
        return;
    }

    if app.show_tour_modal {
        let modal_area = centered_rect(75, 75, size);
        draw_tour_station_modal(frame, app, modal_area);
        return;
    }

    match app.mode {
        crate::tui::app::AppMode::Conjugating => {
            let modal_area = centered_rect(84, 85, size);
            draw_conjugator_modal(frame, app, modal_area);
        }
        crate::tui::app::AppMode::BrowsingReference => {
            let modal_area = centered_rect(88, 88, size);
            draw_reference_browser_modal(frame, app, modal_area);
        }
        crate::tui::app::AppMode::Help => {
            let modal_area = centered_rect(74, 78, size);
            draw_help_modal(frame, app, modal_area);
        }
        crate::tui::app::AppMode::PlacementTest => {
            let modal_area = centered_rect(82, 85, size);
            draw_placement_test_modal(frame, app, modal_area);
        }
        _ => {}
    }
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

    let (topic_title, level_str) = if app.mode == crate::tui::app::AppMode::Searching {
        (
            format!(
                " 🔍 Filter: \"{}\" ({} matches)",
                app.search_query,
                app.filtered_indices.len()
            ),
            "[SEARCH]".to_string(),
        )
    } else {
        match app.current_exercise() {
            Some(ex) => (
                format!(" {} - {}", ex.id, ex.title),
                format!("[{}]", ex.level),
            ),
            None => (" No Exercises Found".to_string(), "[-]".to_string()),
        }
    };

    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if app.mode == crate::tui::app::AppMode::Searching {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
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

    let total_len =
        if app.mode == crate::tui::app::AppMode::Searching || !app.search_query.is_empty() {
            app.filtered_indices.len()
        } else {
            app.exercises.len()
        };
    let current_idx_display = if total_len == 0 {
        0
    } else {
        app.current_index + 1
    };

    let (counter_str, status_span) = if total_len == 0 {
        (
            " 0 / 0 ".to_string(),
            Span::styled(" EMPTY ", Style::default().fg(Color::DarkGray)),
        )
    } else {
        let is_done = app
            .current_exercise()
            .is_some_and(|e| e.is_done || app.state.is_completed(&e.id));
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
        (format!(" {} / {} ", current_idx_display, total_len), status)
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
        let mut in_context_section = false;
        let mut in_instructions_section = false;
        let mut in_exercise_section = false;
        let mut context_lines = Vec::new();
        let mut instructions_lines = Vec::new();
        let mut exercise_lines = Vec::new();

        for line in ex.raw_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("<!--") {
                if in_exercise_section {
                    if trimmed.starts_with("<!-- TODO") || exercise_lines.is_empty() {
                        continue;
                    }
                    break;
                }
                continue;
            }

            if trimmed.starts_with("### Instructions") {
                in_instructions_section = true;
                in_exercise_section = false;
                in_context_section = false;
            } else if trimmed.starts_with("### Exercise") {
                in_exercise_section = true;
                in_instructions_section = false;
                in_context_section = false;
            } else if trimmed.starts_with("### Context")
                || trimmed.starts_with("> **Grammar Rule**")
            {
                in_context_section = true;
                in_instructions_section = false;
                in_exercise_section = false;
                context_lines.push(line);
            } else if in_instructions_section {
                if trimmed.starts_with('#') {
                    in_instructions_section = false;
                } else if !trimmed.is_empty() {
                    instructions_lines.push(line);
                }
            } else if in_exercise_section {
                if trimmed.starts_with('#') {
                    break;
                }
                if !trimmed.is_empty() {
                    exercise_lines.push(line);
                }
            } else if (in_context_section
                || trimmed.starts_with('>')
                || trimmed.starts_with("English:"))
                && !trimmed.is_empty()
            {
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

        if !instructions_lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "Instructions (TODO & Why):",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::UNDERLINED),
            )));
            for il in instructions_lines {
                let trimmed = il.trim();
                if let Some(rest) = trimmed.strip_prefix("**TODO**:") {
                    lines.push(Line::from(vec![
                        Span::styled(
                            "  TODO:",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!(" {}", rest.trim()),
                            Style::default().fg(Color::White),
                        ),
                    ]));
                } else if let Some(rest) = trimmed.strip_prefix("**Why**:") {
                    lines.push(Line::from(vec![
                        Span::styled(
                            "  Why: ",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!(" {}", rest.trim()),
                            Style::default().fg(Color::White),
                        ),
                    ]));
                } else {
                    lines.push(Line::from(Span::styled(
                        format!("  {}", trimmed),
                        Style::default().fg(Color::White),
                    )));
                }
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

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let shortcuts = match app.mode {
        crate::tui::app::AppMode::Searching => Line::from(vec![
            Span::styled(
                " [Type] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Query  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Enter] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Select  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Up/Down] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Navigate  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Esc] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Cancel Search ", Style::default().fg(Color::White)),
        ]),
        crate::tui::app::AppMode::Conjugating => Line::from(vec![
            Span::styled(
                " [Type] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Verb  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Up/Down] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Scroll Table  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Esc / c] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Close Conjugator ", Style::default().fg(Color::White)),
        ]),
        crate::tui::app::AppMode::BrowsingReference => Line::from(vec![
            Span::styled(
                " [Type] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Filter  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Up/Down] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Select Topic  ", Style::default().fg(Color::White)),
            Span::styled(
                " [PgUp/PgDn] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Scroll Card  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Esc / r] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Close Reference ", Style::default().fg(Color::White)),
        ]),
        crate::tui::app::AppMode::Help => Line::from(vec![
            Span::styled(
                " [Esc / ? / F1] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Close Help Overlay ", Style::default().fg(Color::White)),
        ]),
        crate::tui::app::AppMode::PlacementTest => Line::from(vec![
            Span::styled(
                " [Type] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Answer  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Enter] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Submit / Next  ", Style::default().fg(Color::White)),
            Span::styled(
                " [F] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Fast-Track  ", Style::default().fg(Color::White)),
            Span::styled(
                " [Esc] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Exit Placement ", Style::default().fg(Color::White)),
        ]),
        crate::tui::app::AppMode::Editing => Line::from(vec![
            Span::styled(
                " [/] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Search ", Style::default().fg(Color::White)),
            Span::styled(
                " [c] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Conjugate ", Style::default().fg(Color::White)),
            Span::styled(
                " [r] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Ref ", Style::default().fg(Color::White)),
            Span::styled(
                " [T] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Tour ", Style::default().fg(Color::White)),
            Span::styled(
                " [?] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Help ", Style::default().fg(Color::White)),
            Span::styled(
                " [Ctrl-H] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Hint ", Style::default().fg(Color::White)),
            Span::styled(
                " [Tab] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Next ", Style::default().fg(Color::White)),
            Span::styled(
                " [Enter] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Submit ", Style::default().fg(Color::White)),
            Span::styled(
                " [Esc] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit ", Style::default().fg(Color::White)),
        ]),
    };

    let footer_para = Paragraph::new(shortcuts)
        .block(footer_block)
        .alignment(Alignment::Center);
    frame.render_widget(footer_para, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_conjugator_modal(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(Clear, area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 📖 Live Spanish Verb Conjugator (Esc or [c] to close) ")
        .border_style(Style::default().fg(Color::Yellow));
    frame.render_widget(modal_block.clone(), area);

    let inner_area = modal_block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(5),    // Table details
        ])
        .split(inner_area);

    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Infinitive Verb (e.g. tener, hablar, ser, hacer) ")
        .border_style(Style::default().fg(Color::Cyan));

    let cursor_display = if app.conjugator_cursor < app.conjugator_query.chars().count() {
        let byte_idx = app
            .conjugator_query
            .char_indices()
            .nth(app.conjugator_cursor)
            .map(|(i, _)| i)
            .unwrap_or(app.conjugator_query.len());
        let (b, rest) = app.conjugator_query.split_at(byte_idx);
        let mut chars = rest.chars();
        let c = chars.next().unwrap_or(' ');
        let rem = chars.as_str();
        vec![
            Span::styled(
                b,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                c.to_string(),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                rem,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    } else {
        vec![
            Span::styled(
                &app.conjugator_query,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        ]
    };

    let search_para = Paragraph::new(Line::from(cursor_display)).block(search_block);
    frame.render_widget(search_para, chunks[0]);

    if let Some(ref table) = app.conjugator_table {
        let table_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(
                " Conjugations for: {} ({}) ",
                table.infinitive.to_uppercase(),
                table.english
            ))
            .border_style(Style::default().fg(Color::Green));

        let mut lines = Vec::new();
        lines.push(Line::from(vec![
            Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if table.is_irregular {
                    "Irregular Stem"
                } else {
                    "Regular Verb"
                },
                Style::default()
                    .fg(if table.is_irregular {
                        Color::LightRed
                    } else {
                        Color::Green
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Gerund: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&table.gerund, Style::default().fg(Color::Yellow)),
            Span::styled("   Participle: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&table.participle, Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(""));

        let fmt_t = |name: &str, forms: &crate::core::conjugator::PronounForms| -> Vec<Line> {
            vec![
                Line::from(Span::styled(
                    format!("── {} ──", name),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(vec![
                    Span::styled("yo: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:<14}", forms.yo),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("tú: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:<14}", forms.tu),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("él/ella: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:<14}", forms.el_ella_usted),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("nosotros: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:<14}", forms.nosotros),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("vosotros: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:<14}", forms.vosotros),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("ellos/ellas: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:<14}", forms.ellos_ellas_ustedes),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
            ]
        };

        lines.extend(fmt_t("Present Indicative", &table.present));
        lines.extend(fmt_t("Preterite (Indefinido)", &table.preterite));
        lines.extend(fmt_t("Imperfect (Copretérito)", &table.imperfect));
        lines.extend(fmt_t("Future (Futuro Simple)", &table.future));
        lines.extend(fmt_t("Conditional (Pospretérito)", &table.conditional));
        lines.extend(fmt_t("Present Subjunctive", &table.present_subjunctive));
        lines.extend(fmt_t(
            "Imperfect Subjunctive (-ra)",
            &table.imperfect_subjunctive_ra,
        ));
        lines.extend(fmt_t(
            "Imperfect Subjunctive (-se)",
            &table.imperfect_subjunctive_se,
        ));

        lines.push(Line::from(Span::styled(
            "── Imperatives ──",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(vec![
            Span::styled(
                "Affirmative (tú/usted/nosotros/vosotros/ustedes): ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                format!(
                    "{}, {}, {}, {}, {}",
                    table.imperative_affirmative.tu,
                    table.imperative_affirmative.usted,
                    table.imperative_affirmative.nosotros,
                    table.imperative_affirmative.vosotros,
                    table.imperative_affirmative.ustedes
                ),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                "Negative (tú/usted/nosotros/vosotros/ustedes):    ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                format!(
                    "{}, {}, {}, {}, {}",
                    table.imperative_negative.tu,
                    table.imperative_negative.usted,
                    table.imperative_negative.nosotros,
                    table.imperative_negative.vosotros,
                    table.imperative_negative.ustedes
                ),
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        let table_para = Paragraph::new(lines)
            .block(table_block)
            .scroll((app.conjugator_scroll as u16, 0))
            .wrap(Wrap { trim: false });
        frame.render_widget(table_para, chunks[1]);
    } else {
        let empty_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Conjugation Matrix ")
            .border_style(Style::default().fg(Color::DarkGray));

        let msg = if app.conjugator_query.trim().is_empty() {
            "Type an infinitive (e.g. 'tener', 'hablar', 'ser', 'hacer', 'ir', 'escribir') in the search box above."
        } else {
            "No direct conjugation found. Type a valid Spanish verb ending in -ar, -er, or -ir."
        };

        let empty_para = Paragraph::new(msg)
            .block(empty_block)
            .alignment(Alignment::Center);
        frame.render_widget(empty_para, chunks[1]);
    }
}

fn draw_reference_browser_modal(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(Clear, area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 📚 Grammar Reference Browser (Esc or [r] to close) ")
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(modal_block.clone(), area);

    let inner_area = modal_block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Filter
            Constraint::Min(5),    // Topics + Card Split
        ])
        .split(inner_area);

    let filter_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Filter Topics (e.g. subjunctive, por-para, accents) ")
        .border_style(Style::default().fg(Color::Yellow));

    let cursor_display = if app.ref_cursor < app.ref_query.chars().count() {
        let byte_idx = app
            .ref_query
            .char_indices()
            .nth(app.ref_cursor)
            .map(|(i, _)| i)
            .unwrap_or(app.ref_query.len());
        let (b, rest) = app.ref_query.split_at(byte_idx);
        let mut chars = rest.chars();
        let c = chars.next().unwrap_or(' ');
        let rem = chars.as_str();
        vec![
            Span::styled(
                b,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                c.to_string(),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                rem,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    } else {
        vec![
            Span::styled(
                &app.ref_query,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default().fg(Color::Black).bg(Color::Yellow)),
        ]
    };

    let filter_para = Paragraph::new(Line::from(cursor_display)).block(filter_block);
    frame.render_widget(filter_para, chunks[0]);

    let browser_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(30)])
        .split(chunks[1]);

    let topics_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(" Topics ({}) ", app.ref_filtered_topics.len()))
        .border_style(Style::default().fg(Color::DarkGray));

    let topic_lines: Vec<Line> = if app.ref_filtered_topics.is_empty() {
        vec![Line::from(Span::styled(
            "No topics match query",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        app.ref_filtered_topics
            .iter()
            .enumerate()
            .map(|(idx, topic)| {
                if idx == app.ref_selected_idx {
                    Line::from(vec![
                        Span::styled(
                            " ▶ ",
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{:<18}", topic),
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled("   ", Style::default().fg(Color::DarkGray)),
                        Span::styled(*topic, Style::default().fg(Color::White)),
                    ])
                }
            })
            .collect()
    };

    let topics_para = Paragraph::new(topic_lines).block(topics_block);
    frame.render_widget(topics_para, browser_chunks[0]);

    let selected_topic = app
        .ref_filtered_topics
        .get(app.ref_selected_idx)
        .copied()
        .unwrap_or("subjunctive");
    let card_content =
        get_reference_card(selected_topic).unwrap_or("No reference content found for this topic.");

    let card_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(" Cheat Sheet: {} ", selected_topic))
        .border_style(Style::default().fg(Color::Cyan));

    let card_lines: Vec<Line> = card_content
        .lines()
        .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::LightCyan))))
        .collect();

    let card_para = Paragraph::new(card_lines)
        .block(card_block)
        .scroll((app.ref_scroll as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(card_para, browser_chunks[1]);
}

fn draw_help_modal(frame: &mut Frame, _app: &App, area: Rect) {
    frame.render_widget(Clear, area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 💡 Keyboard Shortcuts & Power Tools (Esc, ?, or F1 to close) ")
        .border_style(Style::default().fg(Color::Magenta));

    let help_text = vec![
        Line::from(Span::styled(
            "Navigation & Exercise Control",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(vec![
            Span::styled(
                "  [Tab] / [Ctrl-N]      ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Advance to the next exercise",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [BackTab] / [Ctrl-P]  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Return to the previous exercise",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [Ctrl-R]              ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Reset current exercise state",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "In-TUI Modals & Power Tools",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(vec![
            Span::styled(
                "  [c]                   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Open interactive live Verb Conjugator popup",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [r]                   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Open full Grammar Reference Browser & Cheat Sheets",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [p] / [F5]            ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Launch Diagnostic Placement Test & Level Fast-Track",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [T] / [F6]            ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Launch Interactive Onboarding Guided Tour",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [/]                   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Filter and search exercises by ID, topic, or level",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [?] / [F1]            ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Toggle this keyboard shortcut help overlay",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Hints & Diagnostics",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(vec![
            Span::styled(
                "  [Ctrl-H]              ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Toggle 3-tiered progressive hints in workspace",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [Ctrl-E] / [F2]       ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Toggle topic grammar reference card in workspace",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [Enter]               ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Submit and evaluate answer with rustc-style diagnostics",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "General",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(vec![
            Span::styled(
                "  [Esc]                 ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Dismiss active modal / Exit application",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  [Ctrl-C] / [Ctrl-Q]   ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Force quit Spanglings TUI",
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let help_para = Paragraph::new(help_text)
        .block(modal_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(help_para, area);
}

fn draw_placement_test_modal(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(Clear, area);

    if !app.placement_finished {
        let total = app.placement_battery.len();
        let current_num = (app.placement_current_idx + 1).min(total);
        let q_opt = app.placement_battery.get(app.placement_current_idx);

        let modal_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(
                " 🎯 Diagnostic Placement Test ({}/{}) — [Esc to Exit] ",
                current_num, total
            ))
            .border_style(Style::default().fg(Color::Cyan));
        frame.render_widget(modal_block.clone(), area);

        let inner_area = modal_block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Progress & Tier
                Constraint::Length(4), // English Context
                Constraint::Length(5), // Spanish Cloze Prompt
                Constraint::Length(3), // User Input
                Constraint::Length(2), // Hotkey instructions
            ])
            .split(inner_area);

        if let Some(q) = q_opt {
            let pct = if total > 0 {
                (app.placement_current_idx as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            let progress_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray));
            let progress_text = Paragraph::new(Line::from(vec![
                Span::styled(" CEFR Tier: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("[{:?}]", q.level),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        "   |   Progress: {:.0}% ({}/{})",
                        pct, app.placement_current_idx, total
                    ),
                    Style::default().fg(Color::Cyan),
                ),
            ]))
            .block(progress_block);
            frame.render_widget(progress_text, chunks[0]);

            let ctx_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" English Context & Objective ")
                .border_style(Style::default().fg(Color::White));
            let ctx_para = Paragraph::new(Line::from(Span::styled(
                format!(" {}", q.context_en),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            )))
            .block(ctx_block);
            frame.render_widget(ctx_para, chunks[1]);

            let prompt_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Spanish Prompt (Fill in the blank ___) ")
                .border_style(Style::default().fg(Color::Yellow));
            let prompt_para = Paragraph::new(Line::from(Span::styled(
                format!(" {}", q.prompt_es),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )))
            .block(prompt_block)
            .wrap(Wrap { trim: false });
            frame.render_widget(prompt_para, chunks[2]);

            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Your Answer ")
                .border_style(Style::default().fg(Color::Green));

            let cursor_display = if app.placement_cursor < app.placement_input.chars().count() {
                let byte_idx = app
                    .placement_input
                    .char_indices()
                    .nth(app.placement_cursor)
                    .map(|(i, _)| i)
                    .unwrap_or(app.placement_input.len());
                let (b, rest) = app.placement_input.split_at(byte_idx);
                let mut chars = rest.chars();
                let c = chars.next().unwrap_or(' ');
                let rem = chars.as_str();
                vec![
                    Span::styled(
                        format!(" {}", b),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        c.to_string(),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        rem,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]
            } else {
                vec![
                    Span::styled(
                        format!(" {}", app.placement_input),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" ", Style::default().fg(Color::Black).bg(Color::Green)),
                ]
            };

            let input_para = Paragraph::new(Line::from(cursor_display)).block(input_block);
            frame.render_widget(input_para, chunks[3]);

            let footer_text = Paragraph::new(Line::from(vec![
                Span::styled(
                    " [Enter] ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Submit & Next Question   ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    " [Esc] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Cancel Placement Test",
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
            .alignment(Alignment::Center);
            frame.render_widget(footer_text, chunks[4]);
        }
    } else {
        // Placement Results Screen
        let modal_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" 🏆 CEFR Diagnostic Placement Results ")
            .border_style(Style::default().fg(Color::Green));
        frame.render_widget(modal_block.clone(), area);

        let inner_area = modal_block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Assessed Level Banner
                Constraint::Min(8),    // Breakdown Table
                Constraint::Length(3), // Action bar
            ])
            .split(inner_area);

        if let Some(res) = &app.placement_result {
            let banner_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow));

            let banner_text = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled(" Assessed CEFR Level: ", Style::default().fg(Color::White)),
                    Span::styled(
                        format!(" [{:?}] ", res.assessed_level),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled(" Diagnostic Accuracy: ", Style::default().fg(Color::White)),
                    Span::styled(
                        format!(
                            "{:.1}% ({} of {} correct)",
                            res.percentage, res.total_correct, res.total_questions
                        ),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
            ])
            .block(banner_block);
            frame.render_widget(banner_text, chunks[0]);

            let mut breakdown_lines = vec![
                Line::from(Span::styled(
                    "CEFR Level Mastery Breakdown:",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )),
                Line::from(""),
            ];

            for lvl in [
                crate::core::curriculum::Level::Baseline,
                crate::core::curriculum::Level::B1,
                crate::core::curriculum::Level::B2,
                crate::core::curriculum::Level::C1,
            ] {
                if let Some(&(correct, total)) = res.scores_by_level.get(&lvl) {
                    let pct = if total > 0 {
                        (correct as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };
                    let (status_str, status_style) = if pct >= 75.0 {
                        (
                            "PASS (Mastered)",
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ("IN PROGRESS", Style::default().fg(Color::Yellow))
                    };

                    breakdown_lines.push(Line::from(vec![
                        Span::styled(
                            format!("  • {:<10} ", format!("{:?}", lvl)),
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{:>2}/{} ({:>5.1}%)  ", correct, total, pct),
                            Style::default().fg(Color::Cyan),
                        ),
                        Span::styled(format!("[{}]", status_str), status_style),
                    ]));
                }
            }

            let breakdown_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray));
            let breakdown_para = Paragraph::new(breakdown_lines).block(breakdown_block);
            frame.render_widget(breakdown_para, chunks[1]);

            let action_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan));

            let action_text = if app.placement_fast_tracked {
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        " ✨ Level exercises fast-tracked! ",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " [Enter/Esc] Close Modal ",
                        Style::default().fg(Color::White),
                    ),
                ]))
                .block(action_block)
                .alignment(Alignment::Center)
            } else if !res.passed_levels.is_empty() {
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        " [F] ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "Fast-Track & Auto-Complete Mastered Levels   ",
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        " [Enter/Esc] ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("Close Modal", Style::default().fg(Color::DarkGray)),
                ]))
                .block(action_block)
                .alignment(Alignment::Center)
            } else {
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        " [Enter/Esc] ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "Close Modal & Begin Recommended Tracks",
                        Style::default().fg(Color::White),
                    ),
                ]))
                .block(action_block)
                .alignment(Alignment::Center)
            };

            frame.render_widget(action_text, chunks[2]);
        }
    }
}

fn draw_tour_welcome_modal(frame: &mut Frame, area: Rect) {
    frame.render_widget(Clear, area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Welcome to Spanglings! ")
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(modal_block.clone(), area);

    let inner_area = modal_block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Greeting headline
            Constraint::Min(4),    // Description
            Constraint::Length(3), // Question & prompt action
        ])
        .split(inner_area);

    let headline = Paragraph::new(Line::from(vec![Span::styled(
        " 🇪🇸 ¡Bienvenido a Spanglings! ",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(headline, chunks[0]);

    let body_text = vec![
        Line::from(Span::styled(
            "Spanglings is an active-recall, developer-grade learning environment for Spanish (B1-C1).",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Master grammar rules through compiler-style diagnostics, spaced repetition (SRS), and hands-on drills.",
            Style::default().fg(Color::LightCyan),
        )),
    ];
    let body_para = Paragraph::new(body_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(body_para, chunks[1]);

    let prompt_line = Paragraph::new(Line::from(vec![
        Span::styled(
            "Would you like to take the quick interactive tour? ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " [Y]es ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            " [N]o ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(prompt_line, chunks[2]);
}

fn draw_tour_station_modal(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(Clear, area);

    let stations = crate::cli::commands::tour::get_tour_stations();
    let total = stations.len();
    let station_idx = app.tour_current_station.min(total.saturating_sub(1));
    let station = stations.get(station_idx);

    let title = format!(
        " ✦ Interactive Onboarding Tour ({}/{}) ",
        station_idx + 1,
        total
    );

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(modal_block.clone(), area);

    let inner_area = modal_block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header & Subtitle
            Constraint::Min(6),    // Description, Bullets, Examples / Challenge
            Constraint::Length(3), // Navigation footer
        ])
        .split(inner_area);

    if let Some(st) = station {
        // Station Header & Subtitle
        let header_lines = vec![
            Line::from(vec![
                Span::styled(
                    format!(" [Station {}/{}] ", station_idx + 1, total),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    &st.title,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(
                    &st.subtitle,
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]),
        ];
        let header_para = Paragraph::new(header_lines);
        frame.render_widget(header_para, chunks[0]);

        // Main content
        let mut content_lines = Vec::new();
        content_lines.push(Line::from(Span::styled(
            &st.description,
            Style::default().fg(Color::White),
        )));
        content_lines.push(Line::from(""));

        for point in &st.bullet_points {
            content_lines.push(Line::from(vec![
                Span::styled(
                    "  • ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(point, Style::default().fg(Color::LightCyan)),
            ]));
        }

        if let Some(cmd) = &st.simulated_command {
            content_lines.push(Line::from(""));
            content_lines.push(Line::from(vec![
                Span::styled(
                    "  Example Command: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("$ {}", cmd), Style::default().fg(Color::Green)),
            ]));
        }

        if let Some(out) = &st.simulated_output {
            content_lines.push(Line::from(Span::styled(
                "  Output:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
            for line in out.lines() {
                content_lines.push(Line::from(Span::styled(
                    format!("    {}", line),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }

        if let Some(challenge) = &st.challenge {
            content_lines.push(Line::from(""));
            content_lines.push(Line::from(vec![
                Span::styled(
                    "  Challenge: ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    &challenge.prompt,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if let Some(tip) = &challenge.tip {
                content_lines.push(Line::from(vec![
                    Span::styled("  Tip: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        tip,
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]));
            }
            content_lines.push(Line::from(vec![
                Span::styled("  Expected Answer: ", Style::default().fg(Color::Green)),
                Span::styled(
                    &challenge.expected_input,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" — {}", challenge.explanation),
                    Style::default().fg(Color::Gray),
                ),
            ]));
        }

        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));
        let content_para = Paragraph::new(content_lines)
            .block(content_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(content_para, chunks[1]);

        // Navigation Footer
        let is_last = station_idx + 1 >= total;
        let next_label = if is_last { "Finish Tour" } else { "Next" };
        let footer_spans = vec![
            Span::styled(
                " [← / p] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Previous  |  ", Style::default().fg(Color::White)),
            Span::styled(
                " [→ / n / Enter] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" {}  |  ", next_label),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                " [Esc / q] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Close ", Style::default().fg(Color::White)),
        ];

        let footer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));
        let footer_para = Paragraph::new(Line::from(footer_spans))
            .block(footer_block)
            .alignment(Alignment::Center);
        frame.render_widget(footer_para, chunks[2]);
    }
}
