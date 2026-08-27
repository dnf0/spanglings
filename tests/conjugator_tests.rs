use spanglings::core::conjugator::conjugate_verb;

#[test]
fn test_regular_ar_conjugation() {
    let table = conjugate_verb("hablar").expect("Should conjugate hablar");
    assert_eq!(table.infinitive, "hablar");
    assert!(!table.is_irregular);
    assert_eq!(table.gerund, "hablando");
    assert_eq!(table.participle, "hablado");
    assert_eq!(table.present.yo, "hablo");
    assert_eq!(table.present.tu, "hablas");
    assert_eq!(table.present.vos, "hablás");
    assert_eq!(table.preterite.yo, "hablé");
    assert_eq!(table.imperfect.nosotros, "hablábamos");
    assert_eq!(table.future.el_ella_usted, "hablará");
    assert_eq!(table.conditional.yo, "hablaría");
    assert_eq!(table.present_subjunctive.yo, "hable");
    assert_eq!(table.imperfect_subjunctive_ra.yo, "hablara");
    assert_eq!(table.imperfect_subjunctive_se.yo, "hablase");
    assert_eq!(table.imperative_affirmative.tu, "habla");
    assert_eq!(table.imperative_negative.tu, "no hables");
}

#[test]
fn test_regular_er_conjugation() {
    let table = conjugate_verb("comer").expect("Should conjugate comer");
    assert_eq!(table.infinitive, "comer");
    assert!(!table.is_irregular);
    assert_eq!(table.gerund, "comiendo");
    assert_eq!(table.participle, "comido");
    assert_eq!(table.present.yo, "como");
    assert_eq!(table.present.vos, "comés");
    assert_eq!(table.preterite.yo, "comí");
    assert_eq!(table.preterite.ellos_ellas_ustedes, "comieron");
    assert_eq!(table.imperfect.yo, "comía");
    assert_eq!(table.present_subjunctive.yo, "coma");
    assert_eq!(table.imperfect_subjunctive_ra.nosotros, "comiéramos");
}

#[test]
fn test_regular_ir_conjugation() {
    let table = conjugate_verb("vivir").expect("Should conjugate vivir");
    assert_eq!(table.infinitive, "vivir");
    assert!(!table.is_irregular);
    assert_eq!(table.gerund, "viviendo");
    assert_eq!(table.participle, "vivido");
    assert_eq!(table.present.vos, "vivís");
    assert_eq!(table.preterite.el_ella_usted, "vivió");
    assert_eq!(table.present_subjunctive.nosotros, "vivamos");
}

#[test]
fn test_irregular_ser() {
    let table = conjugate_verb("ser").expect("Should conjugate ser");
    assert!(table.is_irregular);
    assert_eq!(table.present.yo, "soy");
    assert_eq!(table.present.tu, "eres");
    assert_eq!(table.present.vos, "sos");
    assert_eq!(table.preterite.yo, "fui");
    assert_eq!(table.preterite.el_ella_usted, "fue");
    assert_eq!(table.imperfect.yo, "era");
    assert_eq!(table.present_subjunctive.yo, "sea");
    assert_eq!(table.imperfect_subjunctive_ra.yo, "fuera");
    assert_eq!(table.imperative_affirmative.tu, "sé");
}

#[test]
fn test_irregular_haber() {
    let table = conjugate_verb("haber").expect("Should conjugate haber");
    assert!(table.is_irregular);
    assert_eq!(table.present.yo, "he");
    assert_eq!(table.preterite.yo, "hube");
    assert_eq!(table.future.yo, "habré");
    assert_eq!(table.conditional.yo, "habría");
    assert_eq!(table.present_subjunctive.yo, "haya");
    assert_eq!(table.imperfect_subjunctive_ra.yo, "hubiera");
}

#[test]
fn test_irregular_tener() {
    let table = conjugate_verb("tener").expect("Should conjugate tener");
    assert!(table.is_irregular);
    assert_eq!(table.present.yo, "tengo");
    assert_eq!(table.present.vos, "tenés");
    assert_eq!(table.preterite.yo, "tuve");
    assert_eq!(table.future.yo, "tendré");
    assert_eq!(table.conditional.yo, "tendría");
    assert_eq!(table.present_subjunctive.yo, "tenga");
    assert_eq!(table.imperative_affirmative.tu, "ten");
}

#[test]
fn test_irregular_hacer() {
    let table = conjugate_verb("hacer").expect("Should conjugate hacer");
    assert!(table.is_irregular);
    assert_eq!(table.participle, "hecho");
    assert_eq!(table.present.yo, "hago");
    assert_eq!(table.preterite.yo, "hice");
    assert_eq!(table.preterite.el_ella_usted, "hizo");
    assert_eq!(table.future.yo, "haré");
    assert_eq!(table.imperative_affirmative.tu, "haz");
}

#[test]
fn test_irregular_ir() {
    let table = conjugate_verb("ir").expect("Should conjugate ir");
    assert!(table.is_irregular);
    assert_eq!(table.gerund, "yendo");
    assert_eq!(table.present.yo, "voy");
    assert_eq!(table.preterite.yo, "fui");
    assert_eq!(table.imperfect.yo, "iba");
    assert_eq!(table.present_subjunctive.yo, "vaya");
    assert_eq!(table.imperative_affirmative.tu, "ve");
}

#[test]
fn test_invalid_verb() {
    assert!(conjugate_verb("xyz123").is_none());
}
