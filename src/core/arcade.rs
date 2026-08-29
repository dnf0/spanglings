use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::core::conjugator::conjugate_verb;
use crate::core::generator::{DrillItem, FRAMES};

/// Represents one of the 8 high-stakes grammatical showdowns for rapid binary choice drills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShowdownPair {
    PorPara,
    SerEstar,
    SubjInd,
    PretImp,
    TuUsted,
    LoLe,
    SinoPero,
    ParaQuePorque,
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
            | "subjunctive" => Some(ShowdownPair::SubjInd),
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

/// Returns all 8 available showdown pairs.
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
}

static SHOWDOWN_POR_PARA: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Trabajo ____ una empresa multinacional de software.",
        target: "para",
        distractor: "por",
        explanation: "'Para' specifies the employer or direct beneficiary.",
    },
    ShowdownSentence {
        sentence: "Caminamos ____ el centro histórico de la ciudad.",
        target: "por",
        distractor: "para",
        explanation: "'Por' indicates movement through, along, or across a space.",
    },
    ShowdownSentence {
        sentence: "Este informe técnico debe estar terminado ____ el viernes.",
        target: "para",
        distractor: "por",
        explanation: "'Para' sets a firm deadline or temporal goal.",
    },
    ShowdownSentence {
        sentence: "Muchas gracias ____ tu ayuda con la optimización del código.",
        target: "por",
        distractor: "para",
        explanation: "'Por' expresses the reason or cause of gratitude.",
    },
    ShowdownSentence {
        sentence: "Estudio español ____ comunicarme con mis colegas en Bogotá.",
        target: "para",
        distractor: "por",
        explanation: "'Para' + infinitive expresses purpose and intention.",
    },
    ShowdownSentence {
        sentence: "Pagamos cincuenta euros ____ la suscripción anual del servicio.",
        target: "por",
        distractor: "para",
        explanation: "'Por' expresses cost, exchange, or trade value.",
    },
    ShowdownSentence {
        sentence: "El paquete fue enviado ____ mensajería urgente.",
        target: "por",
        distractor: "para",
        explanation: "'Por' denotes the vehicle, channel, or means of transport.",
    },
    ShowdownSentence {
        sentence: "El tren ____ Barcelona sale puntualmente a las diez.",
        target: "para",
        distractor: "por",
        explanation: "'Para' indicates destination or direction of transit.",
    },
    ShowdownSentence {
        sentence: "No pudimos desplegar la versión ____ un corte en el suministro.",
        target: "por",
        distractor: "para",
        explanation: "'Por' indicates the obstacle, cause, or triggering reason.",
    },
    ShowdownSentence {
        sentence: "____ mí, la arquitectura orientada a eventos es superior.",
        target: "para",
        distractor: "por",
        explanation: "'Para mí' introduces personal viewpoint and judgement.",
    },
    ShowdownSentence {
        sentence: "Estuvimos esperando en la sala de juntas ____ dos horas.",
        target: "por",
        distractor: "para",
        explanation: "'Por' indicates the duration or span of time elapsed.",
    },
    ShowdownSentence {
        sentence: "El archivo descargado es únicamente ____ uso interno del equipo.",
        target: "para",
        distractor: "por",
        explanation: "'Para' defines recipient suitability or restricted purpose.",
    },
];

