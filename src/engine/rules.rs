// Registry of global Spanish rule triggers
pub const CODE_GENERAL_MISMATCH: &str = "E0001";
pub const CODE_SER_ESTAR_STATE: &str = "E0101";
pub const CODE_SER_ESTAR_MEANING: &str = "E0102";
pub const CODE_PAST_ASPECT_PRETERITE: &str = "E0201";
pub const CODE_PAST_ASPECT_IMPERFECT: &str = "E0202";
pub const CODE_PAST_MEANING_CHANGE: &str = "E0203";
pub const CODE_SUBJUNCTIVE_WEIRDO: &str = "E0301";
pub const CODE_SUBJUNCTIVE_RELATIVE: &str = "E0401";
pub const CODE_SUBJUNCTIVE_CONJUNCTION: &str = "E0501";
pub const CODE_CONDITIONALS: &str = "E0601";
pub const CODE_POR_PARA: &str = "E0701";
pub const CODE_CLITIC_STACKING: &str = "E0801";
pub const CODE_CLITIC_ACCENT: &str = "E0802";
pub const CODE_PREPOSITIONAL_REGIME: &str = "E0901";
pub const CODE_ACCIDENTAL_SE: &str = "E1001";

pub fn get_rule_title(code: &str) -> String {
    match code {
        "E0001" => "general mismatch".to_string(),
        "E0101" => "ser vs estar state mismatch".to_string(),
        "E0102" => "ser vs estar meaning change".to_string(),
        "E0201" => "past aspect preterite mismatch".to_string(),
        "E0202" => "past aspect imperfect mismatch".to_string(),
        "E0203" => "past meaning change".to_string(),
        "E0301" => "grammatical rule violation".to_string(),
        "E0401" => "subjunctive relative clause".to_string(),
        "E0501" => "subjunctive conjunction".to_string(),
        "E0601" => "conditional tense violation".to_string(),
        "E0701" => "por vs para mismatch".to_string(),
        "E0801" => "clitic stacking error".to_string(),
        "E0802" => "clitic accentuation error".to_string(),
        "E0901" => "prepositional regime mismatch".to_string(),
        "E1001" => "accidental se usage".to_string(),
        _ => "grammatical rule violation".to_string(),
    }
}
