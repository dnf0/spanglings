use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillItem {
    pub topic: String,
    pub formula_cue: String,
    pub trigger_sentence: String,
    pub target_verb: String,
    pub target_subject: String,
    pub target: String,
    pub explanation: String,
    #[serde(default)]
    pub plain_english: String,
}

impl DrillItem {
    pub fn format_prompt(&self, current: usize, total: usize) -> String {
        let concept_header =
            if let Some(concept) = crate::core::reference::get_grammar_concept(&self.topic) {
                if !concept.gloss.is_empty() {
                    format!("{} ({})", concept.title, concept.gloss)
                } else {
                    concept.title.to_string()
                }
            } else if self.topic.is_empty() {
                String::new()
            } else {
                let parts: Vec<String> = self
                    .topic
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

        let badge = if self.formula_cue.is_empty() {
            format!("[{concept_header}]")
        } else if concept_header.is_empty() {
            format!("[{}]", self.formula_cue)
        } else {
            format!("[{concept_header} | {}]", self.formula_cue)
        };

        format!(
            "Q{}/{} {}\nSentence: \"{}\" (verb: {} | subject: {})",
            current, total, badge, self.trigger_sentence, self.target_verb, self.target_subject
        )
    }

    pub fn format_hint(&self) -> String {
        format!("💡 Hint: {}", self.explanation)
    }
}

#[derive(Debug, Clone)]
pub struct SentenceFrame {
    pub topic: &'static str,
    pub formula_cue: &'static str,
    pub template: &'static str,
    pub target_verb: &'static str,
    pub target_subject: &'static str,
    pub target: &'static str,
    pub explanation: &'static str,
    pub plain_english: &'static str,
    pub slots: &'static [(&'static str, &'static [&'static str])],
}

impl SentenceFrame {
    pub fn render<R: Rng + ?Sized>(&self, rng: &mut R) -> DrillItem {
        let mut sentence = self.template.to_string();
        let mut formula = self.formula_cue.to_string();
        let mut target = self.target.to_string();
        let mut verb = self.target_verb.to_string();
        let mut subject = self.target_subject.to_string();
        let mut explanation = self.explanation.to_string();
        let mut plain_english = self.plain_english.to_string();

        for &(slot, options) in self.slots {
            if !options.is_empty() {
                let chosen = options[rng.gen_range(0..options.len())];
                let token = format!("{{{}}}", slot);
                sentence = sentence.replace(&token, chosen);
                formula = formula.replace(&token, chosen);
                target = target.replace(&token, chosen);
                verb = verb.replace(&token, chosen);
                subject = subject.replace(&token, chosen);
                explanation = explanation.replace(&token, chosen);
                plain_english = plain_english.replace(&token, chosen);
            }
        }

        DrillItem {
            topic: self.topic.to_string(),
            formula_cue: formula,
            trigger_sentence: sentence,
            target_verb: verb,
            target_subject: subject,
            target,
            explanation,
            plain_english,
        }
    }
}

