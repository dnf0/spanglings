use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::core::conjugator::conjugate_verb;
use crate::core::generator::{DrillItem, FRAMES};

/// Represents one of the 16 high-stakes grammatical showdowns for rapid binary choice drills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShowdownPair {
    // Original 8
    PorPara,
    SerEstar,
    SubjInd,
    PretImp,
    TuUsted,
    LoLe,
    SinoPero,
    ParaQuePorque,
    // Expanded 8
    TenerHaber,
    SaberConocer,
    MuyMucho,
    PedirPreguntar,
    LlevarTraer,
    HaberEstar,
    IrIrse,
    BienBueno,
}

impl ShowdownPair {
    /// Returns the canonical kebab-case slug for the showdown pair.
    pub fn slug(&self) -> &'static str {
        match self {
            ShowdownPair::PorPara => "por-para",
            ShowdownPair::SerEstar => "ser-estar",
            ShowdownPair::SubjInd => "subj-ind",
            ShowdownPair::PretImp => "pret-imp",
            ShowdownPair::TuUsted => "tu-usted",
            ShowdownPair::LoLe => "lo-le",
            ShowdownPair::SinoPero => "sino-pero",
            ShowdownPair::ParaQuePorque => "para-que-porque",
            ShowdownPair::TenerHaber => "tener-haber",
            ShowdownPair::SaberConocer => "saber-conocer",
            ShowdownPair::MuyMucho => "muy-mucho",
            ShowdownPair::PedirPreguntar => "pedir-preguntar",
            ShowdownPair::LlevarTraer => "llevar-traer",
            ShowdownPair::HaberEstar => "haber-estar",
            ShowdownPair::IrIrse => "ir-irse",
            ShowdownPair::BienBueno => "bien-bueno",
        }
    }

    /// Returns the human-readable display title.
    pub fn title(&self) -> &'static str {
        match self {
            ShowdownPair::PorPara => "Por vs. Para",
            ShowdownPair::SerEstar => "Ser vs. Estar",
            ShowdownPair::SubjInd => "Subjunctive vs. Indicative",
            ShowdownPair::PretImp => "Preterite vs. Imperfect",
            ShowdownPair::TuUsted => "Tú vs. Usted",
            ShowdownPair::LoLe => "Direct (Lo/La) vs. Indirect (Le/Les)",
            ShowdownPair::SinoPero => "Sino vs. Pero",
            ShowdownPair::ParaQuePorque => "Para que vs. Porque",
            ShowdownPair::TenerHaber => "Tener vs Haber (\"to have\" / auxiliary / existential)",
            ShowdownPair::SaberConocer => "Saber vs Conocer (\"to know\" facts vs acquaintance)",
            ShowdownPair::MuyMucho => "Muy vs Mucho (adverb vs quantifier)",
            ShowdownPair::PedirPreguntar => "Pedir vs Preguntar (request vs inquire)",
            ShowdownPair::LlevarTraer => "Llevar vs Traer (away vs toward speaker)",
            ShowdownPair::HaberEstar => "Hay/Haber vs Está/Estar (existence vs location)",
            ShowdownPair::IrIrse => "Ir vs Irse (destination vs departure)",
            ShowdownPair::BienBueno => "Bien vs Bueno/Buen (adverb vs adjective)",
        }
    }

    /// Returns the primary binary option labels for the showdown pair (Option 1 / J, Option 2 / K).
    pub fn options(&self) -> (&'static str, &'static str) {
        match self {
            ShowdownPair::PorPara => ("Por", "Para"),
            ShowdownPair::SerEstar => ("Ser", "Estar"),
            ShowdownPair::SubjInd => ("Subjuntivo", "Indicativo"),
            ShowdownPair::PretImp => ("Pretérito", "Imperfecto"),
            ShowdownPair::TuUsted => ("Tú", "Usted"),
            ShowdownPair::LoLe => ("Lo / La", "Le / Les"),
            ShowdownPair::SinoPero => ("Sino", "Pero"),
            ShowdownPair::ParaQuePorque => ("Para que", "Porque"),
            ShowdownPair::TenerHaber => ("Tener", "Haber"),
            ShowdownPair::SaberConocer => ("Saber", "Conocer"),
            ShowdownPair::MuyMucho => ("Muy", "Mucho"),
            ShowdownPair::PedirPreguntar => ("Pedir", "Preguntar"),
            ShowdownPair::LlevarTraer => ("Llevar", "Traer"),
            ShowdownPair::HaberEstar => ("Hay / Haber", "Está / Estar"),
            ShowdownPair::IrIrse => ("Ir", "Irse"),
            ShowdownPair::BienBueno => ("Bien", "Bueno / Buen"),
        }
    }

    /// Returns a short pedagogical description of the contrast.
    pub fn description(&self) -> &'static str {
        match self {
            ShowdownPair::PorPara => {
                "Motive, means & duration (por) vs. purpose, deadline & recipient (para)"
            }
            ShowdownPair::SerEstar => {
                "Inherent nature & identity (ser) vs. dynamic state & location (estar)"
            }
            ShowdownPair::SubjInd => {
                "Doubt, demand & non-fact (subjunctive) vs. certainty & assertion (indicative)"
            }
            ShowdownPair::PretImp => {
                "Completed punctual actions (preterite) vs. background & habit (imperfect)"
            }
            ShowdownPair::TuUsted => {
                "Informal peer address (tú) vs. formal executive address (usted)"
            }
            ShowdownPair::LoLe => {
                "Direct object target (lo/la) vs. recipient/dative beneficiary (le/les)"
            }
            ShowdownPair::SinoPero => {
                "Negative exclusive rectification (sino) vs. adversative contrast (pero)"
            }
            ShowdownPair::ParaQuePorque => {
                "Purpose requiring subjunctive (para que) vs. reason with indicative (porque)"
            }
            ShowdownPair::TenerHaber => {
                "Possession, age & physical states (tener) vs. auxiliary & existential (haber)"
            }
            ShowdownPair::SaberConocer => {
                "Facts, data & skills (saber) vs. acquaintance with people & places (conocer)"
            }
            ShowdownPair::MuyMucho => {
                "Invariable adverb before adjectives/adverbs (muy) vs. quantifier for nouns/verbs (mucho)"
            }
            ShowdownPair::PedirPreguntar => {
                "Requesting objects or actions (pedir) vs. inquiring for information (preguntar)"
            }
            ShowdownPair::LlevarTraer => {
                "Movement away from speaker (llevar) vs. movement toward speaker (traer)"
            }
            ShowdownPair::HaberEstar => {
                "Indefinite existence (hay/haber) vs. specific location & state (está/estar)"
            }
            ShowdownPair::IrIrse => {
                "Movement toward a destination (ir) vs. departure and leaving (irse)"
            }
            ShowdownPair::BienBueno => {
                "Adverb modifying verbs (bien) vs. adjective describing nouns (bueno/buen)"
            }
        }
    }

    /// Parses a string slice into a `ShowdownPair`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<ShowdownPair> {
        let clean = s.trim().to_lowercase().replace(['_', ' '], "-");
        match clean.as_str() {
            "por-para" | "por_para" | "por-vs-para" | "por" | "para" => Some(ShowdownPair::PorPara),
            "ser-estar" | "ser_estar" | "ser-vs-estar" | "ser" | "estar" => {
                Some(ShowdownPair::SerEstar)
            }
            "subj-ind"
            | "subj_ind"
            | "subjuntivo-indicativo"
            | "subjunctive-indicative"
            | "subj-vs-ind"
            | "subjunctive"
            | "subj" => Some(ShowdownPair::SubjInd),
            "pret-imp"
            | "pret_imp"
            | "preterito-imperfecto"
            | "preterite-imperfect"
            | "pret-vs-imp"
            | "past" => Some(ShowdownPair::PretImp),
            "tu-usted" | "tu_usted" | "tú-usted" | "tu-vs-usted" | "voseo-usted" => {
                Some(ShowdownPair::TuUsted)
            }
            "lo-le" | "lo_le" | "direct-indirect" | "directo-indirecto" | "lo-vs-le"
            | "pronouns" => Some(ShowdownPair::LoLe),
            "sino-pero" | "sino_pero" | "pero-sino" | "pero_sino" | "sino-vs-pero"
            | "adversatives" => Some(ShowdownPair::SinoPero),
            "para-que-porque" | "para_que_porque" | "paraque-porque" | "paraque_porque"
            | "para-que-vs-porque" => Some(ShowdownPair::ParaQuePorque),
            "tener-haber" | "tener_haber" | "tener-vs-haber" | "have" | "tener" | "haber" => {
                Some(ShowdownPair::TenerHaber)
            }
            "saber-conocer" | "saber_conocer" | "saber-vs-conocer" | "know" | "saber"
            | "conocer" => Some(ShowdownPair::SaberConocer),
            "muy-mucho" | "muy_mucho" | "muy-vs-mucho" | "very-much" | "very_much" | "very"
            | "much" | "muy" | "mucho" => Some(ShowdownPair::MuyMucho),
            "pedir-preguntar" | "pedir_preguntar" | "pedir-vs-preguntar" | "ask" | "pedir"
            | "preguntar" => Some(ShowdownPair::PedirPreguntar),
            "llevar-traer" | "llevar_traer" | "llevar-vs-traer" | "take-bring" | "take_bring"
            | "llevar" | "traer" => Some(ShowdownPair::LlevarTraer),
            "haber-estar" | "haber_estar" | "haber-vs-estar" | "hay-esta" | "hay_esta"
            | "exist-locate" | "exist_locate" | "estar-loc" => Some(ShowdownPair::HaberEstar),
            "ir-irse" | "ir_irse" | "ir-vs-irse" | "go-leave" | "go_leave" | "ir" | "irse" => {
                Some(ShowdownPair::IrIrse)
            }
            "bien-bueno" | "bien_bueno" | "bien-vs-bueno" | "well-good" | "well_good" | "bien"
            | "bueno" | "buen" => Some(ShowdownPair::BienBueno),
            _ => None,
        }
    }
}

impl std::str::FromStr for ShowdownPair {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ShowdownPair::from_str(s).ok_or_else(|| format!("Unknown showdown pair: {}", s))
    }
}

/// Returns all 16 available showdown pairs.
pub fn list_showdown_pairs() -> Vec<ShowdownPair> {
    vec![
        ShowdownPair::PorPara,
        ShowdownPair::SerEstar,
        ShowdownPair::SubjInd,
        ShowdownPair::PretImp,
        ShowdownPair::TuUsted,
        ShowdownPair::LoLe,
        ShowdownPair::SinoPero,
        ShowdownPair::ParaQuePorque,
        ShowdownPair::TenerHaber,
        ShowdownPair::SaberConocer,
        ShowdownPair::MuyMucho,
        ShowdownPair::PedirPreguntar,
        ShowdownPair::LlevarTraer,
        ShowdownPair::HaberEstar,
        ShowdownPair::IrIrse,
        ShowdownPair::BienBueno,
    ]
}

/// Represents a rapid single-key choice question (2 choices for showdown, 4 for cloze).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArcadeItem {
    pub topic: String,
    pub trigger_sentence: String,
    pub prompt_cue: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub explanation: String,
    #[serde(default)]
    pub plain_english: String,
}

impl ArcadeItem {
    /// Returns true if the selected index corresponds to the correct option.
    pub fn is_correct(&self, chosen_index: usize) -> bool {
        chosen_index == self.correct_index
    }

    /// Returns a reference to the correct option string.
    pub fn correct_option(&self) -> &str {
        &self.options[self.correct_index]
    }
}

#[derive(Debug, Clone)]
struct ShowdownSentence {
    sentence: &'static str,
    target: &'static str,
    distractor: &'static str,
    explanation: &'static str,
    plain_english: &'static str,
}

static SHOWDOWN_POR_PARA: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Trabajo ____ una empresa multinacional de software.",
        target: "para",
        distractor: "por",
        explanation: "'Para' specifies the employer or direct beneficiary.",
        plain_english: "Points forward to destination, recipient, deadline, or intended goal ('for/in order to').",
    },
    ShowdownSentence {
        sentence: "Caminamos ____ el centro histórico de la ciudad.",
        target: "por",
        distractor: "para",
        explanation: "'Por' indicates movement through, along, or across a space.",
        plain_english: "Looks back to cause, reason, gratitude, medium, exchange, or movement through ('because of/through').",
    },
    ShowdownSentence {
        sentence: "Este informe técnico debe estar terminado ____ el viernes.",
        target: "para",
        distractor: "por",
        explanation: "'Para' sets a firm deadline or temporal goal.",
        plain_english: "Points forward to destination, recipient, deadline, or intended goal ('for/in order to').",
    },
    ShowdownSentence {
        sentence: "Muchas gracias ____ tu ayuda con la optimización del código.",
        target: "por",
        distractor: "para",
        explanation: "'Por' expresses the reason or cause of gratitude.",
        plain_english: "Looks back to cause, reason, gratitude, medium, exchange, or movement through ('because of/through').",
    },
    ShowdownSentence {
        sentence: "Estudio español ____ comunicarme con mis colegas en Bogotá.",
        target: "para",
        distractor: "por",
        explanation: "'Para' + infinitive expresses purpose and intention.",
        plain_english: "Points forward to destination, recipient, deadline, or intended goal ('for/in order to').",
    },
    ShowdownSentence {
        sentence: "Pagamos cincuenta euros ____ la suscripción anual del servicio.",
        target: "por",
        distractor: "para",
        explanation: "'Por' expresses cost, exchange, or trade value.",
        plain_english: "Looks back to cause, reason, gratitude, medium, exchange, or movement through ('because of/through').",
    },
    ShowdownSentence {
        sentence: "El paquete fue enviado ____ mensajería urgente.",
        target: "por",
        distractor: "para",
        explanation: "'Por' denotes the vehicle, channel, or means of transport.",
        plain_english: "Looks back to cause, reason, gratitude, medium, exchange, or movement through ('because of/through').",
    },
    ShowdownSentence {
        sentence: "El tren ____ Barcelona sale puntualmente a las diez.",
        target: "para",
        distractor: "por",
        explanation: "'Para' indicates destination or direction of transit.",
        plain_english: "Points forward to destination, recipient, deadline, or intended goal ('for/in order to').",
    },
    ShowdownSentence {
        sentence: "No pudimos desplegar la versión ____ un corte en el suministro.",
        target: "por",
        distractor: "para",
        explanation: "'Por' indicates the obstacle, cause, or triggering reason.",
        plain_english: "Looks back to cause, reason, gratitude, medium, exchange, or movement through ('because of/through').",
    },
    ShowdownSentence {
        sentence: "____ mí, la arquitectura orientada a eventos es superior.",
        target: "para",
        distractor: "por",
        explanation: "'Para mí' introduces personal viewpoint and judgement.",
        plain_english: "Points forward to destination, recipient, deadline, or intended goal ('for/in order to').",
    },
    ShowdownSentence {
        sentence: "Estuvimos esperando en la sala de juntas ____ dos horas.",
        target: "por",
        distractor: "para",
        explanation: "'Por' indicates the duration or span of time elapsed.",
        plain_english: "Looks back to cause, reason, gratitude, medium, exchange, or movement through ('because of/through').",
    },
    ShowdownSentence {
        sentence: "El archivo descargado es únicamente ____ uso interno del equipo.",
        target: "para",
        distractor: "por",
        explanation: "'Para' defines recipient suitability or restricted purpose.",
        plain_english: "Points forward to destination, recipient, deadline, or intended goal ('for/in order to').",
    },
];