static SHOWDOWN_SER_ESTAR: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El servidor principal ____ fuera de servicio por mantenimiento.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' marks a temporary condition or operational state.",
    },
    ShowdownSentence {
        sentence: "Nuestra empresa ____ especialista en sistemas distribuidos.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' defines fundamental identity and domain classification.",
    },
    ShowdownSentence {
        sentence: "El director de ingeniería ____ en una reunión con clientes.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' locates concrete entities in physical/virtual space.",
    },
    ShowdownSentence {
        sentence: "La presentación técnica ____ en el auditorio del tercer piso.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' locates events taking place at a venue or time.",
    },
    ShowdownSentence {
        sentence: "El nuevo microservicio ya ____ listo para producción.",
        target: "está",
        distractor: "es",
        explanation: "'Estar listo' denotes being prepared / in a ready state.",
    },
    ShowdownSentence {
        sentence: "El arquitecto jefe ____ muy listo resolviendo problemas de concurrencia.",
        target: "es",
        distractor: "está",
        explanation: "'Ser listo' denotes cleverness and inherent intelligence.",
    },
    ShowdownSentence {
        sentence: "La base de datos ____ de solo lectura para evitar corrupciones.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' characterizes permanent structural attributes.",
    },
    ShowdownSentence {
        sentence: "El café en la sala de conferencias ____ frío desde hace rato.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' describes a sensory condition resulting from change.",
    },
    ShowdownSentence {
        sentence: "Elena ____ de Valencia pero reside y trabaja en Madrid.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' indicates origin, nationality, and provenance.",
    },
    ShowdownSentence {
        sentence: "Todos los contenedores Docker ____ corriendo adecuadamente.",
        target: "están",
        distractor: "son",
        explanation: "'Estar' + gerund forms continuous and active state.",
    },
    ShowdownSentence {
        sentence: "Hoy el cielo ____ completamente nublado y hace viento.",
        target: "está",
        distractor: "es",
        explanation: "'Estar' expresses ambient meteorological conditions.",
    },
    ShowdownSentence {
        sentence: "La reunión de planificación ____ a las diez de la mañana.",
        target: "es",
        distractor: "está",
        explanation: "'Ser' denotes the scheduled time of an event.",
    },
];

static SHOWDOWN_SUBJ_IND: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Dudo que el equipo ____ tiempo de terminar la refactorización hoy.",
        target: "tenga",
        distractor: "tiene",
        explanation: "Doubt (dudo que) triggers present subjunctive mood.",
    },
    ShowdownSentence {
        sentence: "Sé con certeza que el equipo ____ la capacidad técnica necesaria.",
        target: "tiene",
        distractor: "tenga",
        explanation: "Factual certainty (sé que) selects indicative mood.",
    },
    ShowdownSentence {
        sentence: "Es necesario que el cliente ____ la propuesta antes del viernes.",
        target: "apruebe",
        distractor: "aprueba",
        explanation: "Impersonal volition/necessity (es necesario que) requires subjunctive.",
    },
    ShowdownSentence {
        sentence: "Es evidente que el cliente ____ el valor de nuestra plataforma.",
        target: "comprende",
        distractor: "comprenda",
        explanation: "Objective certainty (es evidente que) takes indicative.",
    },
    ShowdownSentence {
        sentence: "No creo que el servidor ____ suficiente memoria RAM para el pico.",
        target: "tenga",
        distractor: "tiene",
        explanation: "Negated epistemic belief (no creo que) triggers subjunctive.",
    },
    ShowdownSentence {
        sentence: "Creo firmemente que el servidor ____ suficiente memoria RAM.",
        target: "tiene",
        distractor: "tenga",
        explanation: "Affirmative belief (creo que) takes indicative.",
    },
    ShowdownSentence {
        sentence: "Busco un programador que ____ experiencia en sistemas distribuidos.",
        target: "tenga",
        distractor: "tiene",
        explanation: "Indefinite or hypothetical referent triggers subjunctive.",
    },
    ShowdownSentence {
        sentence: "Conozco a una ingeniera que ____ amplia experiencia en Rust.",
        target: "tiene",
        distractor: "tenga",
        explanation: "Specific, known entity in relative clause takes indicative.",
    },
    ShowdownSentence {
        sentence: "Tan pronto como ____ el reporte mensual, iniciaremos la reunión.",
        target: "llegue",
        distractor: "llega",
        explanation: "Future temporal clause (tan pronto como) requires subjunctive.",
    },
    ShowdownSentence {
        sentence: "Siempre que ____ a la oficina, saluda a todo el equipo.",
        target: "llega",
        distractor: "llegue",
        explanation: "Habitual recurring action takes indicative mood.",
    },
    ShowdownSentence {
        sentence: "Ojalá el parche de seguridad ____ la vulnerabilidad en producción.",
        target: "solucione",
        distractor: "soluciona",
        explanation: "'Ojalá' expresses longing and always governs subjunctive.",
    },
    ShowdownSentence {
        sentence: "Es verdad que el parche ____ la vulnerabilidad en producción.",
        target: "soluciona",
        distractor: "solucione",
        explanation: "Declaration of truth (es verdad que) takes indicative.",
    },
];