pub static FRAMES: &[SentenceFrame] = &[
    // =========================================================================
    // 1. subjunctive
    // =========================================================================
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "yo pongo -> drop -o -> opposite vowel -a",
        template: "{opener} que yo ____ {item} en el directorio correcto.",
        target_verb: "poner",
        target_subject: "yo",
        target: "ponga",
        explanation: "poner in present subjunctive: yo pongo -> drop -o -> ponga",
        plain_english: "Expresses uncertainty, preference, or necessity rather than an established fact.",
        slots: &[
            ("opener", &["Dudo", "No creo", "Espero", "Es necesario", "Es mejor"]),
            ("item", &["los archivos de configuración", "los certificados", "los scripts", "la documentación"]),
        ],
    },
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "yo salgo -> drop -o -> opposite vowel -a",
        template: "{opener} que nosotros ____ {time} para evitar el tráfico de red.",
        target_verb: "salir",
        target_subject: "nosotros",
        target: "salgamos",
        explanation: "salir in present subjunctive: salgamos",
        plain_english: "Expresses a proposal, recommendation, or desired future outcome.",
        slots: &[
            ("opener", &["Es conveniente", "Es preferible", "El líder sugiere", "Recomiendo"]),
            ("time", &["temprano", "antes de las cinco", "con antelación", "puntualmente"]),
        ],
    },
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "irregular present subjunctive: sep- + -as",
        template: "{opener} que tú ____ {topic} antes del lanzamiento a producción.",
        target_verb: "saber",
        target_subject: "tú",
        target: "sepas",
        explanation: "saber in present subjunctive: sepa, sepas, sepa, sepamos, sepáis, sepan",
        plain_english: "Expresses a fundamental requirement or subjective expectation.",
        slots: &[
            ("opener", &["Es fundamental", "Espero", "Dudo", "Es importante"]),
            ("topic", &["la contraseña maestra", "el procedimiento de rollback", "el protocolo de cifrado", "la arquitectura completa"]),
        ],
    },
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "irregular present subjunctive with accent: esté",
        template: "Dudo que el servidor principal ____ {state} antes del mediodía.",
        target_verb: "estar",
        target_subject: "el servidor",
        target: "esté",
        explanation: "estar in present subjunctive takes written accent: esté",
        plain_english: "Expresses doubt about whether a condition will be met in time.",
        slots: &[
            ("state", &["listo", "disponible", "completamente operativo", "restablecido"]),
        ],
    },
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "irregular present subjunctive with accent: dé",
        template: "Ojalá la dirección nos ____ {resource} para completar el proyecto.",
        target_verb: "dar",
        target_subject: "la dirección",
        target: "dé",
        explanation: "dar in present subjunctive: dé (with accent mark)",
        plain_english: "Expresses an earnest wish or hope ('ojalá') for a favorable outcome.",
        slots: &[
            ("resource", &["luz verde", "más tiempo", "presupuesto adicional", "su visto bueno"]),
        ],
    },
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "imperfect subjunctive: tuvieron -> drop -ron -> add -ra",
        template: "Si yo ____ más {resource}, colaboraría con mucho gusto en la iniciativa.",
        target_verb: "tener",
        target_subject: "yo",
        target: "tuviera",
        explanation: "tener in imperfect subjunctive: tuviera (hypothetical condition with 'si')",
        plain_english: "Imagines an unreal or counterfactual hypothetical scenario ('if I had...').",
        slots: &[
            ("resource", &["tiempo libre", "ancho de banda", "experiencia en Rust", "recursos"]),
        ],
    },
    SentenceFrame {
        topic: "subjunctive",
        formula_cue: "irregular subjunctive: vay- + -as",
        template: "Es imprescindible que tú ____ a {event} mañana por la mañana.",
        target_verb: "ir",
        target_subject: "tú",
        target: "vayas",
        explanation: "ir in present subjunctive: vaya, vayas, vaya, vayamos, vayáis, vayan",
        plain_english: "Expresses an essential requirement for someone's attendance.",
        slots: &[
            ("event", &["la reunión de sincronización", "la presentación del cliente", "la sesión de arquitectura", "la oficina central"]),
        ],
    },

    // =========================================================================
    // 2. por-para
    // =========================================================================
    SentenceFrame {
        topic: "por-para",
        formula_cue: "employment / recipient / destination -> para",
        template: "Trabajo ____ {company} desarrollando servicios en la nube.",
        target_verb: "por vs para",
        target_subject: "n/a",
        target: "para",
        explanation: "para denotes employer, recipient, or intended destination",
        plain_english: "Identifies the recipient, employer, or direct beneficiary of the work.",
        slots: &[
            ("company", &["una empresa multinacional", "una consultora internacional", "un banco digital", "una startup tecnológica"]),
        ],
    },
    SentenceFrame {
        topic: "por-para",
        formula_cue: "cause / motive / gratitude -> por",
        template: "Muchas gracias ____ {reason}.",
        target_verb: "por vs para",
        target_subject: "n/a",
        target: "por",
        explanation: "por expresses cause, motive, or gratitude (dar las gracias por algo)",
        plain_english: "Identifies the underlying reason, motive, or cause for gratitude.",
        slots: &[
            ("reason", &["tu colaboración en el sprint", "la rápida respuesta", "revisar la solicitud de extracción", "tu valioso tiempo"]),
        ],
    },
    SentenceFrame {
        topic: "por-para",
        formula_cue: "specific deadline / future timeframe -> para",
        template: "El informe de rendimiento debe estar listo ____ {deadline}.",
        target_verb: "por vs para",
        target_subject: "n/a",
        target: "para",
        explanation: "para indicates a definitive deadline or temporal milestone",
        plain_english: "Sets a definitive future deadline or target milestone.",
        slots: &[
            ("deadline", &["el próximo viernes", "el lunes por la mañana", "finales de mes", "la próxima reunión"]),
        ],
    },
    SentenceFrame {
        topic: "por-para",
        formula_cue: "movement through / along / around -> por",
        template: "Paseamos tranquilamente ____ {location} tras finalizar la reunión.",
        target_verb: "por vs para",
        target_subject: "n/a",
        target: "por",
        explanation: "por denotes physical transit through, along, or around an area",
        plain_english: "Describes physical movement through, along, or across a space.",
        slots: &[
            ("location", &["el centro tecnológico", "el paseo fluvial", "las instalaciones del campus", "la zona financiera"]),
        ],
    },
    SentenceFrame {
        topic: "por-para",
        formula_cue: "monetary exchange / price -> por",
        template: "Compré la suscripción anual del servicio ____ {price}.",
        target_verb: "por vs para",
        target_subject: "n/a",
        target: "por",
        explanation: "por indicates economic exchange, price, or barter",
        plain_english: "Expresses monetary price, exchange value, or trade.",
        slots: &[
            ("price", &["cincuenta euros", "un precio muy ventajoso", "cien dólares", "veinte euros al mes"]),
        ],
    },
    SentenceFrame {
        topic: "por-para",
        formula_cue: "purpose / objective ('in order to' + infinitive) -> para",
        template: "____ {objective}, practicamos ejercicios de sintaxis todos los días.",
        target_verb: "por vs para",
        target_subject: "n/a",
        target: "Para",
        explanation: "para + infinitive expresses purpose or goal ('in order to')",
        plain_english: "States the overarching goal or objective ('in order to').",
        slots: &[
            ("objective", &["Para dominar el lenguaje", "Para escribir código más limpio", "Para aprobar la certificación", "Para optimizar el rendimiento"]),
        ],
    },

    // =========================================================================
    // 3. ser-estar
    // =========================================================================
    SentenceFrame {
        topic: "ser-estar",
        formula_cue: "profession / essential identity -> ser",
        template: "{person} ____ {profession} en el equipo de ingeniería.",
        target_verb: "ser vs estar",
        target_subject: "{person}",
        target: "es",
        explanation: "ser is used for identity, profession, and inherent attributes",
        plain_english: "Defines professional identity, role, or inherent characteristic.",
        slots: &[
            ("person", &["Elena", "Carlos", "Marta", "Daniel", "Laura"]),
            ("profession", &["arquitecta de software", "ingeniero de datos", "analista de seguridad", "directora técnica"]),
        ],
    },
    SentenceFrame {
        topic: "ser-estar",
        formula_cue: "condition / temporary or operational state -> estar",
        template: "La base de datos de pruebas ____ {state} en este momento.",
        target_verb: "ser vs estar",
        target_subject: "la base de datos",
        target: "está",
        explanation: "estar expresses state, condition, or temporary operational status",
        plain_english: "Describes temporary condition, status, or operational state.",
        slots: &[
            ("state", &["inactiva", "caída", "en mantenimiento", "bloqueada", "sobrecargada"]),
        ],
    },
    SentenceFrame {
        topic: "ser-estar",
        formula_cue: "event location / where something takes place -> ser",
        template: "La conferencia anual de tecnología ____ en {venue}.",
        target_verb: "ser vs estar",
        target_subject: "la conferencia",
        target: "es",
        explanation: "ser is used to express the location where an event occurs",
        plain_english: "Identifies the location where an organized event takes place.",
        slots: &[
            ("venue", &["el auditorio principal", "el palacio de congresos", "el hotel Victoria", "la sala de actos"]),
        ],
    },
    SentenceFrame {
        topic: "ser-estar",
        formula_cue: "physical / geographical location -> estar",
        template: "El centro de datos principal ____ en {city}.",
        target_verb: "ser vs estar",
        target_subject: "el centro de datos",
        target: "está",
        explanation: "estar denotes spatial or geographical location of entities",
        plain_english: "Pinpoints the physical or geographic location of an entity.",
        slots: &[
            ("city", &["Madrid", "Frankfurt", "Barcelona", "Ámsterdam", "Valencia"]),
        ],
    },
    SentenceFrame {
        topic: "ser-estar",
        formula_cue: "condition: estar listo = ready (vs ser listo = clever)",
        template: "Todos nosotros ya ____ listos para iniciar la demostración en vivo.",
        target_verb: "ser vs estar",
        target_subject: "nosotros",
        target: "estamos",
        explanation: "estar listo = to be ready; ser listo = to be clever/smart",
        plain_english: "Describes readiness as a temporary state (estar listo) rather than cleverness (ser listo).",
        slots: &[],
    },
    SentenceFrame {
        topic: "ser-estar",
        formula_cue: "inherent physical property -> ser",
        template: "El hielo de las cumbres glaciares ____ frío y resbaladizo.",
        target_verb: "ser vs estar",
        target_subject: "el hielo",
        target: "es",
        explanation: "ser defines inherent characteristics and essential properties",
        plain_english: "Describes an inherent, essential physical property.",
        slots: &[],
    },

    // =========================================================================
    // 4. past
    // =========================================================================
    SentenceFrame {
        topic: "past",
        formula_cue: "irregular preterite stem tuv- + -e",
        template: "{time} yo ____ una reunión urgente con los directores.",
        target_verb: "tener",
        target_subject: "yo",
        target: "tuve",
        explanation: "tener in preterite -> yo tuve (completed past event)",
        plain_english: "Describes a completed, punctual past event at a specific point in time.",
        slots: &[
            ("time", &["Ayer por la tarde", "Anoche", "La semana pasada", "El martes pasado"]),
        ],
    },
    SentenceFrame {
        topic: "past",
        formula_cue: "irregular preterite stem pus- + -o",
        template: "{time} el equipo ____ la versión parcheada en el servidor.",
        target_verb: "poner",
        target_subject: "el equipo",
        target: "puso",
        explanation: "poner in preterite -> él/ella/el equipo puso",
        plain_english: "Marks a single, finished past action with defined completion.",
        slots: &[
            ("time", &["Ayer", "Anoche", "El fin de semana pasado", "Esta madrugada"]),
        ],
    },
    SentenceFrame {
        topic: "past",
        formula_cue: "imperfect for background age / mental state",
        template: "Cuando yo ____ {age}, vivía en {city} y aprendía a programar.",
        target_verb: "tener",
        target_subject: "yo",
        target: "tenía",
        explanation: "imperfect tense (tenía) sets the background age or habitual past context",
        plain_english: "Sets ongoing background age and habitual context in the past.",
        slots: &[
            ("age", &["dieciocho años", "veinte años", "joven", "estudiante"]),
            ("city", &["Sevilla", "Granada", "Buenos Aires", "Madrid", "Salamanca"]),
        ],
    },
    SentenceFrame {
        topic: "past",
        formula_cue: "irregular preterite stem sup- + -imos",
        template: "Ayer nosotros ____ el resultado definitivo de las pruebas de estrés.",
        target_verb: "saber",
        target_subject: "nosotros",
        target: "supimos",
        explanation: "saber in preterite (supimos) means 'found out / learned of'",
        plain_english: "Marks the exact moment of discovering or learning new information.",
        slots: &[],
    },
    SentenceFrame {
        topic: "past",
        formula_cue: "irregular preterite j-stem: dij- + -o",
        template: "El ponente ____ toda la verdad sobre el fallo de diseño en la conferencia.",
        target_verb: "decir",
        target_subject: "el ponente",
        target: "dijo",
        explanation: "decir in preterite -> dijo",
        plain_english: "Reports a completed statement made at a specific past moment.",
        slots: &[],
    },
    SentenceFrame {
        topic: "past",
        formula_cue: "irregular preterite stem estuv- + -ieron",
        template: "Los ingenieros ____ trabajando intensamente durante todo el incidente.",
        target_verb: "estar",
        target_subject: "los ingenieros",
        target: "estuvieron",
        explanation: "estar in preterite -> estuvieron",
        plain_english: "Emphasizes the duration of an activity bounded within a specific past timeframe.",
        slots: &[],
    },

    // =========================================================================
    // 5. pronouns
    // =========================================================================
    SentenceFrame {
        topic: "pronouns",
        formula_cue: "cacophony resolution: le + lo -> se lo",
        template: "Le entregué {item} al cliente -> ____ entregué esta misma mañana.",
        target_verb: "entregar",
        target_subject: "yo",
        target: "se lo",
        explanation: "indirect le transforms to se before direct pronoun lo/la/los/las",
        plain_english: "Replaces indirect 'le' with 'se' before 'lo' to avoid awkward duplicate 'l-' sounds.",
        slots: &[
            ("item", &["el informe técnico", "el contrato firmado", "el documento de diseño", "el presupuesto"]),
        ],
    },
    SentenceFrame {
        topic: "pronouns",
        formula_cue: "cacophony resolution: le + la -> se la",
        template: "Le envié {item} a la jefa de proyecto -> ____ envié por correo.",
        target_verb: "enviar",
        target_subject: "yo",
        target: "se la",
        explanation: "le + la transforms into se la",
        plain_english: "Transforms indirect 'le' to 'se' before direct object pronoun 'la'.",
        slots: &[
            ("item", &["la propuesta comercial", "la factura rectificada", "la presentación técnica", "la versión final"]),
        ],
    },
    SentenceFrame {
        topic: "pronouns",
        formula_cue: "cacophony resolution: les + los -> se los",
        template: "Les mostré {item} a los auditores -> ____ mostré en la sesión de revisión.",
        target_verb: "mostrar",
        target_subject: "yo",
        target: "se los",
        explanation: "les + los transforms into se los",
        plain_english: "Replaces indirect 'le' with 'se' before 'lo' to avoid awkward duplicate 'l-' sounds.",
        slots: &[
            ("item", &["los registros del sistema", "los diagramas de flujo", "los datos de telemetría", "los resultados"]),
        ],
    },
    SentenceFrame {
        topic: "pronouns",
        formula_cue: "cacophony resolution: les + las -> se las",
        template: "Les entregué {item} a los usuarios -> ____ entregué a primera hora.",
        target_verb: "entregar",
        target_subject: "yo",
        target: "se las",
        explanation: "les + las transforms into se las",
        plain_english: "Transforms indirect 'le' to 'se' before direct object pronoun 'la'.",
        slots: &[
            ("item", &["las claves de acceso", "las credenciales seguras", "las instrucciones de instalación", "las notas"]),
        ],
    },
    SentenceFrame {
        topic: "pronouns",
        formula_cue: "attached pronoun stacking with accent on infinitive",
        template: "Voy a explicarte el funcionamiento del compilador -> Voy a ____ ahora mismo.",
        target_verb: "explicar",
        target_subject: "yo",
        target: "explicártelo",
        explanation: "attached clitics on infinitive: explicár + te + lo -> explicártelo (esdrújula)",
        plain_english: "Attaches both pronouns to the infinitive, adding a written accent to preserve stress.",
        slots: &[],
    },
    SentenceFrame {
        topic: "pronouns",
        formula_cue: "attached pronoun stacking with accent on imperative",
        template: "¡Entrega el paquete a mí de inmediato! -> ¡____ sin demora!",
        target_verb: "entregar",
        target_subject: "tú",
        target: "Entrégamelo",
        explanation: "affirmative command + me + lo -> Entrégamelo (esdrújula accent)",
        plain_english: "Attaches stacked pronouns to an affirmative command, requiring an accent on the stressed syllable.",
        slots: &[],
    },

    // =========================================================================
    // 6. prepositions
    // =========================================================================
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: soñar + con",
        template: "A menudo suelo soñar ____ {activity}.",
        target_verb: "soñar",
        target_subject: "yo",
        target: "con",
        explanation: "soñar takes preposition 'con' (soñar con algo/alguien)",
        plain_english: "Verb connects to its topic of dreaming or reliance using the fixed preposition 'con'.",
        slots: &[
            ("activity", &["crear un sistema operativo propio", "viajar por todo el mundo", "fundar una empresa de robótica", "escribir un libro"]),
        ],
    },
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: depender + de",
        template: "El éxito de la migración va a depender ____ {factor}.",
        target_verb: "depender",
        target_subject: "el éxito",
        target: "de",
        explanation: "depender takes preposition 'de' (depender de)",
        plain_english: "Verb connects to its determining condition using the preposition 'de'.",
        slots: &[
            ("factor", &["la rigurosidad de las pruebas", "nuestro esfuerzo colectivo", "la estabilidad de la red", "la planificación previa"]),
        ],
    },
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: insistir + en",
        template: "El director técnico insistió ____ {point} durante el standup.",
        target_verb: "insistir",
        target_subject: "el director",
        target: "en",
        explanation: "insistir takes preposition 'en' (insistir en algo)",
        plain_english: "Verb links to the focal point of emphasis using the preposition 'en'.",
        slots: &[
            ("point", &["la importancia de la seguridad", "mantener una alta cobertura de tests", "cumplir los plazos previstos", "la claridad del código"]),
        ],
    },
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: negarse + a",
        template: "El sospechoso decidió negarse ____ {action} ante el tribunal.",
        target_verb: "negarse",
        target_subject: "el sospechoso",
        target: "a",
        explanation: "negarse takes 'a' + infinitive (negarse a hacer algo)",
        plain_english: "Verb connects to a rejected action using the preposition 'a'.",
        slots: &[
            ("action", &["declarar", "firmar el acta", "responder a las preguntas", "entregar las pruebas"]),
        ],
    },
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: acordarse + de",
        template: "No logré acordarme ____ {task} antes de apagar la máquina.",
        target_verb: "acordarse",
        target_subject: "yo",
        target: "de",
        explanation: "acordarse takes 'de' (acordarse de hacer algo vs recordar without prep)",
        plain_english: "Verb connects to its determining condition using the preposition 'de'.",
        slots: &[
            ("task", &["guardar los cambios", "enviar el reporte diario", "cerrar la sesión remota", "actualizar el repositorio"]),
        ],
    },
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: contar + con",
        template: "Sabes que siempre puedes contar ____ {support} en este proyecto.",
        target_verb: "contar",
        target_subject: "tú",
        target: "con",
        explanation: "contar con = to rely on / count on",
        plain_english: "Verb connects to its topic of dreaming or reliance using the fixed preposition 'con'.",
        slots: &[
            ("support", &["nuestro apoyo incondicional", "la colaboración del equipo", "los recursos necesarios", "mi ayuda técnica"]),
        ],
    },
    SentenceFrame {
        topic: "prepositions",
        formula_cue: "régimen preposicional: tardar + en",
        template: "El microservicio suele tardar varios segundos ____ {action}.",
        target_verb: "tardar",
        target_subject: "el microservicio",
        target: "en",
        explanation: "tardar en (+ inf) = to take time to do something",
        plain_english: "Verb links to the focal point of emphasis using the preposition 'en'.",
        slots: &[
            ("action", &["procesar la solicitud", "responder a la consulta", "inicializar la caché", "validar el token"]),
        ],
    },

    // =========================================================================
    // 7. accidental-se
    // =========================================================================
    SentenceFrame {
        topic: "accidental-se",
        formula_cue: "accidental se: [se] + [IOP 1st sing 'me'] + [verbo]",
        template: "I dropped the glass -> Se ____ cayó el vaso al suelo accidentalmente.",
        target_verb: "caer",
        target_subject: "el vaso",
        target: "me",
        explanation: "se me cayó indicates unintentional dropping affecting 1st person",
        plain_english: "Linguistic shield: the item dropped unexpectedly, and you were the affected bystander ('it fell on me').",
        slots: &[],
    },
    SentenceFrame {
        topic: "accidental-se",
        formula_cue: "accidental se: [se] + [IOP 1st plur 'nos'] + [verbo]",
        template: "We forgot the access cards -> Se ____ olvidaron las tarjetas de acceso en casa.",
        target_verb: "olvidar",
        target_subject: "las tarjetas",
        target: "nos",
        explanation: "se nos olvidaron indicates unintentional oversight affecting 1st plural",
        plain_english: "Frames forgetting as an involuntary slip affecting the group ('it slipped our minds').",
        slots: &[],
    },
    SentenceFrame {
        topic: "accidental-se",
        formula_cue: "accidental se: [se] + [IOP 3rd sing 'le'] + [verbo]",
        template: "He lost the keys -> Se ____ perdieron las llaves del coche en el parque.",
        target_verb: "perder",
        target_subject: "las llaves",
        target: "le",
        explanation: "se le perdieron indicates accidental loss affecting 3rd person",
        plain_english: "Removes direct blame by portraying the keys as going missing from him/her.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accidental-se",
        formula_cue: "accidental se: [se] + [IOP 2nd sing 'te'] + [verbo]",
        template: "You accidentally broke the monitor -> Se ____ rompió la pantalla sin querer.",
        target_verb: "romper",
        target_subject: "la pantalla",
        target: "te",
        explanation: "se te rompió expresses involuntary damage affecting 2nd person singular",
        plain_english: "Frames the damage as an accidental occurrence affecting you rather than deliberate breakage.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accidental-se",
        formula_cue: "accidental se: [se] + [IOP 3rd plur 'les'] + [verbo]",
        template: "They burned the food -> Se ____ quemó el almuerzo en el horno por un descuido.",
        target_verb: "quemar",
        target_subject: "el almuerzo",
        target: "les",
        explanation: "se les quemó indicates unintentional burning affecting 3rd plural",
        plain_english: "Portrays burning food as an unintended accident affecting them.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accidental-se",
        formula_cue: "accidental se: [se] + [IOP 1st sing 'me'] + [acabar]",
        template: "My phone battery ran out -> Se ____ acabó la batería en el momento más inoportuno.",
        target_verb: "acabar",
        target_subject: "la batería",
        target: "me",
        explanation: "se me acabó = ran out on me (accidental exhaustion of a resource)",
        plain_english: "Frames running out of battery as an unexpected depletion affecting me.",
        slots: &[],
    },

    // =========================================================================
    // 8. tech-software
    // =========================================================================
    SentenceFrame {
        topic: "tech-software",
        formula_cue: "technical Spanish: to deploy -> desplegar (avoid *deployar)",
        template: "El equipo de operaciones va a ____ la nueva versión del servicio a producción.",
        target_verb: "desplegar",
        target_subject: "el equipo",
        target: "desplegar",
        explanation: "desplegar is the formal Spanish verb for deploy (avoid anglicism *deployar)",
        plain_english: "Uses standard engineering Spanish ('desplegar') instead of the Spanglish borrowing '*deployar'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "tech-software",
        formula_cue: "technical Spanish: to debug -> depurar (avoid *debuggear)",
        template: "Debemos ____ el controlador de red para eliminar las fugas de memoria.",
        target_verb: "depurar",
        target_subject: "nosotros",
        target: "depurar",
        explanation: "depurar is the proper technical Spanish equivalent for debugging code",
        plain_english: "Uses proper technical Spanish ('depurar') for finding and fixing code bugs instead of '*debuggear'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "tech-software",
        formula_cue: "technical terminology: deadlock -> bloqueo mutuo / interbloqueo",
        template: "El proceso concurrente quedó detenido debido a un ____ entre dos recursos compartidos.",
        target_verb: "bloqueo mutuo",
        target_subject: "el proceso",
        target: "bloqueo mutuo",
        explanation: "bloqueo mutuo or interbloqueo is the precise technical term for deadlock",
        plain_english: "Uses standard computing terminology ('bloqueo mutuo' / 'interbloqueo') for deadlock.",
        slots: &[],
    },
    SentenceFrame {
        topic: "tech-software",
        formula_cue: "technical terminology: pull request -> solicitud de extracción",
        template: "He enviado una ____ con las correcciones solicitadas por el revisor.",
        target_verb: "solicitud de extracción",
        target_subject: "yo",
        target: "solicitud de extracción",
        explanation: "solicitud de extracción translates pull request in standard Spanish",
        plain_english: "Translates pull request into precise technical Spanish ('solicitud de extracción').",
        slots: &[],
    },
    SentenceFrame {
        topic: "tech-software",
        formula_cue: "technical terminology: race condition -> condición de carrera",
        template: "El error intermitente fue ocasionado por una ____ en la actualización de estados.",
        target_verb: "condición de carrera",
        target_subject: "el error",
        target: "condición de carrera",
        explanation: "condición de carrera is the standard translation for race condition",
        plain_english: "Uses standard computing terminology ('condición de carrera') for race condition.",
        slots: &[],
    },
    SentenceFrame {
        topic: "tech-software",
        formula_cue: "technical Spanish: fix/remediate -> subsanar (avoid *fixear)",
        template: "El equipo de seguridad logró ____ la vulnerabilidad antes del informe trimestral.",
        target_verb: "subsanar",
        target_subject: "el equipo",
        target: "subsanar",
        explanation: "subsanar o corregir is the formal technical term for remediating/fixing a vulnerability",
        plain_english: "Uses formal engineering terminology ('subsanar' / 'corregir') for remediating a vulnerability.",
        slots: &[],
    },

    // =========================================================================
    // 9. business
    // =========================================================================
    SentenceFrame {
        topic: "business",
        formula_cue: "formal closing formula: a su entera disposición",
        template: "Quedo a su entera ____ para cualquier aclaración respecto a los presupuestos.",
        target_verb: "disposición",
        target_subject: "yo",
        target: "disposición",
        explanation: "quedo a su entera disposición = I remain at your complete disposal",
        plain_english: "Standard diplomatic closing projecting total availability and professional courtesy.",
        slots: &[],
    },
    SentenceFrame {
        topic: "business",
        formula_cue: "formal correspondence: de antemano",
        template: "Agradeciendo de ____ su atención y colaboración, le envío un cordial saludo.",
        target_verb: "antemano",
        target_subject: "n/a",
        target: "antemano",
        explanation: "agradeciendo de antemano = thanking you in advance",
        plain_english: "Formulaic business courtesy expressing advance gratitude ('de antemano').",
        slots: &[],
    },
    SentenceFrame {
        topic: "business",
        formula_cue: "formal correspondence closing: sin otro particular",
        template: "Sin otro ____ por el momento, quedamos a la espera de sus valiosos comentarios.",
        target_verb: "particular",
        target_subject: "nosotros",
        target: "particular",
        explanation: "sin otro particular = without further matters to discuss for now",
        plain_english: "Traditional executive letter closing signaling transition to sign-off ('sin otro particular').",
        slots: &[],
    },
    SentenceFrame {
        topic: "business",
        formula_cue: "business correspondence: to attach -> adjuntar",
        template: "Procedemos a ____ el borrador del acuerdo para su correspondiente rúbrica.",
        target_verb: "adjuntar",
        target_subject: "nosotros",
        target: "adjuntar",
        explanation: "adjuntar = to attach (formal business phrasing)",
        plain_english: "Precise formal correspondence phrasing for attaching legal and commercial documents.",
        slots: &[],
    },
    SentenceFrame {
        topic: "business",
        formula_cue: "negotiations & contracts: en lo que atañe a",
        template: "En lo que ____ a los plazos de entrega, no se contemplan demoras adicionales.",
        target_verb: "atañer",
        target_subject: "n/a",
        target: "atañe",
        explanation: "en lo que atañe a = as far as it concerns / regarding",
        plain_english: "Contractual formula for isolating and discussing a specific term or scope ('en lo que atañe a').",
        slots: &[],
    },
    SentenceFrame {
        topic: "business",
        formula_cue: "financial settlement: saldar la deuda",
        template: "La empresa se comprometió a ____ la deuda pendiente antes del cierre fiscal.",
        target_verb: "saldar",
        target_subject: "la empresa",
        target: "saldar",
        explanation: "saldar = to settle/pay off an outstanding debt or account balance",
        plain_english: "Precise commercial financial terminology for resolving and clearing an outstanding balance.",
        slots: &[],
    },

    // =========================================================================
    // 10. false-friends
    // =========================================================================
    SentenceFrame {
        topic: "false-friends",
        formula_cue: "false friend: currently -> actualmente (not actually)",
        template: "____ resido en {city} por motivos profesionales (currently).",
        target_verb: "actualmente",
        target_subject: "n/a",
        target: "Actualmente",
        explanation: "actualmente = currently; actually = en realidad / de hecho",
        plain_english: "False friend: 'actualmente' means 'currently/at present', while 'actually' is 'en realidad'.",
        slots: &[
            ("city", &["Madrid", "Barcelona", "Valencia", "Sevilla", "Bogotá"]),
        ],
    },
    SentenceFrame {
        topic: "false-friends",
        formula_cue: "false friend: to pretend -> fingir (not pretender)",
        template: "No es sensato ____ haber comprendido la arquitectura si aún existen dudas (to pretend).",
        target_verb: "fingir",
        target_subject: "n/a",
        target: "fingir",
        explanation: "fingir = to pretend; pretender = to aim / intend",
        plain_english: "False friend: 'fingir' means 'to pretend/fake', while 'pretender' means 'to attempt/aim'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "false-friends",
        formula_cue: "false friend: sensible (prudent) -> sensata (not sensible)",
        template: "El comité adoptó una resolución muy ____ y prudente ante los riesgos de mercado (sensible).",
        target_verb: "sensata",
        target_subject: "la resolución",
        target: "sensata",
        explanation: "sensato/a = sensible/prudent; sensible = sensitive/emotional",
        plain_english: "False friend: 'sensato/a' means 'sensible/prudent', while 'sensible' means 'sensitive'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "false-friends",
        formula_cue: "false friend: to realize -> darse cuenta de (not realizar)",
        template: "El ingeniero tardó en ____ el error que causaba la desconexión (to realize).",
        target_verb: "darse cuenta de",
        target_subject: "el ingeniero",
        target: "darse cuenta de",
        explanation: "darse cuenta de = to realize; realizar = to carry out / execute",
        plain_english: "False friend: 'darse cuenta de' means 'to realize/notice', while 'realizar' means 'to carry out'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "false-friends",
        formula_cue: "false friend: to support -> apoyar (not soportar)",
        template: "La fundación acordó ____ económicamente a los proyectos de código abierto (to support).",
        target_verb: "apoyar",
        target_subject: "la fundación",
        target: "apoyar",
        explanation: "apoyar = to support; soportar = to tolerate / endure",
        plain_english: "False friend: 'apoyar' means 'to support/back', while 'soportar' means 'to tolerate/endure'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "false-friends",
        formula_cue: "false friend: success -> éxito (not suceso)",
        template: "La presentación ante los inversores fue un rotundo ____ institucional (success).",
        target_verb: "éxito",
        target_subject: "la presentación",
        target: "éxito",
        explanation: "éxito = success; suceso = event / incident",
        plain_english: "False friend: 'éxito' means 'success', while 'suceso' means 'event/incident'.",
        slots: &[],
    },

    // =========================================================================
    // 11. voseo
    // =========================================================================
    SentenceFrame {
        topic: "voseo",
        formula_cue: "voseo present indicative: hablar -> vos hablás",
        template: "Vos ____ con mucha seguridad durante las presentaciones técnicas.",
        target_verb: "hablar",
        target_subject: "vos",
        target: "hablás",
        explanation: "voseo present of -AR verbs ends in stressed -ás (vos hablás)",
        plain_english: "Rioplatense informal address 'vos' conjugates -AR verbs with a stressed final vowel (hablás).",
        slots: &[],
    },
    SentenceFrame {
        topic: "voseo",
        formula_cue: "voseo present indicative: comer -> vos comés",
        template: "¿Vos ____ empanadas caseras o preferís pedir pizza?",
        target_verb: "comer",
        target_subject: "vos",
        target: "comés",
        explanation: "voseo present of -ER verbs ends in stressed -és (vos comés)",
        plain_english: "Rioplatense 'vos' conjugates -ER verbs with a stressed final vowel (comés).",
        slots: &[],
    },
    SentenceFrame {
        topic: "voseo",
        formula_cue: "voseo present indicative: vivir -> vos vivís",
        template: "¿En qué barrio de la ciudad ____ vos actualmente?",
        target_verb: "vivir",
        target_subject: "vos",
        target: "vivís",
        explanation: "voseo present of -IR verbs ends in stressed -ís (vos vivís)",
        plain_english: "Rioplatense 'vos' conjugates -IR verbs with a stressed final vowel (vivís).",
        slots: &[],
    },
    SentenceFrame {
        topic: "voseo",
        formula_cue: "voseo irregular: ser -> vos sos",
        template: "Vos ____ una persona de absoluta confianza para nuestro equipo.",
        target_verb: "ser",
        target_subject: "vos",
        target: "sos",
        explanation: "ser in voseo present indicative is 'vos sos'",
        plain_english: "Rioplatense irregular present of 'ser' for informal address (vos sos).",
        slots: &[],
    },
    SentenceFrame {
        topic: "voseo",
        formula_cue: "voseo irregular: tener -> vos tenés",
        template: "¿Vos ____ la clave de acceso a la base de datos de pruebas?",
        target_verb: "tener",
        target_subject: "vos",
        target: "tenés",
        explanation: "tener in voseo present indicative is 'vos tenés'",
        plain_english: "Rioplatense 'vos' form keeps the monophthong and stresses the ending (vos tenés).",
        slots: &[],
    },
    SentenceFrame {
        topic: "voseo",
        formula_cue: "voseo affirmative imperative: decir -> ¡Decí!",
        template: "¡____ toda la verdad y contanos qué ocurrió en la sesión!",
        target_verb: "decir",
        target_subject: "vos",
        target: "Decí",
        explanation: "voseo imperative drops '-r' and stresses final vowel (¡Decí!)",
        plain_english: "Rioplatense affirmative imperative drops the infinitive '-r' and stresses the final vowel (¡Decí!).",
        slots: &[],
    },

    // =========================================================================
    // 12. accents
    // =========================================================================
    SentenceFrame {
        topic: "accents",
        formula_cue: "aguda word ending in vowel requires tilde: compró",
        template: "Ayer el desarrollador ____ una nueva estación de trabajo para el laboratorio.",
        target_verb: "comprar",
        target_subject: "el desarrollador",
        target: "compró",
        explanation: "aguda words ending in N, S, or vowel carry a written accent mark (com-PRÓ)",
        plain_english: "Aguda word ending in a vowel carries a written accent mark on the final syllable.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accents",
        formula_cue: "llana word ending in consonant (not N, S) requires tilde: fácil",
        template: "Resolver este cuello de botella en la red no fue nada ____.",
        target_verb: "fácil",
        target_subject: "el problema",
        target: "fácil",
        explanation: "llana words ending in consonants other than N or S take a tilde (FÁ-cil)",
        plain_english: "Llana word ending in a consonant other than N or S takes a written accent.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accents",
        formula_cue: "esdrújula words ALWAYS take an accent mark: gramática",
        template: "Estudiamos con rigor las reglas de ____ formal para el motor de traducción.",
        target_verb: "gramática",
        target_subject: "la gramática",
        target: "gramática",
        explanation: "all esdrújula words carry a tilde on the antepenultimate syllable (gra-MÁ-ti-ca)",
        plain_english: "Esdrújula word stressed on the antepenultimate syllable always takes a written tilde.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accents",
        formula_cue: "diacritical accent: possessive 'tu' vs pronoun 'tú'",
        template: "¿Sabes si ____ colega participará en el taller de optimización?",
        target_verb: "tu vs tú",
        target_subject: "n/a",
        target: "tu",
        explanation: "tu (unaccented) is possessive adjective; tú (accented) is subject pronoun",
        plain_english: "Possessive adjective 'tu' is unaccented, distinguishing it from pronoun 'tú'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accents",
        formula_cue: "diacritical accent: pronoun 'él' vs article 'el'",
        template: "Mañana ____ expondrá los hallazgos de seguridad ante la junta directiva.",
        target_verb: "él vs el",
        target_subject: "él",
        target: "él",
        explanation: "él (accented) is 3rd person subject pronoun; el (unaccented) is article",
        plain_english: "Subject pronoun 'él' takes a diacritical accent to distinguish it from the article 'el'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "accents",
        formula_cue: "diacritical accent: verb 'sé' (saber) vs pronoun 'se'",
        template: "Yo no ____ cómo reproducir este fallo intermitente en local.",
        target_verb: "saber",
        target_subject: "yo",
        target: "sé",
        explanation: "sé (accented) is 1st person present of saber or imperative of ser",
        plain_english: "Verb 'sé' (from saber/ser) carries a diacritical accent to distinguish it from pronoun 'se'.",
        slots: &[],
    },

    // =========================================================================
    // 13. epistemic-conjecture
    // =========================================================================
    SentenceFrame {
        topic: "epistemic-conjecture",
        formula_cue: "present conjecture with futuro simple: ser -> serán",
        template: "¿Qué hora es en este momento? — No llevo reloj, pero ____ {time}.",
        target_verb: "ser",
        target_subject: "las horas",
        target: "serán",
        explanation: "futuro simple expresses conjecture/probability regarding the present moment",
        plain_english: "Uses future tense to speculate about the present moment ('it must be around {time}').",
        slots: &[
            ("time", &["las cuatro de la tarde", "las cinco y media", "las tres", "las seis aproximadamente"]),
        ],
    },
    SentenceFrame {
        topic: "epistemic-conjecture",
        formula_cue: "present conjecture with futuro simple: estar -> estará",
        template: "¿Dónde está la jefa de ingeniería? — ____ en su oficina atendiendo a un cliente.",
        target_verb: "estar",
        target_subject: "la jefa",
        target: "Estará",
        explanation: "estará expresses speculation or deduction about current whereabouts",
        plain_english: "Uses future tense to deduce or guess current location ('she is probably in her office').",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-conjecture",
        formula_cue: "past conjecture with condicional simple: estar -> estaría",
        template: "Ayer el analista no asistió a la reunión; ____ indispuesto en su domicilio.",
        target_verb: "estar",
        target_subject: "el analista",
        target: "estaría",
        explanation: "condicional simple expresses conjecture or probability about past states",
        plain_english: "Uses conditional tense to conjecture about a past state ('he was probably sick').",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-conjecture",
        formula_cue: "past conjecture with condicional simple: costar -> costaría",
        template: "Ese clúster de servidores de alta gama le ____ una fortuna a la compañía.",
        target_verb: "costar",
        target_subject: "el clúster",
        target: "costaría",
        explanation: "costaría expresses past hypothesis/conjecture ('it probably cost...')",
        plain_english: "Uses conditional tense to estimate a past cost ('it must have cost a fortune').",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-conjecture",
        formula_cue: "prior past conjecture with futuro compuesto: haber -> habrá",
        template: "No encontramos a Carlos en el edificio; ya se ____ marchado a su casa.",
        target_verb: "haber",
        target_subject: "Carlos",
        target: "habrá",
        explanation: "futuro compuesto (habrá + participio) conjectures about completed recent events",
        plain_english: "Uses compound future (habrá + participio) to hypothesize about a completed past action.",
        slots: &[],
    },

    // =========================================================================
    // 14. clitic-doubling
    // =========================================================================
    SentenceFrame {
        topic: "clitic-doubling",
        formula_cue: "mandatory left-dislocation indirect object pronoun: les",
        template: "A los nuevos empleados ____ entregamos las credenciales de bienvenida esta mañana.",
        target_verb: "entregar",
        target_subject: "nosotros",
        target: "les",
        explanation: "fronted indirect objects (A los nuevos empleados) require redundant clitic 'les'",
        plain_english: "Fronted indirect object requires a duplicate clitic pronoun ('les') to maintain syntactic harmony.",
        slots: &[],
    },
    SentenceFrame {
        topic: "clitic-doubling",
        formula_cue: "mandatory left-dislocation direct object pronoun: lo",
        template: "A este informe técnico ____ revisé con minucioso cuidado el fin de semana.",
        target_verb: "revisar",
        target_subject: "yo",
        target: "lo",
        explanation: "fronted specific direct object requires redundant clitic 'lo'",
        plain_english: "Fronted specific direct object requires a redundant clitic pronoun ('lo').",
        slots: &[],
    },
    SentenceFrame {
        topic: "clitic-doubling",
        formula_cue: "mandatory left-dislocation feminine direct object: la",
        template: "A la directora de tecnología ____ vimos ayer en el simposio de arquitectura.",
        target_verb: "ver",
        target_subject: "nosotros",
        target: "la",
        explanation: "fronted feminine direct object requires clitic 'la'",
        plain_english: "Fronted feminine direct object requires an obligatory redundant clitic ('la').",
        slots: &[],
    },
    SentenceFrame {
        topic: "clitic-doubling",
        formula_cue: "mandatory dative reduplication with tonic pronoun 'a mí': me",
        template: "A mí ____ parece una solución sumamente elegante, robusta y escalable.",
        target_verb: "parecer",
        target_subject: "la solución",
        target: "me",
        explanation: "prepositional tonic pronoun 'a mí' requires doubling clitic 'me'",
        plain_english: "Tonic prepositional pronoun 'a mí' strictly requires doubling with clitic 'me'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "clitic-doubling",
        formula_cue: "mandatory dative reduplication with tonic pronoun 'a ti': te",
        template: "A ti ____ cuesta adaptarte a los cambios de paradigma del nuevo framework.",
        target_verb: "costar",
        target_subject: "el cambio",
        target: "te",
        explanation: "tonic pronoun 'a ti' requires doubling clitic 'te' with psych-verbs",
        plain_english: "Tonic pronoun 'a ti' with psych-verbs obligatorily requires doubling with 'te'.",
        slots: &[],
    },

    // =========================================================================
    // 15. personal-a
    // =========================================================================
    SentenceFrame {
        topic: "personal-a",
        formula_cue: "personal 'a' mandatory with specific human DO: a + el -> al",
        template: "Ayer contratamos ____ nuevo arquitecto principal de sistemas.",
        target_verb: "contratar",
        target_subject: "nosotros",
        target: "al",
        explanation: "specific human direct object requires personal 'a' (contracted: a + el = al)",
        plain_english: "Marks a specific, known human direct object with the personal 'a' (contracted: a + el = al).",
        slots: &[],
    },
    SentenceFrame {
        topic: "personal-a",
        formula_cue: "personal 'a' before known/specific person: a la",
        template: "Conozco ____ {person} que coordina el equipo de ciberseguridad.",
        target_verb: "conocer",
        target_subject: "yo",
        target: "a la",
        explanation: "specific female human direct object requires preposition 'a la'",
        plain_english: "Marks a specific female professional acting as the direct object with personal 'a'.",
        slots: &[
            ("person", &["ingeniera principal", "directora técnica", "profesora de redes", "investigadora"]),
        ],
    },
    SentenceFrame {
        topic: "personal-a",
        formula_cue: "personal 'a' with personified domestic pet: a + el -> al",
        template: "Por las mañanas saco a pasear ____ perro por el parque municipal.",
        target_verb: "pasear",
        target_subject: "yo",
        target: "al",
        explanation: "domestic pets and companion animals receive personal 'a' (a + el = al)",
        plain_english: "Applies personal 'a' to a loved domestic pet treated as an animate family companion.",
        slots: &[],
    },
    SentenceFrame {
        topic: "personal-a",
        formula_cue: "personal 'a' mandatory before proper human names: a",
        template: "Llamé por teléfono ____ {name} para ultimar los detalles de la presentación.",
        target_verb: "llamar",
        target_subject: "yo",
        target: "a",
        explanation: "proper names of people always take personal 'a' when acting as direct objects",
        plain_english: "Proper names of individuals always require personal 'a' when functioning as direct objects.",
        slots: &[
            ("name", &["Carlos", "Elena", "Daniel", "Marta", "Sofía"]),
        ],
    },

    // =========================================================================
    // 16. gerund-rules
    // =========================================================================
    SentenceFrame {
        topic: "gerund-rules",
        formula_cue: "prohibition of posteriority gerund: use finite verb 'provocó' instead of *provocando",
        template: "El servidor principal falló y ____ la interrupción temporal del servicio a clientes.",
        target_verb: "provocar",
        target_subject: "el fallo",
        target: "provocó",
        explanation: "gerunds cannot express posterior consequences in Spanish; coordinate with past verb 'provocó'",
        plain_english: "Spanish gerunds cannot express a later consequence; coordinate with a finite past verb ('y provocó').",
        slots: &[],
    },
    SentenceFrame {
        topic: "gerund-rules",
        formula_cue: "prohibition of posteriority gerund: use 'causó' instead of *causando",
        template: "El tren de cercanías descarriló y ____ severos retrasos en la línea ferroviaria.",
        target_verb: "causar",
        target_subject: "el tren",
        target: "causó",
        explanation: "avoid gerund of posteriority (*causando); coordinate with finite past verb 'y causó'",
        plain_english: "Avoids gerund of posteriority (*causando) by connecting with a past finite verb ('y causó').",
        slots: &[],
    },
    SentenceFrame {
        topic: "gerund-rules",
        formula_cue: "valid simultaneous manner gerund: hablar -> hablando",
        template: "El ponente entró en la sala de conferencias ____ por teléfono con el organizador.",
        target_verb: "hablar",
        target_subject: "el ponente",
        target: "hablando",
        explanation: "gerund correctly expresses simultaneous manner occurring alongside the main verb",
        plain_english: "Correctly uses gerund to describe an action occurring at the exact same time as entering.",
        slots: &[],
    },
    SentenceFrame {
        topic: "gerund-rules",
        formula_cue: "prohibition of adjectival gerund: use relative clause 'que regula' instead of *regulando",
        template: "El parlamento aprobó una directiva que ____ la privacidad y protección de datos.",
        target_verb: "regular",
        target_subject: "la directiva",
        target: "regula",
        explanation: "Spanish rejects adjectival gerunds (*directiva regulando); use relative clause 'que regula'",
        plain_english: "Spanish rejects adjectival gerunds (*regulando); use a relative clause ('que regula').",
        slots: &[],
    },

    // =========================================================================
    // 17. adversatives
    // =========================================================================
    SentenceFrame {
        topic: "adversatives",
        formula_cue: "adversative coordination: additive contrast -> pero",
        template: "El algoritmo propuesto es altamente complejo, ____ proporciona un rendimiento sobresaliente.",
        target_verb: "conector adversativo",
        target_subject: "n/a",
        target: "pero",
        explanation: "pero introduces qualification or limitation without negating the prior clause",
        plain_english: "Adds a limitation or qualifying contrast without negating the initial statement.",
        slots: &[],
    },
    SentenceFrame {
        topic: "adversatives",
        formula_cue: "adversative substitution with nouns/phrases -> sino",
        template: "No implementamos una arquitectura monolítica en este servicio, ____ un clúster de microservicios.",
        target_verb: "conector adversativo",
        target_subject: "n/a",
        target: "sino",
        explanation: "sino is used after a negative premise to introduce exclusive substitution of phrases",
        plain_english: "Substitutes an alternative phrase directly following a negated clause ('not X, but rather Y').",
        slots: &[],
    },
    SentenceFrame {
        topic: "adversatives",
        formula_cue: "adversative substitution: no el martes, sino el miércoles",
        template: "No viajamos a la sede el martes, ____ el miércoles por la mañana.",
        target_verb: "conector adversativo",
        target_subject: "n/a",
        target: "sino",
        explanation: "sino rectifies an element after a negated premise (no X sino Y)",
        plain_english: "Substitutes an alternative phrase directly following a negated clause ('not X, but rather Y').",
        slots: &[],
    },
    SentenceFrame {
        topic: "adversatives",
        formula_cue: "adversative substitution with conjugated clause -> sino que",
        template: "No solo corregimos el defecto de memoria, ____ rediseñamos por completo el módulo de red.",
        target_verb: "conector adversativo",
        target_subject: "n/a",
        target: "sino que",
        explanation: "sino que introduces an exclusive rectification containing a conjugated verb clause",
        plain_english: "Substitutes a full conjugated verb clause after a negative statement ('not only X, but rather Y').",
        slots: &[],
    },
    SentenceFrame {
        topic: "adversatives",
        formula_cue: "adversative substitution with conjugated clause -> sino que",
        template: "El cliente no rechazó la propuesta comercial, ____ sugirió varias modificaciones constructivas.",
        target_verb: "conector adversativo",
        target_subject: "n/a",
        target: "sino que",
        explanation: "sino que introduces a full conjugated clause after negation",
        plain_english: "Substitutes a full conjugated verb clause after a negative statement ('not only X, but rather Y').",
        slots: &[],
    },

    // =========================================================================
    // 18. legal-subjunctive
    // =========================================================================
    SentenceFrame {
        topic: "legal-subjunctive",
        formula_cue: "legal future subjunctive (-iere): incumplir -> incumpliere",
        template: "Si alguna de las partes contratantes ____ lo estipulado en el presente acuerdo, se resolverá el contrato.",
        target_verb: "incumplir",
        target_subject: "alguna de las partes",
        target: "incumpliere",
        explanation: "future subjunctive (-iere/-are) is mandatory in statutory and contractual formulations",
        plain_english: "Uses statutory future subjunctive (-iere) to specify hypothetical breach of contract.",
        slots: &[],
    },
    SentenceFrame {
        topic: "legal-subjunctive",
        formula_cue: "legal future subjunctive (-iere): cometer -> cometiere",
        template: "Quien ____ infracción grave contra la propiedad intelectual será sancionado con arreglo a derecho.",
        target_verb: "cometer",
        target_subject: "quien",
        target: "cometiere",
        explanation: "statutory conditional clauses in legal codes preserve future subjunctive (-iere)",
        plain_english: "Formal penal code construction preserving future subjunctive for conditional legal offences.",
        slots: &[],
    },
    SentenceFrame {
        topic: "legal-subjunctive",
        formula_cue: "independent optative longing: ¡Quién + imperfect subjunctive (tuviera)!",
        template: "¡Quién ____ veinte años de nuevo para emprender sin tantas ataduras!",
        target_verb: "tener",
        target_subject: "yo",
        target: "tuviera",
        explanation: "¡Quién + imperfect subjunctive expresses counterfactual longing",
        plain_english: "Uses ¡Quién + imperfect subjunctive to express counterfactual wistful longing.",
        slots: &[],
    },
    SentenceFrame {
        topic: "legal-subjunctive",
        formula_cue: "independent benevolent wish: ¡Que + present subjunctive (tengas)!",
        template: "¡Que ____ un viaje muy fructífero y seguro hacia el congreso internacional!",
        target_verb: "tener",
        target_subject: "tú",
        target: "tengas",
        explanation: "¡Que + present subjunctive expresses an independent optative wish",
        plain_english: "Uses ¡Que + present subjunctive as an independent formula for wishing someone well.",
        slots: &[],
    },
    SentenceFrame {
        topic: "legal-subjunctive",
        formula_cue: "fixed optative formula: cueste lo que cueste",
        template: "Defenderemos la calidad técnica del producto, ____ lo que cueste.",
        target_verb: "costar",
        target_subject: "el precio",
        target: "cueste",
        explanation: "fixed concessive-optative formula: cueste lo que cueste",
        plain_english: "Fixed concessive formula balancing two subjunctive forms ('cueste lo que cueste').",
        slots: &[],
    },

    // =========================================================================
    // 19. verbs-of-becoming
    // =========================================================================
    SentenceFrame {
        topic: "verbs-of-becoming",
        formula_cue: "verbs of becoming: ponerse + adj (rapid temporary emotional state)",
        template: "El desarrollador se ____ muy nervioso durante la demostración ante los inversores.",
        target_verb: "ponerse",
        target_subject: "el desarrollador",
        target: "puso",
        explanation: "ponerse indicates a rapid, temporary, involuntary emotional shift",
        plain_english: "Ponerse describes a rapid, involuntary, temporary emotional reaction.",
        slots: &[],
    },
    SentenceFrame {
        topic: "verbs-of-becoming",
        formula_cue: "verbs of becoming: quedarse + adj (resulting state / shock)",
        template: "Todos nosotros nos ____ atónitos ante la noticia de la fusión empresarial.",
        target_verb: "quedarse",
        target_subject: "nosotros",
        target: "quedamos",
        explanation: "quedarse expresses a resulting state of shock, aftermath, or impression",
        plain_english: "Quedarse describes the resulting state of shock or aftermath from unexpected news.",
        slots: &[],
    },
    SentenceFrame {
        topic: "verbs-of-becoming",
        formula_cue: "verbs of becoming: quedarse sin (depletion of resource)",
        template: "El portátil de trabajo se ____ sin batería a mitad de la videoconferencia.",
        target_verb: "quedarse",
        target_subject: "el portátil",
        target: "quedó",
        explanation: "quedarse sin = to run out of a resource",
        plain_english: "Quedarse sin expresses the involuntary loss or depletion of an essential resource.",
        slots: &[],
    },
    SentenceFrame {
        topic: "verbs-of-becoming",
        formula_cue: "verbs of becoming: hacerse + sustantivo (voluntary career transformation)",
        template: "Tras años de estudio y dedicación, ella se ____ especialista en arquitecturas distribuidas.",
        target_verb: "hacerse",
        target_subject: "ella",
        target: "hizo",
        explanation: "hacerse conveys voluntary transformation resulting from sustained effort or career evolution",
        plain_english: "Hacerse conveys a voluntary professional transformation achieved through sustained effort.",
        slots: &[],
    },
    SentenceFrame {
        topic: "verbs-of-becoming",
        formula_cue: "verbs of becoming: volverse + adj (lasting involuntary personality change)",
        template: "Con el paso del tiempo, el auditor se ____ sumamente desconfiado y riguroso.",
        target_verb: "volverse",
        target_subject: "el auditor",
        target: "volvió",
        explanation: "volverse describes a deep, lasting psychological or personality transformation",
        plain_english: "Volverse describes a deep, lasting shift in personality or mindset over time.",
        slots: &[],
    },
    SentenceFrame {
        topic: "verbs-of-becoming",
        formula_cue: "verbs of becoming: convertirse en + sustantivo (radical transformation)",
        template: "La modesta startup se ____ en una corporación multinacional consolidada.",
        target_verb: "convertirse en",
        target_subject: "la startup",
        target: "convirtió",
        explanation: "convertirse en indicates a radical qualitative or categorical metamorphosis",
        plain_english: "Convertirse en indicates a radical categorical metamorphosis from one entity into another.",
        slots: &[],
    },

    // =========================================================================
    // 20. epistemic-adverbs
    // =========================================================================
    SentenceFrame {
        topic: "epistemic-adverbs",
        formula_cue: "epistemic adverb: 'a lo mejor' requires mandatory indicative: tiene",
        template: "A lo mejor el administrador ____ la copia de seguridad de la base de datos.",
        target_verb: "tener",
        target_subject: "el administrador",
        target: "tiene",
        explanation: "colloquial epistemic adverbs 'a lo mejor', 'igual', 'lo mismo' strictly take indicative",
        plain_english: "Colloquial epistemic adverb 'a lo mejor' strictly requires the indicative mood.",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-adverbs",
        formula_cue: "epistemic adverb: 'igual' requires mandatory indicative: llega",
        template: "Igual el equipo ____ a tiempo para cumplir el hito si optimizamos el pipeline.",
        target_verb: "llegar",
        target_subject: "el equipo",
        target: "llega",
        explanation: "'igual' (perhaps/maybe) takes indicative mood exclusively",
        plain_english: "Epistemic adverb 'igual' (maybe/perhaps) exclusively takes indicative mood.",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-adverbs",
        formula_cue: "pre-verbal quizás with uncertainty takes subjunctive: acepte",
        template: "Quizás la contraparte no ____ los términos de la nueva cláusula contractual.",
        target_verb: "aceptar",
        target_subject: "la contraparte",
        target: "acepte",
        explanation: "pre-verbal quizás triggers subjunctive when expressing genuine epistemic doubt",
        plain_english: "Pre-verbal 'quizás' triggers subjunctive mood when conveying genuine doubt.",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-adverbs",
        formula_cue: "pre-verbal tal vez triggers subjunctive: podamos",
        template: "Tal vez nosotros ____ reducir el consumo de memoria en un cuarenta por ciento.",
        target_verb: "poder",
        target_subject: "nosotros",
        target: "podamos",
        explanation: "tal vez with subjective uncertainty selects subjunctive mood",
        plain_english: "Pre-verbal 'tal vez' selects subjunctive mood when expressing subjective possibility.",
        slots: &[],
    },
    SentenceFrame {
        topic: "epistemic-adverbs",
        formula_cue: "post-verbal quizás requires indicative: es",
        template: "El equipo presentará la demo mañana, quizás ____ la decisión más acertada.",
        target_verb: "ser",
        target_subject: "ello",
        target: "es",
        explanation: "when quizás / tal vez follows the verb clause, the indicative mood is grammatically obligatory",
        plain_english: "When 'quizás' appears after the verb phrase, the verb must be in the indicative mood.",
        slots: &[],
    },

    // =========================================================================
    // 21. possessive-datives
    // =========================================================================
    SentenceFrame {
        topic: "possessive-datives",
        formula_cue: "inalienable possession: dative clitic 'me' + definite article 'las manos'",
        template: "Antes de sentarme a comer, siempre ____ lavo las manos con abundante jabón.",
        target_verb: "lavar",
        target_subject: "yo",
        target: "me",
        explanation: "Spanish uses dative clitic + definite article instead of possessive *mis manos",
        plain_english: "Claims ownership of body parts with dative clitic 'me' and definite article instead of '*mis manos'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "possessive-datives",
        formula_cue: "dative of inalienable possession: le rompí la pantalla",
        template: "Sin querer ____ rompí la pantalla del dispositivo a mi colega de trabajo.",
        target_verb: "romper",
        target_subject: "yo",
        target: "le",
        explanation: "dative clitic 'le' marks the affected possessor alongside article 'la pantalla'",
        plain_english: "Marks the affected owner of a broken item using dative clitic 'le' and article 'la pantalla'.",
        slots: &[],
    },
    SentenceFrame {
        topic: "possessive-datives",
        formula_cue: "dative with accidental event: se le cayó el pasaporte",
        template: "A Elena se ____ cayó el pasaporte en el mostrador del aeropuerto.",
        target_verb: "caer",
        target_subject: "el pasaporte",
        target: "le",
        explanation: "dative pronoun 'le' denotes the person whose personal possession is affected",
        plain_english: "Combines accidental 'se' with dative 'le' to identify whose possession fell.",
        slots: &[],
    },
    SentenceFrame {
        topic: "possessive-datives",
        formula_cue: "ethic / affective dative: no me llores",
        template: "¡No ____ llores más y concentrémonos en resolver la incidencia técnica!",
        target_verb: "llorar",
        target_subject: "tú",
        target: "me",
        explanation: "ethic dative clitic 'me' expresses affectionate/personal involvement in the action",
        plain_english: "Affective/ethic dative 'me' adds emotional personal connection ('don't cry on me').",
        slots: &[],
    },
    SentenceFrame {
        topic: "possessive-datives",
        formula_cue: "collective sympathetic clitic: se nos cayó",
        template: "Durante la demostración ante el cliente, se ____ cayó la conexión de red.",
        target_verb: "caer",
        target_subject: "la conexión",
        target: "nos",
        explanation: "sympathetic dative 'nos' reflects collective shared impact",
        plain_english: "Sympathetic dative 'nos' reflects collective shared impact on the whole team.",
        slots: &[],
    },

    // =========================================================================
    // 22. corrective-polarity
    // =========================================================================
    SentenceFrame {
        topic: "corrective-polarity",
        formula_cue: "corrective polarity: No es que [Subjuntivo] ... sino que [Indicativo]",
        template: "No es que yo no ____ ayudarte con la refactorización, sino que carezco de tiempo libre.",
        target_verb: "querer",
        target_subject: "yo",
        target: "quiera",
        explanation: "the rejected hypothesis after 'no es que' requires subjunctive mood (quiera)",
        plain_english: "Subjunctive after 'no es que' rejects a false assumption before stating the real reason in the indicative.",
        slots: &[],
    },
    SentenceFrame {
        topic: "corrective-polarity",
        formula_cue: "corrective polarity: No es que [sea] ... sino que [faltan]",
        template: "No es que la propuesta ____ desacertada, sino que faltan recursos presupuestarios.",
        target_verb: "ser",
        target_subject: "la propuesta",
        target: "sea",
        explanation: "'no es que' rejects a proposition and mandates subjunctive (sea)",
        plain_english: "Rejects an alleged flaw in the subjunctive ('no es que sea...') to assert lack of time in indicative.",
        slots: &[],
    },
    SentenceFrame {
        topic: "corrective-polarity",
        formula_cue: "rejected cause: No porque [Subjuntivo] ... significa [Indicativo]",
        template: "No porque el framework ____ más popular significa que sea el más adecuado para este caso.",
        target_verb: "ser",
        target_subject: "el framework",
        target: "sea",
        explanation: "'no porque' introduces a rejected causal reason and requires subjunctive (sea)",
        plain_english: "'No porque' rejects an alleged causal factor using the subjunctive mood.",
        slots: &[],
    },
    SentenceFrame {
        topic: "corrective-polarity",
        formula_cue: "rejected cause with subjunctive: No porque tú me lo [pidas]",
        template: "No porque tú me lo ____ voy a omitir las comprobaciones de seguridad exigidas.",
        target_verb: "pedir",
        target_subject: "tú",
        target: "pidas",
        explanation: "'no porque' rejects justification and mandates subjunctive (pidas)",
        plain_english: "'No porque' dismisses an excuse or pressure in the subjunctive.",
        slots: &[],
    },
    SentenceFrame {
        topic: "corrective-polarity",
        formula_cue: "consecutive subjunctive formula: de ahí que [Subjuntivo]",
        template: "El proceso agotó la memoria disponible, de ahí que el sistema operativo ____ la aplicación.",
        target_verb: "reiniciar",
        target_subject: "el sistema operativo",
        target: "reiniciara",
        explanation: "the consecutive connector 'de ahí que' strictly governs subjunctive mood (reiniciara)",
        plain_english: "Formal consecutive connector 'de ahí que' strictly governs the subjunctive mood.",
        slots: &[],
    },
    SentenceFrame {
        topic: "corrective-polarity",
        formula_cue: "consecutive subjunctive formula: de ahí que [suspendiera]",
        template: "Hubo serias irregularidades contables, de ahí que el regulador ____ la licencia de operación.",
        target_verb: "suspender",
        target_subject: "el regulador",
        target: "suspendiera",
        explanation: "'de ahí que' mandates subjunctive mood in formal consecutive causal clauses",
        plain_english: "'De ahí que' links an established cause to its formal consequence using subjunctive.",
        slots: &[],
    },

    // =========================================================================
    // 23. participial-absolutes
    // =========================================================================
    SentenceFrame {
        topic: "participial-absolutes",
        formula_cue: "participial absolute: feminine singular agreement (Terminada la sesión)",
        template: "____ la reunión extraordinaria, los directores procedieron a firmar el acta.",
        target_verb: "terminar",
        target_subject: "la reunión",
        target: "Terminada",
        explanation: "participial absolutes agree in gender and number with postposed subject (Terminada la reunión)",
        plain_english: "Preposed absolute participle agrees in gender and number with feminine singular subject ('la reunión').",
        slots: &[],
    },
    SentenceFrame {
        topic: "participial-absolutes",
        formula_cue: "participial absolute: feminine plural agreement (Aprobadas las reformas)",
        template: "____ las directivas de seguridad, el departamento inició su despliegue gradual.",
        target_verb: "aprobar",
        target_subject: "las directivas",
        target: "Aprobadas",
        explanation: "feminine plural agreement with postposed subject (Aprobadas las directivas)",
        plain_english: "Absolute participle agrees in feminine plural with postposed subject ('las directivas').",
        slots: &[],
    },
    SentenceFrame {
        topic: "participial-absolutes",
        formula_cue: "participial absolute: masculine singular agreement (Firmado el acuerdo)",
        template: "____ el contrato mercantil, ambas empresas comenzaron el intercambio técnico.",
        target_verb: "firmar",
        target_subject: "el contrato",
        target: "Firmado",
        explanation: "masculine singular agreement with subject (Firmado el contrato)",
        plain_english: "Absolute participle agrees in masculine singular with subject ('el contrato').",
        slots: &[],
    },
    SentenceFrame {
        topic: "participial-absolutes",
        formula_cue: "participial absolute: masculine plural agreement (Concluidos los ensayos)",
        template: "____ los experimentos de rendimiento, el equipo redactó el informe final.",
        target_verb: "concluir",
        target_subject: "los experimentos",
        target: "Concluidos",
        explanation: "masculine plural agreement with subject (Concluidos los experimentos)",
        plain_english: "Absolute participle agrees in masculine plural with subject ('los experimentos').",
        slots: &[],
    },
    SentenceFrame {
        topic: "participial-absolutes",
        formula_cue: "fixed participial absolute formula: Dicho esto",
        template: "____ esto, pasemos a revisar las métricas de fiabilidad del sistema.",
        target_verb: "decir",
        target_subject: "esto",
        target: "Dicho",
        explanation: "fixed discourse formula: Dicho esto (= habiéndose dicho esto)",
        plain_english: "Fixed participial absolute formula establishing the transition to the next topic ('Dicho esto...').",
        slots: &[],
    },
    SentenceFrame {
        topic: "participial-absolutes",
        formula_cue: "participial absolute causal formula: Visto el resultado",
        template: "____ el resultado de las pruebas de integración, decidimos posponer la entrega.",
        target_verb: "ver",
        target_subject: "el resultado",
        target: "Visto",
        explanation: "participial absolute expressing causal background: Visto el resultado",
        plain_english: "Participial absolute providing causal background for a decision ('Visto el resultado...').",
        slots: &[],
    },

    // =========================================================================
    // 24. scalar-concession
    // =========================================================================
    SentenceFrame {
        topic: "scalar-concession",
        formula_cue: "scalar intensifier with subjunctive: Por mucho que [insista]",
        template: "Por mucho que ____ el cliente, no podemos eludir las auditorías de seguridad.",
        target_verb: "insistir",
        target_subject: "el cliente",
        target: "insista",
        explanation: "scalar concessive 'por mucho que' strictly takes subjunctive mood",
        plain_english: "Scalar intensifier 'por mucho que' tests maximum hypothetical effort in the subjunctive.",
        slots: &[],
    },
    SentenceFrame {
        topic: "scalar-concession",
        formula_cue: "scalar intensifier with subjunctive: Por más que [trabajemos]",
        template: "Por más que ____ horas extras, no podremos abarcar todo el alcance hoy.",
        target_verb: "trabajar",
        target_subject: "nosotros",
        target: "trabajemos",
        explanation: "'por más que' governs subjunctive when expressing extreme or hypothetical effort",
        plain_english: "'Por más que' pushes hypothetical effort to the limit in the subjunctive while the main clause holds.",
        slots: &[],
    },
    SentenceFrame {
        topic: "scalar-concession",
        formula_cue: "scalar intensifier with subjunctive: Por muy [adjetivo] que [sea]",
        template: "Por muy complejo que ____ el desafío de concurrencia, encontraremos una solución.",
        target_verb: "ser",
        target_subject: "el desafío",
        target: "sea",
        explanation: "'por muy + adj + que' governs subjunctive mood (sea)",
        plain_english: "'Por muy + adj + que' governs subjunctive to acknowledge extreme difficulty without yielding.",
        slots: &[],
    },
    SentenceFrame {
        topic: "scalar-concession",
        formula_cue: "complex concessive of risk: aun a riesgo de que [surjan] (subjunctive)",
        template: "Lanzaremos la actualización, aun a riesgo de que ____ incidencias menores en entornos antiguos.",
        target_verb: "surgir",
        target_subject: "las incidencias",
        target: "surjan",
        explanation: "'aun a riesgo de que' introduces hypothetical danger and requires subjunctive",
        plain_english: "'Aun a riesgo de que' introduces hypothetical danger and strictly requires subjunctive.",
        slots: &[],
    },
    SentenceFrame {
        topic: "scalar-concession",
        formula_cue: "concessive of acknowledged fact: aun a sabiendas de que [era] (indicative)",
        template: "Aceptó la dirección técnica, aun a sabiendas de que ____ una tarea de enorme complejidad.",
        target_verb: "ser",
        target_subject: "la tarea",
        target: "era",
        explanation: "'aun a sabiendas de que' introduces an acknowledged certainty and takes indicative",
        plain_english: "'Aun a sabiendas de que' introduces an acknowledged certainty and strictly takes indicative.",
        slots: &[],
    },
    SentenceFrame {
        topic: "scalar-concession",
        formula_cue: "scalar minimal concession: Por poco que [aportes] (subjunctive)",
        template: "Por poco que ____ al proyecto, cualquier contribución comunitaria será bienvenida.",
        target_verb: "aportar",
        target_subject: "tú",
        target: "aportes",
        explanation: "'por poco que' introduces minimal scalar condition and takes subjunctive",
        plain_english: "'Por poco que' frames a minimal scalar contribution using subjunctive mood.",
        slots: &[],
    },
];