static SHOWDOWN_SER_ESTAR: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El servidor principal ____ fuera de servicio por mantenimiento.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' marks a temporary condition or operational state.",
        plain_english:
            "Describes temporary condition, physical location, or immediate operational state.",
    },
    ShowdownSentence {
        sentence: "Nuestra empresa ____ especialista en sistemas distribuidos.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' defines fundamental identity and domain classification.",
        plain_english: "Defines identity, profession, origin, essential nature, or event location.",
    },
    ShowdownSentence {
        sentence: "El director de ingeniería ____ en una reunión con clientes.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' locates concrete entities in physical/virtual space.",
        plain_english:
            "Describes temporary condition, physical location, or immediate operational state.",
    },
    ShowdownSentence {
        sentence: "La presentación técnica ____ en el auditorio del tercer piso.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' locates events taking place at a venue or time.",
        plain_english: "Defines identity, profession, origin, essential nature, or event location.",
    },
    ShowdownSentence {
        sentence: "El nuevo microservicio ya ____ listo para producción.",
        target: "está",
        distractor: "es",
        explanation: "'Estar listo' denotes being prepared / in a ready state.",
        plain_english:
            "Describes temporary condition, physical location, or immediate operational state.",
    },
    ShowdownSentence {
        sentence: "El arquitecto jefe ____ muy listo resolviendo problemas de concurrencia.",
        target: "es",
        distractor: "está",
        explanation: "'Ser listo' denotes cleverness and inherent intelligence.",
        plain_english: "Defines identity, profession, origin, essential nature, or event location.",
    },
    ShowdownSentence {
        sentence: "La base de datos ____ de solo lectura para evitar corrupciones.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' characterizes permanent structural attributes.",
        plain_english: "Defines identity, profession, origin, essential nature, or event location.",
    },
    ShowdownSentence {
        sentence: "El café en la sala de conferencias ____ frío desde hace rato.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' describes a sensory condition resulting from change.",
        plain_english:
            "Describes temporary condition, physical location, or immediate operational state.",
    },
    ShowdownSentence {
        sentence: "Elena ____ de Valencia pero reside y trabaja en Madrid.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' indicates origin, nationality, and provenance.",
        plain_english: "Defines identity, profession, origin, essential nature, or event location.",
    },
    ShowdownSentence {
        sentence: "Todos los contenedores Docker ____ corriendo adecuadamente.",
        target: "están",
        distractor: "son",
        explanation: "'Estar' + gerund forms continuous and active state.",
        plain_english:
            "Describes temporary condition, physical location, or immediate operational state.",
    },
    ShowdownSentence {
        sentence: "Hoy el cielo ____ completamente nublado y hace viento.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' expresses ambient meteorological conditions.",
        plain_english:
            "Describes temporary condition, physical location, or immediate operational state.",
    },
    ShowdownSentence {
        sentence: "La reunión de planificación ____ a las diez de la mañana.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' denotes the scheduled time of an event.",
        plain_english: "Defines identity, profession, origin, essential nature, or event location.",
    },
];

static SHOWDOWN_SUBJ_IND: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Dudo que el equipo ____ tiempo de terminar la refactorización hoy.",
        target: "tenga",
        distractor: "tiene",
        explanation: "Doubt (dudo que) triggers present subjunctive mood.",
        plain_english: "Expresses doubt, desire, hypothetical possibility, or subjective reaction rather than established fact.",
    },
    ShowdownSentence {
        sentence: "Sé con certeza que el equipo ____ la capacidad técnica necesaria.",
        target: "tiene",
        distractor: "tenga",
        explanation: "Factual certainty (sé que) selects indicative mood.",
        plain_english: "States an objective certainty, verified reality, or observed fact.",
    },
    ShowdownSentence {
        sentence: "Es necesario que el cliente ____ la propuesta antes del viernes.",
        target: "apruebe",
        distractor: "aprueba",
        explanation: "Impersonal volition/necessity (es necesario que) requires subjunctive.",
        plain_english: "Expresses doubt, desire, hypothetical possibility, or subjective reaction rather than established fact.",
    },
    ShowdownSentence {
        sentence: "Es evidente que el cliente ____ el valor de nuestra plataforma.",
        target: "comprende",
        distractor: "comprenda",
        explanation: "Objective certainty (es evidente que) takes indicative.",
        plain_english: "States an objective certainty, verified reality, or observed fact.",
    },
    ShowdownSentence {
        sentence: "No creo que el servidor ____ suficiente memoria RAM para el pico.",
        target: "tenga",
        distractor: "tiene",
        explanation: "Negated epistemic belief (no creo que) triggers subjunctive.",
        plain_english: "Expresses doubt, desire, hypothetical possibility, or subjective reaction rather than established fact.",
    },
    ShowdownSentence {
        sentence: "Creo firmemente que el servidor ____ suficiente memoria RAM.",
        target: "tiene",
        distractor: "tenga",
        explanation: "Affirmative belief (creo que) takes indicative.",
        plain_english: "States an objective certainty, verified reality, or observed fact.",
    },
    ShowdownSentence {
        sentence: "Busco un programador que ____ experiencia en sistemas distribuidos.",
        target: "tenga",
        distractor: "tiene",
        explanation: "Indefinite or hypothetical referent triggers subjunctive.",
        plain_english: "Expresses doubt, desire, hypothetical possibility, or subjective reaction rather than established fact.",
    },
    ShowdownSentence {
        sentence: "Conozco a una ingeniera que ____ amplia experiencia en Rust.",
        target: "tiene",
        distractor: "tenga",
        explanation: "Specific, known entity in relative clause takes indicative.",
        plain_english: "States an objective certainty, verified reality, or observed fact.",
    },
    ShowdownSentence {
        sentence: "Tan pronto como ____ el reporte mensual, iniciaremos la reunión.",
        target: "llegue",
        distractor: "llega",
        explanation: "Future temporal clause (tan pronto como) requires subjunctive.",
        plain_english: "Expresses doubt, desire, hypothetical possibility, or subjective reaction rather than established fact.",
    },
    ShowdownSentence {
        sentence: "Siempre que ____ a la oficina, saluda a todo el equipo.",
        target: "llega",
        distractor: "llegue",
        explanation: "Habitual recurring action takes indicative mood.",
        plain_english: "States an objective certainty, verified reality, or observed fact.",
    },
    ShowdownSentence {
        sentence: "Ojalá el parche de seguridad ____ la vulnerabilidad en producción.",
        target: "solucione",
        distractor: "soluciona",
        explanation: "'Ojalá' expresses longing and always governs subjunctive.",
        plain_english: "Expresses doubt, desire, hypothetical possibility, or subjective reaction rather than established fact.",
    },
    ShowdownSentence {
        sentence: "Es verdad que el parche ____ la vulnerabilidad en producción.",
        target: "soluciona",
        distractor: "solucione",
        explanation: "Declaration of truth (es verdad que) takes indicative.",
        plain_english: "States an objective certainty, verified reality, or observed fact.",
    },
];

static SHOWDOWN_PRET_IMP: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Ayer el equipo ____ la nueva versión en el entorno de producción.",
        target: "desplegó",
        distractor: "desplegaba",
        explanation: "Punctual completed past event at a definite time takes preterite.",
        plain_english: "Marks a completed, bounded past event with a clear beginning and end.",
    },
    ShowdownSentence {
        sentence: "Cuando trabajaba en esa startup, siempre ____ en bicicleta a la oficina.",
        target: "iba",
        distractor: "fue",
        explanation: "Customary past habit or continuous routine takes imperfect.",
        plain_english: "Describes ongoing background conditions, continuous habitual routines, or age in the past.",
    },
    ShowdownSentence {
        sentence: "Mientras nosotros ____ el código fuente, se produjo un corte eléctrico.",
        target: "revisábamos",
        distractor: "revisamos",
        explanation: "Ongoing background process interrupted by an event takes imperfect.",
        plain_english: "Describes ongoing background conditions, continuous habitual routines, or age in the past.",
    },
    ShowdownSentence {
        sentence: "De repente, la alarma del servidor ____ en mitad de la madrugada.",
        target: "sonó",
        distractor: "sonaba",
        explanation: "Sudden discrete event puncturing time takes preterite.",
        plain_english: "Marks a completed, bounded past event with a clear beginning and end.",
    },
    ShowdownSentence {
        sentence: "La fundadora ____ veinticinco años cuando lanzó su primera empresa.",
        target: "tenía",
        distractor: "tuvo",
        explanation: "Age and descriptive background states in the past take imperfect.",
        plain_english: "Describes ongoing background conditions, continuous habitual routines, or age in the past.",
    },
    ShowdownSentence {
        sentence: "El lunes pasado el cliente ____ el contrato de servicios anual.",
        target: "firmó",
        distractor: "firmaba",
        explanation: "Completed transactional event with closed boundary takes preterite.",
        plain_english: "Marks a completed, bounded past event with a clear beginning and end.",
    },
    ShowdownSentence {
        sentence: "En aquella época la infraestructura ____ sumamente rudimentaria.",
        target: "era",
        distractor: "fue",
        explanation: "Descriptive background condition over open duration takes imperfect.",
        plain_english: "Describes ongoing background conditions, continuous habitual routines, or age in the past.",
    },
    ShowdownSentence {
        sentence: "Tan pronto como recibí la alerta, ____ el nodo de emergencia.",
        target: "reinicié",
        distractor: "reiniciaba",
        explanation: "Immediate sequential action in a narrative takes preterite.",
        plain_english: "Marks a completed, bounded past event with a clear beginning and end.",
    },
    ShowdownSentence {
        sentence: "Todos los viernes el departamento ____ pizzas para almorzar.",
        target: "comía",
        distractor: "comió",
        explanation: "Repeated customary routine in past background takes imperfect.",
        plain_english: "Describes ongoing background conditions, continuous habitual routines, or age in the past.",
    },
    ShowdownSentence {
        sentence: "A las nueve en punto la sesión plenaria ____ formalmente.",
        target: "comenzó",
        distractor: "comenzaba",
        explanation: "Point of inception at a precise moment takes preterite.",
        plain_english: "Marks a completed, bounded past event with a clear beginning and end.",
    },
];

static SHOWDOWN_TU_USTED: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Disculpe, señor Pérez, ¿____ firmar la autorización de despliegue?",
        target: "puede",
        distractor: "puedes",
        explanation: "Formal address with title (señor Pérez) requires 'usted' agreement.",
        plain_english: "Formal, respectful address for senior stakeholders, clients, and formal correspondence.",
    },
    ShowdownSentence {
        sentence: "Oye, Mateo, ¿____ revisar mi pull request cuando tengas un momento?",
        target: "puedes",
        distractor: "puede",
        explanation: "Informal peer address among teammates takes 'tú' agreement.",
        plain_english: "Informal, peer-to-peer address with friends, peers, and casual colleagues.",
    },
    ShowdownSentence {
        sentence: "Estimada doctora Gómez: le ruego que ____ asiento en la sala.",
        target: "tome",
        distractor: "toma",
        explanation: "Formal polite imperative with executive/client takes 'usted' (tome).",
        plain_english: "Formal, respectful address for senior stakeholders, clients, and formal correspondence.",
    },
    ShowdownSentence {
        sentence: "Carlos, ____ los cambios locales antes de hacer el merge en git.",
        target: "guarda",
        distractor: "guarde",
        explanation: "Affirmative informal peer command uses 'tú' imperative (guarda).",
        plain_english: "Informal, peer-to-peer address with friends, peers, and casual colleagues.",
    },
    ShowdownSentence {
        sentence: "¿Cómo se encuentra ____ hoy, señor Director Ejecutivo?",
        target: "usted",
        distractor: "tú",
        explanation: "Formal honorific subject pronoun is 'usted'.",
        plain_english: "Formal, respectful address for senior stakeholders, clients, and formal correspondence.",
    },
    ShowdownSentence {
        sentence: "Hola amigo, ¿cómo estás ____ el día de hoy?",
        target: "tú",
        distractor: "usted",
        explanation: "Informal friendly subject pronoun is 'tú'.",
        plain_english: "Informal, peer-to-peer address with friends, peers, and casual colleagues.",
    },
    ShowdownSentence {
        sentence: "Estimado usuario: ¿____ usted interesado en renovar su licencia?",
        target: "estaría",
        distractor: "estarías",
        explanation: "Formal courtesy address with 'usted' requires 3rd-person conditional.",
        plain_english: "Formal, respectful address for senior stakeholders, clients, and formal correspondence.",
    },
    ShowdownSentence {
        sentence: "Hermano, ____ cuidado con los permisos de ese archivo binario.",
        target: "ten",
        distractor: "tenga",
        explanation: "Colloquial informal imperative uses 'tú' form (ten).",
        plain_english: "Informal, peer-to-peer address with friends, peers, and casual colleagues.",
    },
    ShowdownSentence {
        sentence: "Pablo, ____ el terminal y ejecuta el script de verificación.",
        target: "abre",
        distractor: "abra",
        explanation: "Informal direct peer command uses 'tú' imperative (abre).",
        plain_english: "Informal, peer-to-peer address with friends, peers, and casual colleagues.",
    },
    ShowdownSentence {
        sentence: "Por favor, licenciado, no se ____ por ese contratiempo menor.",
        target: "preocupe",
        distractor: "preocupes",
        explanation: "Negative formal imperative with reflexive 'se' takes 'usted' (preocupe).",
        plain_english: "Formal, respectful address for senior stakeholders, clients, and formal correspondence.",
    },
];

