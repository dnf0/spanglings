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
fn test_get_tech_reference_card() {
    let card = get_reference_card("tech").expect("Tech card not found");
    assert!(card.contains("SOFTWARE ENGINEERING"));
    assert!(card.contains("desplegar"));
    assert!(card.contains("bloqueo mutuo"));
}

#[test]
fn test_get_business_reference_card() {
    let card = get_reference_card("business").expect("Business card not found");
    assert!(card.contains("BUSINESS & DIPLOMATIC"));
    assert!(card.contains("Quedo a su entera disposición"));
}

#[test]
fn test_get_false_friends_reference_card() {
    let card = get_reference_card("false-friends").expect("False friends card not found");
    assert!(card.contains("FALSE FRIENDS"));
    assert!(card.contains("actualmente"));
}

#[test]
fn test_get_voseo_reference_card() {
    let card = get_reference_card("voseo").expect("Voseo card not found");
    assert!(card.contains("VOSEO"));
    assert!(card.contains("vos hablás"));
}

#[test]
fn test_get_accents_reference_card() {
    let card = get_reference_card("accents").expect("Accents card not found");
    assert!(card.contains("ACCENTUATION & ORTHOGRAPHIC STRESS"));
    assert!(card.contains("AGUDAS"));
    assert!(card.contains("LLANAS"));
    assert!(card.contains("ESDRÚJULAS"));
    assert!(card.contains("Diptongos vs Hiatos"));
}

#[test]
fn test_get_epistemic_conjecture_reference_card() {
    let card = get_reference_card("epistemic-conjecture").expect("Conjecture card not found");
    assert!(card.contains("EPISTEMIC CONJECTURE"));
    assert!(card.contains("Serán las cuatro"));
    assert!(card.contains("Estaría enfermo"));
}

#[test]
fn test_get_clitic_doubling_reference_card() {
    let card = get_reference_card("clitic-doubling").expect("Clitic doubling card not found");
    assert!(card.contains("CLITIC DOUBLING"));
    assert!(card.contains("A María LE entregué"));
}

#[test]
fn test_get_personal_a_reference_card() {
    let card = get_reference_card("personal-a").expect("Personal a card not found");
    assert!(card.contains("PERSONAL A"));
    assert!(card.contains("Busco programador"));
}

#[test]
fn test_get_gerund_rules_reference_card() {
    let card = get_reference_card("gerund-rules").expect("Gerund rules card not found");
    assert!(card.contains("GERUND RESTRICTIONS"));
    assert!(card.contains("Gerundio de Posterioridad"));
}

#[test]
fn test_get_adversatives_reference_card() {
    let card = get_reference_card("adversatives").expect("Adversatives card not found");
    assert!(card.contains("ADVERSATIVE COORDINATION"));
    assert!(card.contains("SINO QUE"));
}

#[test]
fn test_get_legal_subjunctive_reference_card() {
    let card = get_reference_card("legal-subjunctive").expect("Legal subjunctive card not found");
    assert!(card.contains("OPTATIVES, INDEPENDENT SUBJUNCTIVE"));
    assert!(card.contains("Quién tuviera"));
    assert!(card.contains("FUTURE SUBJUNCTIVE"));
}

#[test]
fn test_get_verbs_of_becoming_reference_card() {
    let card = get_reference_card("verbs-of-becoming").expect("Verbs of becoming card not found");
    assert!(card.contains("VERBOS DE CAMBIO"));
    assert!(card.contains("PONERSE"));
    assert!(card.contains("QUEDARSE"));
    assert!(card.contains("VOLVERSE"));
}

#[test]
fn test_get_epistemic_adverbs_reference_card() {
    let card = get_reference_card("epistemic-adverbs").expect("Epistemic adverbs card not found");
    assert!(card.contains("EPISTEMIC ADVERBS & MOOD SELECTION"));
    assert!(card.contains("A LO MEJOR"));
    assert!(card.contains("QUIZÁS"));
}

#[test]
fn test_get_possessive_datives_reference_card() {
    let card = get_reference_card("possessive-datives").expect("Possessive datives card not found");
    assert!(card.contains("DATIVE OF INALIENABLE POSSESSION"));
    assert!(card.contains("ETHIC & AFFECTIVE CLITICS"));
}

#[test]
fn test_get_corrective_polarity_reference_card() {
    let card =
        get_reference_card("corrective-polarity").expect("Corrective polarity card not found");
    assert!(card.contains("CORRECTIVE & CONCESSIVE POLARITY"));
    assert!(card.contains("DE AHÍ QUE"));
}

#[test]
fn test_get_participial_absolutes_reference_card() {
    let card =
        get_reference_card("participial-absolutes").expect("Participial absolutes card not found");
    assert!(card.contains("PARTICIPIAL ABSOLUTE CONSTRUCTIONS"));
    assert!(card.contains("AGREEMENT RULES"));
}

#[test]
fn test_get_scalar_concession_reference_card() {
    let card = get_reference_card("scalar-concession").expect("Scalar concession card not found");
    assert!(card.contains("SCALAR CONCESSION"));
    assert!(card.contains("POR MUCHO QUE"));
    assert!(card.contains("AUN A RIESGO DE QUE"));
}

#[test]
fn test_list_reference_topics() {
    let topics = list_reference_topics();
    assert_eq!(topics.len(), 24);
    assert!(topics.contains(&"subjunctive"));
    assert!(topics.contains(&"por-para"));
    assert!(topics.contains(&"ser-estar"));
    assert!(topics.contains(&"past"));
    assert!(topics.contains(&"pronouns"));
    assert!(topics.contains(&"prepositions"));
    assert!(topics.contains(&"accidental-se"));
    assert!(topics.contains(&"tech-software"));
    assert!(topics.contains(&"business"));
    assert!(topics.contains(&"false-friends"));
    assert!(topics.contains(&"voseo"));
    assert!(topics.contains(&"accents"));
    assert!(topics.contains(&"epistemic-conjecture"));
    assert!(topics.contains(&"clitic-doubling"));
    assert!(topics.contains(&"personal-a"));
    assert!(topics.contains(&"gerund-rules"));
    assert!(topics.contains(&"adversatives"));
    assert!(topics.contains(&"legal-subjunctive"));
    assert!(topics.contains(&"verbs-of-becoming"));
    assert!(topics.contains(&"epistemic-adverbs"));
    assert!(topics.contains(&"possessive-datives"));
    assert!(topics.contains(&"corrective-polarity"));
    assert!(topics.contains(&"participial-absolutes"));
    assert!(topics.contains(&"scalar-concession"));
}

#[test]
fn test_unknown_topic_returns_none() {
    assert!(get_reference_card("quantum_physics").is_none());
}

#[test]
fn test_error_code_and_concept_lookup() {
    assert!(get_reference_card("E0301").is_some());
    assert!(get_reference_card("e0701").is_some());
    assert!(get_reference_card("E0101").is_some());
    assert!(get_reference_card("E0048").is_some());
    assert!(get_reference_card("E0059").is_some());
    assert!(get_reference_card("subjunctive_volition_influence").is_some());
    assert!(get_reference_card("por_vs_para_purpose_cause").is_some());
}