static SHOWDOWN_PRET_IMP: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Ayer el equipo ____ la nueva versión en el entorno de producción.",
        target: "desplegó",
        distractor: "desplegaba",
        explanation: "Punctual completed past event at a definite time takes preterite.",
    },
    ShowdownSentence {
        sentence: "Cuando trabajaba en esa startup, siempre ____ en bicicleta a la oficina.",
        target: "iba",
        distractor: "fue",
        explanation: "Customary past habit or continuous routine takes imperfect.",
    },
    ShowdownSentence {
        sentence: "Mientras nosotros ____ el código fuente, se produjo un corte eléctrico.",
        target: "revisábamos",
        distractor: "revisamos",
        explanation: "Ongoing background process interrupted by an event takes imperfect.",
    },
    ShowdownSentence {
        sentence: "De repente, la alarma del servidor ____ en mitad de la madrugada.",
        target: "sonó",
        distractor: "sonaba",
        explanation: "Sudden discrete event puncturing time takes preterite.",
    },
    ShowdownSentence {
        sentence: "La fundadora ____ veinticinco años cuando lanzó su primera empresa.",
        target: "tenía",
        distractor: "tuvo",
        explanation: "Age and descriptive background states in the past take imperfect.",
    },
    ShowdownSentence {
        sentence: "El lunes pasado el cliente ____ el contrato de servicios anual.",
        target: "firmó",
        distractor: "firmaba",
        explanation: "Completed transactional event with closed boundary takes preterite.",
    },
    ShowdownSentence {
        sentence: "En aquella época la infraestructura ____ sumamente rudimentaria.",
        target: "era",
        distractor: "fue",
        explanation: "Descriptive background condition over open duration takes imperfect.",
    },
    ShowdownSentence {
        sentence: "Tan pronto como recibí la alerta, ____ el nodo de emergencia.",
        target: "reinicié",
        distractor: "reiniciaba",
        explanation: "Immediate sequential action in a narrative takes preterite.",
    },
    ShowdownSentence {
        sentence: "Todos los viernes el departamento ____ pizzas para almorzar.",
        target: "comía",
        distractor: "comió",
        explanation: "Repeated customary routine in past background takes imperfect.",
    },
    ShowdownSentence {
        sentence: "A las nueve en punto la sesión plenaria ____ formalmente.",
        target: "comenzó",
        distractor: "comenzaba",
        explanation: "Point of inception at a precise moment takes preterite.",
    },
];