static SHOWDOWN_LO_LE: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El informe técnico era confidencial, así que ____ guardé en la caja fuerte.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine inanimate noun (el informe) is 'lo'.",
        plain_english:
            "Direct object pronoun: refers directly to the thing or person receiving the action.",
    },
    ShowdownSentence {
        sentence: "A María ____ entregamos las credenciales de acceso al sistema.",
        target: "le",
        distractor: "la",
        explanation: "Indirect object pronoun for human recipient (a María) is 'le'.",
        plain_english:
            "Indirect object pronoun: refers to the recipient, beneficiary, or affected party.",
    },
    ShowdownSentence {
        sentence: "Ayer vi a Pedro en la conferencia y ____ saludé cordialmente.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine person (accusative) is 'lo'.",
        plain_english:
            "Direct object pronoun: refers directly to the thing or person receiving the action.",
    },
    ShowdownSentence {
        sentence: "Al arquitecto principal ____ propuse una solución con microservicios.",
        target: "le",
        distractor: "lo",
        explanation: "Indirect object pronoun indicating recipient of proposal is 'le'.",
        plain_english:
            "Indirect object pronoun: refers to the recipient, beneficiary, or affected party.",
    },
    ShowdownSentence {
        sentence: "El nuevo framework de Rust es fantástico; ____ descargué esta mañana.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine noun is 'lo'.",
        plain_english:
            "Direct object pronoun: refers directly to the thing or person receiving the action.",
    },
    ShowdownSentence {
        sentence: "A los nuevos desarrolladores ____ explicamos las directrices de estilo.",
        target: "les",
        distractor: "los",
        explanation: "Indirect object pronoun for plural recipients is 'les'.",
        plain_english:
            "Indirect object pronoun: refers to the recipient, beneficiary, or affected party.",
    },
    ShowdownSentence {
        sentence: "La vulnerabilidad era grave; el equipo de seguridad ____ solucionó hoy.",
        target: "la",
        distractor: "le",
        explanation: "Direct object pronoun for feminine noun (la vulnerabilidad) is 'la'.",
        plain_english:
            "Direct object pronoun: refers directly to the thing or person receiving the action.",
    },
    ShowdownSentence {
        sentence: "A la directora general ____ envié un resumen ejecutivo por correo.",
        target: "le",
        distractor: "la",
        explanation: "Indirect object pronoun indicating destination/recipient is 'le'.",
        plain_english:
            "Indirect object pronoun: refers to the recipient, beneficiary, or affected party.",
    },
    ShowdownSentence {
        sentence: "Descargué el archivo comprimido y ____ descomprimí en el servidor.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine entity is 'lo'.",
        plain_english:
            "Direct object pronoun: refers directly to the thing or person receiving the action.",
    },
    ShowdownSentence {
        sentence: "Al usuario final ____ enviamos un token de doble factor por mensaje.",
        target: "le",
        distractor: "lo",
        explanation: "Indirect object pronoun marking recipient of transmission is 'le'.",
        plain_english:
            "Indirect object pronoun: refers to the recipient, beneficiary, or affected party.",
    },
];

static SHOWDOWN_SINO_PERO: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El error no fue de hardware, ____ de configuración de red.",
        target: "sino",
        distractor: "pero",
        explanation: "'Sino' introduces exclusive rectification directly after negation.",
        plain_english:
            "Exclusive substitution: replaces a negated premise ('not X, but rather Y').",
    },
    ShowdownSentence {
        sentence: "El lenguaje es potente y rápido, ____ requiere aprender el borrow checker.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' introduces an adversative qualification or limitation.",
        plain_english:
            "Additive contrast: adds a limitation or nuance without negating the first statement.",
    },
    ShowdownSentence {
        sentence: "No queríamos cancelar la migración, ____ posponerla hasta el fin de semana.",
        target: "sino",
        distractor: "pero",
        explanation: "'Sino' replaces the negated clause with the intended alternative.",
        plain_english:
            "Exclusive substitution: replaces a negated premise ('not X, but rather Y').",
    },
    ShowdownSentence {
        sentence: "Escribimos las pruebas unitarias, ____ nos faltó tiempo para las de carga.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' connects two contrasting coordinate clauses.",
        plain_english:
            "Additive contrast: adds a limitation or nuance without negating the first statement.",
    },
    ShowdownSentence {
        sentence: "Esta herramienta no solo es veloz, ____ además sumamente ligera.",
        target: "sino",
        distractor: "pero",
        explanation: "'No solo... sino (además/también)' forms the correlative addition.",
        plain_english:
            "Exclusive substitution: replaces a negated premise ('not X, but rather Y').",
    },
    ShowdownSentence {
        sentence: "Intentamos conectar a la base de datos, ____ el puerto estaba bloqueado.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' marks an adversative obstacle or complication.",
        plain_english:
            "Additive contrast: adds a limitation or nuance without negating the first statement.",
    },
    ShowdownSentence {
        sentence: "La empresa no busca inversores de riesgo, ____ clientes corporativos.",
        target: "sino",
        distractor: "pero",
        explanation: "'Sino' directly contrasts and substitutes the noun phrase.",
        plain_english:
            "Exclusive substitution: replaces a negated premise ('not X, but rather Y').",
    },
    ShowdownSentence {
        sentence: "El algoritmo es muy elegante, ____ consume demasiada memoria.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' adds a drawback without negating the premise.",
        plain_english:
            "Additive contrast: adds a limitation or nuance without negating the first statement.",
    },
    ShowdownSentence {
        sentence: "No perdimos ningún dato, ____ recuperamos todas las transacciones.",
        target: "sino que",
        distractor: "pero",
        explanation:
            "'Sino que' introduces an affirmative rectified clause with a conjugated verb.",
        plain_english:
            "Exclusive substitution: replaces a negated premise ('not X, but rather Y').",
    },
    ShowdownSentence {
        sentence: "Tenemos el visto bueno técnico, ____ aún falta la aprobación legal.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' introduces a pending constraint.",
        plain_english:
            "Additive contrast: adds a limitation or nuance without negating the first statement.",
    },
];

static SHOWDOWN_PARA_QUE_PORQUE: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Optimizamos la consulta SQL ____ la página cargue en menos de un segundo.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' expresses purpose and governs the subjunctive mood (cargue).",
        plain_english:
            "Expresses intended purpose/goal ('so that'), triggering the subjunctive mood.",
    },
    ShowdownSentence {
        sentence: "El servidor se reinició automáticamente ____ se agotó la memoria swap.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' introduces a causal factual reason with indicative (agotó).",
        plain_english:
            "States the factual cause or reason ('because'), taking the indicative mood.",
    },
    ShowdownSentence {
        sentence: "Te comparto el repositorio ____ puedas revisar los cambios del pull request.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' introduces intentional goal with subjunctive (puedas).",
        plain_english:
            "Expresses intended purpose/goal ('so that'), triggering the subjunctive mood.",
    },
    ShowdownSentence {
        sentence: "No asistimos al webinar ____ estábamos resolviendo una caída del servicio.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' explains the actual cause with indicative (estábamos).",
        plain_english:
            "States the factual cause or reason ('because'), taking the indicative mood.",
    },
    ShowdownSentence {
        sentence: "Añadimos logs detallados ____ el equipo de soporte identifique fallos rápido.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' governs subjunctive (identifique) to denote purpose.",
        plain_english:
            "Expresses intended purpose/goal ('so that'), triggering the subjunctive mood.",
    },
    ShowdownSentence {
        sentence: "Elegimos Rust ____ garantiza seguridad de memoria sin recolector de basura.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' introduces an objective rationale with indicative (garantiza).",
        plain_english:
            "States the factual cause or reason ('because'), taking the indicative mood.",
    },
    ShowdownSentence {
        sentence:
            "Documentamos la API exhaustivamente ____ los nuevos integrantes se adapten pronto.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' sets the target objective with subjunctive (adapten).",
        plain_english:
            "Expresses intended purpose/goal ('so that'), triggering the subjunctive mood.",
    },
    ShowdownSentence {
        sentence:
            "El cliente aceptó la propuesta ____ cumplía con todos los requisitos de seguridad.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' states the factual basis with indicative (cumplía).",
        plain_english:
            "States the factual cause or reason ('because'), taking the indicative mood.",
    },
    ShowdownSentence {
        sentence: "Habilitamos compresión gzip ____ el ancho de banda no sea un cuello de botella.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' governs subjunctive (sea) for purpose.",
        plain_english:
            "Expresses intended purpose/goal ('so that'), triggering the subjunctive mood.",
    },
    ShowdownSentence {
        sentence:
            "Cancelamos el despliegue programado ____ detectamos una regresión en las pruebas.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' introduces the triggering cause with indicative (detectamos).",
        plain_english:
            "States the factual cause or reason ('because'), taking the indicative mood.",
    },
];

static SHOWDOWN_TENER_HABER: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Esta mañana ____ revisado todos los registros del cortafuegos.",
        target: "he",
        distractor: "tengo",
        explanation:
            "'Haber' (he) acts as the auxiliary verb for the compound perfect tense (he revisado).",
        plain_english: "Denotes existence/availability ('there is/are') or serves as auxiliary verb.",
    },
    ShowdownSentence {
        sentence:
            "Para ese puesto directivo, el candidato ____ que tener diez años de experiencia.",
        target: "tiene",
        distractor: "ha",
        explanation:
            "'Tener que' + infinitive expresses personal obligation for a specific subject.",
        plain_english: "Denotes physical possession, ownership, age, or obligations ('tener que').",
    },
    ShowdownSentence {
        sentence: "En el nuevo centro de datos ____ más de cincuenta servidores en bastidor.",
        target: "hay",
        distractor: "tiene",
        explanation: "'Hay' (haber) expresses impersonal existence ('there are').",
        plain_english: "Denotes existence/availability ('there is/are') or serves as auxiliary verb.",
    },
    ShowdownSentence {
        sentence: "Nuestra empresa ____ cinco oficinas comerciales en América Latina.",
        target: "tiene",
        distractor: "ha",
        explanation: "'Tener' expresses ownership and possession of tangible entities.",
        plain_english: "Denotes physical possession, ownership, age, or obligations ('tener que').",
    },
    ShowdownSentence {
        sentence: "Cuando llegamos a la oficina, el equipo ya ____ terminado el despliegue.",
        target: "había",
        distractor: "tenía",
        explanation:
            "'Había' (haber) forms the pluperfect auxiliary tense with the past participle.",
        plain_english: "Denotes existence/availability ('there is/are') or serves as auxiliary verb.",
    },
    ShowdownSentence {
        sentence: "El nuevo ingeniero jefe ____ treinta y dos años recién cumplidos.",
        target: "tiene",
        distractor: "ha",
        explanation: "'Tener' is strictly required to state biological age in Spanish.",
        plain_english: "Denotes physical possession, ownership, age, or obligations ('tener que').",
    },
    ShowdownSentence {
        sentence: "____ que ser sumamente rigurosos con el protocolo de cifrado.",
        target: "Hay",
        distractor: "Tiene",
        explanation:
            "'Hay que' + infinitive expresses general, impersonal necessity ('one must / it is necessary to').",
        plain_english: "Denotes existence/availability ('there is/are') or serves as auxiliary verb.",
    },
    ShowdownSentence {
        sentence: "Los investigadores ____ mucho miedo de que se filtrara la clave privada.",
        target: "tienen",
        distractor: "han",
        explanation:
            "'Tener miedo' uses 'tener' to describe psychological states and emotions.",
        plain_english: "Denotes physical possession, ownership, age, or obligations ('tener que').",
    },
    ShowdownSentence {
        sentence: "Para el final del trimestre los auditores ____ completado el informe.",
        target: "habrán",
        distractor: "tendrán",
        explanation:
            "'Habrán' (haber) forms the future perfect compound tense with the past participle.",
        plain_english: "Denotes existence/availability ('there is/are') or serves as auxiliary verb.",
    },
    ShowdownSentence {
        sentence: "¿No ____ frío en la sala de servidores con el aire acondicionado al máximo?",
        target: "tienes",
        distractor: "has",
        explanation: "'Tener frío' expresses a physical bodily sensation using 'tener'.",
        plain_english: "Denotes physical possession, ownership, age, or obligations ('tener que').",
    },
    ShowdownSentence {
        sentence: "No ____ ninguna duda de que el algoritmo converge rápidamente.",
        target: "hay",
        distractor: "tiene",
        explanation: "'No hay duda' expresses impersonal existence of certainty or doubt.",
        plain_english: "Denotes existence/availability ('there is/are') or serves as auxiliary verb.",
    },
    ShowdownSentence {
        sentence: "Nosotros ____ dos reuniones con el cliente internacional esta tarde.",
        target: "tenemos",
        distractor: "hemos",
        explanation: "'Tener' indicates holding scheduled appointments or events.",
        plain_english: "Denotes physical possession, ownership, age, or obligations ('tener que').",
    },
];

