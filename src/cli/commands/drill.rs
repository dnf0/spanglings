use colored::Colorize;
use rand::seq::SliceRandom;
use std::io::{self, BufRead, Write};

pub use crate::core::generator::DrillItem;

#[derive(Debug, Clone, Copy)]
pub struct StaticDrillItem {
    pub topic: &'static str,
    pub formula_cue: &'static str,
    pub trigger_sentence: &'static str,
    pub target_verb: &'static str,
    pub target_subject: &'static str,
    pub target: &'static str,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Default)]
pub struct DrillFilter {
    pub weak_only: bool,
    pub topic: Option<String>,
    pub level: Option<crate::core::curriculum::Level>,
    pub track: Option<usize>,
    pub count: usize,
}

pub fn select_drill_items(
    state: &crate::core::state::AppState,
    filter: DrillFilter,
) -> Vec<DrillItem> {
    if filter.count == 0 {
        return Vec::new();
    }

    if filter.weak_only {
        let weakest = state.get_weakest_concepts(5);
        let weak_candidates: Vec<String> = weakest
            .into_iter()
            .filter(|(_, mastery)| mastery.mastery_score < 0.8)
            .map(|(id, _)| id.clone())
            .collect();

        let weak_topics = if !weak_candidates.is_empty() {
            weak_candidates
        } else {
            // Fallback to all 24 concepts if no weakness is recorded
            crate::core::reference::list_grammar_concepts()
                .iter()
                .map(|c| c.slug.to_string())
                .collect()
        };

        let mut items = Vec::with_capacity(filter.count);
        for i in 0..filter.count {
            let topic = &weak_topics[i % weak_topics.len()];
            let mut gen = crate::core::generator::generate_drill_items_for_topic(topic, 1);
            if let Some(item) = gen.pop() {
                items.push(item);
            }
        }
        if items.len() < filter.count {
            let needed = filter.count - items.len();
            let mut fallback = crate::core::generator::generate_random_drill_items(needed);
            items.append(&mut fallback);
        }
        return items;
    }

    if filter.level.is_some() || filter.track.is_some() {
        let mut extracted_items = Vec::new();
        if let Ok(curriculum) = crate::core::curriculum::load_curriculum() {
            let matching_exercises: Vec<&crate::core::exercise::Exercise> = curriculum
                .exercises
                .iter()
                .filter(|ex| {
                    if let Some(lvl) = filter.level {
                        if ex.level != lvl {
                            return false;
                        }
                    }
                    if let Some(tr) = filter.track {
                        let tr_pad = format!("{:02}_", tr);
                        let tr_plain = format!("{}_", tr);
                        let matches_track = ex.path.components().any(|c| {
                            let name = c.as_os_str().to_string_lossy();
                            name.starts_with(&tr_pad) || name.starts_with(&tr_plain)
                        });
                        if !matches_track {
                            return false;
                        }
                    }
                    if let Some(ref t) = filter.topic {
                        let t_clean = t.to_lowercase().replace('_', "-");
                        let ex_topic = ex.topic.to_lowercase().replace('_', "-");
                        let matches_topic = ex_topic.contains(&t_clean)
                            || ex
                                .concept_tags
                                .iter()
                                .any(|tag| tag.to_lowercase().replace('_', "-").contains(&t_clean));
                        if !matches_topic {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            for ex in matching_exercises {
                extracted_items.extend(ex.to_drill_items());
            }
        }

        if !extracted_items.is_empty() {
            let mut rng = rand::thread_rng();
            extracted_items.shuffle(&mut rng);
            extracted_items.truncate(filter.count);
            return extracted_items;
        }

        if let Some(ref t) = filter.topic {
            let items = crate::core::generator::generate_drill_items_for_topic(t, filter.count);
            if !items.is_empty() {
                return items;
            }
        }

        return crate::core::generator::generate_random_drill_items(filter.count);
    }

    if let Some(ref t) = filter.topic {
        let t_clean = t.trim().to_lowercase();
        if !t_clean.is_empty() && t_clean != "all" {
            let items =
                crate::core::generator::generate_drill_items_for_topic(&t_clean, filter.count);
            if !items.is_empty() {
                return items;
            }
            let mut static_items = get_drill_items(Some(&t_clean));
            if !static_items.is_empty() {
                let mut rng = rand::thread_rng();
                static_items.shuffle(&mut rng);
                static_items.truncate(filter.count);
                return static_items;
            }
        }
    }

    // Default / Random: Blend combinatorial generated items and curriculum extracted items
    let mut items = Vec::new();
    let gen_count = filter.count.div_ceil(2);
    let curr_count = filter.count / 2;
    let mut gen_items = crate::core::generator::generate_random_drill_items(gen_count);
    items.append(&mut gen_items);

    if let Ok(curriculum) = crate::core::curriculum::load_curriculum() {
        let mut curr_items = Vec::new();
        for ex in &curriculum.exercises {
            curr_items.extend(ex.to_drill_items());
        }
        if !curr_items.is_empty() {
            let mut rng = rand::thread_rng();
            curr_items.shuffle(&mut rng);
            curr_items.truncate(curr_count.max(filter.count.saturating_sub(items.len())));
            items.append(&mut curr_items);
        }
    }

    if items.len() < filter.count {
        let needed = filter.count - items.len();
        let mut topup = crate::core::generator::generate_random_drill_items(needed);
        items.append(&mut topup);
    }

    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
    items.truncate(filter.count);
    items
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillEvaluation {
    Correct,
    Forgiven { expected: String, tip: String },
    Incorrect,
}

pub fn get_topic_cheat_sheet(topic: &str) -> Option<&'static str> {
    let t = topic.trim().to_lowercase().replace('-', "_");
    match t.as_str() {
        "subjunctive" | "subj" => Some(
            "💡 Subjunctive Formula:\n   1. Form Present 'yo' stem: poner ➔ pongo ➔ pong-\n   2. Swap opposite vowel: -AR ➔ -e/-en, -ER/-IR ➔ -a/-an (ponga)\n   3. Triggers: Wants ('quiero que'), Doubt ('dudo que'), Necessity ('es necesario que')",
        ),
        "preterite" | "pret" | "past" => Some(
            "💡 Irregular Preterite Rule:\n   Irregular stems (tuv-, pus-, sup-, hic-, dij-, anduv-, traj-) take unaccented endings:\n   -e, -iste, -o, -imos, -ieron (e.g., yo puse, él puso, ellos pusieron)",
        ),
        "por_para" | "por" | "para" => Some(
            "💡 Por vs. Para Rule:\n   • Por: Cause/Reason, Movement through, Duration, Exchange, Means\n   • Para: Purpose ('in order to' + inf), Recipient, Destination, Deadline",
        ),
        "ser_estar" | "ser" | "estar" => Some(
            "💡 Ser vs. Estar Rule:\n   • Ser: Essential identity, Profession, Origin, Event location ('la fiesta es en...')\n   • Estar: Physical location ('el libro está en...'), Temporary states, Ongoing (-ando/-iendo)",
        ),
        "pronouns" | "pronoun" | "clitics" | "clitic" => Some(
            "💡 Pronoun Clitic Stacking Rule:\n   Indirect Object precedes Direct Object (IOP + DOP).\n   When both start with 'l' (le lo, les las), change IOP to 'se' ('se lo', 'se las').",
        ),
        "prepositions" | "preposition" | "prep" => Some(
            "💡 Prepositional Verbs (Régimen Preposicional):\n   soñar CON, acordarse DE, insistir EN, negarse A, contar CON, tardar EN, depender DE, tratar DE",
        ),
        "accidental_se" | "accidental" => Some(
            "💡 Accidental / Unintentional 'Se':\n   Pattern: [Se] + [IOP person affected: me/te/le/nos/les] + [verb agrees with object]\n   e.g. Se me cayeron las llaves (The keys dropped on me).",
        ),
        "imperative" | "command" | "commands" => Some(
            "💡 Imperative (Commands) Rule:\n   • Affirmative Informal (tú): 3rd person present indicative (habla, come) or irregular (haz, pon, ten, sal, ven, di, ve, sé)\n   • Negative Informal & Formal: Present Subjunctive (no hables, no comas, hable Ud.)",
        ),
        "future" | "conditional" => Some(
            "💡 Future & Conditional Stem Rule:\n   Irregular future/conditional stems append -é/-ás/-á... or -ía/-ías...\n   e.g. tendr- (tener), pondr- (poner), saldr- (salir), vendr- (venir), har- (hacer), dir- (decir), sabr- (saber), podr- (poder), querr- (querer)",
        ),
        "false_friends" | "false_cognates" | "cognates" => Some(
            "💡 False Friends & Cognate Traps:\n   actualmente = currently (not actually) | fingir = pretend | sensato = sensible\n   grabar = record (not remember) | éxito = success | carpeta = folder | constipado = cold/congested",
        ),
        "idioms" | "idiom" => Some(
            "💡 Essential Spanish Idiomatic Collocations:\n   dar por sentado (take for granted), tener en cuenta (keep in mind), echar de menos (miss),\n   valer la pena (worth it), llevar a cabo (carry out)",
        ),
        "all" | "mixed" | "" => Some(
            "💡 Rapid Conjugation Formula Cheat Sheet:\n   • Subjunctive: 'yo' stem + opposite vowel (-ar -> -e, -er/-ir -> -a)\n   • Preterite: Irregular stem + unaccented endings (-e, -iste, -o, -imos, -ieron)\n   • Future/Cond: Special stem (tendr-, pondr-, har-, dir-) + future/cond endings\n   • Por vs Para: Por = Cause/Duration/Through | Para = Purpose/Destination/Deadline\n   • Ser vs Estar: Ser = Identity/Events | Estar = Location/Conditions\n   • Pronouns: IOP before DOP; le + lo -> se lo",
        ),
        _ => None,
    }
}

pub fn get_drill_items(topic_filter: Option<&str>) -> Vec<DrillItem> {
    let all_items = vec![
        // ----------------------------------------------------
        // 1. Irregular Preterite Stems
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem tuv- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Anoche yo ____ una reunión urgente (yo tuve -> raíz: ____).",
            target_verb: "tener",
            target_subject: "yo",
            target: "tuv",
            explanation: "tener -> tuv- (tuve, tuviste, tuvo, tuvimos, tuvieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem pus- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo ____ el informe sobre la mesa (yo puse -> raíz: ____).",
            target_verb: "poner",
            target_subject: "yo",
            target: "pus",
            explanation: "poner -> pus- (puse, pusiste, puso, pusimos, pusieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem sup- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo ____ la verdad sobre el proyecto (yo supe -> raíz: ____).",
            target_verb: "saber",
            target_subject: "yo",
            target: "sup",
            explanation: "saber -> sup- (supe, supiste, supo, supimos, supieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem hic- (él hizo con z) + unaccented endings",
            trigger_sentence: "Ayer yo ____ todo el trabajo pendiente (yo hice -> raíz: ____).",
            target_verb: "hacer",
            target_subject: "yo",
            target: "hic",
            explanation: "hacer -> hic- (hice, hiciste, hizo [z], hicimos, hicieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem dij- (j-stem: -eron) + unaccented endings",
            trigger_sentence: "Ayer yo ____ lo que pensaba sinceramente (yo dije -> raíz: ____).",
            target_verb: "decir",
            target_subject: "yo",
            target: "dij",
            explanation: "decir -> dij- (dije, dijiste, dijo, dijimos, dijeron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem estuv- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo ____ en la oficina hasta tarde (yo estuve -> raíz: ____).",
            target_verb: "estar",
            target_subject: "yo",
            target: "estuv",
            explanation: "estar -> estuv- (estuve, estuviste, estuvo, estuvimos, estuvieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem quis- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo ____ cancelar la suscripción (yo quise -> raíz: ____).",
            target_verb: "querer",
            target_subject: "yo",
            target: "quis",
            explanation: "querer -> quis- (quise, quisiste, quiso, quisimos, quisieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem vin- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo ____ en tren a la conferencia (yo vine -> raíz: ____).",
            target_verb: "venir",
            target_subject: "yo",
            target: "vin",
            explanation: "venir -> vin- (vine, viniste, vino, vinimos, vinieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem cup- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo no ____ en el ascensor lleno (yo cupe -> raíz: ____).",
            target_verb: "caber",
            target_subject: "yo",
            target: "cup",
            explanation: "caber -> cup- (cupe, cupiste, cupo, cupimos, cupieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem anduv- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence:
                "Ayer yo ____ cinco kilómetros por el parque (yo anduve -> raíz: ____).",
            target_verb: "andar",
            target_subject: "yo",
            target: "anduv",
            explanation: "andar -> anduv- (anduve, anduviste, anduvo, anduvimos, anduvieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem traj- (j-stem: -eron) + unaccented endings",
            trigger_sentence: "Ayer yo ____ los documentos requeridos (yo traje -> raíz: ____).",
            target_verb: "traer",
            target_subject: "yo",
            target: "traj",
            explanation: "traer -> traj- (traje, trajiste, trajo, trajimos, trajeron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem pud- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer yo no ____ terminar a tiempo (yo pude -> raíz: ____).",
            target_verb: "poder",
            target_subject: "yo",
            target: "pud",
            explanation: "poder -> pud- (pude, pudiste, pudo, pudimos, pudieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem hub- + unaccented endings (-e, -iste, -o...)",
            trigger_sentence: "Ayer ____ un fallo en los servidores (hubo -> raíz: ____).",
            target_verb: "haber",
            target_subject: "impersonal",
            target: "hub",
            explanation: "haber -> hub- (hube, hubiste, hubo, hubimos, hubieron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem conduj- (j-stem: -eron) + unaccented endings",
            trigger_sentence:
                "Ayer yo ____ durante cuatro horas seguidas (yo conduje -> raíz: ____).",
            target_verb: "conducir",
            target_subject: "yo",
            target: "conduj",
            explanation: "conducir -> conduj- (conduje, condujiste, condujo, condujeron)",
        },
        StaticDrillItem {
            topic: "preterite",
            formula_cue: "stem produj- (j-stem: -eron) + unaccented endings",
            trigger_sentence:
                "El error ____ una interrupción del servicio (produjo -> raíz: ____).",
            target_verb: "producir",
            target_subject: "él/ella",
            target: "produj",
            explanation: "producir -> produj- (produje, produjiste, produjo, produjeron)",
        },
        // ----------------------------------------------------
        // 2. Present Subjunctive Forms
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo tengo -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "Dudo que yo ____ suficiente tiempo para terminar hoy.",
            target_verb: "tener",
            target_subject: "yo",
            target: "tenga",
            explanation: "yo tengo -> drop -o -> add -a -> tenga",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo salgo -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "Es mejor que yo ____ temprano para evitar el tráfico.",
            target_verb: "salir",
            target_subject: "yo",
            target: "salga",
            explanation: "yo salgo -> drop -o -> add -a -> salga",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo pongo -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "Dudo que yo ____ los libros en la mesa equivocada.",
            target_verb: "poner",
            target_subject: "yo",
            target: "ponga",
            explanation: "yo pongo -> drop -o -> add -a -> ponga",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo digo -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "No creo que yo ____ nada inapropiado en la reunión.",
            target_verb: "decir",
            target_subject: "yo",
            target: "diga",
            explanation: "yo digo -> drop -o -> add -a -> diga",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo hago -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "Espero que yo ____ un buen trabajo en la presentación.",
            target_verb: "hacer",
            target_subject: "yo",
            target: "haga",
            explanation: "yo hago -> drop -o -> add -a -> haga",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo veo -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "No es seguro que yo ____ la nueva película esta noche.",
            target_verb: "ver",
            target_subject: "yo",
            target: "vea",
            explanation: "yo veo -> drop -o -> add -a -> vea",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive: quep- + -a",
            trigger_sentence: "Dudo que mi equipaje ____ en el compartimento superior.",
            target_verb: "caber",
            target_subject: "él/ella",
            target: "quepa",
            explanation: "caber -> quepa, quepas, quepa, quepamos, quepáis, quepan",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive: vay- + -a",
            trigger_sentence: "Mi jefe quiere que yo ____ a la sucursal central.",
            target_verb: "ir",
            target_subject: "yo",
            target: "vaya",
            explanation: "ir -> vaya, vayas, vaya, vayamos, vayáis, vayan",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive: sep- + -a",
            trigger_sentence: "Es imposible que yo ____ todas las respuestas de memoria.",
            target_verb: "saber",
            target_subject: "yo",
            target: "sepa",
            explanation: "saber -> sepa, sepas, sepa, sepamos, sepáis, sepan",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive: se- + -a",
            trigger_sentence: "No creo que la solución ____ tan complicada como parece.",
            target_verb: "ser",
            target_subject: "ella",
            target: "sea",
            explanation: "ser -> sea, seas, sea, seamos, seáis, sean",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive: hay- + -a",
            trigger_sentence: "Espero que ____ suficientes plazas disponibles.",
            target_verb: "haber",
            target_subject: "impersonal",
            target: "haya",
            explanation: "haber -> haya, hayas, haya, hayamos, hayáis, hayan",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive with accent: dé",
            trigger_sentence: "Ojalá el profesor me ____ una extensión para la entrega.",
            target_verb: "dar",
            target_subject: "él/ella",
            target: "dé",
            explanation: "dar -> dé (accent distinguishes from preposition 'de')",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "irregular subjunctive with accent: esté",
            trigger_sentence: "Dudo que el servidor ____ listo antes del mediodía.",
            target_verb: "estar",
            target_subject: "él",
            target: "esté",
            explanation: "estar -> esté, estés, esté, estemos, estéis, estén",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo oigo -> drop -o -> opposite vowel '-a'",
            trigger_sentence: "Habla más alto para que yo te ____ con claridad.",
            target_verb: "oír",
            target_subject: "yo",
            target: "oiga",
            explanation: "yo oigo -> drop -o -> oiga",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "yo drop -o -> opposite vowel '-a'",
            trigger_sentence: "Busco a alguien que ____ bien el framework Axum.",
            target_verb: "conocer",
            target_subject: "él/ella",
            target: "conozca",
            explanation: "yo conozco -> drop -o -> conozca",
        },
        // ----------------------------------------------------
        // 3. Imperfect Subjunctive (-ra) Forms
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "3rd pl preterite tuvieron -> drop -ron -> add -ra",
            trigger_sentence: "Si yo ____ más tiempo libre, estudiaría otro idioma.",
            target_verb: "tener",
            target_subject: "yo",
            target: "tuviera",
            explanation: "tuvieron -> drop -ron -> add -ra -> tuviera",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "3rd pl preterite hicieron -> drop -ron -> add -ra",
            trigger_sentence: "Si yo ____ buen tiempo este fin de semana, iría a la playa.",
            target_verb: "hacer",
            target_subject: "yo",
            target: "hiciera",
            explanation: "hicieron -> drop -ron -> add -ra -> hiciera",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "3rd pl preterite dijeron -> drop -ron -> add -era",
            trigger_sentence: "Si yo te ____ la verdad, no me creerías.",
            target_verb: "decir",
            target_subject: "yo",
            target: "dijera",
            explanation: "dijeron -> drop -ron -> add -era -> dijera",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "3rd pl preterite fueron -> drop -ron -> add -ra",
            trigger_sentence: "Si yo ____ presidente, invertiría más en educación.",
            target_verb: "ser",
            target_subject: "yo",
            target: "fuera",
            explanation: "fueron -> drop -ron -> add -ra -> fuera",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "3rd pl preterite supieron -> drop -ron -> add -ra",
            trigger_sentence: "Si yo ____ cómo arreglarlo, te ayudaría con gusto.",
            target_verb: "saber",
            target_subject: "yo",
            target: "supiera",
            explanation: "supieron -> drop -ron -> add -ra -> supiera",
        },
        StaticDrillItem {
            topic: "subjunctive",
            formula_cue: "3rd pl preterite pudieron -> drop -ron -> add -ra",
            trigger_sentence: "Ojalá yo ____ acompañarte al concierto mañana.",
            target_verb: "poder",
            target_subject: "yo",
            target: "pudiera",
            explanation: "pudieron -> drop -ron -> add -ra -> pudiera",
        },
        // ----------------------------------------------------
        // 4. Future / Conditional Irregular Stems
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "future",
            formula_cue: "drop vowel + insert 'd' -> tendr-",
            trigger_sentence:
                "El próximo año yo ____ más responsabilidades (yo tendré -> raíz: ____).",
            target_verb: "tener",
            target_subject: "yo",
            target: "tendr",
            explanation: "tener -> tendr- (tendré, tendrás, tendría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "drop vowel + insert 'd' -> pondr-",
            trigger_sentence: "Mañana yo ____ las fotos en la galería (yo pondré -> raíz: ____).",
            target_verb: "poner",
            target_subject: "yo",
            target: "pondr",
            explanation: "poner -> pondr- (pondré, pondrás, pondría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "drop vowel + insert 'd' -> saldr-",
            trigger_sentence: "El tren ____ puntual de la estación (saldrá -> raíz: ____).",
            target_verb: "salir",
            target_subject: "él",
            target: "saldr",
            explanation: "salir -> saldr- (saldré, saldrás, saldría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "drop vowel + insert 'd' -> vendr-",
            trigger_sentence: "Mis amigos ____ a cenar a casa el sábado (vendrán -> raíz: ____).",
            target_verb: "venir",
            target_subject: "ellos",
            target: "vendr",
            explanation: "venir -> vendr- (vendré, vendrás, vendría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "syncopated stem: har-",
            trigger_sentence: "La próxima semana yo ____ el examen final (yo haré -> raíz: ____).",
            target_verb: "hacer",
            target_subject: "yo",
            target: "har",
            explanation: "hacer -> har- (haré, harás, haría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "syncopated stem: dir-",
            trigger_sentence:
                "En la entrevista yo ____ toda mi experiencia (yo diré -> raíz: ____).",
            target_verb: "decir",
            target_subject: "yo",
            target: "dir",
            explanation: "decir -> dir- (diré, dirás, diría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "drop 'e' from infinitive -> sabr-",
            trigger_sentence:
                "Pronto nosotros ____ los resultados de la prueba (sabremos -> raíz: ____).",
            target_verb: "saber",
            target_subject: "nosotros",
            target: "sabr",
            explanation: "saber -> sabr- (sabré, sabrás, sabría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "drop 'e' from infinitive -> podr-",
            trigger_sentence: "¿Tú crees que ____ venir a la reunión? (podrás -> raíz: ____).",
            target_verb: "poder",
            target_subject: "tú",
            target: "podr",
            explanation: "poder -> podr- (podré, podrás, podría)",
        },
        StaticDrillItem {
            topic: "future",
            formula_cue: "double 'r' stem: querr-",
            trigger_sentence: "Ella no ____ perderse el estreno de la obra (querrá -> raíz: ____).",
            target_verb: "querer",
            target_subject: "ella",
            target: "querr",
            explanation: "querer -> querr- (querré, querrás, querría)",
        },
        // ----------------------------------------------------
        // 5. Imperatives (Commands)
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ el favor de cerrar la puerta al salir!",
            target_verb: "hacer",
            target_subject: "tú",
            target: "haz",
            explanation: "hacer -> haz (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ la mesa para la cena, por favor!",
            target_verb: "poner",
            target_subject: "tú",
            target: "pon",
            explanation: "poner -> pon (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ cuidado con el escalón al bajar!",
            target_verb: "tener",
            target_subject: "tú",
            target: "ten",
            explanation: "tener -> ten (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ inmediatamente de la habitación!",
            target_verb: "salir",
            target_subject: "tú",
            target: "sal",
            explanation: "salir -> sal (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ aquí un momento, por favor!",
            target_verb: "venir",
            target_subject: "tú",
            target: "ven",
            explanation: "venir -> ven (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ la verdad sin rodeos!",
            target_verb: "decir",
            target_subject: "tú",
            target: "di",
            explanation: "decir -> di (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command",
            trigger_sentence: "¡____ a comprar el pan antes de que cierren!",
            target_verb: "ir",
            target_subject: "tú",
            target: "ve",
            explanation: "ir -> ve (irregular affirmative informal command)",
        },
        StaticDrillItem {
            topic: "imperative",
            formula_cue: "irregular affirmative informal command with accent: sé",
            trigger_sentence: "¡____ amable con los invitados a la fiesta!",
            target_verb: "ser",
            target_subject: "tú",
            target: "sé",
            explanation: "ser -> sé (with accent mark)",
        },
        // ----------------------------------------------------
        // 6. Por vs Para
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "employment / recipient / destination",
            trigger_sentence: "Trabajo ____ una empresa tecnológica multinacional.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "para",
            explanation: "para = employment destination / recipient",
        },
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "motive / cause / gratitude",
            trigger_sentence: "Muchas gracias ____ tu ayuda con la mudanza.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "por",
            explanation: "por = motive / cause / gratitude",
        },
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "specific deadline / future point in time",
            trigger_sentence: "El informe técnico debe estar listo ____ el viernes.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "para",
            explanation: "para = specific deadline / future point in time",
        },
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "movement through / along / around",
            trigger_sentence: "Paseamos tranquilamente ____ el centro histórico de la ciudad.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "por",
            explanation: "por = movement along / through / around an area",
        },
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "duration of time",
            trigger_sentence: "Estudié en la biblioteca universitaria ____ tres horas.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "por",
            explanation: "por = duration of time",
        },
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "price / monetary exchange",
            trigger_sentence: "Compré los billetes de tren ____ 50 euros.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "por",
            explanation: "por = price / monetary exchange",
        },
        StaticDrillItem {
            topic: "por_para",
            formula_cue: "purpose / goal ('in order to' + infinitive)",
            trigger_sentence: "____ aprender a programar en Rust, practico todos los días.",
            target_verb: "por vs para",
            target_subject: "n/a",
            target: "para",
            explanation: "para + infinitive = purpose / goal in order to",
        },
        // ----------------------------------------------------
        // 7. Ser vs Estar
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "ser_estar",
            formula_cue: "event location / time of event",
            trigger_sentence: "La conferencia anual de tecnología ____ en el auditorio principal.",
            target_verb: "ser vs estar",
            target_subject: "la conferencia",
            target: "es",
            explanation: "ser = location where an event takes place / occurs",
        },
        StaticDrillItem {
            topic: "ser_estar",
            formula_cue: "physical / spatial location of an object",
            trigger_sentence: "El servidor de producción ____ en el centro de datos de Madrid.",
            target_verb: "ser vs estar",
            target_subject: "el servidor",
            target: "está",
            explanation: "estar = physical / spatial location of an object",
        },
        StaticDrillItem {
            topic: "ser_estar",
            formula_cue: "profession / identity / essential trait",
            trigger_sentence: "Daniel ____ arquitecto de software sénior.",
            target_verb: "ser vs estar",
            target_subject: "Daniel",
            target: "es",
            explanation: "ser = profession / identity / essential trait",
        },
        StaticDrillItem {
            topic: "ser_estar",
            formula_cue: "condition / temporary or resulting state",
            trigger_sentence: "La base de datos ____ caída en este momento.",
            target_verb: "ser vs estar",
            target_subject: "la base de datos",
            target: "está",
            explanation: "estar = condition / temporary or resulting state",
        },
        StaticDrillItem {
            topic: "ser_estar",
            formula_cue: "inherent defining quality",
            trigger_sentence: "El hielo de los glaciares ____ frío por naturaleza.",
            target_verb: "ser vs estar",
            target_subject: "el hielo",
            target: "es",
            explanation: "ser = inherent defining quality",
        },
        // ----------------------------------------------------
        // 8. Clitics & Pronouns (Cacophony & Placement)
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "pronouns",
            formula_cue: "cacophony rule: le + lo -> se lo",
            trigger_sentence: "Le doy el libro a Juan -> ____ doy inmediatamente.",
            target_verb: "dar",
            target_subject: "yo",
            target: "se lo",
            explanation: "le + lo -> se lo (cacophony resolution rule)",
        },
        StaticDrillItem {
            topic: "pronouns",
            formula_cue: "cacophony rule: les + las -> se las",
            trigger_sentence: "Les compro las flores a mis padres -> ____ compro hoy.",
            target_verb: "comprar",
            target_subject: "yo",
            target: "se las",
            explanation: "les + las -> se las (cacophony resolution rule)",
        },
        StaticDrillItem {
            topic: "pronouns",
            formula_cue: "cacophony rule: le + la -> se la",
            trigger_sentence: "Le explico la regla gramatical a María -> ____ explico ahora.",
            target_verb: "explicar",
            target_subject: "yo",
            target: "se la",
            explanation: "le + la -> se la",
        },
        // ----------------------------------------------------
        // 9. Prepositions with Verbs (Régimen Preposicional)
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: soñar + con",
            trigger_sentence: "A menudo suelo soñar ____ viajar por todo el mundo.",
            target_verb: "soñar",
            target_subject: "yo",
            target: "con",
            explanation: "soñar con = to dream about/of (always 'con' in Spanish)",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: insistir + en",
            trigger_sentence:
                "El profesor volvió a insistir ____ la importancia de la práctica diaria.",
            target_verb: "insistir",
            target_subject: "el profesor",
            target: "en",
            explanation: "insistir en = to insist on",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: depender + de",
            trigger_sentence:
                "El éxito del lanzamiento va a depender ____ nuestro esfuerzo conjunto.",
            target_verb: "depender",
            target_subject: "el éxito",
            target: "de",
            explanation: "depender de = to depend on (always 'de')",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: negarse + a",
            trigger_sentence: "El sospechoso decidió negarse ____ declarar ante el juez.",
            target_verb: "negarse",
            target_subject: "el sospechoso",
            target: "a",
            explanation: "negarse a = to refuse to do something",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: acordarse + de",
            trigger_sentence: "No logré acordarme ____ su nombre durante la presentación.",
            target_verb: "acordarse",
            target_subject: "yo",
            target: "de",
            explanation: "acordarse de = to remember (vs recordar without preposition)",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: tratar + de",
            trigger_sentence: "Siempre intento tratar ____ resolver los errores con calma.",
            target_verb: "tratar",
            target_subject: "yo",
            target: "de",
            explanation: "tratar de = to try to (+ inf) / to be about",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: contar + con",
            trigger_sentence: "Sabes que siempre puedes contar ____ mi apoyo incondicional.",
            target_verb: "contar",
            target_subject: "tú",
            target: "con",
            explanation: "contar con = to rely on / have available",
        },
        StaticDrillItem {
            topic: "prepositions",
            formula_cue: "régimen: tardar + en",
            trigger_sentence: "El equipo suele tardar dos semanas ____ completar el sprint.",
            target_verb: "tardar",
            target_subject: "el equipo",
            target: "en",
            explanation: "tardar en (+ inf) = to take time to do something",
        },
        // ----------------------------------------------------
        // 10. Accidental 'Se'
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "accidental_se",
            formula_cue: "accidental se: [se] + [IOP 1st sing 'me'] + [verbo]",
            trigger_sentence:
                "I dropped the glass -> Se ____ cayó el vaso al suelo accidentalmente.",
            target_verb: "caer",
            target_subject: "el vaso",
            target: "me",
            explanation: "se me cayó = I accidentally dropped it",
        },
        StaticDrillItem {
            topic: "accidental_se",
            formula_cue: "accidental se: [se] + [IOP 1st plur 'nos'] + [verbo]",
            trigger_sentence:
                "We forgot the tickets -> Se ____ olvidaron las entradas para el concierto.",
            target_verb: "olvidar",
            target_subject: "las entradas",
            target: "nos",
            explanation: "se nos olvidaron = we accidentally forgot them",
        },
        StaticDrillItem {
            topic: "accidental_se",
            formula_cue: "accidental se: [se] + [IOP 3rd sing 'le'] + [verbo]",
            trigger_sentence:
                "He lost the keys -> Se ____ perdieron las llaves de casa en el parque.",
            target_verb: "perder",
            target_subject: "las llaves",
            target: "le",
            explanation: "se le perdieron = he accidentally lost them",
        },
        // ----------------------------------------------------
        // 11. False Friends & Cognate Traps
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: currently -> actualmente (not 'actually')",
            trigger_sentence: "____ resido en Barcelona por motivos laborales (currently).",
            target_verb: "actualmente",
            target_subject: "n/a",
            target: "actualmente",
            explanation: "actualmente = currently; en realidad / de hecho = actually",
        },
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: to pretend -> fingir (not 'pretender')",
            trigger_sentence: "No es sano ____ estar de acuerdo cuando no lo estás (to pretend).",
            target_verb: "fingir",
            target_subject: "n/a",
            target: "fingir",
            explanation: "fingir = to pretend; pretender = to intend/aspire",
        },
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: sensible / prudent -> sensato (not 'sensible')",
            trigger_sentence: "Tomó una decisión muy ____ y prudente ante la crisis (sensible).",
            target_verb: "sensato",
            target_subject: "n/a",
            target: "sensato",
            explanation: "sensato = sensible/prudent; sensible = sensitive",
        },
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: to record -> grabar (not 'recordar')",
            trigger_sentence:
                "Vamos a ____ un nuevo episodio del podcast hoy (to record audio/video).",
            target_verb: "grabar",
            target_subject: "n/a",
            target: "grabar",
            explanation: "grabar = to record; recordar = to remember",
        },
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: success -> éxito (not 'suceso')",
            trigger_sentence:
                "El lanzamiento del producto fue un rotundo ____ comercial (success).",
            target_verb: "éxito",
            target_subject: "n/a",
            target: "éxito",
            explanation: "éxito = success; suceso = event/incident",
        },
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: folder/binder -> carpeta (not 'carpet')",
            trigger_sentence: "Guarda todos los recibos en esta ____ azul (folder/binder).",
            target_verb: "carpeta",
            target_subject: "n/a",
            target: "carpeta",
            explanation: "carpeta = folder; alfombra = carpet",
        },
        StaticDrillItem {
            topic: "false_friends",
            formula_cue: "false friend: congested/head cold -> constipado",
            trigger_sentence:
                "Tengo congestión nasal y tos porque estoy ____ (having a head cold).",
            target_verb: "constipado",
            target_subject: "n/a",
            target: "constipado",
            explanation: "constipado = congested/head cold; estreñido = constipated",
        },
        // ----------------------------------------------------
        // 12. Idioms & Collocations
        // ----------------------------------------------------
        StaticDrillItem {
            topic: "idioms",
            formula_cue: "idiom: dar POR sentado",
            trigger_sentence: "No deberías dar ____ sentado que todos conocen la respuesta.",
            target_verb: "dar por sentado",
            target_subject: "n/a",
            target: "por",
            explanation: "dar por sentado = to take for granted",
        },
        StaticDrillItem {
            topic: "idioms",
            formula_cue: "idiom: tener EN cuenta",
            trigger_sentence: "Es fundamental tener ____ cuenta los requisitos de seguridad.",
            target_verb: "tener en cuenta",
            target_subject: "n/a",
            target: "en",
            explanation: "tener en cuenta = to take into account / keep in mind",
        },
        StaticDrillItem {
            topic: "idioms",
            formula_cue: "idiom: echar DE menos",
            trigger_sentence:
                "Cuando viajo al extranjero siempre suelo echar ____ menos la comida de casa.",
            target_verb: "echar de menos",
            target_subject: "n/a",
            target: "de",
            explanation: "echar de menos = to miss",
        },
        StaticDrillItem {
            topic: "idioms",
            formula_cue: "idiom: valer LA pena",
            trigger_sentence: "El esfuerzo invertido va a valer ____ pena al final del curso.",
            target_verb: "valer la pena",
            target_subject: "n/a",
            target: "la",
            explanation: "valer la pena = to be worth it",
        },
        StaticDrillItem {
            topic: "idioms",
            formula_cue: "idiom: llevar A cabo",
            trigger_sentence:
                "El equipo logró llevar ____ cabo la migración del sistema con éxito.",
            target_verb: "llevar a cabo",
            target_subject: "n/a",
            target: "a",
            explanation: "llevar a cabo = to execute / carry out",
        },
    ];

    let filtered_static: Vec<StaticDrillItem> = if let Some(filt) = topic_filter {
        let f = filt.to_lowercase().replace('_', "-");
        if f == "all" || f.is_empty() {
            all_items
        } else {
            all_items
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
                .collect()
        }
    } else {
        all_items
    };

    filtered_static
        .into_iter()
        .map(|item| DrillItem {
            topic: item.topic.to_string(),
            formula_cue: item.formula_cue.to_string(),
            trigger_sentence: item.trigger_sentence.to_string(),
            target_verb: item.target_verb.to_string(),
            target_subject: item.target_subject.to_string(),
            target: item.target.to_string(),
            explanation: item.explanation.to_string(),
        })
        .collect()
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
    weak: bool,
    level: Option<&str>,
    track: Option<usize>,
    strict_accents: bool,
) -> anyhow::Result<()> {
    let parsed_level = level
        .map(|l| l.parse::<crate::core::curriculum::Level>())
        .transpose()?;
    let mut state = crate::core::state::AppState::load().unwrap_or_default();
    let initial_masteries = state.get_concept_mastery_scores();

    let chosen_topic = concept.or(topic).map(|s| s.to_string());
    let num_questions = count.unwrap_or(5);
    let filter = DrillFilter {
        weak_only: weak,
        topic: chosen_topic.clone(),
        level: parsed_level,
        track,
        count: num_questions,
    };

    let items = select_drill_items(&state, filter);
    if items.is_empty() {
        println!(
            "{}",
            "No drill questions found for the given criteria.".yellow()
        );
        return Ok(());
    }

    let topic_display = if weak {
        "Weakest Concepts (Adaptive)".to_string()
    } else if let Some(ref t) = chosen_topic {
        t.clone()
    } else if let Some(ref lvl) = parsed_level {
        format!("Level {}", lvl)
    } else if let Some(tr) = track {
        format!("Track {}", tr)
    } else {
        "All Topics (Mixed)".to_string()
    };

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
        "Topic: {} ({} questions). Type your answer and press Enter. (Type '?' or 'hint' for live hint)\n",
        topic_display.cyan().bold(),
        items.len().to_string().yellow().bold()
    );

    if let Some(ref t) = chosen_topic {
        if let Some(sheet) = get_topic_cheat_sheet(t) {
            println!("{}", "--- [TOPIC CHEAT SHEET] ---".yellow().bold());
            println!("{}\n", sheet.cyan());
            println!("{}", "---------------------------".yellow());
        }
    }

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut score = 0;
    let mut attempted_topics = Vec::new();

    'questions: for (i, item) in items.iter().enumerate() {
        println!("{}", item.format_prompt(i + 1, items.len()));
        let mut hint_used = false;

        loop {
            print!("Answer > ");
            io::stdout().flush()?;

            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                // EOF reached (e.g. non-interactive pipeline or Ctrl+D)
                println!();
                break 'questions;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.eq_ignore_ascii_case("?") || trimmed.eq_ignore_ascii_case("hint") {
                hint_used = true;
                println!("  {}\n", item.format_hint().yellow());
                continue;
            }

            let now = chrono::Utc::now();
            attempted_topics.push(item.topic.clone());

            match evaluate_drill_answer(item, trimmed, strict_accents) {
                DrillEvaluation::Correct => {
                    println!("  {} Correct!\n", "✓".green().bold());
                    score += 1;
                    let quality = if hint_used { 3 } else { 5 };
                    state.update_concept_mastery(&item.topic, quality, now);
                    let _ = state.save();
                }
                DrillEvaluation::Forgiven { expected, tip } => {
                    println!(
                        "  {} Correct! ({}) [Target: {}]\n",
                        "✓".green().bold(),
                        tip.yellow(),
                        expected.green().bold()
                    );
                    score += 1;
                    let quality = if hint_used { 3 } else { 5 };
                    state.update_concept_mastery(&item.topic, quality, now);
                    let _ = state.save();
                }
                DrillEvaluation::Incorrect => {
                    println!(
                        "  {} Incorrect. Expected: '{}' ({})\n",
                        "✗".red().bold(),
                        item.target.green().bold(),
                        item.explanation.dimmed()
                    );
                    state.update_concept_mastery(&item.topic, 1, now);
                    let _ = state.save();
                }
            }
            break;
        }
    }

    println!(
        "{}",
        "==========================================================".blue()
    );
    println!(
        "Drill Finished! Score: {} / {} ({:.0}%)",
        score.to_string().green().bold(),
        items.len(),
        if !items.is_empty() {
            (score as f64 / items.len() as f64) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "{}",
        "==========================================================".blue()
    );

    let final_masteries = state.get_concept_mastery_scores();
    let touched_topics: std::collections::BTreeSet<String> = attempted_topics.into_iter().collect();
    if !touched_topics.is_empty() {
        println!("\n📊 Concept Mastery Progress:");
        for topic in &touched_topics {
            let old_score = initial_masteries.get(topic).copied().unwrap_or(0.0);
            let new_score = final_masteries.get(topic).copied().unwrap_or(0.0);
            let old_pct = (old_score * 100.0).round() as i32;
            let new_pct = (new_score * 100.0).round() as i32;
            let delta = new_pct - old_pct;
            let title = if let Some(concept) = crate::core::reference::get_grammar_concept(topic) {
                concept.title.to_string()
            } else {
                let parts: Vec<String> = topic
                    .split(['_', '-'])
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect();
                parts.join(" ")
            };
            let delta_str = if delta > 0 {
                format!("(+{}%)", delta).green().bold()
            } else if delta < 0 {
                format!("({}%)", delta).red().bold()
            } else {
                "(±0%)".dimmed()
            };
            println!(
                "  • {:<16} {:>3}% ➔ {:>3}% {}",
                format!("{}:", title),
                format!("{}%", old_pct),
                format!("{}%", new_pct),
                delta_str
            );
        }
    }

    Ok(())
}
