"""Tests for the Spanglings MkDocs Playground Page, Fullscreen Layout & CI Workflow.

Validates `docs/playground.md`, `mkdocs.yml` navigation and asset declarations,
CI documentation build workflow steps, playground bundle integrity, and strict
MkDocs site generation.
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any

import pytest
import yaml


@pytest.fixture(scope="module")
def repo_root() -> Path:
    """Returns repository root directory."""
    return Path(__file__).resolve().parent.parent


@pytest.fixture(scope="module")
def playground_md_path(repo_root: Path) -> Path:
    """Returns path to docs/playground.md."""
    return repo_root / "docs" / "playground.md"


@pytest.fixture(scope="module")
def playground_html_path(repo_root: Path) -> Path:
    """Returns path to docs/playground/index.html."""
    return repo_root / "docs" / "playground" / "index.html"


@pytest.fixture(scope="module")
def mkdocs_yml_path(repo_root: Path) -> Path:
    """Returns path to mkdocs.yml."""
    return repo_root / "mkdocs.yml"


@pytest.fixture(scope="module")
def docs_workflow_path(repo_root: Path) -> Path:
    """Returns path to .github/workflows/docs.yml."""
    return repo_root / ".github" / "workflows" / "docs.yml"


@pytest.fixture(scope="module")
def bundle_path(repo_root: Path) -> Path:
    """Returns path to docs/assets/playground/playground-bundle.json."""
    return repo_root / "docs" / "assets" / "playground" / "playground-bundle.json"


def test_standalone_playground_html_exists_and_contains_core_elements(
    playground_html_path: Path,
) -> None:
    """Verify docs/playground/index.html exists and contains standalone header, root, app mount, and scripts."""
    assert playground_html_path.exists(), (
        f"docs/playground/index.html must exist at {playground_html_path}"
    )

    content = playground_html_path.read_text(encoding="utf-8")

    # 1. HTML5 document, lang, and data-theme
    assert "<!DOCTYPE html>" in content or "<!doctype html>" in content.lower()
    assert 'lang="en"' in content
    assert 'data-theme="dark"' in content

    # 2. Title
    assert (
        "Spanglings Playground — Interactive Spanish Learning Platform & Rapid Arcade Arena"
        in content
    )

    # 3. Standalone Header and Brand elements
    assert 'id="standalone-header"' in content
    assert "🇪🇸" in content
    assert "Spanglings" in content
    assert "brand-badge" in content
    assert "Interactive Playground" in content

    # 4. Navigation links in header
    assert 'href="../"' in content or "Documentation" in content
    assert 'href="../syllabus/"' in content or "Syllabus" in content
    assert "https://github.com/dnf0/spanglings" in content
    assert 'id="theme-toggle-btn"' in content

    # 5. Standalone Playground Root and Spanglings App Mount
    assert 'id="standalone-playground-root"' in content
    assert 'id="spanglings-app"' in content
    assert "spanglings-playground" in content

    # 6. Scripts & Loader integration
    assert "loader.js" in content
    assert "playground.js" in content
    assert "storage.js" in content or "storage" in content.lower()

    # 7. Theme persistence and switching script
    assert "spanglings-theme" in content
    assert "localStorage" in content


def load_mkdocs_yaml(content: str) -> dict[str, Any]:
    """Load mkdocs yaml ignoring custom python constructor tags."""

    class CustomSafeLoader(yaml.SafeLoader):
        pass

    CustomSafeLoader.add_multi_constructor(
        "tag:yaml.org,2002:python/", lambda loader, suffix, node: None
    )
    CustomSafeLoader.add_multi_constructor(
        "!python/", lambda loader, suffix, node: None
    )
    return yaml.load(content, Loader=CustomSafeLoader) or {}


def test_mkdocs_config_nav_and_extra_assets(mkdocs_yml_path: Path) -> None:
    """Verify mkdocs.yml includes playground/index.html in nav and extra.css in extra_css."""
    assert mkdocs_yml_path.exists(), f"mkdocs.yml must exist at {mkdocs_yml_path}"

    content = mkdocs_yml_path.read_text(encoding="utf-8")
    config = load_mkdocs_yaml(content)

    # Validate nav entries
    nav = config.get("nav", [])
    has_playground_nav = False
    for item in nav:
        if isinstance(item, dict):
            for title, target in item.items():
                if target == "playground/index.html" or (
                    "playground" in str(title).lower()
                    and "playground/index.html" in str(target)
                ):
                    has_playground_nav = True
        elif isinstance(item, str) and item == "playground/index.html":
            has_playground_nav = True

    assert has_playground_nav, (
        "mkdocs.yml nav must include '- Interactive Playground: playground/index.html'"
    )

    # Validate extra_css entries
    extra_css = config.get("extra_css", [])
    assert any("extra.css" in str(css) for css in extra_css), (
        f"mkdocs.yml extra_css must include extra.css, got: {extra_css}"
    )
    assert not any("playground.css" in str(css) for css in extra_css), (
        "mkdocs.yml extra_css must NOT include playground.css (it belongs only in standalone playground HTML)"
    )


def test_mkdocs_theme_palette_is_indigo_slate(mkdocs_yml_path: Path) -> None:
    """Verify mkdocs.yml uses clean indigo/slate palette instead of red/amber."""
    content = mkdocs_yml_path.read_text(encoding="utf-8")
    config = load_mkdocs_yaml(content)
    palette = config.get("theme", {}).get("palette", [])

    assert len(palette) >= 2
    for entry in palette:
        assert entry.get("primary") == "indigo", (
            f"Expected primary indigo, got {entry.get('primary')}"
        )
        assert entry.get("accent") == "indigo", (
            f"Expected accent indigo, got {entry.get('accent')}"
        )


def test_playground_css_scopes_html_body_layout(repo_root: Path) -> None:
    """Verify playground.css does not globally hide overflow on all document pages."""
    playground_css = repo_root / "docs" / "assets" / "playground" / "playground.css"
    content = playground_css.read_text(encoding="utf-8")
    assert (
        "html, body {\n  margin: 0;\n  padding: 0;\n  height: 100%;\n  overflow: hidden;"
        not in content
    )


def test_docs_overview_streamlined_dual_pillars(repo_root: Path) -> None:
    """Verify docs/index.md highlights the Manual, Web Playground, and Syllabus without terminal CLI/TUI cruft."""
    index_path = repo_root / "docs" / "index.md"
    assert index_path.exists()
    content = index_path.read_text(encoding="utf-8")

    # Core 3 navigation cards
    assert "Spanish Language Manual" in content
    assert "Interactive Web Playground" in content
    assert "Curriculum Syllabus" in content
    assert "manual.md" in content
    assert "playground/index.html" in content
    assert "syllabus.md" in content

    # 3 CEFR tiers & Dual-layer approach
    assert "Tier 1: Foundations & Aspectual Geometry" in content
    assert "Tier 2: Mood, Triggers & Pragmatic Voice" in content
    assert "Tier 3: Advanced Nuance, Registers & Edge Mechanics" in content
    assert "Communicative Mental Model" in content
    assert "Structural Decision Matrix" in content

    # Terminal CLI / TUI cruft should be removed
    assert "cargo install spanglings" not in content
    assert "spanglings watch" not in content
    assert "spanglings init" not in content
    assert "spanglings-demo.svg" not in content
    assert "spanglings lsp" not in content
    assert "nvim-lspconfig" not in content
    assert "language-server.spanglings-lsp" not in content


def test_readme_streamlined_wasm_and_manual_focus(repo_root: Path) -> None:
    """Verify README.md focuses on WebAssembly platform and Manual without legacy CLI/TUI docs."""
    readme_path = repo_root / "README.md"
    assert readme_path.exists()
    content = readme_path.read_text(encoding="utf-8")

    # Core introductions and links
    assert "https://dnf0.github.io/spanglings/" in content
    assert "https://dnf0.github.io/spanglings/playground/" in content
    assert "Spanish Language Manual" in content
    assert "WebAssembly" in content
    assert "Curriculum Syllabus" in content

    # Dual-layer model and CEFR tiers
    assert "Communicative Mental Model" in content
    assert "Structural Decision Matrix" in content or "Grammar Rule" in content

    # Legacy terminal / CLI / TUI cruft removed
    assert "cargo install spanglings" not in content
    assert "spanglings watch" not in content
    assert "spanglings init" not in content
    assert "spanglings tui" not in content
    assert "spanglings lsp" not in content
    assert "spanglings-demo.svg" not in content
    assert "nvim-lspconfig" not in content
    assert "Ratatui Terminal TUI" not in content


def test_docs_ci_workflow_builds_playground_bundle(
    docs_workflow_path: Path,
) -> None:
    """Verify .github/workflows/docs.yml executes build_wasm.py before deploy."""
    assert docs_workflow_path.exists(), (
        f".github/workflows/docs.yml must exist at {docs_workflow_path}"
    )

    content = docs_workflow_path.read_text(encoding="utf-8")

    # Workflow must execute build_wasm.py
    assert "build_wasm.py" in content, (
        ".github/workflows/docs.yml must run scripts/build_wasm.py"
    )


def test_playground_bundle_file_validity(bundle_path: Path) -> None:
    """Verify playground-bundle.json exists, is valid JSON, and has all expected sections."""
    assert bundle_path.exists(), f"playground-bundle.json must exist at {bundle_path}"

    raw_data = bundle_path.read_text(encoding="utf-8")
    data: dict[str, Any] = json.loads(raw_data)

    assert "version" in data
    assert "topics" in data and len(data["topics"]) == 24
    assert "frames" in data and len(data["frames"]) == 136
    assert "arcade_items" in data and len(data["arcade_items"]) >= 100
    assert "specialized_engines" in data and len(data["specialized_engines"]) == 5


def test_mkdocs_strict_build_succeeds(repo_root: Path) -> None:
    """Verify mkdocs build --strict runs and completes with exit code 0."""
    result = subprocess.run(
        ["uv", "run", "mkdocs", "build", "--strict"],
        cwd=repo_root,
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode == 0, (
        f"mkdocs build --strict failed with code {result.returncode}:\n"
        f"STDOUT:\n{result.stdout}\n"
        f"STDERR:\n{result.stderr}"
    )


def test_comprehensive_language_manual_structure(repo_root: Path) -> None:
    """Verify docs/manual.md exists and contains all 24 topics with mental models, rules, and deep links."""
    manual_path = repo_root / "docs" / "manual.md"
    assert manual_path.exists(), f"docs/manual.md must exist at {manual_path}"

    content = manual_path.read_text(encoding="utf-8")
    assert "# Spanglings Spanish Language Manual" in content

    # Check 24 topics
    expected_topics = [
        "ser-estar",
        "por-para",
        "past-tenses",
        "pronouns",
        "gustar",
        "reflexive",
        "stem-changing",
        "prepositions",
        "subjunctive",
        "imperfect-subjunctive",
        "imperative",
        "accidental-se",
        "passive-impersonal-se",
        "possessive-datives",
        "relative-pronouns",
        "gerund-rules",
        "verbs-of-becoming",
        "scalar-concession",
        "epistemic-conjecture",
        "adversatives",
        "false-friends",
        "voseo",
        "tech",
        "legal",
    ]
    for topic in expected_topics:
        assert (
            f'id="{topic}"' in content
            or f"#{topic}" in content
            or f"topic={topic}" in content
        ), f"Topic '{topic}' anchor or reference must be present in docs/manual.md"
        assert f"playground/?topic={topic}" in content, (
            f"Studio link for '{topic}' must be present in docs/manual.md"
        )
        assert f"playground/?mode=arcade&topic={topic}" in content, (
            f"Arcade showdown link for '{topic}' must be present in docs/manual.md"
        )

    assert content.count("### 💡 Communicative Mental Model") == 24
    assert content.count("### 📐 Grammar Rules & Decision Matrix") == 24
    assert "playground/?topic=" in content
    assert "playground/?mode=arcade" in content


def test_mkdocs_navigation_includes_manual_and_syllabus(
    mkdocs_yml_path: Path,
) -> None:
    """Verify mkdocs.yml navigation is streamlined around Manual, Syllabus, and Playground."""
    content = mkdocs_yml_path.read_text(encoding="utf-8")
    config = load_mkdocs_yaml(content)

    nav = config.get("nav", [])
    nav_targets: list[str] = []
    for item in nav:
        if isinstance(item, dict):
            for val in item.values():
                if isinstance(val, str):
                    nav_targets.append(val)
        elif isinstance(item, str):
            nav_targets.append(item)

    assert "index.md" in nav_targets
    assert "manual.md" in nav_targets
    assert "syllabus.md" in nav_targets
    assert "playground/index.html" in nav_targets
    assert "contributing.md" in nav_targets


def test_syllabus_links_all_topics_to_manual_and_playground(
    repo_root: Path,
) -> None:
    """Verify docs/syllabus.md links all 24 topics to manual.md anchors and playground."""
    syllabus_path = repo_root / "docs" / "syllabus.md"
    assert syllabus_path.exists(), f"docs/syllabus.md must exist at {syllabus_path}"

    content = syllabus_path.read_text(encoding="utf-8")
    expected_topics = [
        "ser-estar",
        "por-para",
        "past-tenses",
        "pronouns",
        "gustar",
        "reflexive",
        "stem-changing",
        "prepositions",
        "subjunctive",
        "imperfect-subjunctive",
        "imperative",
        "accidental-se",
        "passive-impersonal-se",
        "possessive-datives",
        "relative-pronouns",
        "gerund-rules",
        "verbs-of-becoming",
        "scalar-concession",
        "epistemic-conjecture",
        "adversatives",
        "false-friends",
        "voseo",
        "tech",
        "legal",
    ]
    for topic in expected_topics:
        assert f"manual.md#{topic}" in content, (
            f"Topic '{topic}' link to manual.md must be present in docs/syllabus.md"
        )
        assert f"topic={topic}" in content, (
            f"Topic '{topic}' link to playground must be present in docs/syllabus.md"
        )