static SHOWDOWN_SABER_CONOCER: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "¿____ a la nueva jefa de ingeniería de datos que se incorporó hoy?",
        target: "Conoces",
        distractor: "Sabes",
        explanation:
            "'Conocer a' is used for familiarity or personal acquaintance with individuals.",
        plain_english:
            "Expresses acquaintance, familiarity with people, places, or complex systems.",
    },
    ShowdownSentence {
        sentence: "El desarrollador senior ____ programar en Rust, Go y C++ con fluidez.",
        target: "sabe",
        distractor: "conoce",
        explanation:
            "'Saber' + infinitive denotes a learned skill or knowing how to perform a task.",
        plain_english:
            "Expresses knowledge of facts, information, data, or learned skills ('to know how to').",
    },
    ShowdownSentence {
        sentence: "No ____ la contraseña de acceso raíz para el clúster de producción.",
        target: "sé",
        distractor: "conozco",
        explanation: "'Saber' indicates knowledge of specific facts, data, or credentials.",
        plain_english:
            "Expresses knowledge of facts, information, data, or learned skills ('to know how to').",
    },
    ShowdownSentence {
        sentence: "Nuestra consultora ____ muy bien el mercado financiero europeo.",
        target: "conoce",
        distractor: "sabe",
        explanation: "'Conocer' expresses deep familiarity with a market, domain, or territory.",
        plain_english:
            "Expresses acquaintance, familiarity with people, places, or complex systems.",
    },
    ShowdownSentence {
        sentence: "¿____ dónde se encuentra el centro de respaldo secundario?",
        target: "Sabes",
        distractor: "Conoces",
        explanation:
            "'Saber' is required before embedded interrogative clauses (dónde, cuándo, cómo).",
        plain_english:
            "Expresses knowledge of facts, information, data, or learned skills ('to know how to').",
    },
    ShowdownSentence {
        sentence: "Ayer ____ en la conferencia al creador de esta biblioteca de código abierto.",
        target: "conocí",
        distractor: "supe",
        explanation: "'Conocer' in the preterite means meeting someone for the first time.",
        plain_english:
            "Expresses acquaintance, familiarity with people, places, or complex systems.",
    },
    ShowdownSentence {
        sentence: "Ella ____ de memoria todas las cláusulas del contrato de nivel de servicio.",
        target: "sabe",
        distractor: "conoce",
        explanation: "'Saber de memoria' is the fixed idiom for knowing information by heart.",
        plain_english:
            "Expresses knowledge of facts, information, data, or learned skills ('to know how to').",
    },
    ShowdownSentence {
        sentence: "¿____ esa herramienta de monitorización distribuida llamada Prometheus?",
        target: "Conoces",
        distractor: "Sabes",
        explanation:
            "'Conocer' expresses acquaintance or familiarity with tools, software, or works.",
        plain_english:
            "Expresses acquaintance, familiarity with people, places, or complex systems.",
    },
    ShowdownSentence {
        sentence: "Aún no ____ si el cliente firmará la extensión de la licencia anual.",
        target: "sabemos",
        distractor: "conocemos",
        explanation: "'Saber si' is used when assessing the factual truth of an indirect question.",
        plain_english:
            "Expresses knowledge of facts, information, data, or learned skills ('to know how to').",
    },
    ShowdownSentence {
        sentence: "Los delegados ____ la ciudad de Barcelona gracias a visitas previas.",
        target: "conocen",
        distractor: "saben",
        explanation: "'Conocer' denotes familiarity with geographical locations and cities.",
        plain_english:
            "Expresses acquaintance, familiarity with people, places, or complex systems.",
    },
    ShowdownSentence {
        sentence: "¿____ por qué falló la réplica de la base de datos anoche?",
        target: "Sabes",
        distractor: "Conoces",
        explanation: "'Saber' introduces knowledge of reasons, causes, and explanations.",
        plain_english:
            "Expresses knowledge of facts, information, data, or learned skills ('to know how to').",
    },
    ShowdownSentence {
        sentence: "No ____ a nadie en el equipo de auditoría externa.",
        target: "conozco",
        distractor: "sé",
        explanation: "'Conocer a nadie' denotes acquaintance with people.",
        plain_english:
            "Expresses acquaintance, familiarity with people, places, or complex systems.",
    },
];

static SHOWDOWN_MUY_MUCHO: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El nuevo algoritmo de compresión es ____ eficiente en términos de CPU.",
        target: "muy",
        distractor: "mucho",
        explanation: "'Muy' is an invariable adverb used before adjectives to mean 'very'.",
        plain_english: "Adverb modifying adjectives or adverbs ('very/really').",
    },
    ShowdownSentence {
        sentence: "Hemos dedicado ____ tiempo a la refactorización de la capa de persistencia.",
        target: "mucho",
        distractor: "muy",
        explanation:
            "'Mucho' acts as a quantitative adjective agreeing with masculine singular nouns.",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
    ShowdownSentence {
        sentence: "Los ingenieros de operaciones trabajan ____ bien bajo situaciones de estrés.",
        target: "muy",
        distractor: "mucho",
        explanation: "'Muy' modifies adverbs (e.g., 'muy bien') to intensify manner.",
        plain_english: "Adverb modifying adjectives or adverbs ('very/really').",
    },
    ShowdownSentence {
        sentence: "Se requiere ____ paciencia para depurar condiciones de carrera complejas.",
        target: "mucha",
        distractor: "muy",
        explanation:
            "'Mucha' agrees in gender and number with feminine singular nouns (paciencia).",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
    ShowdownSentence {
        sentence: "Llegamos ____ tarde a la sesión plenaria debido al retraso del vuelo.",
        target: "muy",
        distractor: "mucho",
        explanation: "'Muy' modifies temporal adverbs (e.g., 'muy tarde', 'muy temprano').",
        plain_english: "Adverb modifying adjectives or adverbs ('very/really').",
    },
    ShowdownSentence {
        sentence: "El procesador gráfico trabaja ____ durante el renderizado de modelos 3D.",
        target: "mucho",
        distractor: "muy",
        explanation:
            "'Mucho' acts as an adverb modifying verbs (placed after the verb) to mean 'a lot'.",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
    ShowdownSentence {
        sentence: "Hay ____ clientes esperando la resolución de la incidencia crítica.",
        target: "muchos",
        distractor: "muy",
        explanation: "'Muchos' is a quantifier agreeing with masculine plural nouns (clientes).",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
    ShowdownSentence {
        sentence: "____ gracias por su colaboración en el lanzamiento de la plataforma.",
        target: "Muchas",
        distractor: "Muy",
        explanation: "'Muchas' is the quantifier agreeing with the feminine plural noun 'gracias'.",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
    ShowdownSentence {
        sentence: "La arquitectura de microservicios está ____ avanzada en su diseño.",
        target: "muy",
        distractor: "mucho",
        explanation: "'Muy' modifies adjectives and participial adjectives (muy avanzada).",
        plain_english: "Adverb modifying adjectives or adverbs ('very/really').",
    },
    ShowdownSentence {
        sentence: "Ese departamento viaja ____ para auditar sucursales internacionales.",
        target: "mucho",
        distractor: "muy",
        explanation: "'Mucho' modifies verbs to indicate high frequency or degree.",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
    ShowdownSentence {
        sentence: "El informe de seguridad es ____ claro y detallado en sus conclusiones.",
        target: "muy",
        distractor: "mucho",
        explanation: "'Muy' intensifies adjectives describing qualitative attributes.",
        plain_english: "Adverb modifying adjectives or adverbs ('very/really').",
    },
    ShowdownSentence {
        sentence: "Tuvimos ____ dificultades para sincronizar los nodos distribuidos.",
        target: "muchas",
        distractor: "muy",
        explanation: "'Muchas' agrees with feminine plural nouns (dificultades).",
        plain_english: "Quantifier modifying nouns ('much/many') or verb intensity ('a lot').",
    },
];

static SHOWDOWN_PEDIR_PREGUNTAR: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Al terminar el almuerzo de negocios, vamos a ____ la cuenta al camarero.",
        target: "pedir",
        distractor: "preguntar",
        explanation: "'Pedir la cuenta' means requesting the bill (asking for an item or service).",
        plain_english: "Requests an object, service, action, or favor ('to ask for').",
    },
    ShowdownSentence {
        sentence: "Voy a ____ la hora al recepcionista porque mi reloj se ha parado.",
        target: "preguntar",
        distractor: "pedir",
        explanation: "'Preguntar la hora' means inquiring about information (seeking the time).",
        plain_english: "Inquires about information, facts, or questions ('to ask a question').",
    },
    ShowdownSentence {
        sentence: "No dudes en ____ ayuda técnica si te bloqueas con la configuración.",
        target: "pedir",
        distractor: "preguntar",
        explanation: "'Pedir ayuda' expresses requesting assistance or support.",
        plain_english: "Requests an object, service, action, or favor ('to ask for').",
    },
    ShowdownSentence {
        sentence: "Tuvimos que ____ cómo llegar al centro de convenciones en taxi.",
        target: "preguntar",
        distractor: "pedir",
        explanation: "'Preguntar' is used when inquiring about directions or information.",
        plain_english: "Inquires about information, facts, or questions ('to ask a question').",
    },
    ShowdownSentence {
        sentence: "El director ejecutivo decidió ____ disculpas públicas por la interrupción.",
        target: "pedir",
        distractor: "preguntar",
        explanation: "'Pedir disculpas / perdón' is the fixed idiom for apologizing.",
        plain_english: "Requests an object, service, action, or favor ('to ask for').",
    },
    ShowdownSentence {
        sentence: "Le voy a ____ al jefe de equipo si podemos desplegar el viernes.",
        target: "preguntar",
        distractor: "pedir",
        explanation: "'Preguntar si' introduces an informational inquiry seeking a yes/no answer.",
        plain_english: "Inquires about information, facts, or questions ('to ask a question').",
    },
    ShowdownSentence {
        sentence: "Queremos ____ un presupuesto formal antes de contratar el servicio en la nube.",
        target: "pedir",
        distractor: "preguntar",
        explanation: "'Pedir un presupuesto' means requesting a quote or price estimate document.",
        plain_english: "Requests an object, service, action, or favor ('to ask for').",
    },
    ShowdownSentence {
        sentence: "Varios analistas vinieron a ____ por los resultados del benchmark.",
        target: "preguntar",
        distractor: "pedir",
        explanation: "'Preguntar por' means inquiring about the status or details of something.",
        plain_english: "Inquires about information, facts, or questions ('to ask a question').",
    },
    ShowdownSentence {
        sentence: "Debemos ____ permiso al administrador para modificar la regla de firewall.",
        target: "pedir",
        distractor: "preguntar",
        explanation: "'Pedir permiso' means requesting authorization.",
        plain_english: "Requests an object, service, action, or favor ('to ask for').",
    },
    ShowdownSentence {
        sentence: "Entré en el auditorio para ____ a qué hora comenzaba la ponencia.",
        target: "preguntar",
        distractor: "pedir",
        explanation: "'Preguntar a qué hora' is an informational inquiry.",
        plain_english: "Inquires about information, facts, or questions ('to ask a question').",
    },
    ShowdownSentence {
        sentence: "Te quería ____ un favor personal cuando tengas diez minutos libres.",
        target: "pedir",
        distractor: "preguntar",
        explanation: "'Pedir un favor' expresses requesting a favor (soliciting an action).",
        plain_english: "Requests an object, service, action, or favor ('to ask for').",
    },
    ShowdownSentence {
        sentence: "Si tienes dudas sobre la arquitectura, es mejor ____ al especialista.",
        target: "preguntar",
        distractor: "pedir",
        explanation: "'Preguntar' means posing a question or inquiring for clarification.",
        plain_english: "Inquires about information, facts, or questions ('to ask a question').",
    },
];

static SHOWDOWN_LLEVAR_TRAER: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "____ el paraguas contigo porque el pronóstico anuncia lluvia en el centro.",
        target: "Llévate",
        distractor: "Tráete",
        explanation:
            "'Llevar' expresses moving an item away from the current location to another place.",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "Por favor, ____ un vaso de agua cuando vengas a la sala de juntas.",
        target: "tráeme",
        distractor: "llévame",
        explanation:
            "'Traer' indicates moving an object toward the speaker's location ('bring to me').",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "Si sales hacia la estación, te ____ en mi coche en cinco minutos.",
        target: "llevo",
        distractor: "traigo",
        explanation: "'Llevar' denotes transporting someone away from here to a destination.",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "El mensajero nos ____ las nuevas tarjetas criptográficas esta mañana.",
        target: "trajo",
        distractor: "llevó",
        explanation:
            "'Traer' indicates delivering items toward the speaker / recipient's present location.",
        plain_english: "Movement towards the speaker ('bring here').",
    },
    ShowdownSentence {
        sentence: "Tengo que ____ estos documentos firmados a la sede central de la empresa.",
        target: "llevar",
        distractor: "traer",
        explanation: "'Llevar' indicates carrying something away to another destination.",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "Cuando vengas a visitarnos a la oficina, ____ la muestra del producto.",
        target: "trae",
        distractor: "lleva",
        explanation: "'Traer' expresses bringing something along when moving toward the speaker.",
        plain_english: "Movement towards the speaker ('bring here').",
    },
    ShowdownSentence {
        sentence: "El conferenciante siempre ____ traje oscuro en las presentaciones formales.",
        target: "lleva",
        distractor: "trae",
        explanation: "'Llevar' is used for wearing clothing or apparel.",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "Mi compañero me ____ un café recién hecho a mi escritorio.",
        target: "trajo",
        distractor: "llevó",
        explanation: "'Traer' expresses moving something toward the speaker's workspace.",
        plain_english: "Movement towards the speaker ('bring here').",
    },
    ShowdownSentence {
        sentence: "Para la cena del equipo, nosotros podemos ____ el postre y la fruta.",
        target: "llevar",
        distractor: "traer",
        explanation: "'Llevar' expresses taking food or items along to an external venue.",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "El camarero nos ____ la cuenta a la mesa en una bandeja pequeña.",
        target: "trajo",
        distractor: "llevó",
        explanation:
            "'Traer' describes delivering something toward the table where diners are seated.",
        plain_english: "Movement towards the speaker ('bring here').",
    },
    ShowdownSentence {
        sentence: "Todas las mañanas ____ a mis hijos al colegio antes de ir al trabajo.",
        target: "llevo",
        distractor: "traigo",
        explanation: "'Llevar' denotes conveying people away to their school destination.",
        plain_english: "Movement away from the speaker ('take there') or passage of time/wearing.",
    },
    ShowdownSentence {
        sentence: "El nuevo informe nos ____ conclusiones muy esperanzadoras para el proyecto.",
        target: "trae",
        distractor: "lleva",
        explanation: "'Traer' figuratively expresses bringing insights or outcomes to us.",
        plain_english: "Movement towards the speaker ('bring here').",
    },
];

static SHOWDOWN_HABER_ESTAR: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "¿____ alguna farmacia de guardia cerca de este edificio de oficinas?",
        target: "Hay",
        distractor: "Está",
        explanation:
            "'Hay' (haber) expresses the indefinite existence of an entity ('alguna farmacia').",
        plain_english:
            "Introduces unspecific existence of something new into the scene ('there is/are').",
    },
    ShowdownSentence {
        sentence: "La farmacia de guardia ____ a dos manzanas de aquí, en la avenida principal.",
        target: "está",
        distractor: "hay",
        explanation: "'Está' (estar) locates a specific, definite entity introduced by 'La'.",
        plain_english: "Locates a specific, known entity whose existence is already established.",
    },
    ShowdownSentence {
        sentence: "En este polígono tecnológico ____ muchas empresas de ciberseguridad.",
        target: "hay",
        distractor: "están",
        explanation:
            "'Hay' expresses the existence of indefinite plural entities ('muchas empresas').",
        plain_english:
            "Introduces unspecific existence of something new into the scene ('there is/are').",
    },
    ShowdownSentence {
        sentence: "Los servidores dedicados ____ ubicados en el centro de datos de Frankfurt.",
        target: "están",
        distractor: "hay",
        explanation: "'Están' locates specific definite subjects in geographical space.",
        plain_english: "Locates a specific, known entity whose existence is already established.",
    },
    ShowdownSentence {
        sentence: "No ____ nadie en el laboratorio de pruebas a estas horas de la noche.",
        target: "hay",
        distractor: "está",
        explanation:
            "'No hay' expresses non-existence with indefinite negative pronouns ('nadie').",
        plain_english:
            "Introduces unspecific existence of something new into the scene ('there is/are').",
    },
    ShowdownSentence {
        sentence: "El director de operaciones ____ en una videoconferencia con los inversores.",
        target: "está",
        distractor: "hay",
        explanation: "'Está' indicates the current location and activity of a specific person.",
        plain_english: "Locates a specific, known entity whose existence is already established.",
    },
    ShowdownSentence {
        sentence: "¿Qué ____ dentro de ese paquete postal que acaba de llegar?",
        target: "hay",
        distractor: "está",
        explanation:
            "'¿Qué hay?' inquires about the existence or presence of unidentified contents.",
        plain_english:
            "Introduces unspecific existence of something new into the scene ('there is/are').",
    },
    ShowdownSentence {
        sentence: "El Museo del Prado ____ en el paseo del mismo nombre en Madrid.",
        target: "está",
        distractor: "hay",
        explanation: "'Está' locates a unique, specific landmark or institution.",
        plain_english: "Locates a specific, known entity whose existence is already established.",
    },
    ShowdownSentence {
        sentence: "____ un error crítico de segmentación en la memoria del microcontrolador.",
        target: "Hay",
        distractor: "Está",
        explanation: "'Hay un error' introduces the existence of an indefinite defect or problem.",
        plain_english:
            "Introduces unspecific existence of something new into the scene ('there is/are').",
    },
    ShowdownSentence {
        sentence: "Las copias de seguridad ____ almacenadas en discos de estado sólido cifrados.",
        target: "están",
        distractor: "hay",
        explanation: "'Están' locates specific definite items and describes their state.",
        plain_english: "Locates a specific, known entity whose existence is already established.",
    },
    ShowdownSentence {
        sentence: "No ____ tiempo suficiente para realizar una refactorización completa hoy.",
        target: "hay",
        distractor: "está",
        explanation: "'No hay' expresses the absence or lack of an uncountable noun ('tiempo').",
        plain_english:
            "Introduces unspecific existence of something new into the scene ('there is/are').",
    },
    ShowdownSentence {
        sentence: "Todos los miembros del comité ____ reunidos en la sala principal.",
        target: "están",
        distractor: "hay",
        explanation: "'Están' locates specific identified people in a venue.",
        plain_english: "Locates a specific, known entity whose existence is already established.",
    },
];