static SHOWDOWN_TU_USTED: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Disculpe, señor Pérez, ¿____ firmar la autorización de despliegue?",
        target: "puede",
        distractor: "puedes",
        explanation: "Formal address with title (señor Pérez) requires 'usted' agreement.",
    },
    ShowdownSentence {
        sentence: "Oye, Mateo, ¿____ revisar mi pull request cuando tengas un momento?",
        target: "puedes",
        distractor: "puede",
        explanation: "Informal peer address among teammates takes 'tú' agreement.",
    },
    ShowdownSentence {
        sentence: "Estimada doctora Gómez: le ruego que ____ asiento en la sala.",
        target: "tome",
        distractor: "toma",
        explanation: "Formal polite imperative with executive/client takes 'usted' (tome).",
    },
    ShowdownSentence {
        sentence: "Carlos, ____ los cambios locales antes de hacer el merge en git.",
        target: "guarda",
        distractor: "guarde",
        explanation: "Affirmative informal peer command uses 'tú' imperative (guarda).",
    },
    ShowdownSentence {
        sentence: "¿Cómo se encuentra ____ hoy, señor Director Ejecutivo?",
        target: "usted",
        distractor: "tú",
        explanation: "Formal honorific subject pronoun is 'usted'.",
    },
    ShowdownSentence {
        sentence: "Hola amigo, ¿cómo estás ____ el día de hoy?",
        target: "tú",
        distractor: "usted",
        explanation: "Informal friendly subject pronoun is 'tú'.",
    },
    ShowdownSentence {
        sentence: "Estimado usuario: ¿____ usted interesado en renovar su licencia?",
        target: "estaría",
        distractor: "estarías",
        explanation: "Formal courtesy address with 'usted' requires 3rd-person conditional.",
    },
    ShowdownSentence {
        sentence: "Hermano, ____ cuidado con los permisos de ese archivo binario.",
        target: "ten",
        distractor: "tenga",
        explanation: "Colloquial informal imperative uses 'tú' form (ten).",
    },
    ShowdownSentence {
        sentence: "Pablo, ____ el terminal y ejecuta el script de verificación.",
        target: "abre",
        distractor: "abra",
        explanation: "Informal direct peer command uses 'tú' imperative (abre).",
    },
    ShowdownSentence {
        sentence: "Por favor, licenciado, no se ____ por ese contratiempo menor.",
        target: "preocupe",
        distractor: "preocupes",
        explanation: "Negative formal imperative with reflexive 'se' takes 'usted' (preocupe).",
    },
];

static SHOWDOWN_LO_LE: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El informe técnico era confidencial, así que ____ guardé en la caja fuerte.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine inanimate noun (el informe) is 'lo'.",
    },
    ShowdownSentence {
        sentence: "A María ____ entregamos las credenciales de acceso al sistema.",
        target: "le",
        distractor: "la",
        explanation: "Indirect object pronoun for human recipient (a María) is 'le'.",
    },
    ShowdownSentence {
        sentence: "Ayer vi a Pedro en la conferencia y ____ saludé cordialmente.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine person (accusative) is 'lo'.",
    },
    ShowdownSentence {
        sentence: "Al arquitecto principal ____ propuse una solución con microservicios.",
        target: "le",
        distractor: "lo",
        explanation: "Indirect object pronoun indicating recipient of proposal is 'le'.",
    },
    ShowdownSentence {
        sentence: "El nuevo framework de Rust es fantástico; ____ descargué esta mañana.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine noun is 'lo'.",
    },
    ShowdownSentence {
        sentence: "A los nuevos desarrolladores ____ explicamos las directrices de estilo.",
        target: "les",
        distractor: "los",
        explanation: "Indirect object pronoun for plural recipients is 'les'.",
    },
    ShowdownSentence {
        sentence: "La vulnerabilidad era grave; el equipo de seguridad ____ solucionó hoy.",
        target: "la",
        distractor: "le",
        explanation: "Direct object pronoun for feminine noun (la vulnerabilidad) is 'la'.",
    },
    ShowdownSentence {
        sentence: "A la directora general ____ envié un resumen ejecutivo por correo.",
        target: "le",
        distractor: "la",
        explanation: "Indirect object pronoun indicating destination/recipient is 'le'.",
    },
    ShowdownSentence {
        sentence: "Descargué el archivo comprimido y ____ descomprimí en el servidor.",
        target: "lo",
        distractor: "le",
        explanation: "Direct object pronoun for masculine entity is 'lo'.",
    },
    ShowdownSentence {
        sentence: "Al usuario final ____ enviamos un token de doble factor por mensaje.",
        target: "le",
        distractor: "lo",
        explanation: "Indirect object pronoun marking recipient of transmission is 'le'.",
    },
];

