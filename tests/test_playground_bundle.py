"""Tests for the Spanglings WebAssembly & Browser Playground Bundle Generator.

Validates the structure, linguistic completeness, and dual-layer pedagogical
explanations across all curriculum topics, sentence frames, binary showdowns, and
specialized drill engines in `docs/assets/playground/playground-bundle.json`.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

from scripts.build_playground_bundle import (
    generate_playground_bundle,
    parse_cargo_version,
)


@pytest.fixture(scope="module")
def repo_root() -> Path:
    """Returns repository root directory."""
    return Path(__file__).resolve().parent.parent


@pytest.fixture(scope="module")
def bundle_path(repo_root: Path) -> Path:
    """Returns path to the compiled playground-bundle.json."""
    return repo_root / "docs" / "assets" / "playground" / "playground-bundle.json"


@pytest.fixture(scope="module")
def bundle_data(bundle_path: Path) -> dict[str, Any]:
    """Loads and returns the playground bundle JSON data."""
    assert bundle_path.exists(), f"Playground bundle not found at {bundle_path}"
    content = bundle_path.read_text(encoding="utf-8")
    data = json.loads(content)
    assert isinstance(data, dict), "Bundle must be a JSON object"
    return data


def test_playground_bundle_exists_and_is_valid_json(
    bundle_data: dict[str, Any], repo_root: Path
) -> None:
    """Verify that playground-bundle.json exists and contains package version."""
    expected_version = parse_cargo_version(repo_root / "Cargo.toml")
    assert bundle_data["version"] == expected_version, (
        f"Bundle version {bundle_data['version']} != Cargo.toml version {expected_version}"
    )


def test_bundle_has_all_24_topics(bundle_data: dict[str, Any]) -> None:
    """Verify all 24 grammar topics with metadata, cards, and keywords."""
    topics = bundle_data.get("topics", [])
    assert len(topics) == 24, f"Expected exactly 24 topics, got {len(topics)}"

    seen_slugs: set[str] = set()
    for topic in topics:
        slug = topic.get("slug")
        assert slug, "Topic must have a slug"
        assert slug not in seen_slugs, f"Duplicate topic slug: {slug}"
        seen_slugs.add(slug)

        assert topic.get("title"), f"Topic {slug} missing title"
        assert topic.get("gloss"), f"Topic {slug} missing gloss"
        assert topic.get("mental_model"), f"Topic {slug} missing mental_model"
        assert topic.get("card"), f"Topic {slug} missing reference card"
        assert len(topic.get("keywords", [])) > 0, f"Topic {slug} has empty keywords"

        # Dual-layer explanations
        assert topic.get("meaning"), f"Topic {slug} missing meaning"
        assert topic.get("plain_english"), f"Topic {slug} missing plain_english"
        assert topic.get("rule"), f"Topic {slug} missing rule"
        assert topic.get("explanation"), f"Topic {slug} missing explanation"


def test_bundle_has_all_sentence_frames(bundle_data: dict[str, Any]) -> None:
    """Verify all sentence frames (>= 136) across curriculum topics."""
    frames = bundle_data.get("frames", [])
    assert len(frames) >= 136, f"Expected at least 136 frames, got {len(frames)}"

    seen_ids: set[str] = set()
    for frame in frames:
        frame_id = frame.get("id")
        assert frame_id, "Frame must have an id"
        assert frame_id not in seen_ids, f"Duplicate frame id: {frame_id}"
        seen_ids.add(frame_id)

        assert frame.get("topic"), f"Frame {frame_id} missing topic"
        assert frame.get("template"), f"Frame {frame_id} missing template"
        assert "____" in frame.get("template", ""), (
            f"Frame {frame_id} template must contain '____'"
        )
        assert frame.get("formula_cue"), f"Frame {frame_id} missing formula_cue"
        assert frame.get("target"), f"Frame {frame_id} missing target"

        # Dual-layer explanations
        assert frame.get("meaning"), f"Frame {frame_id} missing meaning"
        assert frame.get("plain_english"), f"Frame {frame_id} missing plain_english"
        assert frame.get("rule"), f"Frame {frame_id} missing rule"
        assert frame.get("explanation"), f"Frame {frame_id} missing explanation"

        # Slots dictionary check
        slots = frame.get("slots")
        assert isinstance(slots, dict), f"Frame {frame_id} slots must be a dict"
        for slot_name, options in slots.items():
            assert isinstance(slot_name, str), "Slot key must be a string"
            assert isinstance(options, list), "Slot options must be a list"
            assert len(options) > 0, f"Slot {slot_name} has empty options"


def test_bundle_has_all_arcade_items(bundle_data: dict[str, Any]) -> None:
    """Verify all arcade items (>= 260 items) including showdowns and specialized engines."""
    arcade_items = bundle_data.get("arcade_items", [])
    assert len(arcade_items) >= 260, (
        f"Expected >= 260 arcade items, got {len(arcade_items)}"
    )

    showdowns = [item for item in arcade_items if item.get("mode") == "showdown"]
    engines = [item for item in arcade_items if item.get("mode") == "engine"]

    assert len(showdowns) >= 180, (
        f"Expected >= 180 showdown items, got {len(showdowns)}"
    )
    assert len(engines) >= 80, f"Expected >= 80 engine items, got {len(engines)}"

    seen_ids: set[str] = set()
    for item in arcade_items:
        item_id = item.get("id")
        assert item_id, "Arcade item must have an id"
        assert item_id not in seen_ids, f"Duplicate arcade item id: {item_id}"
        seen_ids.add(item_id)

        assert item.get("mode") in ("showdown", "engine"), (
            f"Invalid mode: {item.get('mode')}"
        )
        assert item.get("topic"), f"Item {item_id} missing topic"
        assert item.get("trigger_sentence"), f"Item {item_id} missing trigger_sentence"
        assert "____" in item.get("trigger_sentence", ""), (
            f"Item {item_id} sentence must contain '____'"
        )
        assert item.get("prompt_cue"), f"Item {item_id} missing prompt_cue"

        options = item.get("options", [])
        assert isinstance(options, list), f"Item {item_id} options must be a list"
        if item.get("mode") == "showdown":
            assert len(options) == 2, f"Showdown item {item_id} must have 2 options"
        elif item.get("mode") == "engine":
            assert len(options) == 4, f"Engine item {item_id} must have 4 options"

        correct_idx = item.get("correct_index")
        assert isinstance(correct_idx, int), f"Item {item_id} correct_index must be int"
        assert 0 <= correct_idx < len(options), (
            f"Item {item_id} correct_index out of bounds"
        )
        assert item.get("correct_option") == options[correct_idx], (
            f"Item {item_id} correct_option mismatch"
        )

        # Dual-layer explanations
        assert item.get("meaning"), f"Item {item_id} missing meaning"
        assert item.get("plain_english"), f"Item {item_id} missing plain_english"
        assert item.get("rule"), f"Item {item_id} missing rule"
        assert item.get("explanation"), f"Item {item_id} missing explanation"


def test_bundle_has_specialized_engines(bundle_data: dict[str, Any]) -> None:
    """Verify all 5 specialized drill engines metadata."""
    engines = bundle_data.get("specialized_engines", [])
    assert len(engines) == 5, f"Expected 5 specialized engines, got {len(engines)}"

    expected_slugs = {
        "regimen",
        "irregulars",
        "false-friends",
        "se-matrix",
        "connectors",
    }
    actual_slugs = {e.get("slug") for e in engines}
    assert actual_slugs == expected_slugs, (
        f"Engine slugs mismatch: {actual_slugs} vs {expected_slugs}"
    )

    for engine in engines:
        assert engine.get("title"), f"Engine {engine.get('slug')} missing title"
        assert engine.get("description"), (
            f"Engine {engine.get('slug')} missing description"
        )


def test_dual_layer_explanations_non_empty(bundle_data: dict[str, Any]) -> None:
    """Assert every single topic, frame, and arcade item has non-empty dual-layer explanations."""
    for topic in bundle_data.get("topics", []):
        assert topic["meaning"].strip(), f"Empty topic meaning for {topic.get('slug')}"
        assert topic["plain_english"].strip(), (
            f"Empty topic plain_english for {topic.get('slug')}"
        )
        assert topic["rule"].strip(), f"Empty topic rule for {topic.get('slug')}"
        assert topic["explanation"].strip(), (
            f"Empty topic explanation for {topic.get('slug')}"
        )

    for frame in bundle_data.get("frames", []):
        assert frame["meaning"].strip(), f"Empty frame meaning for {frame.get('id')}"
        assert frame["plain_english"].strip(), (
            f"Empty frame plain_english for {frame.get('id')}"
        )
        assert frame["rule"].strip(), f"Empty frame rule for {frame.get('id')}"
        assert frame["explanation"].strip(), (
            f"Empty frame explanation for {frame.get('id')}"
        )

    for item in bundle_data.get("arcade_items", []):
        assert item["meaning"].strip(), f"Empty arcade meaning for {item.get('id')}"
        assert item["plain_english"].strip(), (
            f"Empty arcade plain_english for {item.get('id')}"
        )
        assert item["rule"].strip(), f"Empty arcade rule for {item.get('id')}"
        assert item["explanation"].strip(), (
            f"Empty arcade explanation for {item.get('id')}"
        )


def test_generator_bundle_reproducible(
    repo_root: Path, bundle_data: dict[str, Any]
) -> None:
    """Verify that running the generator produces identical results."""
    generated = generate_playground_bundle(repo_root)
    assert generated == bundle_data, "Generated bundle does not match saved file"