static SHOWDOWN_IR_IRSE: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Mañana por la mañana ____ a la oficina central para firmar el acuerdo.",
        target: "voy",
        distractor: "me voy",
        explanation: "'Ir' emphasizes movement toward a destination ('voy a la oficina').",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "Ya son más de las ocho de la tarde; ____ a casa porque estoy agotado.",
        target: "me voy",
        distractor: "voy",
        explanation: "'Irse' (reflexive) emphasizes departure and leaving the current location.",
        plain_english: "Focuses on departure, leaving a place, or heading out ('to take off').",
    },
    ShowdownSentence {
        sentence: "La reunión ha concluido satisfactoriamente, ¡____ a comer!",
        target: "vamos",
        distractor: "vámonos",
        explanation: "'Vamos a' + infinitive expresses joint movement toward an activity.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "El ambiente en la sala es insostenible, ¡____ de aquí ahora mismo!",
        target: "vámonos",
        distractor: "vamos",
        explanation:
            "'¡Vámonos!' is the imperative command meaning 'let's leave / let's get out of here'.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "Los fines de semana siempre ____ a la montaña a practicar senderismo.",
        target: "vamos",
        distractor: "nos vamos",
        explanation: "'Ir' expresses regular travel or movement toward a destination.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "El ingeniero principal ____ de la empresa para emprender su propio proyecto.",
        target: "se va",
        distractor: "va",
        explanation: "'Irse de' denotes departing, quitting, or leaving an organization.",
        plain_english: "Focuses on departure, leaving a place, or heading out ('to take off').",
    },
    ShowdownSentence {
        sentence: "¿____ al supermercado a comprar café o prefieres que lo pida en línea?",
        target: "Vas",
        distractor: "Te vas",
        explanation: "'Ir' is used for a targeted errand or trip to a destination.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "Los asistentes al evento ya ____ porque se ha hecho medianoche.",
        target: "se van",
        distractor: "van",
        explanation: "'Irse' emphasizes the guests departing from the venue.",
        plain_english: "Focuses on departure, leaving a place, or heading out ('to take off').",
    },
    ShowdownSentence {
        sentence: "En el próximo trimestre ____ a implementar autenticación biométrica.",
        target: "vamos",
        distractor: "nos vamos",
        explanation: "'Ir a' + infinitive forms the periphrastic future for planned actions.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "Disculpen, tengo que ____ antes de que cierre la estación de tren.",
        target: "irme",
        distractor: "ir",
        explanation: "'Tener que irse' uses the reflexive 'irse' to signify needing to depart.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "Ella ____ a la biblioteca universitaria todos los días después de clase.",
        target: "va",
        distractor: "se va",
        explanation: "'Ir' describes regular transit to a specific place of study.",
        plain_english: "Focuses on travel towards a specific destination ('to go to').",
    },
    ShowdownSentence {
        sentence: "No soporto este calor sofocante, ____ a la piscina ahora mismo.",
        target: "me voy",
        distractor: "voy",
        explanation:
            "'Irse' highlights breaking away and departing the current uncomfortable setting.",
        plain_english: "Focuses on departure, leaving a place, or heading out ('to take off').",
    },
];

static SHOWDOWN_BIEN_BUENO: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El equipo de desarrollo ha estructurado el código sumamente ____.",
        target: "bien",
        distractor: "bueno",
        explanation:
            "'Bien' is an adverb that modifies verbs to describe how an action was performed.",
        plain_english: "Adverb describing manner, condition, or health ('well/properly').",
    },
    ShowdownSentence {
        sentence: "Ese arquitecto de software es un ____ profesional con amplia trayectoria.",
        target: "buen",
        distractor: "bien",
        explanation:
            "'Buen' is the shortened (apocopated) adjective used before masculine singular nouns.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "Este vino de la comarca del Duero es realmente ____ y aromático.",
        target: "bueno",
        distractor: "bien",
        explanation: "'Bueno' is an adjective describing high inherent quality after 'ser'.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "Hoy no me encuentro ____ de salud, creo que tengo fiebre.",
        target: "bien",
        distractor: "bueno",
        explanation:
            "'Encontrarse bien' uses the adverb 'bien' to denote state of physical wellbeing.",
        plain_english: "Adverb describing manner, condition, or health ('well/properly').",
    },
    ShowdownSentence {
        sentence: "Esta oferta de empleo en Múnich es una ____ oportunidad profesional.",
        target: "buena",
        distractor: "bien",
        explanation: "'Buena' is the feminine singular adjective describing positive quality.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "Todos los consultores hablan español e inglés muy ____.",
        target: "bien",
        distractor: "bueno",
        explanation: "'Bien' is an adverb modifying the verb 'hablar' to express fluency.",
        plain_english: "Adverb describing manner, condition, or health ('well/properly').",
    },
    ShowdownSentence {
        sentence: "Nuestra jefa de proyecto es una persona sumamente ____ y empática.",
        target: "buena",
        distractor: "bien",
        explanation:
            "'Buena persona' uses the feminine adjective to describe moral quality and kindness.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "El nuevo motor de renderizado funciona bastante ____ en dispositivos móviles.",
        target: "bien",
        distractor: "bueno",
        explanation: "'Funcionar bien' uses the adverb 'bien' to describe correct operation.",
        plain_english: "Adverb describing manner, condition, or health ('well/properly').",
    },
    ShowdownSentence {
        sentence: "¡Qué ____ momento para anunciar la ronda de financiación!",
        target: "buen",
        distractor: "bien",
        explanation: "'Buen' is the apocopated adjective preceding the masculine noun 'momento'.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "El diseño de la experiencia de usuario no es especialmente ____.",
        target: "bueno",
        distractor: "bien",
        explanation: "'Bueno' is the predicate adjective describing the design quality.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "Las prestaciones de este nuevo servidor en la nube son muy ____.",
        target: "buenas",
        distractor: "bien",
        explanation: "'Buenas' is the feminine plural adjective agreeing with 'prestaciones'.",
        plain_english: "Adjective evaluating quality, character, or intrinsic value ('good').",
    },
    ShowdownSentence {
        sentence: "El contrato marco está ____ redactado y cumple todas las normativas.",
        target: "bien",
        distractor: "bueno",
        explanation: "'Bien' is an adverb modifying the participle 'redactado'.",
        plain_english: "Adverb describing manner, condition, or health ('well/properly').",
    },
];

fn get_showdown_pool(pair: ShowdownPair) -> &'static [ShowdownSentence] {
    match pair {
        ShowdownPair::PorPara => SHOWDOWN_POR_PARA,
        ShowdownPair::SerEstar => SHOWDOWN_SER_ESTAR,
        ShowdownPair::SubjInd => SHOWDOWN_SUBJ_IND,
        ShowdownPair::PretImp => SHOWDOWN_PRET_IMP,
        ShowdownPair::TuUsted => SHOWDOWN_TU_USTED,
        ShowdownPair::LoLe => SHOWDOWN_LO_LE,
        ShowdownPair::SinoPero => SHOWDOWN_SINO_PERO,
        ShowdownPair::ParaQuePorque => SHOWDOWN_PARA_QUE_PORQUE,
        ShowdownPair::TenerHaber => SHOWDOWN_TENER_HABER,
        ShowdownPair::SaberConocer => SHOWDOWN_SABER_CONOCER,
        ShowdownPair::MuyMucho => SHOWDOWN_MUY_MUCHO,
        ShowdownPair::PedirPreguntar => SHOWDOWN_PEDIR_PREGUNTAR,
        ShowdownPair::LlevarTraer => SHOWDOWN_LLEVAR_TRAER,
        ShowdownPair::HaberEstar => SHOWDOWN_HABER_ESTAR,
        ShowdownPair::IrIrse => SHOWDOWN_IR_IRSE,
        ShowdownPair::BienBueno => SHOWDOWN_BIEN_BUENO,
    }
}

/// Generates a sequence of binary showdown arcade items for a given showdown pair.
pub fn generate_showdown_items(pair: ShowdownPair, count: usize) -> Vec<ArcadeItem> {
    if count == 0 {
        return Vec::new();
    }

    let pool = get_showdown_pool(pair);
    let mut rng = rand::thread_rng();
    let mut items = Vec::with_capacity(count);

    // Shuffle indices for variety
    let mut indices: Vec<usize> = (0..pool.len()).collect();
    indices.shuffle(&mut rng);

    for i in 0..count {
        let sentence_data = &pool[indices[i % pool.len()]];
        let swap_options: bool = rng.gen_bool(0.5);

        let (options, correct_index) = if swap_options {
            (
                vec![
                    sentence_data.distractor.to_string(),
                    sentence_data.target.to_string(),
                ],
                1,
            )
        } else {
            (
                vec![
                    sentence_data.target.to_string(),
                    sentence_data.distractor.to_string(),
                ],
                0,
            )
        };

        let prompt_cue = format!("[1] {}  |  [2] {}", options[0], options[1]);

        items.push(ArcadeItem {
            topic: pair.slug().to_string(),
            trigger_sentence: sentence_data.sentence.to_string(),
            prompt_cue,
            options,
            correct_index,
            explanation: sentence_data.explanation.to_string(),
            plain_english: sentence_data.plain_english.to_string(),
        });
    }

    items
}

