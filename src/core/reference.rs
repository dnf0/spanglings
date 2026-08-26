pub const TOPICS: &[&str] = &[
    "subjunctive",
    "por-para",
    "ser-estar",
    "past",
    "pronouns",
    "prepositions",
    "accidental-se",
    "tech-software",
    "business",
    "false-friends",
    "voseo",
    "accents",
];

pub fn list_reference_topics() -> &'static [&'static str] {
    TOPICS
}

pub fn get_reference_card(topic: &str) -> Option<&'static str> {
    match topic.to_lowercase().replace('_', "-").as_str() {
        "subjunctive" | "subj" | "weirdo" => Some(SUBJUNCTIVE_CARD),
        "por-para" | "por_para" | "por" | "para" => Some(POR_PARA_CARD),
        "ser-estar" | "ser_estar" | "ser" | "estar" => Some(SER_ESTAR_CARD),
        "past" | "preterite" | "imperfect" | "past-aspect" => Some(PAST_TENSES_CARD),
        "pronouns" | "clitics" | "stacking" | "direct-indirect" => Some(PRONOUN_STACKING_CARD),
        "prepositions" | "regimen" | "prep" => Some(PREPOSITIONS_CARD),
        "accidental-se" | "se-accidental" | "accidental" => Some(ACCIDENTAL_SE_CARD),
        "tech-software" | "tech" | "software" | "dev" => Some(TECH_SOFTWARE_CARD),
        "business" | "business-correspondence" | "biz" | "diplomatic" => {
            Some(BUSINESS_CORRESPONDENCE_CARD)
        }
        "false-friends" | "falsos-amigos" | "cognates" | "traps" => Some(FALSE_FRIENDS_CARD),
        "voseo" | "regional" | "rioplatense" => Some(VOSEO_CARD),
        "accents" | "accentuation" | "tildes" | "acentuacion" | "stress" => Some(ACCENTS_CARD),
        _ => None,
    }
}

pub const SUBJUNCTIVE_CARD: &str = r#"
================================================================================
                     SPANISH SUBJUNCTIVE CHEAT SHEET (WEIRDO)
================================================================================
Triggers require: [Main Clause Verb] + "QUE" + [Subject Change]

  W - Wishes / Desires:       querer, desear, preferir, esperar, exigir
  E - Emotions:               alegrarse de, sentir, temer, molestar, encantar
  I - Impersonal Expressions: es necesario que, es importante que, es bueno que
  R - Recommendations:        recomendar, aconsejar, sugerir, pedir
  D - Doubt / Denial:         dudar, no creer, no pensar, negar, no estar seguro
  O - Ojalá:                  ojalá (que)... (Always triggers subjunctive)

Present Subjunctive Endings & Formation:
  1. Take the "YO" form of the present indicative (e.g. tengo, hablo, como)
  2. Drop the "-o"
  3. Add OPPOSITE endings:
     -AR verbs:  -e,  -es,  -e,  -emos,  -éis,  -en
     -ER/-IR:    -a,  -as,  -a,  -amos,  -áis,  -an

DOUBT VS CERTAINTY:
  Indicative:  Creo que viene. / Es cierto que viene. / No dudo que viene.
  Subjunctive: No creo que venga. / No es cierto que venga. / Dudo que venga.
================================================================================
"#;

pub const POR_PARA_CARD: &str = r#"
================================================================================
                          POR VS PARA REFERENCE CARD
================================================================================
POR (Cause, Motive, Passage, Exchange):
  - Cause / Reason:           Lo hizo por amor. (Out of love)
  - Means / Communication:    Hablamos por teléfono. / Enviado por correo.
  - Duration / Approximate:   Estudié por tres horas. / Por la mañana.
  - Movement through/along:   Caminamos por el parque.
  - Exchange / Price:         Te doy 10 euros por el libro.
  - In search of:             Fue a la tienda por leche.

