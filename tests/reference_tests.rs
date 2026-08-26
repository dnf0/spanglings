use spanglings::core::reference::{get_reference_card, list_reference_topics};

#[test]
fn test_get_subjunctive_reference_card() {
    let card = get_reference_card("subjunctive").expect("Subjunctive card not found");
    assert!(card.contains("WEIRDO"));
    assert!(card.contains("Wishes"));
    assert!(card.contains("Present Subjunctive Endings"));
}

#[test]
fn test_get_por_para_reference_card() {
    let card = get_reference_card("por-para").expect("Por/Para card not found");
    assert!(card.contains("Por"));
    assert!(card.contains("Para"));
    assert!(card.contains("Cause / Reason") || card.contains("Cause"));
}

#[test]
fn test_get_ser_estar_reference_card() {
    let card = get_reference_card("ser-estar").expect("Ser/Estar card not found");
    assert!(card.contains("SER"));
    assert!(card.contains("ESTAR"));
}

#[test]
fn test_get_past_tenses_reference_card() {
    let card = get_reference_card("past").expect("Past tenses card not found");
    assert!(card.contains("PRETERITE"));
    assert!(card.contains("IMPERFECT"));
}

#[test]
fn test_get_pronouns_reference_card() {
    let card = get_reference_card("pronouns").expect("Pronouns card not found");
    assert!(card.contains("DOUBLE OBJECT PRONOUNS"));
    assert!(card.contains("SE LO DOY"));
}

#[test]
fn test_get_prepositions_reference_card() {
    let card = get_reference_card("prepositions").expect("Prepositions card not found");
    assert!(card.contains("VERBAL REGIMEN"));
    assert!(card.contains("acostumbrarse a"));
}

#[test]
fn test_get_accidental_se_reference_card() {
    let card = get_reference_card("accidental-se").expect("Accidental se card not found");
    assert!(card.contains("ACCIDENTAL"));
    assert!(card.contains("Se me cayeron"));
}

#[test]
fn test_list_reference_topics() {
    let topics = list_reference_topics();
    assert!(topics.contains(&"subjunctive"));
    assert!(topics.contains(&"por-para"));
    assert!(topics.contains(&"ser-estar"));
    assert!(topics.contains(&"past"));
    assert!(topics.contains(&"pronouns"));
    assert!(topics.contains(&"prepositions"));
    assert!(topics.contains(&"accidental-se"));
}

#[test]
fn test_unknown_topic_returns_none() {
    assert!(get_reference_card("quantum_physics").is_none());
}
