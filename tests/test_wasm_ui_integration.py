"""Tests for Spanglings Rust WebAssembly browser playground and arcade UI integration.

Validates `docs/assets/playground/playground.js`, `storage.js`, and `pkg/spanglings.js`,
verifying WebAssembly lifecycle initialization, catalog extraction, exercise evaluation
routing through `evaluate_exercise_wasm`, arcade showdown evaluation routing through
`evaluate_arcade_choice_wasm` with speed bonus scoring, SM-2 spaced repetition calculations,
and resilient fallback when WebAssembly is unavailable.
"""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
from typing import Any

import pytest


@pytest.fixture(scope="module")
def repo_root() -> Path:
    """Returns repository root directory."""
    return Path(__file__).resolve().parent.parent


@pytest.fixture(scope="module")
def playground_js_path(repo_root: Path) -> Path:
    """Returns path to playground.js."""
    return repo_root / "docs" / "assets" / "playground" / "playground.js"


@pytest.fixture(scope="module")
def playground_css_path(repo_root: Path) -> Path:
    """Returns path to playground.css."""
    return repo_root / "docs" / "assets" / "playground" / "playground.css"


@pytest.fixture(scope="module")
def storage_js_path(repo_root: Path) -> Path:
    """Returns path to storage.js."""
    return repo_root / "docs" / "assets" / "playground" / "storage.js"


@pytest.fixture(scope="module")
def bundle_path(repo_root: Path) -> Path:
    """Returns path to playground-bundle.json."""
    return repo_root / "docs" / "assets" / "playground" / "playground-bundle.json"


@pytest.fixture(scope="module")
def pkg_dir(repo_root: Path) -> Path:
    """Returns path to pkg directory."""
    return repo_root / "docs" / "assets" / "playground" / "pkg"


