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
    "epistemic-conjecture",
    "clitic-doubling",
    "personal-a",
    "gerund-rules",
    "adversatives",
    "legal-subjunctive",
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
        "epistemic-conjecture" | "conjecture" | "probability" | "probabilidad" => {
            Some(EPISTEMIC_CONJECTURE_CARD)
        }
        "clitic-doubling" | "duplicacion" | "left-dislocation" | "reduplicacion" => {
            Some(CLITIC_DOUBLING_CARD)
        }
        "personal-a" | "a-personal" | "dom" | "animacy" => Some(PERSONAL_A_CARD),
        "gerund-rules" | "gerundio" | "gerunds" | "posteriority" => Some(GERUND_RULES_CARD),
        "adversatives" | "pero-sino" | "sino-que" | "sino" => Some(ADVERSATIVES_CARD),
        "legal-subjunctive" | "optatives" | "futuro-subjuntivo" | "archaic-subjunctive" => {
            Some(LEGAL_SUBJUNCTIVE_CARD)
        }
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

pub const EPISTEMIC_CONJECTURE_CARD: &str = r#"
================================================================================
          EPISTEMIC CONJECTURE & PROBABILITY (FUTURO Y CONDICIONAL)
================================================================================
In Spanish, the future and conditional tenses frequently express conjecture,
hypothesis, or probability in place of adverbs like 'probablemente'.

