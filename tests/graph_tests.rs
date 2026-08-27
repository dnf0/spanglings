use spanglings::core::curriculum::Level;
use spanglings::core::graph::{
    get_default_linguistic_graph, ConceptCategory, ConceptId, ConceptNode, LinguisticGraph,
};
use std::collections::{HashMap, HashSet};

#[test]
fn test_default_graph_ontology_is_valid_dag() {
    let graph = get_default_linguistic_graph();
    assert!(
        graph.nodes.len() >= 40,
        "Expected at least 40 concepts in ontology, found {}",
        graph.nodes.len()
    );

    // Validate that graph has no cycles (DAG property)
    assert!(graph.validate_no_cycles().is_ok(), "Graph contains cycles!");
}

#[test]
fn test_graph_prerequisite_traversal() {
    let graph = get_default_linguistic_graph();
    let target = ConceptId("subjunctive_temporal_future".to_string());
    let prereqs = graph.get_all_ancestor_prerequisites(&target);
    assert!(!prereqs.is_empty());
    assert!(prereqs.iter().any(
        |id| id.0 == "subjunctive_volition_influence" || id.0 == "irregular_subjunctive_stems"
    ));
}

#[test]
fn test_learning_frontier_calculation() {
    let graph = get_default_linguistic_graph();
    let mut mastered = HashSet::new();
    mastered.insert(ConceptId("irregular_present_stems".to_string()));
    mastered.insert(ConceptId("irregular_preterite_stems".to_string()));
    mastered.insert(ConceptId("irregular_subjunctive_stems".to_string()));

    let frontier = graph.get_learning_frontier(&mastered);
    assert!(!frontier.is_empty());
    assert!(frontier
        .iter()
        .any(|node| node.id.0 == "subjunctive_volition_influence"
            || node.id.0 == "ser_vs_estar_essence_state"));
}

#[test]
fn test_find_weakest_root_concept() {
    let graph = get_default_linguistic_graph();
    let target = ConceptId("travel_logistics_disruptions".to_string());
    let mut scores = HashMap::new();
    scores.insert("irregular_subjunctive_stems".to_string(), 0.35);
    scores.insert("subjunctive_temporal_future".to_string(), 0.40);
    scores.insert("travel_logistics_disruptions".to_string(), 0.20);

    let weakest = graph.find_weakest_prerequisite_root(&target, &scores);
    assert!(weakest.is_some());
    assert_eq!(weakest.unwrap().0, "irregular_subjunctive_stems");
}

#[test]
fn test_cycle_detection_on_cyclic_graph() {
    let mut graph = LinguisticGraph::new();
    let mut node_a = ConceptNode::new(
        "a",
        "Node A",
        ConceptCategory::MoodSelection,
        Level::Baseline,
        "Description A",
    );
    let mut node_b = ConceptNode::new(
        "b",
        "Node B",
        ConceptCategory::MoodSelection,
        Level::B1,
        "Description B",
    );
    let mut node_c = ConceptNode::new(
        "c",
        "Node C",
        ConceptCategory::MoodSelection,
        Level::B2,
        "Description C",
    );

    node_a.prerequisite_concepts = vec![ConceptId("b".to_string())];
    node_b.prerequisite_concepts = vec![ConceptId("c".to_string())];
    node_c.prerequisite_concepts = vec![ConceptId("a".to_string())];

    graph.add_node(node_a);
    graph.add_node(node_b);
    graph.add_node(node_c);

    assert!(graph.validate_no_cycles().is_err());
}

#[test]
fn test_default_graph_covers_all_categories_and_levels() {
    let graph = get_default_linguistic_graph();

    let categories: HashSet<_> = graph.nodes.values().map(|n| n.category).collect();
    assert!(categories.contains(&ConceptCategory::AspectAndTense));
    assert!(categories.contains(&ConceptCategory::MoodSelection));
    assert!(categories.contains(&ConceptCategory::PronounsAndVoice));
    assert!(categories.contains(&ConceptCategory::PrepositionsAndRelators));
    assert!(categories.contains(&ConceptCategory::SyntaxAndRhetoric));
    assert!(categories.contains(&ConceptCategory::SociolinguisticRegisters));
    assert!(categories.contains(&ConceptCategory::PracticalPragmatics));

    let levels: HashSet<_> = graph.nodes.values().map(|n| n.level).collect();
    assert!(levels.contains(&Level::Baseline));
    assert!(levels.contains(&Level::B1));
    assert!(levels.contains(&Level::B2));
    assert!(levels.contains(&Level::C1));
}