def run_node_wasm_ui_eval(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
    script: str,
) -> dict[str, Any]:
    """Helper to execute JavaScript in Node.js with simulated browser/Wasm environment."""
    js_pkg_path = (pkg_dir / "spanglings.js").resolve().as_uri()
    wasm_bin_path = (pkg_dir / "spanglings_bg.wasm").resolve().as_posix()

    runner_code = f"""
    import fs from 'node:fs';

    // Mock localStorage for headless Node environment
    class MockLocalStorage {{
        constructor() {{
            this.store = {{}};
        }}
        getItem(key) {{
            return Object.prototype.hasOwnProperty.call(this.store, key) ? this.store[key] : null;
        }}
        setItem(key, value) {{
            this.store[key] = String(value);
        }}
        removeItem(key) {{
            delete this.store[key];
        }}
        clear() {{
            this.store = {{}};
        }}
    }}

    class MockClassList {{
        constructor(element) {{
            this._el = element;
            this._classes = new Set();
        }}
        add(...tokens) {{
            tokens.forEach(t => this._classes.add(t));
            this._sync();
        }}
        remove(...tokens) {{
            tokens.forEach(t => this._classes.delete(t));
            this._sync();
        }}
        toggle(token) {{
            const has = this._classes.has(token);
            if (has) this._classes.delete(token);
            else this._classes.add(token);
            this._sync();
            return !has;
        }}
        contains(token) {{
            return this._classes.has(token);
        }}
        _sync() {{
            this._el._className = Array.from(this._classes).join(' ');
        }}
    }}

    function createMockElement(tag = 'DIV', id = '') {{
        const el = {{
            tagName: tag.toUpperCase(),
            id: id,
            _className: '',
            dataset: {{}},
            style: {{}},
            children: [],
            appendChild(child) {{ this.children.push(child); }},
            setAttribute(name, val) {{ this['_' + name] = String(val); }},
            getAttribute(name) {{ return this['_' + name] !== undefined ? this['_' + name] : null; }},
            addEventListener(ev, fn) {{
                this._listeners = this._listeners || {{}};
                this._listeners[ev] = this._listeners[ev] || [];
                this._listeners[ev].push(fn);
            }},
            dispatchEvent(ev) {{
                const fns = (this._listeners && this._listeners[ev.type]) || [];
                fns.forEach(fn => fn(ev));
            }},
            innerHTML: '',
            textContent: '',
            querySelector(sel) {{
                if (sel.startsWith('.')) {{
                    const cls = sel.slice(1);
                    return this.classList.contains(cls) ? this : null;
                }}
                if (sel.startsWith('#')) {{
                    const targetId = sel.slice(1);
                    return this.id === targetId ? this : null;
                }}
                return null;
            }},
            querySelectorAll() {{ return []; }},
        }};
        el.classList = new MockClassList(el);
        Object.defineProperty(el, 'className', {{
            get() {{ return this._className; }},
            set(v) {{
                this._className = v;
                this.classList._classes = new Set((v || '').split(/\\s+/).filter(Boolean));
            }}
        }});
        return el;
    }}

    const elementsRegistry = new Map();
    const mockDocumentElement = createMockElement('HTML');
    mockDocumentElement.setAttribute('data-theme', 'dark');

    globalThis.localStorage = new MockLocalStorage();
    globalThis.window = globalThis;
    globalThis.window.addEventListener = (ev, fn) => {{
        globalThis.window._listeners = globalThis.window._listeners || {{}};
        globalThis.window._listeners[ev] = globalThis.window._listeners[ev] || [];
        globalThis.window._listeners[ev].push(fn);
    }};

    globalThis.document = {{
        documentElement: mockDocumentElement,
        body: createMockElement('BODY'),
        getElementById(id) {{
            if (!elementsRegistry.has(id)) {{
                const el = createMockElement('DIV', id);
                elementsRegistry.set(id, el);
            }}
            return elementsRegistry.get(id);
        }},
        querySelector(sel) {{
            if (sel.startsWith('#')) return this.getElementById(sel.slice(1));
            return null;
        }},
        querySelectorAll() {{ return []; }},
        createElement: (tag) => createMockElement(tag),
        addEventListener: () => {{}},
    }};

    import initWasmPkg, * as wasmRawExports from '{js_pkg_path}';
    import {{ SpanglingsStorage }} from '{storage_js_path.as_uri()}';
    import * as playgroundModule from '{playground_js_path.as_uri()}';

    const wasmBytes = fs.readFileSync('{wasm_bin_path}');
    const bundleData = JSON.parse(fs.readFileSync('{bundle_path.as_posix()}', 'utf-8'));

    async function main() {{
        {script}
    }}

    main().then(res => {{
        console.log(JSON.stringify({{ success: true, result: res }}));
    }}).catch(err => {{
        console.error(JSON.stringify({{ success: false, error: err.message, stack: err.stack }}));
        process.exit(1);
    }});
    """

    res = subprocess.run(
        ["node", "--input-type=module", "-e", runner_code],
        capture_output=True,
        text=True,
        check=False,
    )

    if res.returncode != 0:
        pytest.fail(
            f"Node execution failed with code {res.returncode}:\nSTDERR:\n{res.stderr}\nSTDOUT:\n{res.stdout}"
        )

    try:
        output_json = json.loads(res.stdout.strip())
        return output_json
    except json.JSONDecodeError as exc:
        pytest.fail(
            f"Failed to decode JSON from node output:\n{res.stdout}\nError: {exc}"
        )