static TOPIC_DISTRACTOR_POOLS: &[(&str, &[&str])] = &[
    (
        "subjunctive",
        &[
            "ponga",
            "pone",
            "pusiera",
            "pondría",
            "salga",
            "sale",
            "saliera",
            "sepa",
            "sabe",
            "supiera",
            "esté",
            "está",
            "estuviera",
            "dé",
            "da",
            "diera",
            "vaya",
            "va",
            "fuera",
            "haya",
            "ha",
            "hubiera",
            "tenga",
            "tiene",
            "tuviera",
            "haga",
            "hace",
            "hiciera",
            "pueda",
            "puede",
            "pudiera",
            "vea",
            "ve",
            "viera",
            "diga",
            "dice",
            "dijera",
        ],
    ),
    (
        "por-para",
        &[
            "por", "para", "a", "hacia", "de", "con", "en", "según", "mediante", "durante", "sobre",
        ],
    ),
    (
        "ser-estar",
        &[
            "es", "está", "son", "están", "era", "estaba", "fue", "estuvo", "sea", "esté", "sería",
            "estaría", "siendo", "estando",
        ],
    ),
    (
        "past",
        &[
            "desplegó",
            "desplegaba",
            "iba",
            "fue",
            "revisaba",
            "revisó",
            "sonó",
            "sonaba",
            "tenía",
            "tuvo",
            "firmó",
            "firmaba",
            "era",
            "reinicié",
            "reiniciaba",
            "comía",
            "comió",
            "comenzó",
            "comenzaba",
            "pudo",
            "podía",
            "supo",
            "sabía",
        ],
    ),
    (
        "pronouns",
        &[
            "se lo", "se la", "se los", "se las", "le lo", "lo le", "me lo", "te lo", "nos lo",
            "se le", "le", "les", "lo", "la", "los", "las",
        ],
    ),
    (
        "prepositions",
        &[
            "con", "de", "en", "a", "por", "para", "hacia", "desde", "sobre", "ante", "tras",
            "sin", "contra", "según",
        ],
    ),
    (
        "accidental-se",
        &[
            "se me",
            "se te",
            "se le",
            "se nos",
            "se les",
            "me se",
            "se me cayó",
            "se le olvidó",
            "se nos perdió",
            "se me rompieron",
            "se te borraron",
        ],
    ),
    (
        "tech-software",
        &[
            "compilar",
            "desplegar",
            "refactorizar",
            "depurar",
            "optimizar",
            "migrar",
            "escalar",
            "autenticar",
            "encapsular",
            "concurrencia",
            "latencia",
            "rendimiento",
            "repositorio",
        ],
    ),
    (
        "business",
        &[
            "estimado",
            "atentamente",
            "cordialmente",
            "adjunto",
            "convocar",
            "facturar",
            "presupuesto",
            "sinergia",
            "proveedor",
            "asamblea",
            "cláusula",
            "dictamen",
        ],
    ),
    (
        "false-friends",
        &[
            "actual",
            "realizar",
            "éxito",
            "pretender",
            "atender",
            "constipado",
            "sensible",
            "embarazada",
            "carpeta",
            "soportar",
            "lectura",
            "discusión",
        ],
    ),
    (
        "voseo",
        &[
            "tenés", "tienes", "tenéis", "sos", "eres", "sois", "podés", "puedes", "hacés",
            "haces", "hacé", "haz", "querés", "quieres", "mirá", "mira",
        ],
    ),
    (
        "accents",
        &[
            "sí", "si", "sé", "se", "tú", "tu", "él", "el", "mí", "mi", "dé", "de", "más", "mas",
            "aún", "aun", "por qué", "porque", "por que", "porqué",
        ],
    ),
    (
        "epistemic-conjecture",
        &[
            "serán",
            "serían",
            "habrán sido",
            "habrían sido",
            "son",
            "eran",
            "fueron",
            "estarán",
            "estaría",
            "tendrá",
            "tendría",
            "habrá tenido",
        ],
    ),
    (
        "clitic-doubling",
        &[
            "le", "les", "lo", "la", "los", "las", "se", "me", "te", "nos", "a él", "a ella",
        ],
    ),
    (
        "personal-a",
        &[
            "a", "al", "a la", "a los", "para", "hacia", "con", "de", "en", "por",
        ],
    ),
    (
        "gerund-rules",
        &[
            "desarrollando",
            "habiendo desarrollado",
            "tras desarrollar",
            "al desarrollar",
            "desarrollado",
            "desarrollar",
            "optimizando",
            "ejecutando",
        ],
    ),
    (
        "adversatives",
        &[
            "pero",
            "sino",
            "sino que",
            "aunque",
            "sin embargo",
            "no obstante",
            "mas",
            "por el contrario",
        ],
    ),
    (
        "legal-subjunctive",
        &[
            "hubiere",
            "tuviere",
            "fuere",
            "hiciere",
            "hubiera",
            "haya",
            "tenga",
            "tuviera",
            "sea",
            "fuera",
            "dispusiere",
            "contraviniere",
        ],
    ),
    (
        "verbs-of-becoming",
        &[
            "se convirtió en",
            "se volvió",
            "se puso",
            "se hizo",
            "se quedó",
            "llegó a ser",
            "pasó a ser",
            "transformó en",
        ],
    ),
    (
        "epistemic-adverbs",
        &[
            "a lo mejor",
            "quizás",
            "tal vez",
            "probablemente",
            "seguramente",
            "acaso",
            "igual",
            "posiblemente",
        ],
    ),
    (
        "possessive-datives",
        &[
            "me", "te", "le", "nos", "les", "se", "mi", "tu", "su", "mis", "tus", "sus",
        ],
    ),
    (
        "corrective-polarity",
        &[
            "sino que",
            "pero",
            "sino",
            "en cambio",
            "por el contrario",
            "antes bien",
            "no obstante",
        ],
    ),
    (
        "participial-absolutes",
        &[
            "terminado",
            "terminada",
            "terminados",
            "terminadas",
            "habiendo terminado",
            "concluido",
            "finalizado",
            "resuelto",
            "aprobado",
        ],
    ),
    (
        "scalar-concession",
        &[
            "por mucho que",
            "por muy",
            "por más que",
            "aun a riesgo de que",
            "aun a sabiendas de que",
            "por poco que",
            "a pesar de que",
            "si bien",
        ],
    ),
    (
        "tener-haber",
        &[
            "he", "tengo", "ha", "tiene", "hay", "había", "tenía", "habrán", "tendrán", "hemos",
            "tenemos",
        ],
    ),
    (
        "saber-conocer",
        &[
            "sé",
            "conozco",
            "sabe",
            "conoce",
            "sabes",
            "conoces",
            "sabemos",
            "conocemos",
            "conocí",
            "supe",
            "conocen",
            "saben",
        ],
    ),
    (
        "muy-mucho",
        &[
            "muy",
            "mucho",
            "mucha",
            "muchos",
            "muchas",
            "muchísimo",
            "tanto",
            "demasiado",
        ],
    ),
    (
        "pedir-preguntar",
        &[
            "pedir",
            "preguntar",
            "pido",
            "pregunto",
            "pidió",
            "preguntó",
            "pedimos",
            "preguntamos",
            "pide",
            "pregunta",
        ],
    ),
    (
        "llevar-traer",
        &[
            "llevar", "traer", "llevo", "traigo", "lleva", "trae", "llevó", "trajo", "llévate",
            "tráeme", "llevamos", "traemos",
        ],
    ),
    (
        "haber-estar",
        &[
            "hay", "está", "están", "hubo", "estuvo", "había", "estaba", "haya", "esté",
        ],
    ),
    (
        "ir-irse",
        &[
            "ir",
            "irse",
            "voy",
            "me voy",
            "va",
            "se va",
            "vamos",
            "nos vamos",
            "vámonos",
            "irme",
            "vas",
            "te vas",
        ],
    ),
    (
        "bien-bueno",
        &[
            "bien", "bueno", "buen", "buena", "buenos", "buenas", "mal", "malo",
        ],
    ),
];

fn canonicalize_topic(topic: &str) -> String {
    let clean = topic.trim().to_lowercase().replace('_', "-");
    if let Some(concept) = crate::core::reference::get_grammar_concept(&clean) {
        concept.slug.to_string()
    } else {
        clean
    }
}

/// Generates candidate distractors for a drill item across conjugations, topic lexicon, and frames.
fn collect_distractor_candidates(drill: &DrillItem, canonical_slug: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    // 1. If target_verb is provided, collect all inflected forms of the verb
    if !drill.target_verb.is_empty() {
        if let Some(table) = conjugate_verb(&drill.target_verb) {
            let forms = [
                &table.present.yo,
                &table.present.tu,
                &table.present.vos,
                &table.present.el_ella_usted,
                &table.present.nosotros,
                &table.present.vosotros,
                &table.present.ellos_ellas_ustedes,
                &table.preterite.yo,
                &table.preterite.tu,
                &table.preterite.el_ella_usted,
                &table.preterite.nosotros,
                &table.preterite.ellos_ellas_ustedes,
                &table.imperfect.yo,
                &table.imperfect.tu,
                &table.imperfect.el_ella_usted,
                &table.imperfect.nosotros,
                &table.imperfect.ellos_ellas_ustedes,
                &table.future.yo,
                &table.future.tu,
                &table.future.el_ella_usted,
                &table.conditional.yo,
                &table.conditional.tu,
                &table.conditional.el_ella_usted,
                &table.present_subjunctive.yo,
                &table.present_subjunctive.tu,
                &table.present_subjunctive.el_ella_usted,
                &table.present_subjunctive.nosotros,
                &table.present_subjunctive.ellos_ellas_ustedes,
                &table.imperfect_subjunctive_ra.yo,
                &table.imperfect_subjunctive_ra.tu,
                &table.imperfect_subjunctive_ra.el_ella_usted,
                &table.imperfect_subjunctive_ra.nosotros,
                &table.imperfect_subjunctive_se.yo,
                &table.imperfect_subjunctive_se.tu,
                &table.imperfect_subjunctive_se.el_ella_usted,
                &table.imperative_affirmative.tu,
                &table.imperative_affirmative.usted,
                &table.imperative_affirmative.nosotros,
                &table.imperative_affirmative.ustedes,
                &table.gerund,
                &table.participle,
            ];
            for form in forms {
                if !form.is_empty() {
                    candidates.push(form.clone());
                }
            }
        }
    }

    // 2. Collect targets from other frames sharing the same topic
    for frame in FRAMES.iter().filter(|f| f.topic == canonical_slug) {
        if !frame.target.is_empty() {
            candidates.push(frame.target.to_string());
        }
    }

    // 3. Topic distractor pool lookup
    for &(topic, pool) in TOPIC_DISTRACTOR_POOLS {
        if topic == canonical_slug {
            for &item in pool {
                candidates.push(item.to_string());
            }
        }
    }

    // 4. Morphological variations on the target word
    let t = drill.target.trim();
    if t.ends_with('a') && t.len() > 1 {
        candidates.push(format!("{}o", &t[..t.len() - 1]));
        candidates.push(format!("{}e", &t[..t.len() - 1]));
        candidates.push(format!("{}as", &t[..t.len() - 1]));
        candidates.push(format!("{}an", &t[..t.len() - 1]));
    } else if t.ends_with('o') && t.len() > 1 {
        candidates.push(format!("{}a", &t[..t.len() - 1]));
        candidates.push(format!("{}e", &t[..t.len() - 1]));
        candidates.push(format!("{}os", &t[..t.len() - 1]));
        candidates.push(format!("{}ió", &t[..t.len() - 1]));
    } else if t.ends_with('e') && t.len() > 1 {
        candidates.push(format!("{}a", &t[..t.len() - 1]));
        candidates.push(format!("{}o", &t[..t.len() - 1]));
        candidates.push(format!("{}en", &t[..t.len() - 1]));
        candidates.push(format!("{}es", &t[..t.len() - 1]));
    }

    // 5. Generic grammatical distractors fallback
    static GENERIC_FALLBACK: &[&str] = &[
        "haya",
        "tenga",
        "sea",
        "esté",
        "haga",
        "pueda",
        "vaya",
        "diga",
        "sepa",
        "quiera",
        "ha",
        "tiene",
        "es",
        "está",
        "hace",
        "puede",
        "va",
        "dice",
        "sabe",
        "quiere",
        "hubiera",
        "tuviera",
        "fuera",
        "estuviera",
        "hiciera",
        "pudiera",
        "por",
        "para",
        "con",
        "de",
        "en",
        "a",
        "se lo",
        "se la",
        "le",
        "lo",
        "la",
        "les",
        "pero",
        "sino",
        "sino que",
        "porque",
        "para que",
    ];

    for &g in GENERIC_FALLBACK {
        candidates.push(g.to_string());
    }

    candidates
}

/// Synthesizes 3 unique distractors for a target answer.
fn synthesize_distractors<R: Rng + ?Sized>(
    drill: &DrillItem,
    canonical_slug: &str,
    rng: &mut R,
) -> Vec<String> {
    let raw_candidates = collect_distractor_candidates(drill, canonical_slug);
    let target_clean = drill.target.trim().to_lowercase();

    let mut distinct: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(target_clean.clone());

    let mut shuffled = raw_candidates;
    shuffled.shuffle(rng);

    for cand in shuffled {
        let clean = cand.trim().to_string();
        let lower = clean.to_lowercase();
        if !clean.is_empty() && !seen.contains(&lower) {
            seen.insert(lower);
            distinct.push(clean);
            if distinct.len() == 3 {
                break;
            }
        }
    }

    // Guarantee exactly 3 unique distractors even in extreme edge cases
    let mut counter = 1;
    while distinct.len() < 3 {
        let fallback = format!("{}_{}", drill.target, counter);
        let lower = fallback.to_lowercase();
        if !seen.contains(&lower) {
            seen.insert(lower);
            distinct.push(fallback);
        }
        counter += 1;
    }

    distinct
}