PARA (Goal, Recipient, Deadline, Destination):
  - Purpose / "In order to":  Estudio para aprender. (para + infinitive)
  - Recipient:                Este regalo es para ti.
  - Deadline / Specific time: La tarea es para el lunes.
  - Destination:              Salgo para Madrid mañana.
  - Opinion:                  Para mí, es la mejor opción.
  - Standard of comparison:   Para un niño, habla muy bien.
================================================================================
"#;

pub const SER_ESTAR_CARD: &str = r#"
================================================================================
                         SER VS ESTAR REFERENCE CARD
================================================================================
SER (Identity, Essence, Characteristics, Origin, Time):
  - D - Description:          Soy alto y moreno.
  - O - Occupation:           Es ingeniera de software.
  - C - Characteristic:       El hielo es frío.
  - T - Time / Date:          Son las tres y media. / Hoy es martes.
  - O - Origin / Material:    Soy de España. / La mesa es de madera.
  - R - Relationship:         Es mi hermano.

ESTAR (State, Condition, Location, Progressive):
  - P - Position:             Está sentado.
  - L - Location:             Madrid está en España.
  - A - Action (Progressive): Estoy aprendiendo español.
  - C - Condition:            El café está frío. / Estoy cansado.
  - E - Emotion:              Está muy feliz hoy.

MEANING SHIFTS WITH ADJECTIVES:
  - ser listo (clever)        vs estar listo (ready)
  - ser rico (wealthy)        vs estar rico (delicious)
  - ser aburrido (boring)     vs estar aburrido (bored)
  - ser atento (courteous)    vs estar atento (paying attention)
================================================================================
"#;

pub const PAST_TENSES_CARD: &str = r#"
================================================================================
                     PRETERITE VS IMPERFECT ASPECT
================================================================================
PRETERITE (Completed Action, Fixed Timeframe, Succession):
  - Definite beginning/end:   Llegó a las 8:00.
  - Sequence of events:       Se levantó, desayunó y salió.
  - Specific duration:        Viví en Sevilla durante dos años.

IMPERFECT (Ongoing, Habitual, Background, Age, Time):
  - Habitual / Repeated:      Siempre íbamos a la playa en verano.
  - Background setting:       Hacía frío y llovía intensamente.
  - Age / Mental states:      Tenía diez años cuando ocurrió. / Quería viajar.

