use crate::core::curriculum::Level;
use crate::engine::accents::{check_accent_match, AccentMode, AccentResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementQuestion {
    pub id: String,
    pub level: Level,
    pub topic: String,
    pub context_en: String,
    pub prompt_es: String,
    pub solution: String,
    pub alternatives: Vec<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementItemResult {
    pub question_id: String,
    pub level: Level,
    pub user_answer: String,
    pub is_correct: bool,
    pub correct_solution: String,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementResult {
    pub assessed_level: Level,
    pub total_questions: usize,
    pub total_correct: usize,
    pub percentage: f64,
    pub scores_by_level: HashMap<Level, (usize, usize)>,
    pub passed_levels: Vec<Level>,
    pub detailed_results: Vec<PlacementItemResult>,
}

pub fn check_placement_answer(
    user_input: &str,
    solution: &str,
    alternatives: &[String],
    accent_mode: AccentMode,
) -> bool {
    let strict = accent_mode == AccentMode::Strict;
    match check_accent_match(user_input, solution, strict) {
        AccentResult::ExactMatch | AccentResult::ForgivenMatch { .. } => return true,
        AccentResult::Mismatch => {}
    }

    for alt in alternatives {
        match check_accent_match(user_input, alt, strict) {
            AccentResult::ExactMatch | AccentResult::ForgivenMatch { .. } => return true,
            AccentResult::Mismatch => {}
        }
    }

    false
}

pub fn get_placement_battery(level_filter: Option<Level>) -> Vec<PlacementQuestion> {
    let all_questions = vec![
        // Baseline / A2
        PlacementQuestion {
            id: "place_base_01".to_string(),
            level: Level::Baseline,
            topic: "ser_vs_estar_locations".to_string(),
            context_en: "Events and meetings take place (occur) using 'ser'.".to_string(),
            prompt_es: "La reunión general (es / tiene lugar) ___ en el auditorio principal.".to_string(),
            solution: "es".to_string(),
            alternatives: vec!["tiene lugar".to_string(), "se celebra".to_string()],
            explanation: "Events take 'ser' because they occur/take place, not physical object location ('estar').".to_string(),
        },
        PlacementQuestion {
            id: "place_base_02".to_string(),
            level: Level::Baseline,
            topic: "direct_object_pronoun".to_string(),
            context_en: "Did you see the report? Yes, I saw it this morning.".to_string(),
            prompt_es: "¿Viste el informe técnico? Sí, (lo) ___ vi esta mañana temprano.".to_string(),
            solution: "lo".to_string(),
            alternatives: vec![],
            explanation: "Masculine direct object pronoun referring to 'el informe' is 'lo'.".to_string(),
        },
        PlacementQuestion {
            id: "place_base_03".to_string(),
            level: Level::Baseline,
            topic: "regular_preterite".to_string(),
            context_en: "Yesterday we sent the complete documentation to the client.".to_string(),
            prompt_es: "Ayer nosotros (enviamos) ___ la documentación completa al cliente.".to_string(),
            solution: "enviamos".to_string(),
            alternatives: vec!["mandamos".to_string()],
            explanation: "Preterite of 'enviar' (1st person plural) is 'enviamos'.".to_string(),
        },
        // B1 Core
        PlacementQuestion {
            id: "place_b1_01".to_string(),
            level: Level::B1,
            topic: "past_aspects".to_string(),
            context_en: "When I arrived at the office, Juan had already reviewed the code.".to_string(),
            prompt_es: "Cuando llegué a la oficina, Juan ya (había revisado / habia revisado) ___ el código.".to_string(),
            solution: "había revisado".to_string(),
            alternatives: vec!["habia revisado".to_string()],
            explanation: "Pluperfect indicative 'había revisado' indicates an action completed before another past action.".to_string(),
        },
        PlacementQuestion {
            id: "place_b1_02".to_string(),
            level: Level::B1,
            topic: "subjunctive_weirdo".to_string(),
            context_en: "It is essential that the team updates the deployment documentation.".to_string(),
            prompt_es: "Es fundamental que el equipo (actualice) ___ la documentación de despliegue.".to_string(),
            solution: "actualice".to_string(),
            alternatives: vec![],
            explanation: "Impersonal trigger 'es fundamental que' requires present subjunctive 'actualice'.".to_string(),
        },
        PlacementQuestion {
            id: "place_b1_03".to_string(),
            level: Level::B1,
            topic: "por_vs_para".to_string(),
            context_en: "We sent the package by courier so that it arrives on time.".to_string(),
            prompt_es: "Enviamos el paquete (por) ___ mensajería urgente (para) ___ que llegue a tiempo.".to_string(),
            solution: "por, para".to_string(),
            alternatives: vec!["por para".to_string()],
            explanation: "'Por' indicates means/transport; 'para que' introduces purpose requiring subjunctive.".to_string(),
        },
        PlacementQuestion {
            id: "place_b1_04".to_string(),
            level: Level::B1,
            topic: "accidental_se".to_string(),
            context_en: "Maria dropped the keys by accident without meaning to.".to_string(),
            prompt_es: "A María (se le cayeron) ___ las llaves al salir del edificio.".to_string(),
            solution: "se le cayeron".to_string(),
            alternatives: vec![],
            explanation: "Accidental 'se' construction: 'se' + indirect object 'le' + verb agreeing with plural subject 'cayeron'.".to_string(),
        },
        // B2 Upper Intermediate
        PlacementQuestion {
            id: "place_b2_01".to_string(),
            level: Level::B2,
            topic: "imperfect_subjunctive_conditional".to_string(),
            context_en: "If the server had more memory, we would not experience crashes.".to_string(),
            prompt_es: "Si el servidor (tuviera / tuviese) ___ más memoria, no tendríamos caídas de servicio.".to_string(),
            solution: "tuviera".to_string(),
            alternatives: vec!["tuviese".to_string()],
            explanation: "Hypothetical conditional clause requires imperfect subjunctive 'tuviera' / 'tuviese'.".to_string(),
        },
        PlacementQuestion {
            id: "place_b2_02".to_string(),
            level: Level::B2,
            topic: "concessive_subjunctive".to_string(),
            context_en: "No matter how much the client requests changes, we must adhere to the scope.".to_string(),
            prompt_es: "Por más que el cliente (solicite / pida) ___ cambios, nos apegaremos al alcance firmado.".to_string(),
            solution: "solicite".to_string(),
            alternatives: vec!["pida".to_string(), "exija".to_string()],
            explanation: "'Por más que' + subjunctive 'solicite' introduces concessive obstacle.".to_string(),
        },
        PlacementQuestion {
            id: "place_b2_03".to_string(),
            level: Level::B2,
            topic: "proportional_adverbial".to_string(),
            context_en: "As user traffic increases in the coming months, we will optimize resources.".to_string(),
            prompt_es: "A medida que (aumente / crezca) ___ el tráfico en los próximos meses, optimizaremos recursos.".to_string(),
            solution: "aumente".to_string(),
            alternatives: vec!["crezca".to_string(), "se incremente".to_string()],
            explanation: "Future anticipated progression with 'a medida que' requires subjunctive 'aumente'.".to_string(),
        },
        PlacementQuestion {
            id: "place_b2_04".to_string(),
            level: Level::B2,
            topic: "nuanced_prepositions".to_string(),
            context_en: "We walked toward the data center until reaching the main security barrier.".to_string(),
            prompt_es: "Caminamos (hacia) ___ el centro de datos (hasta) ___ llegar a la barrera de seguridad.".to_string(),
            solution: "hacia, hasta".to_string(),
            alternatives: vec!["hacia hasta".to_string()],
            explanation: "'Hacia' expresses direction toward; 'hasta' expresses the terminus/endpoint.".to_string(),
        },
        // C1 Advanced
        PlacementQuestion {
            id: "place_c1_01".to_string(),
            level: Level::C1,
            topic: "reduplicative_subjunctive".to_string(),
            context_en: "Be that as it may, we will resolve the incident before morning.".to_string(),
            prompt_es: "(Sea) ___ como fuere, resolveremos la incidencia crítica antes de que amanezca.".to_string(),
            solution: "Sea".to_string(),
            alternatives: vec!["sea".to_string()],
            explanation: "Reduplicative subjunctive idiom: 'Sea como fuere / Sea como sea'.".to_string(),
        },
        PlacementQuestion {
            id: "place_c1_02".to_string(),
            level: Level::C1,
            topic: "formal_connectors".to_string(),
            context_en: "The database failed unexpectedly; hence we suspended the launch.".to_string(),
            prompt_es: "La base de datos falló de improvisto; (de ahí / de ahi) ___ que hayamos suspendido el lanzamiento.".to_string(),
            solution: "de ahí".to_string(),
            alternatives: vec!["de ahi".to_string()],
            explanation: "Discourse connector 'de ahí que' + subjunctive expresses consecutive consequence.".to_string(),
        },
        PlacementQuestion {
            id: "place_c1_03".to_string(),
            level: Level::C1,
            topic: "absolute_participle_inversion".to_string(),
            context_en: "Once the security audit was concluded, the board approved the budget.".to_string(),
            prompt_es: "(Concluida / Finalizada) ___ la auditoría de seguridad, la junta aprobó el presupuesto.".to_string(),
            solution: "Concluida".to_string(),
            alternatives: vec!["concluida".to_string(), "Finalizada".to_string(), "finalizada".to_string()],
            explanation: "Absolute participle construction with gender/number agreement: 'Concluida la auditoría'.".to_string(),
        },
        PlacementQuestion {
            id: "place_c1_04".to_string(),
            level: Level::C1,
            topic: "register_elevation".to_string(),
            context_en: "The recent policy change sparked serious doubts among executives.".to_string(),
            prompt_es: "El cambio reciente en la política (suscitó / suscito) ___ serias dudas entre los directivos.".to_string(),
            solution: "suscitó".to_string(),
            alternatives: vec!["suscito".to_string(), "generó".to_string(), "provocó".to_string()],
            explanation: "Elevated high-register verb for provoking/sparking intellectual doubts is 'suscitar' (suscitó).".to_string(),
        },
    ];

    if let Some(lvl) = level_filter {
        all_questions.into_iter().filter(|q| q.level == lvl).collect()
    } else {
        all_questions
    }
}

pub fn evaluate_placement_test(
    battery: &[PlacementQuestion],
    answers: &[String],
    accent_mode: AccentMode,
) -> PlacementResult {
    let mut detailed_results = Vec::new();
    let mut scores_by_level: HashMap<Level, (usize, usize)> = HashMap::new();
    let mut total_correct = 0;

    for (i, question) in battery.iter().enumerate() {
        let user_ans = answers.get(i).cloned().unwrap_or_default();
        let is_correct = check_placement_answer(
            &user_ans,
            &question.solution,
            &question.alternatives,
            accent_mode,
        );

        if is_correct {
            total_correct += 1;
        }

        let entry = scores_by_level.entry(question.level).or_insert((0, 0));
        entry.1 += 1;
        if is_correct {
            entry.0 += 1;
        }

        detailed_results.push(PlacementItemResult {
            question_id: question.id.clone(),
            level: question.level,
            user_answer: user_ans,
            is_correct,
            correct_solution: question.solution.clone(),
            explanation: question.explanation.clone(),
        });
    }

    let (assessed_level, percentage) = calculate_cefr_level(&scores_by_level);

    let mut passed_levels = Vec::new();
    for (lvl, (correct, total)) in &scores_by_level {
        if *total > 0 && (*correct as f64 / *total as f64) >= 0.75 {
            passed_levels.push(*lvl);
        }
    }
    passed_levels.sort_by_key(|l| match l {
        Level::Baseline => 0,
        Level::B1 => 1,
        Level::B2 => 2,
        Level::C1 => 3,
    });

    PlacementResult {
        assessed_level,
        total_questions: battery.len(),
        total_correct,
        percentage,
        scores_by_level,
        passed_levels,
        detailed_results,
    }
}

pub fn calculate_cefr_level(scores: &HashMap<Level, (usize, usize)>) -> (Level, f64) {
    let base_pct = get_level_pct(scores, Level::Baseline);
    let b1_pct = get_level_pct(scores, Level::B1);
    let b2_pct = get_level_pct(scores, Level::B2);
    let c1_pct = get_level_pct(scores, Level::C1);

    let mut total_correct = 0;
    let mut total_questions = 0;
    for (correct, total) in scores.values() {
        total_correct += correct;
        total_questions += total;
    }
    let overall_pct = if total_questions > 0 {
        (total_correct as f64 / total_questions as f64) * 100.0
    } else {
        0.0
    };

    let assessed_level = if c1_pct >= 0.75 && b2_pct >= 0.75 && b1_pct >= 0.75 {
        Level::C1
    } else if b2_pct >= 0.70 && b1_pct >= 0.70 {
        Level::B2
    } else if b1_pct >= 0.65 && base_pct >= 0.65 {
        Level::B1
    } else {
        Level::Baseline
    };

    (assessed_level, overall_pct)
}

fn get_level_pct(scores: &HashMap<Level, (usize, usize)>, level: Level) -> f64 {
    if let Some(&(correct, total)) = scores.get(&level) {
        if total > 0 {
            return correct as f64 / total as f64;
        }
    }
    0.0
}
