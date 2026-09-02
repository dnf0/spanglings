"""Tests for the Spanglings MkDocs Playground Page, Fullscreen Layout & CI Workflow.

Validates `docs/playground.md`, `mkdocs.yml` navigation and asset declarations,
CI documentation build workflow steps, playground bundle integrity, and strict
MkDocs site generation.
"""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
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


def test_playground_md_exists_and_contains_core_elements(
    playground_md_path: Path,
) -> None:
    """Verify docs/playground.md exists and contains header, mount, scripts, and instructions."""
    # Ensure playground page exists in docs directory
    assert playground_md_path.exists(), (
        f"docs/playground.md must exist at {playground_md_path}"
    )

    content = playground_md_path.read_text(encoding="utf-8")

    # 1. Title and header descriptions
    assert "# Spanglings WebAssembly Playground & Arcade Arena" in content
    assert "zero-backend client-side spanish learning environment" in content.lower()
    assert "webassembly" in content.lower()
    assert "storage" in content.lower()

    # 2. DOM mount container
    assert (
        '<div id="spanglings-app"></div>' in content
        or '<div id="spanglings-app"' in content
    )

    # 3. Scripts and loader integration
    assert "playground.js" in content
    assert "loader.js" in content or "monaco" in content.lower()

    # 4. Fullscreen / Zen Mode instructions
    assert "fullscreen" in content.lower() or "zen" in content.lower()

    # 5. Dual Mode instructions: Mode A (Curriculum Workspace) & Mode B (Rapid Arcade Arena)
    assert "curriculum workspace" in content.lower() or "mode a" in content.lower()
    assert "arcade arena" in content.lower() or "mode b" in content.lower()
    assert "showdown" in content.lower()
    assert (
        "pedagogical" in content.lower()
        or "grammar rule" in content.lower()
        or "mental model" in content.lower()
    )


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


def test_mkdocs_config_nav_and_extra_assets(mkdocs_yml_path: Path) -> None:
    """Verify mkdocs.yml includes playground/index.html in nav, playground/** in not_in_nav, and playground.css in extra_css."""
    assert mkdocs_yml_path.exists(), f"mkdocs.yml must exist at {mkdocs_yml_path}"

    content = mkdocs_yml_path.read_text(encoding="utf-8")
    config = yaml.safe_load(content)

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

    # Validate not_in_nav includes playground/**
    not_in_nav = config.get("not_in_nav", "")
    assert "playground/**" in str(not_in_nav), (
        f"mkdocs.yml not_in_nav must include 'playground/**', got: {not_in_nav}"
    )

    # Validate extra_css entries
    extra_css = config.get("extra_css", [])
    assert any("playground.css" in str(css) for css in extra_css), (
        f"mkdocs.yml extra_css must include playground.css, got: {extra_css}"
    )


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
