"""Curriculum & Arcade Web Bundle Generator.

This script parses the Spanglings Rust curriculum, sentence frames, binary showdowns,
and specialized arcade drill engines to build a unified, zero-filesystem JSON payload
(`docs/assets/playground/playground-bundle.json`) for browser and WebAssembly execution.

Every concept, frame, and arcade item includes dual-layer pedagogical explanations:
- Communicative Mental Model (`meaning` / `plain_english`)
- Structural Grammar Rule (`rule` / `explanation`)
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


def clean_rust_str(raw: str) -> str:
    """Decode escaped Rust string literals into clean Python strings.

    Args:
        raw: Raw Rust string literal with surrounding quotes or escape characters.

    Returns:
        Unescaped string content.
    """
    cleaned = raw.strip()
    if cleaned.startswith('"') and cleaned.endswith('"'):
        cleaned = cleaned[1:-1]
    # Replace standard Rust string escape sequences
    cleaned = cleaned.replace(r"\"", '"').replace(r"\'", "'").replace(r"\n", "\n")
    return cleaned.strip()


def parse_cargo_version(cargo_toml_path: Path) -> str:
    """Extract package version from Cargo.toml.

    Args:
        cargo_toml_path: Path to Cargo.toml.

    Returns:
        Package version string (e.g. '0.5.4').

    Raises:
        ValueError: If version field is not found.
    """
    content = cargo_toml_path.read_text(encoding="utf-8")
    # Match package version at the top level
    match = re.search(r'version\s*=\s*"([^"]+)"', content)
    if not match:
        raise ValueError(f"Could not find version in {cargo_toml_path}")
    return match.group(1)


def parse_reference_topics(reference_rs_path: Path) -> list[dict[str, Any]]:
    """Extract all 24 grammar topics, concepts, and cheat cards from reference.rs.

    Args:
        reference_rs_path: Path to src/core/reference.rs.

    Returns:
        List of topic dictionaries with metadata, keywords, cards, and dual-layer explanations.
    """
    content = reference_rs_path.read_text(encoding="utf-8")

    # Extract all static cheat cards defined as raw string constants
    cards_map: dict[str, str] = {}
    card_matches = re.finditer(
        r'pub const\s+([A-Z0-9_]+_CARD)\s*:\s*&str\s*=\s*r#"([\s\S]*?)"#;',
        content,
    )
    for m in card_matches:
        card_const_name = m.group(1)
        card_body = m.group(2).strip()
        cards_map[card_const_name] = card_body

    # Extract all GrammarConcept struct instances
    topics: list[dict[str, Any]] = []
    concept_blocks = re.finditer(r"GrammarConcept\s*\{([\s\S]*?)\n\s*\},", content)

    for block_match in concept_blocks:
        block = block_match.group(1)

        slug_match = re.search(r'slug:\s*"([^"]+)"', block)
        title_match = re.search(r'title:\s*"([^"]+)"', block)
        gloss_match = re.search(r'gloss:\s*"([^"]+)"', block)
        mental_match = re.search(
            r'mental_model:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL
        )
        card_ident_match = re.search(r"card:\s*([A-Z0-9_]+_CARD)", block)
        keywords_block = re.search(r"keywords:\s*&\[([\s\S]*?)\]", block)

        if not (slug_match and title_match and gloss_match and mental_match):
            continue

        slug = slug_match.group(1)
        title = title_match.group(1)
        gloss = gloss_match.group(1)
        mental_model = clean_rust_str(mental_match.group(1))
        card_ident = card_ident_match.group(1) if card_ident_match else ""
        card_text = cards_map.get(card_ident, "")

        keywords: list[str] = []
        if keywords_block:
            keywords = re.findall(r'"([^"]+)"', keywords_block.group(1))

        # Include dual-layer explanations for consistency
        topics.append(
            {
                "slug": slug,
                "title": title,
                "gloss": gloss,
                "mental_model": mental_model,
                "card": card_text,
                "keywords": keywords,
                "plain_english": mental_model,
                "meaning": mental_model,
                "rule": gloss,
                "explanation": gloss,
            }
        )

    return topics


def parse_generator_frames(generator_rs_path: Path) -> list[dict[str, Any]]:
    """Extract all 136 sentence frames across all 24 topics from generator.rs.

    Args:
        generator_rs_path: Path to src/core/generator.rs.

    Returns:
        List of frame dictionaries with template, target, explanation, and slot options.
    """
    content = generator_rs_path.read_text(encoding="utf-8")

    frames: list[dict[str, Any]] = []
    topic_counters: dict[str, int] = {}

    frame_blocks = re.finditer(r"SentenceFrame\s*\{([\s\S]*?)\n\s*\},", content)

    for block_match in frame_blocks:
        block = block_match.group(1)

        topic_match = re.search(r'topic:\s*"([^"]+)"', block)
        formula_match = re.search(
            r'formula_cue:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL
        )
        template_match = re.search(r'template:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL)
        verb_match = re.search(r'target_verb:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL)
        subj_match = re.search(
            r'target_subject:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL
        )
        target_match = re.search(r'target:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL)
        explanation_match = re.search(
            r'explanation:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL
        )
        plain_english_match = re.search(
            r'plain_english:\s*"((?:[^"\\]|\\.)*)"', block, re.DOTALL
        )

        if not (
            topic_match
            and formula_match
            and template_match
            and target_match
            and explanation_match
        ):
            continue

        topic = topic_match.group(1)
        formula_cue = clean_rust_str(formula_match.group(1))
        template = clean_rust_str(template_match.group(1))
        target_verb = clean_rust_str(verb_match.group(1)) if verb_match else ""
        target_subject = clean_rust_str(subj_match.group(1)) if subj_match else ""
        target = clean_rust_str(target_match.group(1))
        explanation = clean_rust_str(explanation_match.group(1))
        plain_english = (
            clean_rust_str(plain_english_match.group(1))
            if plain_english_match
            else explanation
        )

        # Parse slots mapping: e.g. ("opener", &["Dudo", "No creo"])
        slots: dict[str, list[str]] = {}
        slots_start = block.find("slots:")
        if slots_start != -1:
            slots_part = block[slots_start:]
            slot_tuples = re.finditer(
                r'\(\s*"([^"]+)"\s*,\s*&\[([\s\S]*?)\]\s*\)', slots_part
            )
            for st in slot_tuples:
                slot_key = st.group(1)
                slot_options = re.findall(r'"([^"]+)"', st.group(2))
                slots[slot_key] = [clean_rust_str(opt) for opt in slot_options]

        # Format unique id per topic (e.g. subjunctive-01)
        topic_counters[topic] = topic_counters.get(topic, 0) + 1
        frame_id = f"{topic}-{topic_counters[topic]:02d}"

        frames.append(
            {
                "id": frame_id,
                "topic": topic,
                "template": template,
                "formula_cue": formula_cue,
                "target_verb": target_verb,
                "target_subject": target_subject,
                "target": target,
                "explanation": explanation,
                "plain_english": plain_english,
                "meaning": plain_english,
                "rule": explanation,
                "slots": slots,
            }
        )

    return frames


def parse_arcade_showdowns_and_engines(
    arcade_rs_path: Path,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    """Extract all binary showdown items and specialized engine items from arcade.rs.

    Args:
        arcade_rs_path: Path to src/core/arcade.rs.

    Returns:
        Tuple of (arcade_items list, specialized_engines list).
    """
    content = arcade_rs_path.read_text(encoding="utf-8")

    # 1. Showdown Pairs Definitions & Pools
    showdown_defs = [
        ("por-para", "Por vs. Para", "SHOWDOWN_POR_PARA"),
        ("ser-estar", "Ser vs. Estar", "SHOWDOWN_SER_ESTAR"),
        ("subj-ind", "Subjunctive vs. Indicative", "SHOWDOWN_SUBJ_IND"),
        ("pret-imp", "Preterite vs. Imperfect", "SHOWDOWN_PRET_IMP"),
        ("tu-usted", "Tú vs. Usted", "SHOWDOWN_TU_USTED"),
        ("lo-le", "Direct (Lo/La) vs. Indirect (Le/Les)", "SHOWDOWN_LO_LE"),
        ("sino-pero", "Sino vs. Pero", "SHOWDOWN_SINO_PERO"),
        ("para-que-porque", "Para que vs. Porque", "SHOWDOWN_PARA_QUE_PORQUE"),
        (
            "tener-haber",
            'Tener vs Haber ("to have" / auxiliary / existential)',
            "SHOWDOWN_TENER_HABER",
        ),
        (
            "saber-conocer",
            'Saber vs Conocer ("to know" facts vs acquaintance)',
            "SHOWDOWN_SABER_CONOCER",
        ),
        ("muy-mucho", "Muy vs Mucho (adverb vs quantifier)", "SHOWDOWN_MUY_MUCHO"),
        (
            "pedir-preguntar",
            "Pedir vs Preguntar (request vs inquire)",
            "SHOWDOWN_PEDIR_PREGUNTAR",
        ),
        (
            "llevar-traer",
            "Llevar vs Traer (away vs toward speaker)",
            "SHOWDOWN_LLEVAR_TRAER",
        ),
        (
            "haber-estar",
            "Hay/Haber vs Está/Estar (existence vs location)",
            "SHOWDOWN_HABER_ESTAR",
        ),
        ("ir-irse", "Ir vs Irse (destination vs departure)", "SHOWDOWN_IR_IRSE"),
        (
            "bien-bueno",
            "Bien vs Bueno/Buen (adverb vs adjective)",
            "SHOWDOWN_BIEN_BUENO",
        ),
    ]

    arcade_items: list[dict[str, Any]] = []

    for slug, title, pool_name in showdown_defs:
        # Extract pool array content
        pool_pattern = rf"static\s+{pool_name}\s*:\s*&\[ShowdownSentence\]\s*=\s*&\[([\s\S]*?)\n\];"
        pool_match = re.search(pool_pattern, content)
        if not pool_match:
            continue

        pool_body = pool_match.group(1)
        sentence_blocks = re.finditer(
            r"ShowdownSentence\s*\{([\s\S]*?)\n\s*\},?", pool_body
        )

        for idx, s_match in enumerate(sentence_blocks):
            s_block = s_match.group(1)
            sent_match = re.search(
                r'sentence:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            target_match = re.search(
                r'target:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            distractor_match = re.search(
                r'distractor:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            explanation_match = re.search(
                r'explanation:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            plain_match = re.search(
                r'plain_english:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )

            if not (
                sent_match and target_match and distractor_match and explanation_match
            ):
                continue

            sentence = clean_rust_str(sent_match.group(1))
            target = clean_rust_str(target_match.group(1))
            distractor = clean_rust_str(distractor_match.group(1))
            explanation = clean_rust_str(explanation_match.group(1))
            plain_english = (
                clean_rust_str(plain_match.group(1)) if plain_match else explanation
            )

            arcade_items.append(
                {
                    "id": f"{slug}_{idx}",
                    "mode": "showdown",
                    "topic": slug,
                    "trigger_sentence": sentence,
                    "prompt_cue": title,
                    "options": [target, distractor],
                    "correct_index": 0,
                    "correct_option": target,
                    "explanation": explanation,
                    "plain_english": plain_english,
                    "meaning": plain_english,
                    "rule": explanation,
                }
            )

    # 2. Specialized Engines Definitions & Pools
    specialized_defs = [
        (
            "regimen",
            "Prepositional Regimen Engine (Verbos con Régimen)",
            "Prepositional Regimen",
            "Master verb-bound prepositions (soñar con, depender de, fijarse en).",
            "ENGINE_REGIMEN_POOL",
        ),
        (
            "irregulars",
            "Irregular Verb Speed Gun (Conjugación Irregular)",
            "Irregular Verb Speed Gun",
            "High-speed drills on stem changers, g-verbs, and irregular forms.",
            "ENGINE_IRREGULARS_POOL",
        ),
        (
            "false-friends",
            "False Friends Trap Detector (Falsos Amigos)",
            "False Friends Trap Detector",
            "Avoid deceptive cognates and misleading English-Spanish traps.",
            "ENGINE_FALSE_FRIENDS_POOL",
        ),
        (
            "se-matrix",
            'The "Se" Matrix (Las 5 Caras del Se)',
            'The "Se" Matrix',
            "Disambiguate reflexive, reciprocal, passive, impersonal, and accidental 'se'.",
            "ENGINE_SE_MATRIX_POOL",
        ),
        (
            "connectors",
            "Discourse Connectors & Flow (Conectores B2/C1)",
            "Discourse Connectors & Flow",
            "Advanced logical transitions, nuance markers, and argumentative flow.",
            "ENGINE_CONNECTORS_POOL",
        ),
    ]

    specialized_engines: list[dict[str, Any]] = []

    for slug, title, cue_title, desc, pool_name in specialized_defs:
        specialized_engines.append(
            {
                "id": slug,
                "slug": slug,
                "title": title,
                "cue": cue_title,
                "description": desc,
            }
        )

        pool_pattern = rf"static\s+{pool_name}\s*:\s*&\[SpecializedEngineSentence\]\s*=\s*&\[([\s\S]*?)\n\];"
        pool_match = re.search(pool_pattern, content)
        if not pool_match:
            continue

        pool_body = pool_match.group(1)
        sentence_blocks = re.finditer(
            r"SpecializedEngineSentence\s*\{([\s\S]*?)\n\s*\},?", pool_body
        )

        for idx, s_match in enumerate(sentence_blocks):
            s_block = s_match.group(1)
            sent_match = re.search(
                r'sentence:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            target_match = re.search(
                r'target:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            distractors_block = re.search(r"distractors:\s*\[([\s\S]*?)\]", s_block)
            explanation_match = re.search(
                r'explanation:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )
            plain_match = re.search(
                r'plain_english:\s*"((?:[^"\\]|\\.)*)"', s_block, re.DOTALL
            )

            if not (
                sent_match and target_match and distractors_block and explanation_match
            ):
                continue

            sentence = clean_rust_str(sent_match.group(1))
            target = clean_rust_str(target_match.group(1))
            distractors = [
                clean_rust_str(d)
                for d in re.findall(r'"([^"]+)"', distractors_block.group(1))
            ]
            explanation = clean_rust_str(explanation_match.group(1))
            plain_english = (
                clean_rust_str(plain_match.group(1)) if plain_match else explanation
            )

            options = [target] + distractors

            arcade_items.append(
                {
                    "id": f"{slug}_{idx}",
                    "mode": "engine",
                    "topic": slug,
                    "trigger_sentence": sentence,
                    "prompt_cue": cue_title,
                    "options": options,
                    "correct_index": 0,
                    "correct_option": target,
                    "explanation": explanation,
                    "plain_english": plain_english,
                    "meaning": plain_english,
                    "rule": explanation,
                }
            )

    return arcade_items, specialized_engines


def generate_playground_bundle(
    repo_root: Path | None = None,
) -> dict[str, Any]:
    """Construct full playground bundle JSON dictionary from repository sources.

    Args:
        repo_root: Optional root directory of the Spanglings repository.

    Returns:
        Dictionary containing bundle metadata, topics, frames, arcade items, and engines.
    """
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent

    cargo_toml = repo_root / "Cargo.toml"
    reference_rs = repo_root / "src" / "core" / "reference.rs"
    generator_rs = repo_root / "src" / "core" / "generator.rs"
    arcade_rs = repo_root / "src" / "core" / "arcade.rs"

    # Extract all data models
    version = parse_cargo_version(cargo_toml)
    topics = parse_reference_topics(reference_rs)
    frames = parse_generator_frames(generator_rs)
    arcade_items, specialized_engines = parse_arcade_showdowns_and_engines(arcade_rs)

    return {
        "version": version,
        "topics": topics,
        "frames": frames,
        "arcade_items": arcade_items,
        "specialized_engines": specialized_engines,
    }


def write_playground_bundle(
    output_path: Path | None = None, repo_root: Path | None = None
) -> Path:
    """Build and write playground-bundle.json to disk.

    Args:
        output_path: Target path to write JSON.
        repo_root: Repository root path.

    Returns:
        Path of written file.
    """
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent

    if output_path is None:
        output_path = (
            repo_root / "docs" / "assets" / "playground" / "playground-bundle.json"
        )

    bundle = generate_playground_bundle(repo_root)

    # Ensure parent directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(bundle, f, indent=2, ensure_ascii=False)

    print(
        f"✓ Successfully generated playground bundle at {output_path} "
        f"({len(bundle['topics'])} topics, {len(bundle['frames'])} frames, "
        f"{len(bundle['arcade_items'])} arcade items, "
        f"{len(bundle['specialized_engines'])} specialized engines)."
    )
    return output_path


if __name__ == "__main__":
    write_playground_bundle()