static SHOWDOWN_SINO_PERO: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "El error no fue de hardware, ____ de configuración de red.",
        target: "sino",
        distractor: "pero",
        explanation: "'Sino' introduces exclusive rectification directly after negation.",
    },
    ShowdownSentence {
        sentence: "El lenguaje es potente y rápido, ____ requiere aprender el borrow checker.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' introduces an adversative qualification or limitation.",
    },
    ShowdownSentence {
        sentence: "No queríamos cancelar la migración, ____ posponerla hasta el fin de semana.",
        target: "sino",
        distractor: "pero",
        explanation: "'Sino' replaces the negated clause with the intended alternative.",
    },
    ShowdownSentence {
        sentence: "Escribimos las pruebas unitarias, ____ nos faltó tiempo para las de carga.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' connects two contrasting coordinate clauses.",
    },
    ShowdownSentence {
        sentence: "Esta herramienta no solo es veloz, ____ además sumamente ligera.",
        target: "sino",
        distractor: "pero",
        explanation: "'No solo... sino (además/también)' forms the correlative addition.",
    },
    ShowdownSentence {
        sentence: "Intentamos conectar a la base de datos, ____ el puerto estaba bloqueado.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' marks an adversative obstacle or complication.",
    },
    ShowdownSentence {
        sentence: "La empresa no busca inversores de riesgo, ____ clientes corporativos.",
        target: "sino",
        distractor: "pero",
        explanation: "'Sino' directly contrasts and substitutes the noun phrase.",
    },
    ShowdownSentence {
        sentence: "El algoritmo es muy elegante, ____ consume demasiada memoria.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' adds a drawback without negating the premise.",
    },
    ShowdownSentence {
        sentence: "No perdimos ningún dato, ____ recuperamos todas las transacciones.",
        target: "sino que",
        distractor: "pero",
        explanation:
            "'Sino que' introduces an affirmative rectified clause with a conjugated verb.",
    },
    ShowdownSentence {
        sentence: "Tenemos el visto bueno técnico, ____ aún falta la aprobación legal.",
        target: "pero",
        distractor: "sino",
        explanation: "'Pero' introduces a pending constraint.",
    },
];

static SHOWDOWN_PARA_QUE_PORQUE: &[ShowdownSentence] = &[
    ShowdownSentence {
        sentence: "Optimizamos la consulta SQL ____ la página cargue en menos de un segundo.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' expresses purpose and governs the subjunctive mood (cargue).",
    },
    ShowdownSentence {
        sentence: "El servidor se reinició automáticamente ____ se agotó la memoria swap.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' introduces a causal factual reason with indicative (agotó).",
    },
    ShowdownSentence {
        sentence: "Te comparto el repositorio ____ puedas revisar los cambios del pull request.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' introduces intentional goal with subjunctive (puedas).",
    },
    ShowdownSentence {
        sentence: "No asistimos al webinar ____ estábamos resolviendo una caída del servicio.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' explains the actual cause with indicative (estábamos).",
    },
    ShowdownSentence {
        sentence: "Añadimos logs detallados ____ el equipo de soporte identifique fallos rápido.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' governs subjunctive (identifique) to denote purpose.",
    },
    ShowdownSentence {
        sentence: "Elegimos Rust ____ garantiza seguridad de memoria sin recolector de basura.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' introduces an objective rationale with indicative (garantiza).",
    },
    ShowdownSentence {
        sentence:
            "Documentamos la API exhaustivamente ____ los nuevos integrantes se adapten pronto.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' sets the target objective with subjunctive (adapten).",
    },
    ShowdownSentence {
        sentence:
            "El cliente aceptó la propuesta ____ cumplía con todos los requisitos de seguridad.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' states the factual basis with indicative (cumplía).",
    },
    ShowdownSentence {
        sentence: "Habilitamos compresión gzip ____ el ancho de banda no sea un cuello de botella.",
        target: "para que",
        distractor: "porque",
        explanation: "'Para que' governs subjunctive (sea) for purpose.",
    },
    ShowdownSentence {
        sentence:
            "Cancelamos el despliegue programado ____ detectamos una regresión en las pruebas.",
        target: "porque",
        distractor: "para que",
        explanation: "'Porque' introduces the triggering cause with indicative (detectamos).",
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

/// Generates 4-choice rapid cloze arcade items for any grammar topic slug (1 correct + 3 distractors).
pub fn generate_4choice_items(topic: &str, count: usize) -> Vec<ArcadeItem> {
    if count == 0 {
        return Vec::new();
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
        });
    }

    items
}
