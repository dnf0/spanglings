use crate::core::curriculum::Level;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConceptId(pub String);

impl fmt::Display for ConceptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ConceptId {
    fn from(s: &str) -> Self {
        ConceptId(s.to_string())
    }
}

impl From<String> for ConceptId {
    fn from(s: String) -> Self {
        ConceptId(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConceptCategory {
    AspectAndTense,
    MoodSelection,
    PronounsAndVoice,
    PrepositionsAndRelators,
    SyntaxAndRhetoric,
    SociolinguisticRegisters,
    PracticalPragmatics,
}

impl fmt::Display for ConceptCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConceptCategory::AspectAndTense => "Aspect and Tense",
            ConceptCategory::MoodSelection => "Mood Selection",
            ConceptCategory::PronounsAndVoice => "Pronouns and Voice",
            ConceptCategory::PrepositionsAndRelators => "Prepositions and Relators",
            ConceptCategory::SyntaxAndRhetoric => "Syntax and Rhetoric",
            ConceptCategory::SociolinguisticRegisters => "Sociolinguistic Registers",
            ConceptCategory::PracticalPragmatics => "Practical Pragmatics",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: ConceptId,
    pub title: String,
    pub category: ConceptCategory,
    pub level: Level,
    pub description: String,
    pub reference_topic: Option<String>,
    pub prerequisite_concepts: Vec<ConceptId>,
    pub foundational_track: Option<String>,
}

impl ConceptNode {
    pub fn new(
        id: impl Into<ConceptId>,
        title: impl Into<String>,
        category: ConceptCategory,
        level: Level,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category,
            level,
            description: description.into(),
            reference_topic: None,
            prerequisite_concepts: Vec::new(),
            foundational_track: None,
        }
    }

    pub fn with_prerequisites(mut self, prereqs: Vec<impl Into<ConceptId>>) -> Self {
        self.prerequisite_concepts = prereqs.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_reference_topic(mut self, topic: impl Into<String>) -> Self {
        self.reference_topic = Some(topic.into());
        self
    }

    pub fn with_foundational_track(mut self, track: impl Into<String>) -> Self {
        self.foundational_track = Some(track.into());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinguisticGraph {
    pub nodes: HashMap<ConceptId, ConceptNode>,
}

impl LinguisticGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: ConceptNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_prerequisite(&mut self, concept: &ConceptId, prereq: ConceptId) {
        if let Some(node) = self.nodes.get_mut(concept) {
            if !node.prerequisite_concepts.contains(&prereq) {
                node.prerequisite_concepts.push(prereq);
            }
        }
    }

    pub fn validate_no_cycles(&self) -> Result<(), String> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum State {
            Unvisited,
            Visiting,
            Visited,
        }

        let mut states: HashMap<&ConceptId, State> =
            self.nodes.keys().map(|id| (id, State::Unvisited)).collect();

        fn dfs<'a>(
            node_id: &'a ConceptId,
            nodes: &'a HashMap<ConceptId, ConceptNode>,
            states: &mut HashMap<&'a ConceptId, State>,
            path: &mut Vec<&'a ConceptId>,
        ) -> Result<(), String> {
            states.insert(node_id, State::Visiting);
            path.push(node_id);

            if let Some(node) = nodes.get(node_id) {
                for prereq in &node.prerequisite_concepts {
                    match states.get(prereq) {
                        Some(State::Visiting) => {
                            let cycle_str = path
                                .iter()
                                .map(|id| id.0.as_str())
                                .collect::<Vec<_>>()
                                .join(" -> ");
                            return Err(format!("Cycle detected: {} -> {}", cycle_str, prereq.0));
                        }
                        Some(State::Unvisited) => {
                            dfs(prereq, nodes, states, path)?;
                        }
                        Some(State::Visited) | None => {}
                    }
                }
            }

            path.pop();
            states.insert(node_id, State::Visited);
            Ok(())
        }

        let mut path = Vec::new();
        for node_id in self.nodes.keys() {
            if states.get(node_id) == Some(&State::Unvisited) {
                dfs(node_id, &self.nodes, &mut states, &mut path)?;
            }
        }

        Ok(())
    }

    pub fn get_all_ancestor_prerequisites(&self, target: &ConceptId) -> Vec<ConceptId> {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();

        fn dfs(
            graph: &LinguisticGraph,
            current: &ConceptId,
            visited: &mut HashSet<ConceptId>,
            ancestors: &mut Vec<ConceptId>,
        ) {
            if let Some(node) = graph.nodes.get(current) {
                for prereq in &node.prerequisite_concepts {
                    if visited.insert(prereq.clone()) {
                        dfs(graph, prereq, visited, ancestors);
                        ancestors.push(prereq.clone());
                    }
                }
            }
        }

        if let Some(target_node) = self.nodes.get(target) {
            for prereq in &target_node.prerequisite_concepts {
                if visited.insert(prereq.clone()) {
                    dfs(self, prereq, &mut visited, &mut ancestors);
                    ancestors.push(prereq.clone());
                }
            }
        }

        ancestors
    }

    pub fn get_learning_frontier(&self, mastered: &HashSet<ConceptId>) -> Vec<&ConceptNode> {
        let mut frontier: Vec<&ConceptNode> = self
            .nodes
            .values()
            .filter(|node| {
                !mastered.contains(&node.id)
                    && node
                        .prerequisite_concepts
                        .iter()
                        .all(|p| mastered.contains(p))
            })
            .collect();

        // Deterministic ordering: Level, Category, Concept ID
        frontier.sort_by(|a, b| {
            a.level
                .cmp(&b.level)
                .then_with(|| a.category.cmp(&b.category))
                .then_with(|| a.id.0.cmp(&b.id.0))
        });

        frontier
    }

    pub fn find_weakest_prerequisite_root(
        &self,
        target: &ConceptId,
        mastery_scores: &HashMap<String, f32>,
    ) -> Option<ConceptId> {
        let ancestors = self.get_all_ancestor_prerequisites(target);
        ancestors
            .into_iter()
            .filter_map(|id| mastery_scores.get(&id.0).map(|&score| (id, score)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id)
    }
}

pub fn get_default_linguistic_graph() -> LinguisticGraph {
    let mut graph = LinguisticGraph::new();

    let concepts = vec![
        // ==========================================
        // Category: AspectAndTense (10 concepts)
        // ==========================================
        ConceptNode::new(
            "irregular_present_stems",
            "Irregular Present Indicative Stems",
            ConceptCategory::AspectAndTense,
            Level::Baseline,
            "Mastery of stem diphthongization (e->ie, o->ue) and irregular yo forms in present indicative.",
        )
        .with_reference_topic("past")
        .with_foundational_track("00_baseline"),

        ConceptNode::new(
            "irregular_preterite_stems",
            "Irregular Preterite Stems & Metaphony",
            ConceptCategory::AspectAndTense,
            Level::Baseline,
            "Irregular preterite root changes (uv, u, j, i roots) and 3rd person vowel raising.",
        )
        .with_prerequisites(vec!["irregular_present_stems"])
        .with_reference_topic("past")
        .with_foundational_track("00_baseline"),

        ConceptNode::new(
            "ser_vs_estar_essence_state",
            "Ser vs Estar Fundamental Predication",
            ConceptCategory::AspectAndTense,
            Level::Baseline,
            "Core semantic distinction between essential characteristic/identity (ser) and contingent state/location (estar).",
        )
        .with_reference_topic("ser-estar")
        .with_foundational_track("01_ser_vs_estar"),

        ConceptNode::new(
            "ser_vs_estar_adjectival_shifts",
            "Ser vs Estar Meaning Shifts with Adjectives",
            ConceptCategory::AspectAndTense,
            Level::B1,
            "Systematic semantic mutations of qualifying adjectives (listo, rico, atento, verde, orgulloso).",
        )
        .with_prerequisites(vec!["ser_vs_estar_essence_state"])
        .with_reference_topic("ser-estar")
        .with_foundational_track("01_ser_vs_estar"),

        ConceptNode::new(
            "preterite_aspect_completion",
            "Preterite Aspect for Bounded Completed Events",
            ConceptCategory::AspectAndTense,
            Level::B1,
            "Aspectual framing of past actions as closed, bounded, and sequentially punctual.",
        )
        .with_prerequisites(vec!["irregular_preterite_stems"])
        .with_reference_topic("past")
        .with_foundational_track("02_past_aspects"),

        ConceptNode::new(
            "imperfect_aspect_habitual_background",
            "Imperfect Aspect for Habitual & Background Action",
            ConceptCategory::AspectAndTense,
            Level::B1,
            "Unbounded past aspect denoting ongoing background circumstances, routines, and descriptive states.",
        )
        .with_prerequisites(vec!["irregular_present_stems"])
        .with_reference_topic("past")
        .with_foundational_track("02_past_aspects"),

        ConceptNode::new(
            "aspectual_meaning_shifts",
            "Aspectual Meaning Shifts (saber, conocer, querer, poder)",
            ConceptCategory::AspectAndTense,
            Level::B1,
            "Lexical aspectual shifts triggered by preterite vs imperfect (e.g. supe = found out vs sabía = knew).",
        )
        .with_prerequisites(vec!["preterite_aspect_completion", "imperfect_aspect_habitual_background"])
        .with_reference_topic("past")
        .with_foundational_track("02_past_aspects"),

        ConceptNode::new(
            "haber_impersonal_vs_auxiliary",
            "Haber Impersonal vs Auxiliary Forms",
            ConceptCategory::AspectAndTense,
            Level::Baseline,
            "Distinction between existential haber (hay, hubo, había) and compound tense auxiliary usage.",
        )
        .with_reference_topic("past")
        .with_foundational_track("00_baseline"),

        ConceptNode::new(
            "verbal_periphrases_aspect",
            "Verbal Periphrases of Phase, Duration, and Inception",
            ConceptCategory::AspectAndTense,
            Level::B2,
            "Nuanced aspectual markers: ir + gerundio, llevar + gerundio, dejar de, ponerse a, acabar de.",
        )
        .with_prerequisites(vec!["preterite_aspect_completion", "imperfect_aspect_habitual_background"])
        .with_reference_topic("past")
        .with_foundational_track("12_verbal_periphrases"),

        ConceptNode::new(
            "advanced_verbal_periphrases",
            "Advanced Modal & Aspectual Periphrases",
            ConceptCategory::AspectAndTense,
            Level::C1,
            "Sophisticated periphrases expressing inception, imminence, and epistemic necessity (dar en, venir a, tener por).",
        )
        .with_prerequisites(vec!["verbal_periphrases_aspect"])
        .with_reference_topic("past")
        .with_foundational_track("29_advanced_verbal_periphrases"),

        // ==========================================
        // Category: MoodSelection (12 concepts)
        // ==========================================
        ConceptNode::new(
            "irregular_subjunctive_stems",
            "Present Subjunctive Stem Formation",
            ConceptCategory::MoodSelection,
            Level::Baseline,
            "Morphology of the present subjunctive derived from the 1st person singular indicative base.",
        )
        .with_prerequisites(vec!["irregular_present_stems"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("00_baseline"),

        ConceptNode::new(
            "subjunctive_volition_influence",
            "Subjunctive of Volition, Influence, and Desire",
            ConceptCategory::MoodSelection,
            Level::B1,
            "Triggering subjunctive mood in subordinate clauses of command, request, permission, and desire (WEIRDO - W/R).",
        )
        .with_prerequisites(vec!["irregular_present_stems", "irregular_subjunctive_stems"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("03_subjunctive_weirdo"),

        ConceptNode::new(
            "subjunctive_emotion_evaluation",
            "Subjunctive with Emotion and Value Judgments",
            ConceptCategory::MoodSelection,
            Level::B1,
            "Subordinate mood selection following predicates of feeling, concern, and evaluative impersonal triggers.",
        )
        .with_prerequisites(vec!["subjunctive_volition_influence"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("03_subjunctive_weirdo"),

        ConceptNode::new(
            "subjunctive_doubt_denial",
            "Subjunctive of Doubt, Uncertainty, and Negated Belief",
            ConceptCategory::MoodSelection,
            Level::B1,
            "Epistemic modality switches between assertion (indicative) and non-assertion (subjunctive: no creer, dudar).",
        )
        .with_prerequisites(vec!["subjunctive_volition_influence"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("03_subjunctive_weirdo"),

        ConceptNode::new(
            "subjunctive_relative_clauses",
            "Subjunctive in Relative Clauses with Indefinite/Negative Antecedents",
            ConceptCategory::MoodSelection,
            Level::B1,
            "Relative clause mood alternation conditioned by specificity, existence, or non-existence of antecedent.",
        )
        .with_prerequisites(vec!["subjunctive_volition_influence"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("04_subjunctive_relative"),

        ConceptNode::new(
            "subjunctive_temporal_future",
            "Subjunctive in Temporal Clauses Referring to Future Events",
            ConceptCategory::MoodSelection,
            Level::B1,
            "Temporal conjunctions (en cuanto, tan pronto como, cuando, hasta que) requiring subjunctive for unaccomplished future actions.",
        )
        .with_prerequisites(vec!["subjunctive_volition_influence", "irregular_subjunctive_stems"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("05_subjunctive_conjunctions"),

        ConceptNode::new(
            "subjunctive_adverbial_clauses",
            "Subjunctive in Purpose, Concession, and Conditional Conjunctions",
            ConceptCategory::MoodSelection,
            Level::B2,
            "Mandatory and conditioned adverbial connectors (para que, sin que, a menos que, con tal de que, antes de que).",
        )
        .with_prerequisites(vec!["subjunctive_temporal_future"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("05_subjunctive_conjunctions"),

        ConceptNode::new(
            "imperfect_subjunctive_formation",
            "Imperfect Subjunctive Derivation & Usage",
            ConceptCategory::MoodSelection,
            Level::B2,
            "Derivation from preterite 3rd plural (-ra / -se) and sequence of tenses in secondary clauses.",
        )
        .with_prerequisites(vec!["irregular_preterite_stems", "irregular_subjunctive_stems"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("06_imperfect_subjunctive_conditionals"),

        ConceptNode::new(
            "hypothetical_conditionals_si_clauses",
            "Hypothetical & Counterfactual Conditional Clauses",
            ConceptCategory::MoodSelection,
            Level::B2,
            "Syntactic structure of potential and counterfactual conditions (si + imperfect/pluperfect subjunctive + conditional).",
        )
        .with_prerequisites(vec!["imperfect_subjunctive_formation"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("06_imperfect_subjunctive_conditionals"),

        ConceptNode::new(
            "pluperfect_subjunctive_counterfactuals",
            "Pluperfect Subjunctive in Past Counterfactuals",
            ConceptCategory::MoodSelection,
            Level::B2,
            "Hubiera/hubiese + participle in impossible past conditions and retroactive regrets.",
        )
        .with_prerequisites(vec!["imperfect_subjunctive_formation", "haber_impersonal_vs_auxiliary"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("11_pluperfect_subjunctive"),

        ConceptNode::new(
            "advanced_concessive_mood_alternation",
            "Advanced Concessive Clauses (por mucho que, aun a riesgo de)",
            ConceptCategory::MoodSelection,
            Level::C1,
            "Subtle mood oscillation in concessive structures reflecting speaker informational stance and epistemic commitments.",
        )
        .with_prerequisites(vec!["subjunctive_adverbial_clauses", "hypothetical_conditionals_si_clauses"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("13_advanced_concessives"),

        ConceptNode::new(
            "advanced_subjunctive_nuances",
            "Subjunctive in Dubitative Adverbs & Reduplications",
            ConceptCategory::MoodSelection,
            Level::C1,
            "Mood alternation with acaso, tal vez, quizá, and reduplicative concessives (pase lo que pase, sea como sea).",
        )
        .with_prerequisites(vec!["advanced_concessive_mood_alternation"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("28_advanced_subjunctive_clauses"),

        // ==========================================
        // Category: PronounsAndVoice (6 concepts)
        // ==========================================
        ConceptNode::new(
            "direct_indirect_pronouns",
            "Direct and Indirect Object Pronoun Differentiation",
            ConceptCategory::PronounsAndVoice,
            Level::Baseline,
            "Accusative vs dative clitic pronoun discrimination (lo/la vs le/les) and leísmo awareness.",
        )
        .with_reference_topic("pronouns")
        .with_foundational_track("00_baseline"),

        ConceptNode::new(
            "gustar_dative_structures",
            "Dative of Interest and Gustar-Type Verb Structures",
            ConceptCategory::PronounsAndVoice,
            Level::Baseline,
            "Syntactic inversion in psych-verbs (gustar, encantar, costar, faltar, sobrar).",
        )
        .with_prerequisites(vec!["direct_indirect_pronouns"])
        .with_reference_topic("pronouns")
        .with_foundational_track("00_baseline"),

        ConceptNode::new(
            "clitic_pronoun_stacking",
            "Clitic Pronoun Stacking & Spurious 'Se'",
            ConceptCategory::PronounsAndVoice,
            Level::B1,
            "Double pronoun combinations (se lo, se la) and positional rules with infinitives, gerunds, and imperatives.",
        )
        .with_prerequisites(vec!["direct_indirect_pronouns"])
        .with_reference_topic("pronouns")
        .with_foundational_track("08_pronoun_stacking"),

        ConceptNode::new(
            "accidental_se_structures",
            "Involuntary and Accidental 'Se' Constructions",
            ConceptCategory::PronounsAndVoice,
            Level::B2,
            "Pragmatic de-agentification for unplanned occurrences (se me cayó, se nos olvidó, se le descompuso).",
        )
        .with_prerequisites(vec!["gustar_dative_structures", "clitic_pronoun_stacking"])
        .with_reference_topic("accidental-se")
        .with_foundational_track("10_accidental_se"),

        ConceptNode::new(
            "passive_refleja_vs_impersonal",
            "Passive Refleja and Impersonal 'Se' Agreement",
            ConceptCategory::PronounsAndVoice,
            Level::B2,
            "Agreement rules for passive reflejo (se venden casas) vs impersonal human agent constructions (se busca a los candidatos).",
        )
        .with_prerequisites(vec!["clitic_pronoun_stacking"])
        .with_reference_topic("pronouns")
        .with_foundational_track("20_passive_refleja"),

        ConceptNode::new(
            "middle_voice_reflexive_shifts",
            "Middle Voice, Inchoative Shifts, and Telic Reflexives",
            ConceptCategory::PronounsAndVoice,
            Level::C1,
            "Aspectual and voice shifts via reflexive markers (dormir/dormirse, ir/irse, comer/comerse, quedar/quedarse).",
        )
        .with_prerequisites(vec!["passive_refleja_vs_impersonal", "accidental_se_structures"])
        .with_reference_topic("pronouns")
        .with_foundational_track("40_middle_voice_and_reflexive_shifts"),

        // ==========================================
        // Category: PrepositionsAndRelators (5 concepts)
        // ==========================================
        ConceptNode::new(
            "por_vs_para_foundations",
            "Por vs Para Core Contrasts (Cause vs Purpose)",
            ConceptCategory::PrepositionsAndRelators,
            Level::Baseline,
            "Foundational dichotomy between origin/cause/medium (por) and destination/purpose/goal (para).",
        )
        .with_reference_topic("por-para")
        .with_foundational_track("07_por_vs_para"),

        ConceptNode::new(
            "por_vs_para_nuances",
            "Por vs Para Nuances (Recipient, Deadline, Exchange, Rate)",
            ConceptCategory::PrepositionsAndRelators,
            Level::B1,
            "Nuanced distributions: temporal deadlines, recipients, exchange, standard of comparison, opinion.",
        )
        .with_prerequisites(vec!["por_vs_para_foundations"])
        .with_reference_topic("por-para")
        .with_foundational_track("07_por_vs_para"),

        ConceptNode::new(
            "prepositional_regimes",
            "Verbal Prepositional Regimes (Régimen Preposicional)",
            ConceptCategory::PrepositionsAndRelators,
            Level::B1,
            "Fixed verb + preposition combinations (soñar con, consistir en, tender a, empeñarse en, percatarse de).",
        )
        .with_prerequisites(vec!["por_vs_para_foundations"])
        .with_reference_topic("prepositions")
        .with_foundational_track("09_prepositional_regimes"),

        ConceptNode::new(
            "relative_pronouns_and_prepositions",
            "Complex Relative Pronouns with Prepositions",
            ConceptCategory::PrepositionsAndRelators,
            Level::B2,
            "Syntactic mastery of el que, la cual, cuyo, donde preceded by monosyllabic and complex prepositions.",
        )
        .with_prerequisites(vec!["prepositional_regimes"])
        .with_reference_topic("prepositions")
        .with_foundational_track("04_subjunctive_relative"),

        ConceptNode::new(
            "nuanced_prepositions_and_locutions",
            "Prepositional Locutions and Advanced Regimes",
            ConceptCategory::PrepositionsAndRelators,
            Level::C1,
            "High-register locutions (a tenor de, con miras a, so pena de, a expensas de, en aras de).",
        )
        .with_prerequisites(vec!["relative_pronouns_and_prepositions", "prepositional_regimes"])
        .with_reference_topic("prepositions")
        .with_foundational_track("39_nuanced_prepositions_and_locutions"),

        // ==========================================
        // Category: SyntaxAndRhetoric (6 concepts)
        // ==========================================
        ConceptNode::new(
            "discourse_connectors",
            "Discourse Markers and Argumentative Connectors",
            ConceptCategory::SyntaxAndRhetoric,
            Level::B2,
            "Formal argumentative articulation (sin embargo, no obstante, por ende, en cambio, puesto que).",
        )
        .with_prerequisites(vec!["por_vs_para_nuances"])
        .with_reference_topic("business")
        .with_foundational_track("14_connectors"),

        ConceptNode::new(
            "indirect_speech_transformations",
            "Reported Speech and Tense Concordance Shift Rules",
            ConceptCategory::SyntaxAndRhetoric,
            Level::B2,
            "Consecutio temporum in indirect statements, questions, and commands across past reporting frames.",
        )
        .with_prerequisites(vec!["imperfect_subjunctive_formation", "aspectual_meaning_shifts"])
        .with_reference_topic("business")
        .with_foundational_track("15_indirect_speech"),

        ConceptNode::new(
            "negated_perception_predicates",
            "Negated Perception and Mental Act Predicates",
            ConceptCategory::SyntaxAndRhetoric,
            Level::B2,
            "Syntactic polarity effects in perception and cognition verbs (no ver que, no notar que).",
        )
        .with_prerequisites(vec!["subjunctive_doubt_denial"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("17_negated_perception"),

        ConceptNode::new(
            "cleft_sentences_focus",
            "Cleft Sentences and Focalizing Periphrases",
            ConceptCategory::SyntaxAndRhetoric,
            Level::B2,
            "Information packaging via focal cleft structures (fue entonces cuando, es por eso que, lo que importa es).",
        )
        .with_prerequisites(vec!["ser_vs_estar_essence_state", "discourse_connectors"])
        .with_reference_topic("business")
        .with_foundational_track("18_cleft_sentences"),

        ConceptNode::new(
            "formal_inversion_hyperbaton",
            "Stylistic Inversion, Fronting, and Hyperbaton",
            ConceptCategory::SyntaxAndRhetoric,
            Level::C1,
            "Predicate fronting, locative inversion, and rhetorical word order in high-register Spanish.",
        )
        .with_prerequisites(vec!["cleft_sentences_focus"])
        .with_reference_topic("business")
        .with_foundational_track("19_formal_inversion"),

        ConceptNode::new(
            "adverbial_clauses_syntax",
            "Complex Adverbial Subordination & Modal Polarity",
            ConceptCategory::SyntaxAndRhetoric,
            Level::C1,
            "Syntactic cohesion and modal polarity in complex consecutive, causal, and modal subordinations.",
        )
        .with_prerequisites(vec!["subjunctive_adverbial_clauses", "discourse_connectors"])
        .with_reference_topic("subjunctive")
        .with_foundational_track("41_adverbial_clauses_and_conjunctions"),

        // ==========================================
        // Category: SociolinguisticRegisters (6 concepts)
        // ==========================================
        ConceptNode::new(
            "idiomatic_expressions_colloquial",
            "Sociolects and High-Frequency Idiomatic Expressions",
            ConceptCategory::SociolinguisticRegisters,
            Level::B2,
            "Idiomatic lexical phrases, metaphorical colloquialisms, and conversational formulas.",
        )
        .with_prerequisites(vec!["aspectual_meaning_shifts"])
        .with_reference_topic("business")
        .with_foundational_track("16_idioms"),

        ConceptNode::new(
            "false_friends_anglicisms",
            "False Cognates and Lexical Calques from English",
            ConceptCategory::SociolinguisticRegisters,
            Level::B2,
            "Disambiguation of treacherous cognates (actual vs current, pretender vs pretend, realizar vs realize).",
        )
        .with_reference_topic("false-friends")
        .with_foundational_track("24_false_friends"),

        ConceptNode::new(
            "register_elevation_formal",
            "Register Elevation, Nominalization, and Administrative Style",
            ConceptCategory::SociolinguisticRegisters,
            Level::C1,
            "Elevating conversational phrases to institutional, diplomatic, and executive prose via nominalization.",
        )
        .with_prerequisites(vec!["discourse_connectors", "formal_inversion_hyperbaton"])
        .with_reference_topic("business")
        .with_foundational_track("25_register_elevation"),

        ConceptNode::new(
            "regional_voseo_rioplatense",
            "Rioplatense Voseo Conjugation and Pronoun Systems",
            ConceptCategory::SociolinguisticRegisters,
            Level::B2,
            "Morphology, pronoun paradigms, and imperative forms of Southern Cone voseo.",
        )
        .with_prerequisites(vec!["irregular_present_stems"])
        .with_reference_topic("voseo")
        .with_foundational_track("33_rioplatense_production_voseo"),

        ConceptNode::new(
            "mexican_professional_pragmatics",
            "Mexican Corporate Lexicon, Softeners, and Idiomatic Pragmatics",
            ConceptCategory::SociolinguisticRegisters,
            Level::B2,
            "Mexican professional conventions: diminutive mitigation, workplace idioms, and courteous framing.",
        )
        .with_prerequisites(vec!["register_elevation_formal"])
        .with_reference_topic("business")
        .with_foundational_track("31_mexican_tech_and_startups"),

        ConceptNode::new(
            "colombian_professional_nuances",
            "Colombian Formal Registrations, Usteo, and Courteous Requests",
            ConceptCategory::SociolinguisticRegisters,
            Level::B2,
            "Colombian business etiquette, formal honorifics, respectful indirectness, and transactional norms.",
        )
        .with_prerequisites(vec!["register_elevation_formal"])
        .with_reference_topic("business")
        .with_foundational_track("32_colombian_professional_nuances"),

        // ==========================================
        // Category: PracticalPragmatics (8 concepts, Tracks 42-47)
        // ==========================================
        ConceptNode::new(
            "tech_software_engineering",
            "Software Architecture, PR Reviews, and Incident Triage",
            ConceptCategory::PracticalPragmatics,
            Level::B2,
            "Native technical dialogue for code reviews, distributed system design, post-mortems, and sprint planning.",
        )
        .with_prerequisites(vec!["false_friends_anglicisms", "discourse_connectors"])
        .with_reference_topic("tech-software")
        .with_foundational_track("22_tech_software"),

        ConceptNode::new(
            "executive_leadership_communication",
            "Strategic Alignment, Executive Negotiation, and Board Presentations",
            ConceptCategory::PracticalPragmatics,
            Level::C1,
            "High-stakes executive communication, strategic trade-offs, and shareholder diplomacy.",
        )
        .with_prerequisites(vec!["register_elevation_formal", "tech_software_engineering"])
        .with_reference_topic("business")
        .with_foundational_track("30_executive_leadership"),

        ConceptNode::new(
            "travel_logistics_disruptions",
            "Travel Itineraries, Transit Logistics, Delays, and Claims",
            ConceptCategory::PracticalPragmatics,
            Level::B2,
            "Managing flight cancellations, train re-routings, customs documentation, and compensation claims.",
        )
        .with_prerequisites(vec!["subjunctive_temporal_future", "por_vs_para_nuances"])
        .with_reference_topic("business")
        .with_foundational_track("42_travel_and_transportation"),

        ConceptNode::new(
            "customer_service_disputes",
            "Customer Support Escalations, Refunds, and Dispute Resolution",
            ConceptCategory::PracticalPragmatics,
            Level::B2,
            "Handling consumer escalations, warranty claims, defective merchandise, and mediation of disputes.",
        )
        .with_prerequisites(vec!["subjunctive_emotion_evaluation", "accidental_se_structures"])
        .with_reference_topic("business")
        .with_foundational_track("43_customer_support_and_negotiations"),

        ConceptNode::new(
            "lease_agreements_contracts",
            "Real Estate Leases, Tenancy Rights, and Contractual Clauses",
            ConceptCategory::PracticalPragmatics,
            Level::B2,
            "Lease contract analysis, security deposit terms, maintenance liabilities, and eviction safeguards.",
        )
        .with_prerequisites(vec!["por_vs_para_nuances", "passive_refleja_vs_impersonal"])
        .with_reference_topic("business")
        .with_foundational_track("44_real_estate_and_leases"),

        ConceptNode::new(
            "financial_transactions_taxation",
            "Banking Operations, International Wires, Invoicing, and Tax Filings",
            ConceptCategory::PracticalPragmatics,
            Level::C1,
            "Cross-border wire transfers, tax declarations, withholding mechanisms, and fintech ledger reconciliation.",
        )
        .with_prerequisites(vec!["lease_agreements_contracts", "register_elevation_formal"])
        .with_reference_topic("business")
        .with_foundational_track("45_banking_finance_and_fintech"),

        ConceptNode::new(
            "bureaucratic_procedures_documentation",
            "Immigration, Notarized Affidavits, and Government Registrations",
            ConceptCategory::PracticalPragmatics,
            Level::C1,
            "Visa applications, apostilles, sworn legal affidavits, municipal registrations (empadronamiento), and regulatory compliance.",
        )
        .with_prerequisites(vec!["register_elevation_formal", "passive_refleja_vs_impersonal"])
        .with_reference_topic("business")
        .with_foundational_track("46_legal_administrative_procedures"),

        ConceptNode::new(
            "academic_discourse_rhetoric",
            "Peer Review, Scholarly Methodologies, and Epistemic Hedging",
            ConceptCategory::PracticalPragmatics,
            Level::C1,
            "Epistemic mitigation, scholarly argumentation, thesis defense, bibliography referencing, and peer critique.",
        )
        .with_prerequisites(vec!["register_elevation_formal", "cleft_sentences_focus", "adverbial_clauses_syntax"])
        .with_reference_topic("business")
        .with_foundational_track("47_academic_research_and_publishing"),
    ];

    for concept in concepts {
        graph.add_node(concept);
    }

    graph
}
