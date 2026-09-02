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


def test_mkdocs_config_nav_and_extra_assets(mkdocs_yml_path: Path) -> None:
    """Verify mkdocs.yml includes playground.md in nav and playground.css in extra_css."""
    assert mkdocs_yml_path.exists(), f"mkdocs.yml must exist at {mkdocs_yml_path}"

    content = mkdocs_yml_path.read_text(encoding="utf-8")
    config = yaml.safe_load(content)

    # Validate nav entries
    nav = config.get("nav", [])
    has_playground_nav = False
    for item in nav:
        if isinstance(item, dict):
            for title, target in item.items():
                if "playground" in str(title).lower() or target == "playground.md":
                    has_playground_nav = True
        elif isinstance(item, str) and item == "playground.md":
            has_playground_nav = True

    assert has_playground_nav, "mkdocs.yml nav must include playground.md"

    # Validate extra_css entries
    extra_css = config.get("extra_css", [])
    assert any("playground.css" in str(css) for css in extra_css), (
        f"mkdocs.yml extra_css must include playground.css, got: {extra_css}"
    )


def test_docs_ci_workflow_builds_playground_bundle(
    docs_workflow_path: Path,
) -> None:
    """Verify .github/workflows/docs.yml executes build_playground_bundle.py before deploy."""
    assert docs_workflow_path.exists(), (
        f".github/workflows/docs.yml must exist at {docs_workflow_path}"
    )

    content = docs_workflow_path.read_text(encoding="utf-8")

    # Workflow must execute build_playground_bundle.py
    assert "build_playground_bundle.py" in content, (
        ".github/workflows/docs.yml must run scripts/build_playground_bundle.py"
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
