use colored::Colorize;
use rand::seq::SliceRandom;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillItem {
    pub prompt: &'static str,
    pub target: &'static str,
    pub topic: &'static str,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillEvaluation {
    Correct,
    Forgiven { expected: String, tip: String },
    Incorrect,
}

pub fn get_drill_items(topic_filter: Option<&str>) -> Vec<DrillItem> {
    let all_items = vec![
        // ----------------------------------------------------
        // 1. Irregular Preterite Stems
        // ----------------------------------------------------
        DrillItem {
            prompt: "Irregular Preterite Stem for 'tener' (yo tuve -> stem: ?)",
            target: "tuv",
            topic: "preterite",
            explanation: "tener -> tuv- (tuve, tuviste, tuvo, tuvimos, tuvieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'poner' (yo puse -> stem: ?)",
            target: "pus",
            topic: "preterite",
            explanation: "poner -> pus- (puse, pusiste, puso, pusimos, pusieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'saber' (yo supe -> stem: ?)",
            target: "sup",
            topic: "preterite",
            explanation: "saber -> sup- (supe, supiste, supo, supimos, supieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'hacer' (yo hice -> stem: ?)",
            target: "hic",
            topic: "preterite",
            explanation: "hacer -> hic- (hice, hiciste, hizo [z], hicimos, hicieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'decir' (yo dije -> stem: ?)",
            target: "dij",
            topic: "preterite",
            explanation: "decir -> dij- (dije, dijiste, dijo, dijimos, dijeron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'estar' (yo estuve -> stem: ?)",
            target: "estuv",
            topic: "preterite",
            explanation: "estar -> estuv- (estuve, estuviste, estuvo, estuvimos, estuvieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'querer' (yo quise -> stem: ?)",
            target: "quis",
            topic: "preterite",
            explanation: "querer -> quis- (quise, quisiste, quiso, quisimos, quisieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'venir' (yo vine -> stem: ?)",
            target: "vin",
            topic: "preterite",
            explanation: "venir -> vin- (vine, viniste, vino, vinimos, vinieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'caber' (yo cupe -> stem: ?)",
            target: "cup",
            topic: "preterite",
            explanation: "caber -> cup- (cupe, cupiste, cupo, cupimos, cupieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'andar' (yo anduve -> stem: ?)",
            target: "anduv",
            topic: "preterite",
            explanation: "andar -> anduv- (anduve, anduviste, anduvo, anduvimos, anduvieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'traer' (yo traje -> stem: ?)",
            target: "traj",
            topic: "preterite",
            explanation: "traer -> traj- (traje, trajiste, trajo, trajimos, trajeron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'poder' (yo pude -> stem: ?)",
            target: "pud",
            topic: "preterite",
            explanation: "poder -> pud- (pude, pudiste, pudo, pudimos, pudieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'haber' (yo hube -> stem: ?)",
            target: "hub",
            topic: "preterite",
            explanation: "haber -> hub- (hube, hubiste, hubo, hubimos, hubieron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'conducir' (yo conduje -> stem: ?)",
            target: "conduj",
            topic: "preterite",
            explanation: "conducir -> conduj- (conduje, condujiste, condujo, condujeron)",
        },
        DrillItem {
            prompt: "Irregular Preterite Stem for 'producir' (yo produje -> stem: ?)",
            target: "produj",
            topic: "preterite",
            explanation: "producir -> produj- (produje, produjiste, produjo, produjeron)",
        },

        // ----------------------------------------------------
        // 2. Present Subjunctive Forms
        // ----------------------------------------------------
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'tener' (que yo...)",
            target: "tenga",
            topic: "subjunctive",
            explanation: "yo tengo -> drop -o -> add -a -> tenga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'salir' (que yo...)",
            target: "salga",
            topic: "subjunctive",
            explanation: "yo salgo -> drop -o -> add -a -> salga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'poner' (que yo...)",
            target: "ponga",
            topic: "subjunctive",
            explanation: "yo pongo -> drop -o -> add -a -> ponga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'decir' (que yo...)",
            target: "diga",
            topic: "subjunctive",
            explanation: "yo digo -> drop -o -> add -a -> diga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'hacer' (que yo...)",
            target: "haga",
            topic: "subjunctive",
            explanation: "yo hago -> drop -o -> add -a -> haga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'ver' (que yo...)",
            target: "vea",
            topic: "subjunctive",
            explanation: "yo veo -> drop -o -> add -a -> vea",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'caber' (que yo...)",
            target: "quepa",
            topic: "subjunctive",
            explanation: "caber -> quepa, quepas, quepa, quepamos, quepáis, quepan",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'ir' (que yo...)",
            target: "vaya",
            topic: "subjunctive",
            explanation: "ir -> vaya, vayas, vaya, vayamos, vayáis, vayan",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'saber' (que yo...)",
            target: "sepa",
            topic: "subjunctive",
            explanation: "saber -> sepa, sepas, sepa, sepamos, sepáis, sepan",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'ser' (que yo...)",
            target: "sea",
            topic: "subjunctive",
            explanation: "ser -> sea, seas, sea, seamos, seáis, sean",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'haber' (que yo...)",
            target: "haya",
            topic: "subjunctive",
            explanation: "haber -> haya, hayas, haya, hayamos, hayáis, hayan",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'dar' (que yo...)",
            target: "dé",
            topic: "subjunctive",
            explanation: "dar -> dé (accent distinguishes from preposition 'de')",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'estar' (que yo...)",
            target: "esté",
            topic: "subjunctive",
            explanation: "estar -> esté, estés, esté, estemos, estéis, estén",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'oír' (que yo...)",
            target: "oiga",
            topic: "subjunctive",
            explanation: "yo oigo -> drop -o -> oiga",
        },
        DrillItem {
            prompt: "Present Subjunctive 'yo' form for 'conocer' (que yo...)",
            target: "conozca",
            topic: "subjunctive",
            explanation: "yo conozco -> drop -o -> conozca",
        },

        // ----------------------------------------------------
        // 3. Imperfect Subjunctive (-ra) Forms
        // ----------------------------------------------------
        DrillItem {
            prompt: "Imperfect Subjunctive (-ra) 'yo' form for 'tener' (si yo...)",
            target: "tuviera",
            topic: "subjunctive",
            explanation: "tuvieron -> drop -ron -> add -ra -> tuviera",
        },
        DrillItem {
            prompt: "Imperfect Subjunctive (-ra) 'yo' form for 'hacer' (si yo...)",
            target: "hiciera",
            topic: "subjunctive",
            explanation: "hicieron -> drop -ron -> add -ra -> hiciera",
        },
        DrillItem {
            prompt: "Imperfect Subjunctive (-ra) 'yo' form for 'decir' (si yo...)",
            target: "dijera",
            topic: "subjunctive",
            explanation: "dijeron -> drop -ron -> add -era -> dijera",
        },
        DrillItem {
            prompt: "Imperfect Subjunctive (-ra) 'yo' form for 'ser' / 'ir' (si yo...)",
            target: "fuera",
            topic: "subjunctive",
            explanation: "fueron -> drop -ron -> add -ra -> fuera",
        },
        DrillItem {
            prompt: "Imperfect Subjunctive (-ra) 'yo' form for 'saber' (si yo...)",
            target: "supiera",
            topic: "subjunctive",
            explanation: "supieron -> drop -ron -> add -ra -> supiera",
        },
        DrillItem {
            prompt: "Imperfect Subjunctive (-ra) 'yo' form for 'poder' (si yo...)",
            target: "pudiera",
            topic: "subjunctive",
            explanation: "pudieron -> drop -ron -> add -ra -> pudiera",
        },

        // ----------------------------------------------------
        // 4. Future / Conditional Irregular Stems
        // ----------------------------------------------------
        DrillItem {
            prompt: "Future/Conditional Stem for 'tener' (yo tendré -> stem: ?)",
            target: "tendr",
            topic: "future",
            explanation: "tener -> tendr- (tendré, tendrás, tendría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'poner' (yo pondré -> stem: ?)",
            target: "pondr",
            topic: "future",
            explanation: "poner -> pondr- (pondré, pondrás, pondría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'salir' (yo saldré -> stem: ?)",
            target: "saldr",
            topic: "future",
            explanation: "salir -> saldr- (saldré, saldrás, saldría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'venir' (yo vendré -> stem: ?)",
            target: "vendr",
            topic: "future",
            explanation: "venir -> vendr- (vendré, vendrás, vendría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'hacer' (yo haré -> stem: ?)",
            target: "har",
            topic: "future",
            explanation: "hacer -> har- (haré, harás, haría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'decir' (yo diré -> stem: ?)",
            target: "dir",
            topic: "future",
            explanation: "decir -> dir- (diré, dirás, diría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'saber' (yo sabré -> stem: ?)",
            target: "sabr",
            topic: "future",
            explanation: "saber -> sabr- (sabré, sabrás, sabría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'poder' (yo podré -> stem: ?)",
            target: "podr",
            topic: "future",
            explanation: "poder -> podr- (podré, podrás, podría)",
        },
        DrillItem {
            prompt: "Future/Conditional Stem for 'querer' (yo querré -> stem: ?)",
            target: "querr",
            topic: "future",
            explanation: "querer -> querr- (querré, querrás, querría)",
        },

        // ----------------------------------------------------
        // 5. Imperatives (Commands)
        // ----------------------------------------------------
        DrillItem {
            prompt: "Affirmative 'tú' command for 'hacer' (Do it! -> ¡___!)",
            target: "haz",
            topic: "imperative",
            explanation: "hacer -> haz (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'poner' (Put it! -> ¡___!)",
            target: "pon",
            topic: "imperative",
            explanation: "poner -> pon (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'tener' (Have it! -> ¡___!)",
            target: "ten",
            topic: "imperative",
            explanation: "tener -> ten (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'salir' (Leave! -> ¡___!)",
            target: "sal",
            topic: "imperative",
            explanation: "salir -> sal (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'venir' (Come! -> ¡___!)",
            target: "ven",
            topic: "imperative",
            explanation: "venir -> ven (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'decir' (Say/Tell! -> ¡___!)",
            target: "di",
            topic: "imperative",
            explanation: "decir -> di (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'ir' (Go! -> ¡___!)",
            target: "ve",
            topic: "imperative",
            explanation: "ir -> ve (irregular affirmative informal command)",
        },
        DrillItem {
            prompt: "Affirmative 'tú' command for 'ser' (Be kind! -> ¡___ amable!)",
            target: "sé",
            topic: "imperative",
            explanation: "ser -> sé (with accent mark)",
        },

        // ----------------------------------------------------
        // 6. Por vs Para
        // ----------------------------------------------------
        DrillItem {
            prompt: "Por vs Para: 'Trabajo ___ una empresa tecnológica' (employer/destination)",
            target: "para",
            topic: "por_para",
            explanation: "para = employment destination / recipient",
        },
        DrillItem {
            prompt: "Por vs Para: 'Muchas gracias ___ tu ayuda' (cause/gratitude)",
            target: "por",
            topic: "por_para",
            explanation: "por = motive / cause / gratitude",
        },
        DrillItem {
            prompt: "Por vs Para: 'El informe debe estar listo ___ el viernes' (deadline)",
            target: "para",
            topic: "por_para",
            explanation: "para = specific deadline / future point in time",
        },
        DrillItem {
            prompt: "Por vs Para: 'Paseamos ___ el centro de la ciudad' (motion through)",
            target: "por",
            topic: "por_para",
            explanation: "por = movement along / through / around an area",
        },
        DrillItem {
            prompt: "Por vs Para: 'Estudié en la biblioteca ___ tres horas' (duration)",
            target: "por",
            topic: "por_para",
            explanation: "por = duration of time",
        },
        DrillItem {
            prompt: "Por vs Para: 'Compré los billetes ___ 50 euros' (exchange/price)",
            target: "por",
            topic: "por_para",
            explanation: "por = price / monetary exchange",
        },
        DrillItem {
            prompt: "Por vs Para: '___ aprender a programar, practico todos los días' (goal/purpose)",
            target: "para",
            topic: "por_para",
            explanation: "para + infinitive = purpose / goal in order to",
        },

        // ----------------------------------------------------
        // 7. Ser vs Estar
        // ----------------------------------------------------
        DrillItem {
            prompt: "Ser vs Estar: 'La conferencia anual ___ en el auditorio' (event location)",
            target: "es",
            topic: "ser_estar",
            explanation: "ser = location where an event takes place / occurs",
        },
        DrillItem {
            prompt: "Ser vs Estar: 'El servidor ___ en el centro de datos' (physical spatial location)",
            target: "está",
            topic: "ser_estar",
            explanation: "estar = physical / spatial location of an object",
        },
        DrillItem {
            prompt: "Ser vs Estar: 'Daniel ___ arquitecto de software' (profession)",
            target: "es",
            topic: "ser_estar",
            explanation: "ser = profession / identity / essential trait",
        },
        DrillItem {
            prompt: "Ser vs Estar: 'La base de datos ___ caída en este momento' (temporary state)",
            target: "está",
            topic: "ser_estar",
            explanation: "estar = condition / temporary or resulting state",
        },
        DrillItem {
            prompt: "Ser vs Estar: 'El hielo ___ frío' (inherent quality/definition)",
            target: "es",
            topic: "ser_estar",
            explanation: "ser = inherent defining quality",
        },

        // ----------------------------------------------------
        // 8. Clitics & Pronouns (Cacophony & Placement)
        // ----------------------------------------------------
        DrillItem {
            prompt: "Replace 'le lo' with cacophony rule: 'Le doy el libro' -> '___ doy'",
            target: "se lo",
            topic: "pronouns",
            explanation: "le + lo -> se lo (cacophony resolution rule)",
        },
        DrillItem {
            prompt: "Replace 'les las' with cacophony rule: 'Les compro las flores' -> '___ compro'",
            target: "se las",
            topic: "pronouns",
            explanation: "les + las -> se las (cacophony resolution rule)",
        },
        DrillItem {
            prompt: "Replace 'le la' with cacophony rule: 'Le explico la regla' -> '___ explico'",
            target: "se la",
            topic: "pronouns",
            explanation: "le + la -> se la",
        },

        // ----------------------------------------------------
        // 9. Prepositions with Verbs (Régimen Preposicional)
        // ----------------------------------------------------
        DrillItem {
            prompt: "Preposition: 'Soñar ___' (to dream of/about) -> preposition: ?",
            target: "con",
            topic: "prepositions",
            explanation: "soñar con = to dream about/of (always 'con' in Spanish)",
        },
        DrillItem {
            prompt: "Preposition: 'Insistir ___' (to insist on) -> preposition: ?",
            target: "en",
            topic: "prepositions",
            explanation: "insistir en = to insist on",
        },
        DrillItem {
            prompt: "Preposition: 'Depender ___' (to depend on) -> preposition: ?",
            target: "de",
            topic: "prepositions",
            explanation: "depender de = to depend on (always 'de')",
        },
        DrillItem {
            prompt: "Preposition: 'Negarse ___' (to refuse to) -> preposition: ?",
            target: "a",
            topic: "prepositions",
            explanation: "negarse a = to refuse to do something",
        },
        DrillItem {
            prompt: "Preposition: 'Acordarse ___' (to remember / recall) -> preposition: ?",
            target: "de",
            topic: "prepositions",
            explanation: "acordarse de = to remember (vs recordar without preposition)",
        },
        DrillItem {
            prompt: "Preposition: 'Tratar ___' (to try to / deal with) -> preposition: ?",
            target: "de",
            topic: "prepositions",
            explanation: "tratar de = to try to (+ inf) / to be about",
        },
        DrillItem {
            prompt: "Preposition: 'Contar ___' (to count on / rely on) -> preposition: ?",
            target: "con",
            topic: "prepositions",
            explanation: "contar con = to rely on / have available",
        },
        DrillItem {
            prompt: "Preposition: 'Tardar ___' (to take time doing) -> preposition: ?",
            target: "en",
            topic: "prepositions",
            explanation: "tardar en (+ inf) = to take time to do something",
        },

        // ----------------------------------------------------
        // 10. Accidental 'Se'
        // ----------------------------------------------------
        DrillItem {
            prompt: "Accidental se: 'I dropped the glass' -> 'Se ___ cayó el vaso' (pronoun for 'me')",
            target: "me",
            topic: "accidental_se",
            explanation: "se me cayó = I accidentally dropped it",
        },
        DrillItem {
            prompt: "Accidental se: 'We forgot the tickets' -> 'Se ___ olvidaron las entradas' (pronoun for 'us')",
            target: "nos",
            topic: "accidental_se",
            explanation: "se nos olvidaron = we accidentally forgot them",
        },
        DrillItem {
            prompt: "Accidental se: 'He lost the keys' -> 'Se ___ perdieron las llaves' (pronoun for 'him')",
            target: "le",
            topic: "accidental_se",
            explanation: "se le perdieron = he accidentally lost them",
        },

        // ----------------------------------------------------
        // 11. False Friends & Cognate Traps
        // ----------------------------------------------------
        DrillItem {
            prompt: "Translate 'currently / at present' to Spanish (not 'actually'):",
            target: "actualmente",
            topic: "false_friends",
            explanation: "actualmente = currently; en realidad / de hecho = actually",
        },
        DrillItem {
            prompt: "Translate 'to pretend / feign' to Spanish (verb starting with f):",
            target: "fingir",
            topic: "false_friends",
            explanation: "fingir = to pretend; pretender = to intend/aspire",
        },
        DrillItem {
            prompt: "Translate 'sensible / prudent' to Spanish (not 'sensible'):",
            target: "sensato",
            topic: "false_friends",
            explanation: "sensato = sensible/prudent; sensible = sensitive",
        },
        DrillItem {
            prompt: "Translate 'to record audio/video' to Spanish (not 'recordar'):",
            target: "grabar",
            topic: "false_friends",
            explanation: "grabar = to record; recordar = to remember",
        },
        DrillItem {
            prompt: "Translate 'success' to Spanish (not 'suceso'):",
            target: "éxito",
            topic: "false_friends",
            explanation: "éxito = success; suceso = event/incident",
        },
        DrillItem {
            prompt: "Translate 'folder / binder' to Spanish (not 'carpet'):",
            target: "carpeta",
            topic: "false_friends",
            explanation: "carpeta = folder; alfombra = carpet",
        },
        DrillItem {
            prompt: "Translate 'to have a head cold / congested' to Spanish (looks like constipated):",
            target: "constipado",
            topic: "false_friends",
            explanation: "constipado = congested/head cold; estreñido = constipated",
        },

        // ----------------------------------------------------
        // 12. Idioms & Collocations
        // ----------------------------------------------------
        DrillItem {
            prompt: "Idiom: 'To take for granted' -> 'Dar ___ sentado' (word: ?)",
            target: "por",
            topic: "idioms",
            explanation: "dar por sentado = to take for granted",
        },
        DrillItem {
            prompt: "Idiom: 'To take into account' -> 'Tener ___ cuenta' (word: ?)",
            target: "en",
            topic: "idioms",
            explanation: "tener en cuenta = to take into account / keep in mind",
        },
        DrillItem {
            prompt: "Idiom: 'To miss someone/something' -> 'Echar ___ menos' (word: ?)",
            target: "de",
            topic: "idioms",
            explanation: "echar de menos = to miss",
        },
        DrillItem {
            prompt: "Idiom: 'To be worth the trouble' -> 'Valer ___ pena' (word: ?)",
            target: "la",
            topic: "idioms",
            explanation: "valer la pena = to be worth it",
        },
        DrillItem {
            prompt: "Idiom: 'To carry out / implement' -> 'Llevar ___ cabo' (word: ?)",
            target: "a",
            topic: "idioms",
            explanation: "llevar a cabo = to execute / carry out",
        },
    ];

    if let Some(filt) = topic_filter {
        let f = filt.to_lowercase().replace('_', "-");
        if f == "all" || f.is_empty() {
            all_items
        } else {
            let filtered: Vec<DrillItem> = all_items
                .into_iter()
                .filter(|item| {
                    let top = item.topic.to_lowercase().replace('_', "-");
                    top.contains(&f)
                        || (f.contains("pret") && top.contains("preterite"))
                        || (f.contains("past")
                            && (top.contains("preterite") || top.contains("subjunctive")))
                        || (f.contains("subj") && top.contains("subjunctive"))
                        || (f.contains("ser") && top.contains("ser_estar"))
                        || (f.contains("estar") && top.contains("ser_estar"))
                        || (f.contains("por") && top.contains("por_para"))
                        || (f.contains("para") && top.contains("por_para"))
                        || (f.contains("clitic") && top.contains("pronouns"))
                        || (f.contains("pronoun") && top.contains("pronouns"))
                        || (f.contains("false") && top.contains("false_friends"))
                        || (f.contains("prep") && top.contains("prepositions"))
                        || (f.contains("command") && top.contains("imperative"))
                        || (f.contains("idiom") && top.contains("idioms"))
                        || (f.contains("accidental") && top.contains("accidental_se"))
                })
                .collect();
            filtered
        }
    } else {
        all_items
    }
}

