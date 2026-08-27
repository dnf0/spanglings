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
pub const CODE_EPISTEMIC_CONJECTURE: &str = "E0048";
pub const CODE_CLITIC_DOUBLING: &str = "E0049";
pub const CODE_PERSONAL_A: &str = "E0050";
pub const CODE_GERUND_RESTRICTION: &str = "E0051";
pub const CODE_ADVERSATIVE_CONTRAST: &str = "E0052";
pub const CODE_OPTATIVE_LEGAL: &str = "E0053";
pub const CODE_VERBS_OF_BECOMING: &str = "E0054";
pub const CODE_EPISTEMIC_ADVERBS: &str = "E0055";
pub const CODE_POSSESSIVE_DATIVES: &str = "E0056";
pub const CODE_CORRECTIVE_POLARITY: &str = "E0057";
pub const CODE_PARTICIPIAL_ABSOLUTES: &str = "E0058";
pub const CODE_SCALAR_CONCESSION: &str = "E0059";

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
        "E0048" => "epistemic conjecture / probability".to_string(),
        "E0049" => "clitic doubling / left-dislocation".to_string(),
        "E0050" => "personal a / animacy marking".to_string(),
        "E0051" => "gerund restriction / posteriority".to_string(),
        "E0052" => "adversative contrast (pero / sino / sino que)".to_string(),
        "E0053" => "optative / legal subjunctive".to_string(),
        "E0054" => "verbs of becoming (ponerse / quedarse / hacerse / volverse)".to_string(),
        "E0055" => "epistemic adverbs mood selection (a lo mejor vs quizás)".to_string(),
        "E0056" => "inalienable possession / ethic dative clitics".to_string(),
        "E0057" => "corrective / concessive polarity (no es que... / de ahí que)".to_string(),
        "E0058" => "participial absolute construction & agreement".to_string(),
        "E0059" => "scalar concession / intensive connectors (por mucho que)".to_string(),
        _ => "grammatical rule violation".to_string(),
    }
}
