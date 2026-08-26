use spanglings::engine::accents::{check_accent_match, strip_accents, AccentResult};
use spanglings::engine::normalizer::normalize;

#[test]
fn test_punctuation_and_whitespace_normalization() {
    assert_eq!(normalize("  ¿Hola!  "), "Hola!");
    assert_eq!(normalize("¡Qué tal!"), "Qué tal!");
    assert_eq!(normalize("  hablo   mucho  "), "hablo mucho");
    assert_eq!(normalize("\"vengas\""), "vengas");
    assert_eq!(normalize("'vengas'"), "vengas");
}

#[test]
fn test_accent_stripping() {
    assert_eq!(strip_accents("habló"), "hablo");
    assert_eq!(strip_accents("pingüino"), "pinguino");
    assert_eq!(strip_accents("año"), "ano");
    assert_eq!(strip_accents("ÁÉÍÓÚÜÑ"), "AEIOUUN");
}

#[test]
fn test_smart_accent_matching() {
    // Exact match
    assert_eq!(
        check_accent_match("habló", "habló", false),
        AccentResult::ExactMatch
    );

    // Forgiven match with tip when strict = false
    match check_accent_match("hablo", "habló", false) {
        AccentResult::ForgivenMatch { expected, tip } => {
            assert_eq!(expected, "habló");
            assert!(tip.contains("habló"));
        }
        other => panic!("Expected ForgivenMatch, got {:?}", other),
    }

    // Strict mode rejects missing accent
    assert_eq!(
        check_accent_match("hablo", "habló", true),
        AccentResult::Mismatch
    );

    // Completely wrong answer is a Mismatch
    assert_eq!(
        check_accent_match("comió", "habló", false),
        AccentResult::Mismatch
    );
}