fn canonicalize_topic(topic: &str) -> String {
    let clean = topic.trim().to_lowercase().replace('_', "-");
    if let Some(concept) = crate::core::reference::get_grammar_concept(&clean) {
        concept.slug.to_string()
    } else {
        clean
    }
}

pub fn generate_drill_items_for_topic(topic: &str, count: usize) -> Vec<DrillItem> {
    if count == 0 {
        return Vec::new();
    }

    let slug = canonicalize_topic(topic);
    let matching_frames: Vec<&SentenceFrame> = FRAMES.iter().filter(|f| f.topic == slug).collect();

    let pool = if !matching_frames.is_empty() {
        matching_frames
    } else {
        FRAMES.iter().collect()
    };

    let mut rng = rand::thread_rng();
    let mut items = Vec::with_capacity(count);

    for _ in 0..count {
        let frame_idx = rng.gen_range(0..pool.len());
        items.push(pool[frame_idx].render(&mut rng));
    }

    items
}

pub fn generate_random_drill_items(count: usize) -> Vec<DrillItem> {
    if count == 0 {
        return Vec::new();
    }

    let mut rng = rand::thread_rng();
    let mut items = Vec::with_capacity(count);

    for _ in 0..count {
        let frame_idx = rng.gen_range(0..FRAMES.len());
        items.push(FRAMES[frame_idx].render(&mut rng));
    }

    items
}
