use crate::engine::normalizer::normalize;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccentResult {
    ExactMatch,
    ForgivenMatch { expected: String, tip: String },
    Mismatch,
}

pub fn strip_accents(s: &str) -> String {
    s.nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect::<String>()
        .nfc()
        .collect::<String>()
}

pub fn check_accent_match(user_input: &str, target: &str, strict: bool) -> AccentResult {
    let norm_user = normalize(user_input);
    let norm_target = normalize(target);

    if norm_user == norm_target {
        AccentResult::ExactMatch
    } else if norm_user.to_lowercase() == norm_target.to_lowercase() {
        // If they differ only in case, let's treat it as an ExactMatch for accent checking
        AccentResult::ExactMatch
    } else if strip_accents(&norm_user).to_lowercase() == strip_accents(&norm_target).to_lowercase()
    {
        if strict {
            AccentResult::Mismatch
        } else {
            let expected = norm_target;
            let tip = format!("Accent tip: '{}' has an accent mark.", expected);
            AccentResult::ForgivenMatch { expected, tip }
        }
    } else {
        AccentResult::Mismatch
    }
}