pub fn evaluate_drill_answer(
    item: &DrillItem,
    user_input: &str,
    strict_accents: bool,
) -> DrillEvaluation {
    let clean_user = user_input.trim();
    let clean_target = item.target.trim();

    if clean_user.is_empty() {
        return DrillEvaluation::Incorrect;
    }

    match crate::engine::accents::check_accent_match(clean_user, clean_target, strict_accents) {
        crate::engine::accents::AccentResult::ExactMatch => DrillEvaluation::Correct,
        crate::engine::accents::AccentResult::ForgivenMatch { expected, tip } => {
            DrillEvaluation::Forgiven { expected, tip }
        }
        crate::engine::accents::AccentResult::Mismatch => DrillEvaluation::Incorrect,
    }
}

pub fn run_drill(
    topic: Option<&str>,
    concept: Option<&str>,
    count: Option<usize>,
    strict_accents: bool,
) -> anyhow::Result<()> {
    let t = concept.or(topic).unwrap_or("all").to_lowercase();
    let mut items = get_drill_items(Some(&t));

    if items.is_empty() {
        // If specific sub-filter didn't match, fall back to all items
        items = get_drill_items(None);
    }

    // Always shuffle items randomly so each drill session is fresh and non-repetitive
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);

    let num_questions = count.unwrap_or(5).min(items.len());
    let selected_items = &items[..num_questions];

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "{}",
        "          SPANGLINGS RAPID-FIRE CONJUGATION DRILL         ".bold()
    );
    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Topic: {} ({} questions). Type your answer and press Enter.\n",
        t.cyan().bold(),
        num_questions.to_string().yellow().bold()
    );

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut score = 0;

    for (i, item) in selected_items.iter().enumerate() {
        print!(
            "Q{}/{}: {} > ",
            i + 1,
            num_questions,
            item.prompt.bright_white()
        );
        io::stdout().flush()?;

        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            // EOF reached (e.g. non-interactive pipeline or Ctrl+D)
            println!();
            break;
        }

        match evaluate_drill_answer(item, &line, strict_accents) {
            DrillEvaluation::Correct => {
                println!("  {} Correct!\n", "✓".green().bold());
                score += 1;
            }
            DrillEvaluation::Forgiven { expected, tip } => {
                println!(
                    "  {} Correct! ({}) [Target: {}]\n",
                    "✓".green().bold(),
                    tip.yellow(),
                    expected.green().bold()
                );
                score += 1;
            }
            DrillEvaluation::Incorrect => {
                println!(
                    "  {} Incorrect. Expected: '{}' ({})\n",
                    "✗".red().bold(),
                    item.target.green().bold(),
                    item.explanation.dimmed()
                );
            }
        }
    }

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Drill Finished! Score: {} / {} ({:.0}%)",
        score.to_string().green().bold(),
        num_questions,
        if num_questions > 0 {
            (score as f64 / num_questions as f64) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    Ok(())
}
