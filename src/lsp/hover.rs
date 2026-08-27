use crate::core::conjugator::conjugate_verb;
use crate::core::reference::get_reference_card;
use crate::lsp::protocol::{Hover, MarkupContent, Position};

pub fn compute_hover(content: &str, pos: Position) -> Option<Hover> {
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(pos.line as usize)?;

    // Extract word at position
    let chars: Vec<char> = line.chars().collect();
    let char_idx = pos.character as usize;
    if char_idx > chars.len() {
        return None;
    }

    let mut start = char_idx;
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }

    let mut end = char_idx;
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }

    let word: String = chars[start..end].iter().collect();
    let word_lower = word.to_lowercase();

    // 1. Check for grammar reference topic match
    if let Some(card) = get_reference_card(&word_lower) {
        return Some(Hover {
            contents: MarkupContent {
                kind: "markdown".to_string(),
                value: format!("### Spanglings Grammar Reference: `{}`\n\n{}", word, card),
            },
            range: None,
        });
    }

    // 2. Check for verb conjugation
    if let Some(table) = conjugate_verb(&word_lower) {
        return Some(Hover {
            contents: MarkupContent {
                kind: "markdown".to_string(),
                value: format!(
                    "### Verb Conjugation: `{}`\n\n**Present:**\n- yo: `{}` | tú: `{}` | él/ella: `{}`\n- nosotros: `{}` | ellos/ellas: `{}`\n\n**Preterite:**\n- yo: `{}` | tú: `{}` | él/ella: `{}`\n\n**Subjunctive:**\n- yo: `{}` | tú: `{}` | él/ella: `{}`",
                    table.infinitive,
                    table.present.yo,
                    table.present.tu,
                    table.present.el_ella_usted,
                    table.present.nosotros,
                    table.present.ellos_ellas_ustedes,
                    table.preterite.yo,
                    table.preterite.tu,
                    table.preterite.el_ella_usted,
                    table.present_subjunctive.yo,
                    table.present_subjunctive.tu,
                    table.present_subjunctive.el_ella_usted,
                ),
            },
            range: None,
        });
    }

    // 3. If hovering anywhere on topic line, extract topic
    if line.contains("topic:") {
        if let Some(topic_idx) = line.find("topic:") {
            let topic_slice = &line[topic_idx + 6..];
            let topic = topic_slice
                .split(|c: char| c == '|' || c.is_whitespace() || c == '-')
                .next()
                .unwrap_or("")
                .trim();
            if let Some(card) = get_reference_card(topic) {
                return Some(Hover {
                    contents: MarkupContent {
                        kind: "markdown".to_string(),
                        value: format!("### Spanglings Grammar Reference: `{}`\n\n{}", topic, card),
                    },
                    range: None,
                });
            }
        }
    }

    None
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || "áéíóúÁÉÍÓÚñÑüÜ".contains(c)
}
