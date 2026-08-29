use spanglings::core::arcade::{
    generate_4choice_items, generate_showdown_items, list_showdown_pairs, ArcadeItem, ShowdownPair,
};
use spanglings::core::reference::list_grammar_concepts;
use std::str::FromStr;

#[test]
fn test_all_16_showdown_pairs_generate_valid_items() {
    let pairs = list_showdown_pairs();
    assert_eq!(pairs.len(), 16);
    for pair in pairs {
        assert!(!pair.slug().is_empty());
        assert!(!pair.title().is_empty());
        assert!(!pair.description().is_empty());

        let (opt1, opt2) = pair.options();
        assert!(!opt1.is_empty());
        assert!(!opt2.is_empty());
        assert_ne!(opt1, opt2);

        let empty = generate_showdown_items(pair, 0);
        assert!(empty.is_empty());

        let items = generate_showdown_items(pair, 12);
        assert_eq!(
            items.len(),
            12,
            "Should generate 12 items for pair {:?}",
            pair
        );
        for item in &items {
            assert_eq!(
                item.options.len(),
                2,
                "Showdown must have exactly 2 options"
            );
            assert!(item.correct_index < 2);
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.explanation.is_empty());
            assert_ne!(item.options[0], item.options[1], "Options must be distinct");
            assert!(item.is_correct(item.correct_index));
            assert!(!item.is_correct(1 - item.correct_index));
            assert_eq!(item.correct_option(), &item.options[item.correct_index]);
        }
    }
}

#[test]
fn test_4choice_generator_across_all_24_concepts() {
    let concepts = list_grammar_concepts();
    for concept in concepts {
        let empty = generate_4choice_items(concept.slug, 0);
        assert!(empty.is_empty());

        let items = generate_4choice_items(concept.slug, 5);
        assert_eq!(
            items.len(),
            5,
            "Should generate 5 choice items for {}",
            concept.slug
        );
        for item in &items {
            assert_eq!(
                item.options.len(),
                4,
                "Choice items must have exactly 4 options"
            );
            assert!(item.correct_index < 4);
            assert!(!item.trigger_sentence.is_empty());
            assert!(!item.explanation.is_empty());
            assert!(item.is_correct(item.correct_index));
            assert_eq!(item.correct_option(), &item.options[item.correct_index]);

            let mut set = std::collections::HashSet::new();
            for opt in &item.options {
                set.insert(opt.clone());
            }
            assert_eq!(
                set.len(),
                4,
                "All 4 options must be unique: {:?}",
                item.options
            );
        }
    }
}

