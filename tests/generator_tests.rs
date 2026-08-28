use spanglings::core::generator::{generate_drill_items_for_topic, generate_random_drill_items};
use spanglings::core::reference::list_grammar_concepts;

#[test]
fn test_all_24_concepts_have_combinatorial_generator_support() {
    let concepts = list_grammar_concepts();
    assert_eq!(concepts.len(), 24);
    for concept in concepts {
        let items = generate_drill_items_for_topic(concept.slug, 10);
        assert!(
            !items.is_empty(),
            "Generator should produce items for concept '{}'",
            concept.slug
        );
        for item in &items {
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.target.is_empty());
            assert!(!item.explanation.is_empty());
            assert!(
                !item.trigger_sentence.contains('{'),
                "Unrendered template token in: {}",
                item.trigger_sentence
            );
            assert!(
                !item.trigger_sentence.contains('}'),
                "Unrendered template token in: {}",
                item.trigger_sentence
            );
        }
    }
}

#[test]
fn test_generate_large_question_batch_without_panics() {
    let items = generate_random_drill_items(500);
    assert_eq!(items.len(), 500);
}