def test_wasm_bootstrap_and_status_pill(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
) -> None:
    """Verify WebAssembly initialization, wasmReady flag, and status pill transitions."""
    script = """
    const storage = new SpanglingsStorage('test_wasm_boot_key');
    storage.resetAll();

    const app = new playgroundModule.SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    // 1. Initial state before Wasm init
    const initialWasmReady = playgroundModule.isWasmReady ? playgroundModule.isWasmReady() : false;

    // 2. Initialize Wasm with binary bytes
    const initSuccess = await playgroundModule.initWasm({ module_or_path: wasmBytes });
    const readyAfterInit = playgroundModule.isWasmReady ? playgroundModule.isWasmReady() : false;

    // 3. Inspect Status Pill
    const pill = document.getElementById('playground-status-pill');
    const dot = document.getElementById('status-dot');
    const label = document.getElementById('status-label');

    return {
        initialWasmReady,
        initSuccess,
        readyAfterInit,
        dotClass: dot ? dot.className : '',
        labelText: label ? label.textContent : '',
        pillClass: pill ? pill.className : ''
    };
    """
    out = run_node_wasm_ui_eval(
        playground_js_path, storage_js_path, bundle_path, pkg_dir, script
    )
    res = out["result"]

    assert res["initSuccess"] is True
    assert res["readyAfterInit"] is True
    assert "status-ready" in res["dotClass"]
    assert "Rust Wasm" in res["labelText"] or "Active" in res["labelText"]


def test_wasm_curriculum_catalog_population(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
) -> None:
    """Verify populating curriculum catalog using get_curriculum_catalog_json()."""
    script = """
    await playgroundModule.initWasm({ module_or_path: wasmBytes });
    const catalog = playgroundModule.getCurriculumCatalog ? playgroundModule.getCurriculumCatalog() : null;

    return {
        hasCatalog: catalog !== null,
        count: catalog ? (catalog.count || (catalog.exercises ? catalog.exercises.length : 0)) : 0,
        firstExercise: catalog && catalog.exercises ? catalog.exercises[0] : null
    };
    """
    out = run_node_wasm_ui_eval(
        playground_js_path, storage_js_path, bundle_path, pkg_dir, script
    )
    res = out["result"]

    assert res["hasCatalog"] is True
    assert res["count"] >= 100
    first = res["firstExercise"]
    assert first is not None
    assert "id" in first
    assert "meaning" in first or "plain_english" in first
    assert "rule" in first or "explanation" in first


def test_wasm_exercise_evaluation_dual_layer(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
) -> None:
    """Verify exercise evaluation routing through evaluate_exercise_wasm with dual-layer feedback."""
    script = """
    await playgroundModule.initWasm({ module_or_path: wasmBytes });

    const frame = {
        id: 'b0_ser_estar_basics_01',
        topic: 'ser-estar',
        template: 'Yo ____ estudiante.',
        target: 'soy',
        solution: 'soy',
        explanation: 'Ser is used for identity and professions.',
        plain_english: 'Points to inherent identity or profession.'
    };

    // 1. Correct submission
    const evalCorrect = playgroundModule.evaluateExercise(frame, 'soy', 'Strict');

    // 2. Incorrect submission with diagnostic code
    const evalWrong = playgroundModule.evaluateExercise(frame, 'estoy', 'Strict');

    return { evalCorrect, evalWrong };
    """
    out = run_node_wasm_ui_eval(
        playground_js_path, storage_js_path, bundle_path, pkg_dir, script
    )
    res = out["result"]

    # Correct
    corr = res["evalCorrect"]
    assert corr["isValid"] is True or corr.get("is_correct") is True
    assert len(corr["meaning"]) > 0 or len(corr.get("plain_english", "")) > 0
    assert len(corr["rule"]) > 0 or len(corr.get("explanation", "")) > 0

    # Incorrect
    wrong = res["evalWrong"]
    assert wrong["isValid"] is False or wrong.get("is_correct") is False
    assert len(wrong["meaning"]) > 0 or len(wrong.get("plain_english", "")) > 0
    assert len(wrong["rule"]) > 0 or len(wrong.get("explanation", "")) > 0