1. PRESENT CONJECTURE (Futuro Simple):
   Expresses probability or speculation about the PRESENT moment.
   - ¿Qué hora es? -> Serán las cuatro. (It must be / is probably around 4:00)
   - ¿Dónde está Juan? -> Estará en la oficina. (He's probably in the office)
   - ¿Quién llama a la puerta? -> Será el mensajero. (It must be the courier)

2. PAST CONJECTURE / HYPOTHESIS (Condicional Simple):
   Expresses speculation about an action occurring in the PAST.
   - Ayer no asistió. -> Estaría enfermo. (He was probably sick)
   - Tenía un reloj caro. -> Le costaría una fortuna. (It probably cost him a fortune)
   - Tendría unos 20 años cuando migró. (He must have been about 20 years old)

3. PRIOR PAST CONJECTURE (Futuro Compuesto / Condicional Compuesto):
   - Futuro Compuesto: Habrá terminado ya. (He has probably finished by now)
   - Condicional Compuesto: Habría salido antes de las ocho. (He must have left earlier)
================================================================================
"#;

pub const CLITIC_DOUBLING_CARD: &str = r#"
================================================================================
            CLITIC DOUBLING & LEFT-DISLOCATION (DUPLICACIÓN CLÍTICA)
================================================================================
Spanish has rigorous syntactic rules requiring redundant object pronouns:

1. MANDATORY LEFT-DISLOCATION (Fronted Direct & Indirect Objects):
   When an object is placed BEFORE the verb for topicalization, the clitic is OBLIGATORY:
   - A María LE entregué el informe. (NOT: *A María entregué el informe)
   - A los clientes LOS llamé temprano. (NOT: *A los clientes llamé temprano)
   - Este libro LO leí el año pasado. (NOT: *Este libro leí el año pasado)

2. MANDATORY DATIVE REDUPLICATION WITH TONIC PRONOUNS:
   Tonic prepositional pronouns (a mí, a ti, a él) REQUIRE the clitic pronoun:
   - A mí ME gusta la arquitectura distribuida. (NOT: *A mí gusta...)
   - TE vi A TI en la conferencia. (NOT: *Vi a ti...)

3. DATIVE OF INHERENT INTEREST & PSYCH-VERBS (Gustar, Parecer, Costar):
   - A los ingenieros LES parece razonable el plazo.
   - A mi colega LE cuesta adaptarse al nuevo framework.
================================================================================
"#;

pub const PERSONAL_A_CARD: &str = r#"
================================================================================
             THE 'PERSONAL A' SYSTEM & ANIMACY (A PERSONAL)
================================================================================
The preposition 'A' precedes direct objects according to specificity and animacy:

1. MANDATORY WITH SPECIFIC, KNOWN HUMAN BEINGS:
   - Vi A María en el standup. / Contrataron AL nuevo arquitecto.
   - Conozco A todos los miembros del equipo de backend.

2. OMITTED WITH NON-SPECIFIC, INDEFINITE HUMANS:
   - Busco programador con experiencia en Rust. (Any programmer)
   - Necesitamos secretaria para la oficina central. (Generic role)
   - BUT: Busco AL programador que diseñó el pipeline. (Specific person)

3. PERSONIFIED OBJECTS & DOMESTIC ANIMALS:
   - Paseo AL perro todas las mañanas. (Treated as an animate family companion)
   - Defendemos A la patria. (Personified collective noun)

4. VERBS WITH SEMANTIC SHIFTS:
   - TENER: Tengo dos servidores. vs Tengo A mi hijo enfermo en casa.
   - PERDER: Perdió el tren. vs Perdió A su socio en la disputa legal.
================================================================================
"#;

pub const GERUND_RULES_CARD: &str = r#"
================================================================================
        GERUND RESTRICTIONS & ANGLICISM TRAPS (GERUNDIO CORRECTO)
================================================================================
Spanish gerunds (-ando, -iendo) MUST express actions that are SIMULTANEOUS or
IMMEDIATELY PRECEDING the main verb.

1. PROHIBITED: GERUND OF POSTERIORITY (Gerundio de Posterioridad):
   A gerund cannot express a subsequent outcome or consequence of the main verb:
   - INCORRECT: *El servidor se cayó, provocando una interrupción del servicio.
   - CORRECT:   El servidor se cayó Y PROVOCÓ una interrupción del servicio.
   - INCORRECT: *Llegó a Madrid, reuniéndose al día siguiente con el cliente.
   - CORRECT:   Llegó a Madrid Y SE REUNIÓ al día siguiente con el cliente.

2. PROHIBITED: ADJECTIVAL GERUNDS (Gerundio Especificativo):
   A gerund cannot act as an adjective modifying a non-fluid noun:
   - INCORRECT: *Una directiva regulando la privacidad de los datos.
   - CORRECT:   Una directiva QUE REGULA la privacidad de los datos.
   - (ONLY ALLOWED with: agua hirviendo, clavo ardiendo).

3. CORRECT SPANISH GERUND USAGE:
   - Continuous aspect: Estamos refactorizando el módulo de pagos.
   - Simultaneous manner: Entró en la sala gritando consignas.
================================================================================
"#;

pub const ADVERSATIVES_CARD: &str = r#"
================================================================================
         ADVERSATIVE COORDINATION: PERO VS SINO VS SINO QUE
================================================================================
1. PERO (Additive Contrast / Restriction):
   Adds a qualification or limitation to the first clause without negating it:
   - El algoritmo es complejo, PERO es extremadamente rápido.
   - No tenemos mucho presupuesto, PERO alcanzaremos el objetivo.

2. SINO (Exclusive Substitution with Words & Phrases):
   Used after a NEGATIVE clause to substitute with an alternative element:
   - No usamos una arquitectura monolítica, SINO microservicios.
   - No vino el martes, SINO el miércoles.
   - (Pattern: NO [X], SINO [Y])

3. SINO QUE (Exclusive Substitution with CONJUGATED CLAUSES):
   Used after a negative clause when substituting with a full finite verb phrase:
   - No solo corregimos el bug, SINO QUE rediseñamos todo el flujo.
   - No rechazó la propuesta, SINO QUE sugirió ajustes menores.
   - (Pattern: NO [Cláusula], SINO QUE [Cláusula con verbo conjugado])
================================================================================
"#;

pub const LEGAL_SUBJUNCTIVE_CARD: &str = r#"
================================================================================
       OPTATIVES, INDEPENDENT SUBJUNCTIVE & LEGAL TENSES
================================================================================
1. INDEPENDENT OPTATIVE FORMULAS (Wishes & Imprecations):
   - ¡Quién + Imperfecto de Subjuntivo! (Counterfactual longing):
     ¡Quién tuviera veinte años otra vez! (If only I were twenty again!)
     ¡Quién pudiera resolver este bug tan fácilmente!
   - ¡Que + Presente de Subjuntivo! (Independent benevolent wish):
     ¡Que tengas un excelente día! / ¡Que aproveche!
   - Fixed expressions: ¡Maldita sea! / ¡Viva la ciencia! / ¡Cueste lo que cueste!

2. FUTURE SUBJUNCTIVE (-are, -iere):
   Archaic in modern speech, but MANDATORY in legal statutes, contracts, and proverbs:
   - Si alguna de las partes INCUMPLIERE lo pactado en este contrato...
   - Quien COMETIERE delito de estafa será sancionado...
   - Donde FUERES, haz lo que vieres. (Proverb)

3. LITERARY PLUPERFECT IN '-RA':
   In high journalistic & literary prose, '-ra' often replaces 'había + participio':
   - El proyecto que INICIARA la empresa hace dos años culminó con éxito.
     (= que había iniciado)
================================================================================
"#;