#[derive(Debug, Clone)]
pub struct SpecializedEngineSentence {
    pub sentence: &'static str,
    pub target: &'static str,
    pub distractors: [&'static str; 3],
    pub explanation: &'static str,
    pub plain_english: &'static str,
}

static ENGINE_REGIMEN_POOL: &[SpecializedEngineSentence] = &[
    SpecializedEngineSentence {
        sentence: "Sueño ____ viajar por el mundo entero.",
        target: "con",
        distractors: ["de", "en", "por"],
        explanation: "'Soñar con' requires 'con' to express dreaming about or of something.",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'con'.",
    },
    SpecializedEngineSentence {
        sentence: "Siempre pienso ____ las soluciones más simples.",
        target: "en",
        distractors: ["de", "con", "a"],
        explanation: "'Pensar en' requires 'en' to express thinking about someone or something.",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'en'.",
    },
    SpecializedEngineSentence {
        sentence: "No me acordé ____ cerrar la sesión en el servidor.",
        target: "de",
        distractors: ["en", "con", "a"],
        explanation: "'Acordarse de' requires 'de' (to remember to do something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'de'.",
    },
    SpecializedEngineSentence {
        sentence: "Fíjate ____ los detalles del contrato antes de firmar.",
        target: "en",
        distractors: ["a", "de", "con"],
        explanation: "'Fijarse en' requires 'en' (to notice / pay attention to something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'en'.",
    },
    SpecializedEngineSentence {
        sentence: "El éxito del proyecto depende ____ nuestro esfuerzo.",
        target: "de",
        distractors: ["en", "por", "a"],
        explanation: "'Depender de' requires 'de' (to depend on).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'de'.",
    },
    SpecializedEngineSentence {
        sentence: "Se enamoró ____ la cultura y gastronomía española.",
        target: "de",
        distractors: ["con", "en", "a"],
        explanation: "'Enamorarse de' requires 'de' (to fall in love with).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'de'.",
    },
    SpecializedEngineSentence {
        sentence: "Me di cuenta ____ que cometí un error grave.",
        target: "de",
        distractors: ["en", "con", "a"],
        explanation: "'Darse cuenta de' requires 'de' (to realize something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'de'.",
    },
    SpecializedEngineSentence {
        sentence: "El cliente insiste ____ cambiar el diseño actual.",
        target: "en",
        distractors: ["de", "por", "con"],
        explanation: "'Insistir en' requires 'en' (to insist on doing something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'en'.",
    },
    SpecializedEngineSentence {
        sentence: "El examen técnico consiste ____ tres secciones prácticas.",
        target: "en",
        distractors: ["de", "por", "con"],
        explanation: "'Consistir en' requires 'en' (to consist of).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'en'.",
    },
    SpecializedEngineSentence {
        sentence: "Estoy aprendiendo ____ programar sistemas en Rust.",
        target: "a",
        distractors: ["en", "de", "por"],
        explanation: "'Aprender a' + infinitive requires 'a' (to learn to do something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'a'.",
    },
    SpecializedEngineSentence {
        sentence: "No me acostumbro ____ este nuevo horario de trabajo.",
        target: "a",
        distractors: ["con", "en", "de"],
        explanation: "'Acostumbrarse a' requires 'a' (to get used to something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'a'.",
    },
    SpecializedEngineSentence {
        sentence: "El testigo se negó ____ firmar el documento.",
        target: "a",
        distractors: ["de", "en", "por"],
        explanation: "'Negarse a' + infinitive requires 'a' (to refuse to do something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'a'.",
    },
    SpecializedEngineSentence {
        sentence: "No te preocupes ____ cosas que no puedes controlar.",
        target: "por",
        distractors: ["de", "en", "para"],
        explanation: "'Preocuparse por' requires 'por' to denote the cause of worry.",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'por'.",
    },
    SpecializedEngineSentence {
        sentence: "Siempre se queja ____ la lentitud de la conexión.",
        target: "de",
        distractors: ["en", "por", "con"],
        explanation: "'Quejarse de' requires 'de' (to complain about something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'de'.",
    },
    SpecializedEngineSentence {
        sentence: "Nadie se atreve ____ contradecir al director general.",
        target: "a",
        distractors: ["de", "en", "con"],
        explanation: "'Atreverse a' + infinitive requires 'a' (to dare to do something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'a'.",
    },
    SpecializedEngineSentence {
        sentence: "Tardamos dos horas ____ compilar el proyecto entero.",
        target: "en",
        distractors: ["a", "de", "para"],
        explanation:
            "'Tardar [tiempo] en' + infinitive requires 'en' (to take time to do something).",
        plain_english:
            "Prepositional regimen: this specific verb obligatorily links to its object via 'en'.",
    },
];

static ENGINE_IRREGULARS_POOL: &[SpecializedEngineSentence] = &[
    SpecializedEngineSentence {
        sentence: "Ayer yo ____ el archivo en el servidor.",
        target: "puse",
        distractors: ["poní", "ponía", "pongí"],
        explanation: "Preterite of 'poner' (yo) has irregular stem 'pus-': puse (not poní).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'puse'.",
    },
    SpecializedEngineSentence {
        sentence: "Ella no ____ aceptar la oferta laboral.",
        target: "quiso",
        distractors: ["querió", "quería", "querrá"],
        explanation: "Preterite of 'querer' (ella) has irregular stem 'quis-': quiso (not querió).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'quiso'.",
    },
    SpecializedEngineSentence {
        sentence: "La caja no ____ en el maletero del coche.",
        target: "cupo",
        distractors: ["cabió", "cabía", "caberá"],
        explanation: "Preterite of 'caber' (ella/él) has irregular stem 'cup-': cupo (not cabió).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'cupo'.",
    },
    SpecializedEngineSentence {
        sentence: "¿Qué ____ tú para la reunión de hoy?",
        target: "trajiste",
        distractors: ["traíste", "traías", "traerás"],
        explanation:
            "Preterite of 'traer' (tú) has irregular stem 'traj-': trajiste (not traíste).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'trajiste'.",
    },
    SpecializedEngineSentence {
        sentence: "Él ____ toda la noche por carreteras oscuras.",
        target: "condujo",
        distractors: ["conducía", "conducirá", "conducio"],
        explanation: "Preterite of 'conducir' (él) has j-stem: condujo (no accent, ending in -o).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'condujo'.",
    },
    SpecializedEngineSentence {
        sentence: "Nosotros ____ diez kilómetros bajo la lluvia ayer.",
        target: "anduvimos",
        distractors: ["andamos", "andábamos", "andaremos"],
        explanation: "Preterite of 'andar' (nosotros) has irregular stem 'anduv-': anduvimos.",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'anduvimos'.",
    },
    SpecializedEngineSentence {
        sentence: "Cuando lo ____, llamé a la policía de inmediato.",
        target: "supe",
        distractors: ["sabí", "sabía", "sabré"],
        explanation:
            "Preterite of 'saber' (yo) has irregular stem 'sup-': supe (meaning 'found out').",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'supe'.",
    },
    SpecializedEngineSentence {
        sentence: "Ayer ____ muchos problemas de red en la oficina.",
        target: "hubo",
        distractors: ["habió", "había", "habrá"],
        explanation: "Preterite impersonal of 'haber' is 'hubo' (never habió).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'hubo'.",
    },
    SpecializedEngineSentence {
        sentence: "Yo no ____ en este asiento tan estrecho.",
        target: "quepo",
        distractors: ["cabo", "caberé", "caigo"],
        explanation:
            "Present indicative 1st person (yo) of 'caber' is irregular: quepo (never cabo).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'quepo'.",
    },
    SpecializedEngineSentence {
        sentence: "Yo no ____ cómo resolver este problema complejo.",
        target: "sé",
        distractors: ["sabo", "sepa", "supe"],
        explanation:
            "Present indicative 1st person (yo) of 'saber' is 'sé' with accent (never sabo).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'sé'.",
    },
    SpecializedEngineSentence {
        sentence: "Espero que tú lo ____ pronto para decidir.",
        target: "sepas",
        distractors: ["sabes", "sabas", "supieras"],
        explanation: "Present subjunctive (tú) of 'saber' uses stem 'sep-': sepas.",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'sepas'.",
    },
    SpecializedEngineSentence {
        sentence: "Ojalá que todo ____ en una sola maleta.",
        target: "quepa",
        distractors: ["caba", "cabe", "cabiera"],
        explanation: "Present subjunctive of 'caber' uses stem 'quep-': quepa (never caba).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'quepa'.",
    },
    SpecializedEngineSentence {
        sentence: "Dudo que ____ otra oportunidad similar.",
        target: "haya",
        distractors: ["haiga", "ha", "hubo"],
        explanation:
            "Present subjunctive of 'haber' is 'haya' ('haiga' is incorrect/non-standard).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'haya'.",
    },
    SpecializedEngineSentence {
        sentence: "Mañana yo ____ la nueva versión en producción.",
        target: "pondré",
        distractors: ["poneré", "pondría", "ponga"],
        explanation:
            "Future indicative of 'poner' (yo) has syncopated d-stem: pondré (never poneré).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'pondré'.",
    },
    SpecializedEngineSentence {
        sentence: "¿Qué ____ tú este fin de semana en Madrid?",
        target: "harás",
        distractors: ["hacerás", "hacías", "hiciste"],
        explanation:
            "Future indicative of 'hacer' (tú) has contracted stem 'har-': harás (never hacerás).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'harás'.",
    },
    SpecializedEngineSentence {
        sentence: "Nosotros ____ a las ocho de la mañana.",
        target: "saldremos",
        distractors: ["saliremos", "saldríamos", "salgamos"],
        explanation:
            "Future indicative of 'salir' (nosotros) has d-stem: saldremos (never saliremos).",
        plain_english:
            "Irregular verb morphology: applies the specialized stem and ending for 'saldremos'.",
    },
];

static ENGINE_FALSE_FRIENDS_POOL: &[SpecializedEngineSentence] = &[
    SpecializedEngineSentence {
        sentence: "La situación política ____ es muy compleja.",
        target: "actual",
        distractors: ["real", "actualizada", "revelada"],
        explanation:
            "'Actual' means current/present-day in Spanish, whereas 'real' means real/royal.",
        plain_english: "False friend: 'actual' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "María está ____ de seis meses y dará a luz pronto.",
        target: "embarazada",
        distractors: ["avergonzada", "molesta", "apenada"],
        explanation: "'Embarazada' means pregnant. 'Avergonzado/a' means embarrassed.",
        plain_english: "False friend: 'embarazada' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "La nueva aplicación tuvo un gran ____ de ventas.",
        target: "éxito",
        distractors: ["salida", "suceso", "partida"],
        explanation: "'Éxito' means success. 'Salida' means exit.",
        plain_english: "False friend: 'éxito' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "El médico va a ____ a los pacientes ahora.",
        target: "atender",
        distractors: ["asistir", "pretender", "apoyar"],
        explanation: "'Atender' means to serve/attend to. To attend an event is 'asistir a'.",
        plain_english: "False friend: 'atender' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "Mañana voy a ____ a la conferencia internacional.",
        target: "asistir",
        distractors: ["atender", "esperar", "pretender"],
        explanation:
            "'Asistir a' means to attend an event. 'Atender' means to serve or pay attention.",
        plain_english: "False friend: 'asistir' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "Él es muy ____ y se ofende con facilidad.",
        target: "sensible",
        distractors: ["sensato", "razonable", "prudente"],
        explanation: "'Sensible' means sensitive/emotional. 'Sensato' means sensible/prudent.",
        plain_english: "False friend: 'sensible' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "Tomar precauciones antes del viaje fue una decisión ____.",
        target: "sensata",
        distractors: ["sensible", "sentimental", "sencilla"],
        explanation: "'Sensato/a' means sensible/prudent. 'Sensible' means sensitive.",
        plain_english: "False friend: 'sensata' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "El discurso fue demasiado ____ y aburrido.",
        target: "largo",
        distractors: ["grande", "ancho", "grueso"],
        explanation: "'Largo' means long in length/duration. 'Grande' means large/big.",
        plain_english: "False friend: 'largo' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "Guarda todos los documentos en esta ____ azul.",
        target: "carpeta",
        distractors: ["alfombra", "cartelera", "cartera"],
        explanation: "'Carpeta' means folder/binder. 'Alfombra' means carpet/rug.",
        plain_english: "False friend: 'carpeta' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "No puedo ____ el ruido constante de las obras.",
        target: "soportar",
        distractors: ["apoyar", "sostener", "asistir"],
        explanation: "'Soportar' means to bear/tolerate. To support someone is 'apoyar'.",
        plain_english: "False friend: 'soportar' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "No ____ ofenderte con mis comentarios.",
        target: "pretendo",
        distractors: ["finjo", "presumo", "aparento"],
        explanation: "'Pretender' means to aim/intend. To pretend/fake is 'fingir'.",
        plain_english: "False friend: 'pretendo' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "Vamos a ____ una auditoría exhaustiva del código.",
        target: "realizar",
        distractors: ["darnos cuenta", "comprender", "advertir"],
        explanation: "'Realizar' means to carry out/execute. To realize is 'darse cuenta de'.",
        plain_english: "False friend: 'realizar' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "Tengo congestión nasal porque estoy ____.",
        target: "constipado",
        distractors: ["estreñido", "bloqueado", "atrapado"],
        explanation: "'Constipado' means having a cold/congested. 'Estreñido' means constipated.",
        plain_english: "False friend: 'constipado' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "No puedo ____ dónde dejé las llaves del coche.",
        target: "recordar",
        distractors: ["grabar", "guardar", "registrar"],
        explanation: "'Recordar' means to remember. 'Grabar' means to record audio/video.",
        plain_english: "False friend: 'recordar' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "No me gusta ____ sobre temas polémicos en público.",
        target: "discutir",
        distractors: ["charlar", "conversar", "dialogar"],
        explanation:
            "'Discutir' means to argue/dispute. To discuss casually is 'hablar' or 'charlar'.",
        plain_english: "False friend: 'discutir' correctly conveys the intended meaning without falling for the English lookalike.",
    },
    SpecializedEngineSentence {
        sentence: "El periódico reportó un grave ____ en el centro.",
        target: "suceso",
        distractors: ["éxito", "logro", "triunfo"],
        explanation: "'Suceso' means event/incident. 'Éxito' means success.",
        plain_english: "False friend: 'suceso' correctly conveys the intended meaning without falling for the English lookalike.",
    },
];

static ENGINE_SE_MATRIX_POOL: &[SpecializedEngineSentence] = &[
    SpecializedEngineSentence {
        sentence: "A Juan ____ la contraseña del servidor.",
        target: "se le olvidó",
        distractors: ["le olvidó", "se olvidó", "olvidó"],
        explanation: "Accidental/involuntary 'se' (se le olvidó) marks unintentional forgetting.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "A mí ____ las llaves al suelo.",
        target: "se me cayeron",
        distractors: ["me cayeron", "se cayeron", "las caí"],
        explanation: "Accidental 'se' with 1st-person dative (se me cayeron) denotes accidental dropping.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "A nosotros ____ el coche en la autopista.",
        target: "se nos descompuso",
        distractors: ["nos descompuso", "se descompuso", "lo descompusimos"],
        explanation: "Accidental 'se' with 1st-person plural dative (se nos descompuso) marks unexpected breakdown.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "A ellos ____ el billete de tren en la estación.",
        target: "se les perdió",
        distractors: ["les perdió", "se perdió", "perdieron"],
        explanation: "Accidental 'se' with 3rd-person plural dative (se les perdió) marks involuntary loss.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "En este país ____ muy bien y con tranquilidad.",
        target: "se vive",
        distractors: ["vive", "se viven", "viven"],
        explanation: "Impersonal 'se' with intransitive verb takes singular verb (se vive).",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "En la biblioteca ____ guardar silencio absoluto.",
        target: "se debe",
        distractors: ["debe", "se deben", "deben"],
        explanation: "Impersonal 'se' (se debe) expresses general obligation without named agent.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "Aquí ____ con mucha dedicación y entrega.",
        target: "se trabaja",
        distractors: ["trabaja", "se trabajan", "trabajan"],
        explanation: "Impersonal 'se' (se trabaja) expresses general collective action in singular.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "En esa agencia ____ oficinas comerciales.",
        target: "se alquilan",
        distractors: ["se alquila", "alquilan", "están alquilando"],
        explanation: "Passive reflexive (pasiva refleja): plural verb agrees with plural subject (oficinas).",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "En la recepción ____ español e inglés.",
        target: "se hablan",
        distractors: ["se habla", "hablan", "está hablando"],
        explanation: "Passive reflexive: plural verb agrees with compound subject (español e inglés).",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "Ayer ____ el vuelo por el mal tiempo.",
        target: "se canceló",
        distractors: ["se cancelaron", "canceló", "cancelaron"],
        explanation: "Passive reflexive with singular subject (el vuelo se canceló).",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "En esta imprenta ____ miles de libros al día.",
        target: "se imprimen",
        distractors: ["se imprime", "imprimen", "están imprimiendo"],
        explanation: "Passive reflexive agrees with plural grammatical subject (miles de libros se imprimen).",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "Los dos presidentes ____ cordialmente.",
        target: "se saludaron",
        distractors: ["saludaron", "le saludaron", "se saludó"],
        explanation: "Reciprocal 'se' (se saludaron) indicates mutual action between two parties.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "Mis amigos y yo ____ mutuamente en el proyecto.",
        target: "nos ayudamos",
        distractors: ["se ayudan", "ayudamos", "les ayudamos"],
        explanation: "Reciprocal 'nos' (nos ayudamos) expresses mutual assistance among 'nosotros'.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "Él ____ toda la tarta él solo.",
        target: "se comió",
        distractors: ["comió", "se comieron", "le comió"],
        explanation: "Aspectual/telic 'se' (se comió) emphasizes total consumption of the entire object.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "Ya es tarde, así que yo ____ a casa.",
        target: "me voy",
        distractors: ["voy", "me va", "se va"],
        explanation: "Aspectual departure 'irse' (me voy) emphasizes departing from the current location.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
    SpecializedEngineSentence {
        sentence: "El bebé ____ en cuanto lo acostamos.",
        target: "se durmió",
        distractors: ["durmió", "le durmió", "se duerme"],
        explanation: "Inchoative/aspectual 'dormirse' (se durmió) denotes entering the state of sleep.",
        plain_english: "Se matrix: identifies the distinct functional role of 'se' in the sentence.",
    },
];

static ENGINE_CONNECTORS_POOL: &[SpecializedEngineSentence] = &[
    SpecializedEngineSentence {
        sentence: "Estudió mucho; ____, no aprobó el examen.",
        target: "sin embargo",
        distractors: ["por lo tanto", "es decir", "además"],
        explanation: "'Sin embargo' introduces an adversative contrast / counter-expectation.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "El plan es ambicioso; ____, contamos con el presupuesto necesario.",
        target: "no obstante",
        distractors: ["por consiguiente", "o sea", "por ende"],
        explanation: "'No obstante' expresses formal adversative concession or contrast.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "A él le gusta viajar; ____, su hermano prefiere quedarse en casa.",
        target: "en cambio",
        distractors: ["por tanto", "así que", "de modo que"],
        explanation: "'En cambio' highlights contrast and opposition between two subjects.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "No tenemos fondos suficientes; ____, debemos posponer el proyecto.",
        target: "por lo tanto",
        distractors: ["sin embargo", "a pesar de que", "en cambio"],
        explanation: "'Por lo tanto' introduces a consecutive result or deduction.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "El servidor falló; ____, el servicio quedó interrumpido.",
        target: "por consiguiente",
        distractors: ["no obstante", "aunque", "sino"],
        explanation: "'Por consiguiente' introduces a formal direct consequence (consequently).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "No guardó los cambios; ____ perdiera todo el trabajo del día.",
        target: "de ahí que",
        distractors: ["por eso", "ya que", "dado que"],
        explanation:
            "'De ahí que' expresses consequence and governs the subjunctive mood (perdiera).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "Llegamos tarde ____ había un tráfico tremendo en la autopista.",
        target: "dado que",
        distractors: ["por lo tanto", "sin embargo", "es decir"],
        explanation: "'Dado que' introduces a causal explanation or premise (given that).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "Suspendieron el vuelo ____ la tormenta de nieve.",
        target: "debido a",
        distractors: ["a pesar de", "en cambio", "por consiguiente"],
        explanation: "'Debido a' + noun phrase indicates the cause or reason for the action.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "____ llovía intensamente, salimos a correr.",
        target: "A pesar de que",
        distractors: ["Por lo tanto", "Es decir", "Por consiguiente"],
        explanation: "'A pesar de que' introduces a concessive clause (even though / despite).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "Fuimos a la montaña, ____ el pronóstico anunciaba tormentas.",
        target: "aun cuando",
        distractors: ["por lo tanto", "en cambio", "es decir"],
        explanation: "'Aun cuando' introduces a concessive clause (even though).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "El código es modular, ____, cada componente tiene una sola responsabilidad.",
        target: "es decir",
        distractors: ["sin embargo", "por consiguiente", "a pesar de"],
        explanation: "'Es decir' introduces an explanatory reformulation or clarification.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "No es un fallo crítico, ____, no compromete la seguridad del sistema.",
        target: "o sea",
        distractors: ["sin embargo", "a pesar de que", "por el contrario"],
        explanation: "'O sea' provides a clarifying or reformulating phrase (in other words).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "Hemos analizado los datos; ____, concluimos que la hipótesis es válida.",
        target: "por ende",
        distractors: ["no obstante", "en cambio", "a pesar de"],
        explanation: "'Por ende' denotes formal conclusive deduction (therefore / hence).",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "____, reducimos costes; por otro, aumentamos la productividad.",
        target: "Por un lado",
        distractors: ["Sin embargo", "Por lo tanto", "Es decir"],
        explanation:
            "'Por un lado... por otro' organizes discourse into balanced comparative points.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "____, los resultados respaldan completamente nuestra hipótesis inicial.",
        target: "En conclusión",
        distractors: ["Por el contrario", "A pesar de que", "No obstante"],
        explanation: "'En conclusión' introduces an overarching summary or final conclusion.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
    SpecializedEngineSentence {
        sentence: "No solo cumplimos los plazos, ____ superamos los objetivos establecidos.",
        target: "sino que además",
        distractors: ["por lo tanto", "sin embargo", "es decir"],
        explanation: "'No solo... sino que además' introduces an emphatic additive clause.",
        plain_english: "Discourse connector: logically structures contrast, concession, cause, or sequence across arguments.",
    },
];

/// Returns the list of 5 specialized drill engine slugs.
pub fn list_specialized_engines() -> &'static [&'static str] {
    &[
        "regimen",
        "irregulars",
        "false-friends",
        "se-matrix",
        "connectors",
    ]
}

/// Returns a human-friendly display title for a specialized drill engine slug or alias.
pub fn get_engine_title(slug: &str) -> Option<&'static str> {
    let (canonical, _, _) = canonicalize_engine_slug(slug)?;
    match canonical {
        "regimen" => Some("Prepositional Regimen Engine (Verbos con Régimen)"),
        "irregulars" => Some("Irregular Verb Speed Gun (Conjugación Irregular)"),
        "false-friends" => Some("False Friends Trap Detector (Falsos Amigos)"),
        "se-matrix" => Some("The \"Se\" Matrix (Las 5 Caras del Se)"),
        "connectors" => Some("Discourse Connectors & Flow (Conectores B2/C1)"),
        _ => None,
    }
}

pub fn canonicalize_engine_slug(
    slug: &str,
) -> Option<(
    &'static str,
    &'static str,
    &'static [SpecializedEngineSentence],
)> {
    let clean = slug.trim().to_lowercase().replace(['_', ' '], "-");
    match clean.as_str() {
        "regimen"
        | "prepositions"
        | "prep"
        | "verb-regimen"
        | "regimen-preposicional"
        | "verbos-con-preposicion" => {
            Some(("regimen", "Prepositional Regimen", ENGINE_REGIMEN_POOL))
        }
        "irregulars" | "verbs" | "irregular" | "verb-speed" | "irregular-verbs" | "speed-gun" => {
            Some((
                "irregulars",
                "Irregular Verb Speed Gun",
                ENGINE_IRREGULARS_POOL,
            ))
        }
        "false-friends" | "cognates" | "falsos-amigos" | "false-cognates" | "traps" => Some((
            "false-friends",
            "False Friends Trap Detector",
            ENGINE_FALSE_FRIENDS_POOL,
        )),
        "se-matrix" | "se" | "se-types" | "se-matrix-drill" | "valores-de-se" => {
            Some(("se-matrix", "The \"Se\" Matrix", ENGINE_SE_MATRIX_POOL))
        }
        "connectors" | "discourse" | "transitions" | "connectors-flow" | "conectores" => Some((
            "connectors",
            "Discourse Connectors & Flow",
            ENGINE_CONNECTORS_POOL,
        )),
        _ => None,
    }
}

/// Generates 4-choice rapid arcade items for one of the 5 specialized drill engines.
pub fn generate_specialized_engine_items(slug: &str, count: usize) -> Vec<ArcadeItem> {
    if count == 0 {
        return Vec::new();
    }

    let Some((canonical_slug, cue_title, pool)) = canonicalize_engine_slug(slug) else {
        return Vec::new();
    };

    let mut rng = rand::thread_rng();
    let mut items = Vec::with_capacity(count);

    let mut indices: Vec<usize> = (0..pool.len()).collect();
    indices.shuffle(&mut rng);

    for i in 0..count {
        let entry = &pool[indices[i % pool.len()]];
        let mut options = vec![
            entry.target.to_string(),
            entry.distractors[0].to_string(),
            entry.distractors[1].to_string(),
            entry.distractors[2].to_string(),
        ];
        options.shuffle(&mut rng);

        let correct_index = options
            .iter()
            .position(|opt| opt == entry.target)
            .unwrap_or(0);

        let prompt_cue = cue_title.to_string();

        items.push(ArcadeItem {
            topic: canonical_slug.to_string(),
            trigger_sentence: entry.sentence.to_string(),
            prompt_cue,
            options,
            correct_index,
            explanation: entry.explanation.to_string(),
            plain_english: entry.plain_english.to_string(),
        });
    }

    items
}

/// Generates 4-choice rapid cloze arcade items for any grammar topic slug (1 correct + 3 distractors).
pub fn generate_4choice_items(topic: &str, count: usize) -> Vec<ArcadeItem> {
    if count == 0 {
        return Vec::new();
    }

    // Check if the topic matches any of the 5 specialized drill engines first
    if canonicalize_engine_slug(topic).is_some() {
        return generate_specialized_engine_items(topic, count);
    }

    let slug = canonicalize_topic(topic);
    let matching_frames: Vec<&crate::core::generator::SentenceFrame> =
        FRAMES.iter().filter(|f| f.topic == slug).collect();

    let pool = if !matching_frames.is_empty() {
        matching_frames
    } else {
        FRAMES.iter().collect()
    };

    let mut rng = rand::thread_rng();
    let mut items = Vec::with_capacity(count);

    for _ in 0..count {
        let frame_idx = rng.gen_range(0..pool.len());
        let drill = pool[frame_idx].render(&mut rng);

        let distractors = synthesize_distractors(&drill, &slug, &mut rng);

        let mut options = vec![
            drill.target.clone(),
            distractors[0].clone(),
            distractors[1].clone(),
            distractors[2].clone(),
        ];
        options.shuffle(&mut rng);

        let correct_index = options
            .iter()
            .position(|opt| opt == &drill.target)
            .unwrap_or(0);

        let prompt_cue = if !drill.formula_cue.is_empty() {
            drill.formula_cue
        } else {
            format!("Choose the correct form for {}", drill.topic)
        };

        items.push(ArcadeItem {
            topic: slug.clone(),
            trigger_sentence: drill.trigger_sentence,
            prompt_cue,
            options,
            correct_index,
            explanation: drill.explanation,
            plain_english: drill.plain_english,
        });
    }

    items
}