def test_wasm_arcade_evaluation_and_speed_scoring(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
) -> None:
    """Verify arcade evaluation routing through evaluate_arcade_choice_wasm."""
    script = """
    await playgroundModule.initWasm({ module_or_path: wasmBytes });

    const item = {
        id: 'ser-estar_0',
        topic: 'ser-estar',
        trigger_sentence: 'El servidor principal ____ fuera de servicio por mantenimiento.',
        options: ['está', 'es'],
        correct_index: 0,
        correct_option: 'está',
        meaning: 'Describes temporary condition, physical location, or immediate operational state.',
        rule: "'Estar' marks a temporary condition or operational state."
    };

    // 1. Fast correct choice (400ms)
    const fastRes = playgroundModule.evaluateArcadeChoice(item, 0, 400);

    // 2. Incorrect choice (1200ms)
    const wrongRes = playgroundModule.evaluateArcadeChoice(item, 1, 1200);

    return { fastRes, wrongRes };
    """
    out = run_node_wasm_ui_eval(
        playground_js_path, storage_js_path, bundle_path, pkg_dir, script
    )
    res = out["result"]

    # Fast correct
    fast = res["fastRes"]
    assert fast["isCorrect"] is True
    assert fast["totalScore"] > 100
    assert fast["speedBonus"] > 0
    assert len(fast["meaning"]) > 0
    assert len(fast["rule"]) > 0

    # Wrong
    wrong = res["wrongRes"]
    assert wrong["isCorrect"] is False
    assert wrong["totalScore"] == 0
    assert wrong["speedBonus"] == 0
    assert len(wrong["meaning"]) > 0
    assert len(wrong["rule"]) > 0


def test_wasm_sm2_spaced_repetition_integration(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
) -> None:
    """Verify SM-2 algorithm execution via calculate_sm2_review_wasm."""
    script = """
    await playgroundModule.initWasm({ module_or_path: wasmBytes });

    const storage = new SpanglingsStorage('test_wasm_sm2_key');
    storage.resetAll();

    // Perform review with grade 5 (perfect recall)
    const res5 = storage.updateSrs('b0_ser_estar_basics_01', 5);

    // Perform review with grade 1 (blackout)
    const res1 = storage.updateSrs('b0_ser_estar_basics_02', 1);

    return { res5, res1 };
    """
    out = run_node_wasm_ui_eval(
        playground_js_path, storage_js_path, bundle_path, pkg_dir, script
    )
    res = out["result"]

    assert res["res5"]["repetitions"] >= 1
    assert res["res5"]["ease_factor"] >= 2.5
    assert res["res1"]["repetitions"] == 0
    assert res["res1"]["interval_days"] == 1


def test_wasm_fallback_mode_when_unavailable(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    pkg_dir: Path,
) -> None:
    """Verify resilient fallback when WebAssembly fails to initialize (e.g. file:// protocol)."""
    script = """
    const storage = new SpanglingsStorage('test_wasm_fallback_key');
    storage.resetAll();

    const app = new playgroundModule.SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    // Simulate failed Wasm init (e.g. invalid bytes)
    let failedInitResult = false;
    try {
        failedInitResult = await playgroundModule.initWasm({ module_or_path: new Uint8Array([0, 1, 2, 3]) });
    } catch {
        failedInitResult = false;
    }

    const wasmReady = playgroundModule.isWasmReady ? playgroundModule.isWasmReady() : false;

    // Evaluate exercise using fallback JS implementation
    const frame = bundleData.frames[0];
    const fallbackEval = playgroundModule.evaluateExercise(frame, frame.target, 'Forgiving');

    // Evaluate arcade choice using fallback JS implementation
    const arcadeItem = bundleData.arcade_items[0];
    const fallbackArcade = playgroundModule.evaluateArcadeChoice(arcadeItem, arcadeItem.correct_index, 500);

    const pill = document.getElementById('playground-status-pill');
    const label = document.getElementById('status-label');

    return {
        failedInitResult,
        wasmReady,
        fallbackEvalValid: fallbackEval.isValid,
        fallbackArcadeCorrect: fallbackArcade.isCorrect,
        labelText: label ? label.textContent : '',
    };
    """
    out = run_node_wasm_ui_eval(
        playground_js_path, storage_js_path, bundle_path, pkg_dir, script
    )
    res = out["result"]

    assert res["wasmReady"] is False
    assert res["fallbackEvalValid"] is True
    assert res["fallbackArcadeCorrect"] is True