VERB MEANING SHIFTS IN THE PAST:
  - saber:    supe (found out)           vs sabía (knew / was aware)
  - conocer:  conocí (met for 1st time)  vs conocía (already knew someone)
  - querer:   quise (tried to)           vs quería (wanted to)
  - no querer:no quise (refused to)      vs no quería (didn't want to)
  - poder:    pude (managed to / did)    vs podía (had the ability to)
================================================================================
"#;

pub const PRONOUN_STACKING_CARD: &str = r#"
================================================================================
                    DOUBLE OBJECT PRONOUNS & ACCENTS
================================================================================
ORDER RULE: [REFLEXIVE] -> [INDIRECT] -> [DIRECT]
  Indirect: me, te, le, nos, os, les
  Direct:   me, te, lo/la, nos, os, los/las

THE "LE / LES -> SE" RULE (Avoiding Cacophony):
  When Indirect (le/les) is followed by Direct (lo/la/los/las), 'le' becomes 'SE'
  to avoid the awkward consecutive 'l' sounds ("le lo"):
  * Le lo doy -> SE LO DOY.

PLACEMENT RULES:
  1. BEFORE conjugated verb:  "Se lo dije ayer."
  2. ATTACHED to infinitive:  "Voy a decírselo." (Needs written accent!)
  3. ATTACHED to gerund:      "Estoy explicándotelo." (Needs written accent!)
  4. ATTACHED to command:     "¡Dímelo ahora!" (Needs written accent!)
================================================================================
"#;

pub const PREPOSITIONS_CARD: &str = r#"
================================================================================
                     VERBAL REGIMEN (VERBS + PREPOSITIONS)
================================================================================
Common verb + preposition combinations (régimen preposicional):
  - a:       acostumbrarse a, atreverse a, negarse a, aspirar a, comprometerse a
  - de:      acordarse de, alegrarse de, enterarse de, quejarse de, depender de
  - en:      pensar en, confiar en, insistir en, tardar en, fijarse en
  - con:     contar con, soñar con, conformarse con, tropezar con, casarse con
  - por:     preocuparse por, interesarse por, luchar por, votar por
================================================================================
"#;

pub const ACCIDENTAL_SE_CARD: &str = r#"
================================================================================
                       ACCIDENTAL / UNINTENDED "SE"
================================================================================
STRUCTURE:
  "Se" + [Indirect Object Pronoun] + [Verb] + [Subject (the dropped item)]

PRONOUNS: me, te, le, nos, os, les (indicates who was affected/blamed)

EXAMPLES:
  - Se me cayeron las llaves.     (I dropped the keys / The keys fell from me.)
  - Se le olvidó la cartera.      (He/she forgot the wallet.)
  - Se nos rompió el vaso.        (We broke the glass / The glass broke on us.)
  - Se te quemó la comida.        (You burned the food.)

COMMON VERBS: caer, olvidar, romper, perder, acabar, quemar, descomponer
================================================================================
"#;

pub const TECH_SOFTWARE_CARD: &str = r#"
================================================================================
                 SOFTWARE ENGINEERING & TECH SPANISH
================================================================================
DEVELOPMENT & GIT:
  - to deploy:                desplegar (a producción) [avoid *deployar]
  - pull request:             solicitud de extracción
  - branch / merge:           rama / fusionar (o integrar)
  - commit / checkout:        confirmar cambios / cambiar de rama
  - repository:               repositorio

DEBUGGING & ARCHITECTURE:
  - to debug:                 depurar el código [avoid *debuggear]
  - to remediate/fix:         subsanar / corregir la vulnerabilidad [avoid *fixear]
  - deadlock:                 bloqueo mutuo / interbloqueo
  - race condition:           condición de carrera
  - throughput / performance: rendimiento / desempeño
  - latency / routing:        latencia de red / enrutamiento de peticiones
  - batch processing:         procesamiento por lotes
================================================================================
"#;

pub const BUSINESS_CORRESPONDENCE_CARD: &str = r#"
================================================================================
             FORMAL BUSINESS & DIPLOMATIC CORRESPONDENCE
================================================================================
FORMAL OPENINGS & CLOSINGS:
  - "I remain at your full disposal":    Quedo a su entera disposición
  - "Thanking you in advance":           Agradeciendo de antemano su atención
  - "Without further ado for now":       Sin otro particular por el momento
  - "I hereby attach":                   Procedo a adjuntar el documento

NEGOTIATIONS & CONTRACTS:
  - As regards / concerning:             En lo que atañe a / En lo que concierne a
  - Settle an outstanding debt:          Saldar la deuda / Liquidar el saldo
  - Formally dismiss a motion/proposal:  Desestimar la propuesta / el recurso
  - Stipulated in clause:                Estipulado / Dispuesto en la cláusula
================================================================================
"#;

pub const FALSE_FRIENDS_CARD: &str = r#"
================================================================================
                 HIGH-FREQUENCY FALSE FRIENDS (FALSOS AMIGOS)
================================================================================
1. actualmente != actually
   - actualmente = currently / at present
   - actually = en realidad / de hecho / en verdad

2. eventualmente != eventually
   - eventualmente = occasionally / by chance / sporadically
   - eventually = finalmente / con el tiempo / al final

3. pretender != to pretend
   - pretender = to attempt / aim to / claim
   - to pretend = fingir / simular / aparentar

4. realizar != to realize
   - realizar = to carry out / execute / make
   - to realize = darse cuenta de

5. soportar != to support
   - soportar = to tolerate / endure / bear weight
   - to support = apoyar / respaldar / sostener económicamente

6. sensible != sensible
   - sensible = sensitive / emotional
   - sensible = sensato / prudente / juicioso
================================================================================
"#;

pub const VOSEO_CARD: &str = r#"
================================================================================
                    RIOPLATENSE & PAN-AMERICAN VOSEO
================================================================================
PRESENT TENSE CONJUGATION (Stressed final vowel, no diphthong):
  - -AR (hablar):     vos hablás   (tú hablas)
  - -ER (comer):      vos comés    (tú comes)
  - -IR (vivir):      vos vivís    (tú vives)
  - IRREGULARS:       vos sos (ser), vos tenés (tener), vos querés (querer)

AFFIRMATIVE IMPERATIVES (Drop '-r', stress final vowel):
  - hablar -> ¡Hablá!      (tú: ¡Habla!)
  - comer  -> ¡Comé!       (tú: ¡Come!)
  - decir  -> ¡Decí!       (tú: ¡Di!)  -> ¡Decime! (¡Dime!)
  - sentar -> ¡Sentate!    (tú: ¡Siéntate!)
================================================================================
"#;

pub const ACCENTS_CARD: &str = r#"
================================================================================
              SPANISH ACCENTUATION & ORTHOGRAPHIC STRESS (TILDES)
================================================================================
GENERAL RULES BY STRESS POSITION:

1. AGUDAS (Stressed on the LAST syllable):
   - Accent mark ONLY if the word ends in: N, S, or a VOWEL (A, E, I, O, U)
   - Examples WITH tilde:    can-CIÓN, ca-FÉ, a-diÓS, co-mer-É
   - Examples WITHOUT tilde: can-tar, pa-pel, re-loj, pa-red, Ma-drid

2. LLANAS / GRAVES (Stressed on the PENULTIMATE syllable):
   - Accent mark ONLY if the word DOES NOT end in: N, S, or a VOWEL
   - Examples WITH tilde:    ÁR-bol, FÁ-cil, LÁ-piz, CÁR-cel, LÍ-der
   - Examples WITHOUT tilde: ca-sa, me-sa, car-tas, can-tan, li-bro

3. ESDRÚJULAS (Stressed on the ANTEPENULTIMATE syllable):
   - ALWAYS take an accent mark!
   - Examples:               MÚ-si-ca, RÁ-pi-do, GRA-má-ti-ca, PÁ-gi-na

4. SOBREESDRÚJULAS (Stressed BEFORE the antepenultimate syllable):
   - ALWAYS take an accent mark!
   - Examples:               CÓM-pra-me-lo, EX-PLÍ-ca-se-lo, DI-CIÉN-do-te-lo

DIPHTHONGS & HIATUSES (Diptongos vs Hiatos):
  - Strong vowels (abiertas): A, E, O
  - Weak vowels (cerradas):   I, U
  - Diptongo (Strong + Weak or Weak + Weak): 1 syllable (bai-le, puer-ta)
  - Hiato (Broken Diphthong): When stress falls on the WEAK vowel, it ALWAYS takes a tilde!
    Examples: pa-ÍS, dÍ-a, ba-ÚL, con-ti-nÚ-o, ca-fe-te-RÍ-a

DIACRITICAL ACCENT (Acento Diacrítico - Distinguishing Homonyms):
  - tú (you - pronoun)        vs tu (your - possessive)
  - él (he - pronoun)         vs el (the - masculine article)
  - mí (me - prep object)     vs mi (my - possessive)
  - sí (yes / oneself)        vs si (if / musical note)
  - té (tea - noun)           vs te (you - object pronoun)
  - dé (give - verb dar)      vs de (of/from - preposition)
  - sé (I know / be!)         vs se (reflexive pronoun)
  - más (more - adverb)       vs mas (but - literary conjunction)
  - aún (still / yet)         vs aun (even / including)

INTERROGATIVE & EXCLAMATORY PRONOUNS:
  - ¿Qué?, ¿Quién?, ¿Cómo?, ¿Dónde?, ¿Cuándo?, ¿Por qué?, ¿Cuánto?
  (Always carry a tilde in questions and exclamations, direct or indirect!)
================================================================================
"#;