#[test]
fn test_showdown_pair_parsing() {
    // Original 8 pairs
    assert_eq!(
        ShowdownPair::from_str("por-para"),
        Some(ShowdownPair::PorPara)
    );
    assert_eq!(
        ShowdownPair::from_str("ser_estar"),
        Some(ShowdownPair::SerEstar)
    );
    assert_eq!(
        ShowdownPair::from_str("subj-ind"),
        Some(ShowdownPair::SubjInd)
    );
    assert_eq!(
        ShowdownPair::from_str("pret-imp"),
        Some(ShowdownPair::PretImp)
    );
    assert_eq!(
        ShowdownPair::from_str("tu-usted"),
        Some(ShowdownPair::TuUsted)
    );
    assert_eq!(ShowdownPair::from_str("lo-le"), Some(ShowdownPair::LoLe));
    assert_eq!(
        ShowdownPair::from_str("sino-pero"),
        Some(ShowdownPair::SinoPero)
    );
    assert_eq!(
        ShowdownPair::from_str("para-que-porque"),
        Some(ShowdownPair::ParaQuePorque)
    );

    // Expanded 8 pairs
    assert_eq!(
        ShowdownPair::from_str("tener-haber"),
        Some(ShowdownPair::TenerHaber)
    );
    assert_eq!(
        ShowdownPair::from_str("have"),
        Some(ShowdownPair::TenerHaber)
    );
    assert_eq!(
        ShowdownPair::from_str("tener"),
        Some(ShowdownPair::TenerHaber)
    );
    assert_eq!(
        ShowdownPair::from_str("haber"),
        Some(ShowdownPair::TenerHaber)
    );

    assert_eq!(
        ShowdownPair::from_str("saber-conocer"),
        Some(ShowdownPair::SaberConocer)
    );
    assert_eq!(
        ShowdownPair::from_str("know"),
        Some(ShowdownPair::SaberConocer)
    );
    assert_eq!(
        ShowdownPair::from_str("saber"),
        Some(ShowdownPair::SaberConocer)
    );
    assert_eq!(
        ShowdownPair::from_str("conocer"),
        Some(ShowdownPair::SaberConocer)
    );

    assert_eq!(
        ShowdownPair::from_str("muy-mucho"),
        Some(ShowdownPair::MuyMucho)
    );
    assert_eq!(
        ShowdownPair::from_str("very-much"),
        Some(ShowdownPair::MuyMucho)
    );
    assert_eq!(
        ShowdownPair::from_str("very_much"),
        Some(ShowdownPair::MuyMucho)
    );
    assert_eq!(ShowdownPair::from_str("very"), Some(ShowdownPair::MuyMucho));
    assert_eq!(ShowdownPair::from_str("much"), Some(ShowdownPair::MuyMucho));
    assert_eq!(ShowdownPair::from_str("muy"), Some(ShowdownPair::MuyMucho));
    assert_eq!(
        ShowdownPair::from_str("mucho"),
        Some(ShowdownPair::MuyMucho)
    );

    assert_eq!(
        ShowdownPair::from_str("pedir-preguntar"),
        Some(ShowdownPair::PedirPreguntar)
    );
    assert_eq!(
        ShowdownPair::from_str("ask"),
        Some(ShowdownPair::PedirPreguntar)
    );
    assert_eq!(
        ShowdownPair::from_str("pedir"),
        Some(ShowdownPair::PedirPreguntar)
    );
    assert_eq!(
        ShowdownPair::from_str("preguntar"),
        Some(ShowdownPair::PedirPreguntar)
    );

    assert_eq!(
        ShowdownPair::from_str("llevar-traer"),
        Some(ShowdownPair::LlevarTraer)
    );
    assert_eq!(
        ShowdownPair::from_str("take-bring"),
        Some(ShowdownPair::LlevarTraer)
    );
    assert_eq!(
        ShowdownPair::from_str("take_bring"),
        Some(ShowdownPair::LlevarTraer)
    );
    assert_eq!(
        ShowdownPair::from_str("llevar"),
        Some(ShowdownPair::LlevarTraer)
    );
    assert_eq!(
        ShowdownPair::from_str("traer"),
        Some(ShowdownPair::LlevarTraer)
    );

    assert_eq!(
        ShowdownPair::from_str("haber-estar"),
        Some(ShowdownPair::HaberEstar)
    );
    assert_eq!(
        ShowdownPair::from_str("hay-esta"),
        Some(ShowdownPair::HaberEstar)
    );
    assert_eq!(
        ShowdownPair::from_str("exist-locate"),
        Some(ShowdownPair::HaberEstar)
    );
    assert_eq!(
        ShowdownPair::from_str("estar-loc"),
        Some(ShowdownPair::HaberEstar)
    );

    assert_eq!(
        ShowdownPair::from_str("ir-irse"),
        Some(ShowdownPair::IrIrse)
    );
    assert_eq!(
        ShowdownPair::from_str("go-leave"),
        Some(ShowdownPair::IrIrse)
    );
    assert_eq!(
        ShowdownPair::from_str("go_leave"),
        Some(ShowdownPair::IrIrse)
    );
    assert_eq!(ShowdownPair::from_str("ir"), Some(ShowdownPair::IrIrse));
    assert_eq!(ShowdownPair::from_str("irse"), Some(ShowdownPair::IrIrse));

    assert_eq!(
        ShowdownPair::from_str("bien-bueno"),
        Some(ShowdownPair::BienBueno)
    );
    assert_eq!(
        ShowdownPair::from_str("well-good"),
        Some(ShowdownPair::BienBueno)
    );
    assert_eq!(
        ShowdownPair::from_str("well_good"),
        Some(ShowdownPair::BienBueno)
    );
    assert_eq!(
        ShowdownPair::from_str("bien"),
        Some(ShowdownPair::BienBueno)
    );
    assert_eq!(
        ShowdownPair::from_str("bueno"),
        Some(ShowdownPair::BienBueno)
    );
    assert_eq!(
        ShowdownPair::from_str("buen"),
        Some(ShowdownPair::BienBueno)
    );

    assert_eq!(ShowdownPair::from_str("unknown-pair"), None);

    // Test FromStr trait
    assert_eq!(
        <ShowdownPair as FromStr>::from_str("por-para").unwrap(),
        ShowdownPair::PorPara
    );
    assert_eq!(
        <ShowdownPair as FromStr>::from_str("tener-haber").unwrap(),
        ShowdownPair::TenerHaber
    );
    assert!(<ShowdownPair as FromStr>::from_str("invalid").is_err());
}

#[test]
fn test_arcade_item_methods() {
    let item = ArcadeItem {
        topic: "ser-estar".to_string(),
        trigger_sentence: "El servidor ____ activo.".to_string(),
        prompt_cue: "[1] está | [2] es".to_string(),
        options: vec!["está".to_string(), "es".to_string()],
        correct_index: 0,
        explanation: "Estar for state".to_string(),
    };
    assert!(item.is_correct(0));
    assert!(!item.is_correct(1));
    assert_eq!(item.correct_option(), "está");
}
