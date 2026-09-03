"""Tests for the Spanglings Interactive Split-Pane Workspace & Monaco Editor.

Validates `docs/assets/playground/playground.css` and `docs/assets/playground/playground.js`,
verifying layout styles, developer-grade slate theme, zero animation disruptions,
progressive 3-tier hints, accent toolbar math, exercise evaluation across accent modes,
and storage synchronization.
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any

import pytest


@pytest.fixture(scope="module")
def repo_root() -> Path:
    """Returns repository root directory."""
    return Path(__file__).resolve().parent.parent


@pytest.fixture(scope="module")
def playground_css_path(repo_root: Path) -> Path:
    """Returns path to playground.css."""
    return repo_root / "docs" / "assets" / "playground" / "playground.css"


@pytest.fixture(scope="module")
def playground_js_path(repo_root: Path) -> Path:
    """Returns path to playground.js."""
    return repo_root / "docs" / "assets" / "playground" / "playground.js"


@pytest.fixture(scope="module")
def storage_js_path(repo_root: Path) -> Path:
    """Returns path to storage.js."""
    return repo_root / "docs" / "assets" / "playground" / "storage.js"


@pytest.fixture(scope="module")
def bundle_path(repo_root: Path) -> Path:
    """Returns path to playground-bundle.json."""
    return repo_root / "docs" / "assets" / "playground" / "playground-bundle.json"


def run_node_playground_eval(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    script: str,
) -> dict[str, Any]:
    """Helper to execute JavaScript in Node.js importing playground.js and dependencies."""
    runner_code = f"""
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
    globalThis.window.__currentMonacoTheme = 'vs-dark';
    globalThis.window.monaco = {{
        editor: {{
            setTheme(th) {{
                globalThis.window.__currentMonacoTheme = th;
            }},
            create(mount, opts) {{
                const inst = {{
                    _opts: opts,
                    getValue() {{ return this._val || opts.value || ''; }},
                    setValue(v) {{ this._val = v; }},
                    layout() {{ globalThis.window.__layoutCalled = (globalThis.window.__layoutCalled || 0) + 1; }},
                    addCommand() {{}},
                    getSelection() {{ return {{}}; }},
                    executeEdits() {{}},
                    focus() {{}},
                }};
                globalThis.window.__lastMonacoInstance = inst;
                return inst;
            }},
        }},
        KeyMod: {{ CtrlCmd: 2048 }},
        KeyCode: {{ Enter: 3 }},
    }};

    globalThis.MutationObserver = class MockMutationObserver {{
        constructor(callback) {{
            this.callback = callback;
            this.target = null;
            this.options = null;
            globalThis.__lastMutationObserver = this;
        }}
        observe(target, options) {{
            this.target = target;
            this.options = options;
        }}
        disconnect() {{}}
        trigger(mutations) {{
            if (this.callback) this.callback(mutations);
        }}
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

    import * as fs from 'fs';
    import {{ SpanglingsStorage }} from '{storage_js_path.as_uri()}';
    import {{
        ACCENT_CHARS,
        normalizeSpanish,
        generateProgressiveHint,
        insertAccentAtCursor,
        evaluateExercise,
        buildSyllabusModel,
        SpanglingsPlaygroundApp,
        syncMonacoTheme,
        initThemeObserver,
        updateStatusPill,
    }} from '{playground_js_path.as_uri()}';

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


def test_playground_css_exists_and_conforms_to_spec(
    playground_css_path: Path,
) -> None:
    """Verify that playground.css exists, defines slate palette, and has zero shake animations."""
    assert playground_css_path.exists(), (
        f"playground.css missing at {playground_css_path}"
    )
    content = playground_css_path.read_text(encoding="utf-8")
    assert len(content) > 200, "playground.css is too short"

    # Key layout selectors
    assert ".playground-container" in content or ".spanglings-playground" in content
    assert ".syllabus-pane" in content or "#syllabus-pane" in content
    assert ".editor-pane" in content or "#editor-pane" in content
    assert ".diagnostics-pane" in content or "#diagnostics-pane" in content
    assert ".accent-toolbar" in content or ".accent-btn" in content
    assert ".dual-layer-card" in content

    # Zero sound/shake/flashing constraint
    assert "@keyframes shake" not in content.lower()
    assert "@keyframes flash" not in content.lower()


def test_playground_css_kubelings_theme_variables_and_standalone_layout(
    playground_css_path: Path,
) -> None:
    """Verify Kubelings CSS variable tokens, light/slate theme selectors, and standalone rules."""
    content = playground_css_path.read_text(encoding="utf-8")

    # Required Kubelings CSS variables
    required_vars = [
        "--pg-bg",
        "--pg-card-bg",
        "--pg-sidebar-bg",
        "--pg-header-bg",
        "--pg-border",
        "--pg-border-focus",
        "--pg-text",
        "--pg-text-muted",
        "--pg-accent",
        "--pg-accent-hover",
        "--pg-accent-fg",
        "--pg-btn-bg",
        "--pg-btn-hover",
        "--pg-btn-active",
        "--pg-btn-text",
        "--pg-term-bg",
        "--pg-term-header-bg",
        "--pg-term-text",
        "--pg-term-border",
        "--pg-term-dim",
        "--pg-success-bg",
        "--pg-error-bg",
        "--pg-warning-bg",
        "--pg-info-bg",
        "--pg-radius",
        "--pg-shadow",
    ]
    for var_name in required_vars:
        assert var_name in content, f"Missing Kubelings CSS variable {var_name}"

    # Theme selectors
    assert '[data-md-color-scheme="slate"]' in content
    assert 'html[data-theme="dark"]' in content
    assert 'html[data-theme="light"]' in content or ":root" in content

    # Standalone navigation and edge-to-edge layout rules
    assert "#standalone-header" in content
    assert "#standalone-playground-root" in content
    assert ".spanglings-playground" in content
    assert ".playground-split-layout" in content or ".playground-body" in content

    # Status pill with pulsing dot
    assert ".playground-status-pill" in content or ".status-dot" in content
    assert "@keyframes pg-pulse" in content
    assert ".status-loading" in content
    assert ".status-ready" in content
    assert ".status-running" in content
    assert ".status-error" in content

    # Action toolbar & rounded buttons
    assert ".playground-btn" in content
    assert ".playground-btn-primary" in content
    assert ".playground-btn-kbd" in content

    # Gradient progress fill
    assert "linear-gradient" in content


def test_playground_js_exists(playground_js_path: Path) -> None:
    """Verify that playground.js exists and is non-empty."""
    assert playground_js_path.exists(), f"playground.js missing at {playground_js_path}"
    content = playground_js_path.read_text(encoding="utf-8")
    assert len(content) > 300, "playground.js is too short"


def test_accent_characters_constant(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify ACCENT_CHARS array contains all standard Spanish diacritics and punctuation."""
    script = """
    return { accents: ACCENT_CHARS };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    accents = out["result"]["accents"]
    expected = ["á", "é", "í", "ó", "ú", "ñ", "ü", "¿", "¡"]
    for ch in expected:
        assert ch in accents, f"Missing accent character: {ch}"


def test_insert_accent_at_cursor_math(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify insertAccentAtCursor correctly updates text content and selection indices."""
    script = """
    // 1. Insert at end of text
    const r1 = insertAccentAtCursor('est', 3, 3, 'á');
    // 2. Insert at start of text
    const r2 = insertAccentAtCursor('que', 0, 0, '¿');
    // 3. Replace 'u' with 'ü' in 'verguenza'
    const r3 = insertAccentAtCursor('verguenza', 4, 5, 'ü');
    // 4. Replace selection range
    const r4 = insertAccentAtCursor('hola mundo', 5, 10, 'amigo');

    return { r1, r2, r3, r4 };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["r1"] == {"text": "está", "selectionStart": 4, "selectionEnd": 4}
    assert res["r2"] == {"text": "¿que", "selectionStart": 1, "selectionEnd": 1}
    assert res["r3"] == {"text": "vergüenza", "selectionStart": 5, "selectionEnd": 5}
    assert res["r4"] == {
        "text": "hola amigo",
        "selectionStart": 10,
        "selectionEnd": 10,
    }


def test_progressive_3tier_hints(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify 3-tier progressive hint generation for sentence frames."""
    script = """
    const frame = bundleData.frames.find(f => f.id === 'subjunctive-06') || bundleData.frames[0];
    const hint0 = generateProgressiveHint(frame, 0);
    const hint1 = generateProgressiveHint(frame, 1);
    const hint2 = generateProgressiveHint(frame, 2);
    const hint3 = generateProgressiveHint(frame, 3);

    return { frameId: frame.id, target: frame.target, hint0, hint1, hint2, hint3 };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["hint0"] is None or res["hint0"] == ""
    # Tier 1: Grammatical category and target cue
    assert "tier" in res["hint1"] and res["hint1"]["tier"] == 1
    assert "Grammar Cue" in res["hint1"]["title"] or "Tier 1" in res["hint1"]["title"]
    assert len(res["hint1"]["content"]) > 0

    # Tier 2: Mental model communicative context
    assert res["hint2"]["tier"] == 2
    assert (
        "Communicative Context" in res["hint2"]["title"]
        or "Tier 2" in res["hint2"]["title"]
    )
    assert len(res["hint2"]["content"]) > 0

    # Tier 3: Non-spoiling structural pattern (first/last letter pattern mask)
    assert res["hint3"]["tier"] == 3
    assert (
        "Structural Pattern" in res["hint3"]["title"]
        or "Tier 3" in res["hint3"]["title"]
    )
    pattern = res["hint3"]["pattern"]
    target = res["target"]
    assert pattern.startswith(target[0])
    assert pattern.endswith(target[-1])
    assert str(len(target)) in res["hint3"]["content"] or "_" in pattern


def test_exercise_evaluation_modes(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify evaluation across Strict, Forgiving, and Off accent modes."""
    script = """
    const frame = {
        id: 'test-subjunctive-05',
        topic: 'subjunctive',
        template: 'Ojalá nos ____ una oportunidad.',
        target: 'dé',
        explanation: 'dar in present subjunctive (dé)',
        plain_english: 'Expresses hope or wish.'
    };

    // 1. Exact match in Strict mode
    const exactStrict = evaluateExercise(frame, 'dé', 'Strict');
    
    // 2. Missing accent in Strict mode
    const missingStrict = evaluateExercise(frame, 'de', 'Strict');

    // 3. Missing accent in Forgiving mode
    const missingForgiving = evaluateExercise(frame, 'de', 'Forgiving');

    // 4. Exact in Forgiving mode
    const exactForgiving = evaluateExercise(frame, 'dé', 'Forgiving');

    // 5. Wrong word in any mode
    const wrongWord = evaluateExercise(frame, 'tuviera', 'Forgiving');

    // 6. Full sentence submission with target replaced
    const fullSentence = evaluateExercise(frame, 'Ojalá nos dé una oportunidad.', 'Strict');

    // 7. Full sentence submission missing accent in Forgiving
    const fullSentenceForgiving = evaluateExercise(frame, 'Ojalá nos de una oportunidad.', 'Forgiving');

    return {
        exactStrict,
        missingStrict,
        missingForgiving,
        exactForgiving,
        wrongWord,
        fullSentence,
        fullSentenceForgiving
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    # 1. Exact strict
    assert res["exactStrict"]["isValid"] is True
    assert res["exactStrict"]["score"] == 100
    assert res["exactStrict"]["accentWarning"] is None

    # 2. Missing strict
    assert res["missingStrict"]["isValid"] is False
    assert res["missingStrict"]["accentError"] is True
    assert "accent" in res["missingStrict"]["feedback"].lower()

    # 3. Missing forgiving -> valid with helpful reminder
    assert res["missingForgiving"]["isValid"] is True
    assert res["missingForgiving"]["score"] == 100
    assert res["missingForgiving"]["accentWarning"] is not None
    assert (
        "accent" in res["missingForgiving"]["accentWarning"].lower()
        or "dé" in res["missingForgiving"]["accentWarning"]
    )

    # 4. Exact forgiving
    assert res["exactForgiving"]["isValid"] is True
    assert res["exactForgiving"]["accentWarning"] is None

    # 5. Wrong word
    assert res["wrongWord"]["isValid"] is False
    assert (
        "meaning" in res["wrongWord"] or "plain_english" in res["wrongWord"]
    )  # dual-layer feedback
    assert "rule" in res["wrongWord"] or "explanation" in res["wrongWord"]

    # 6. Full sentence exact
    assert res["fullSentence"]["isValid"] is True

    # 7. Full sentence forgiving
    assert res["fullSentenceForgiving"]["isValid"] is True
    assert res["fullSentenceForgiving"]["accentWarning"] is not None


def test_build_syllabus_model_groups_and_progress(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify buildSyllabusModel correctly groups all 24 topics and computes progress."""
    script = """
    const storage = new SpanglingsStorage('test_syllabus_key');
    const state = storage.load();
    // Complete 2 exercises in subjunctive
    storage.recordReview('subjunctive', 'subjunctive-01', 4, true);
    storage.recordReview('subjunctive', 'subjunctive-02', 4, true);

    const syllabus = buildSyllabusModel(bundleData, storage);
    return { syllabus, totalTopics: syllabus.length };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["totalTopics"] == 24
    subjunctive_topic = next(
        (t for t in res["syllabus"] if t["slug"] == "subjunctive"), None
    )
    assert subjunctive_topic is not None
    assert subjunctive_topic["completedCount"] == 2
    assert subjunctive_topic["totalCount"] >= 2
    assert len(subjunctive_topic["frames"]) >= 2
    assert subjunctive_topic["frames"][0]["isCompleted"] is True
    assert subjunctive_topic["frames"][1]["isCompleted"] is True


def test_playground_app_state_lifecycle(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify SpanglingsPlaygroundApp initialization, exercise selection, submission, and reset."""
    script = """
    const storage = new SpanglingsStorage('test_app_lifecycle_key');
    storage.resetAll();

    const app = new SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    // 1. Initial exercise should be first available frame
    const initialFrame = app.getCurrentFrame();
    
    // 2. Select specific exercise
    app.selectExercise('subjunctive-06');
    const selectedFrame = app.getCurrentFrame();

    // 3. Submit valid answer
    const evalResult = app.submitExercise('tuviera');
    const isCompletedAfter = storage.load().completed_exercises.includes('subjunctive-06');

    // 4. Request hint progression
    const hint1 = app.nextHint();
    const hint2 = app.nextHint();
    const hint3 = app.nextHint();

    // 5. Reset exercise
    app.resetCurrentExercise();

    return {
        initialId: initialFrame.id,
        selectedId: selectedFrame.id,
        evalValid: evalResult.isValid,
        isCompletedAfter,
        hint1Tier: hint1.tier,
        hint2Tier: hint2.tier,
        hint3Tier: hint3.tier,
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["initialId"] is not None
    assert res["selectedId"] == "subjunctive-06"
    assert res["evalValid"] is True
    assert res["isCompletedAfter"] is True
    assert res["hint1Tier"] == 1
    assert res["hint2Tier"] == 2
    assert res["hint3Tier"] == 3


def test_monaco_theme_sync_and_mutation_observer(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify Monaco editor theme synchronization with data-theme/data-md-color-scheme mutations."""
    script = """
    // 1. Initial dark theme
    document.documentElement.setAttribute('data-theme', 'dark');
    const t1 = syncMonacoTheme();
    const monacoTheme1 = globalThis.window.__currentMonacoTheme;

    // 2. Switch to light theme
    document.documentElement.setAttribute('data-theme', 'light');
    const t2 = syncMonacoTheme();
    const monacoTheme2 = globalThis.window.__currentMonacoTheme;

    // 3. Switch via MkDocs slate color scheme
    document.documentElement.setAttribute('data-theme', '');
    document.documentElement.setAttribute('data-md-color-scheme', 'slate');
    const t3 = syncMonacoTheme();
    const monacoTheme3 = globalThis.window.__currentMonacoTheme;

    // 4. Test MutationObserver integration
    let observerCallbackTheme = null;
    const observer = initThemeObserver((isDark, monacoTheme) => {
        observerCallbackTheme = monacoTheme;
    });

    // Simulate mutation trigger
    document.documentElement.setAttribute('data-theme', 'light');
    if (globalThis.__lastMutationObserver) {
        globalThis.__lastMutationObserver.trigger([
            { type: 'attributes', attributeName: 'data-theme' }
        ]);
    }

    const observerTriggeredTheme = observerCallbackTheme;

    return {
        t1, monacoTheme1,
        t2, monacoTheme2,
        t3, monacoTheme3,
        observerTriggeredTheme
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["t1"] == "vs-dark"
    assert res["monacoTheme1"] == "vs-dark"
    assert res["t2"] == "vs"
    assert res["monacoTheme2"] == "vs"
    assert res["t3"] == "vs-dark"
    assert res["monacoTheme3"] == "vs-dark"
    assert res["observerTriggeredTheme"] == "vs"


def test_status_pill_lifecycle_state_management(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify status pill transitions across loading, ready, running, and error states."""
    script = """
    const storage = new SpanglingsStorage('test_status_pill_key');
    const app = new SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    // Setup DOM elements for status pill
    const pill = document.getElementById('playground-status-pill');
    const dot = document.getElementById('status-dot');
    const label = document.getElementById('status-label');

    // 1. Loading state
    updateStatusPill('loading', 'Loading Wasm runtime...');
    const loadingClass = dot.className;
    const loadingText = label.textContent;

    // 2. Ready state
    updateStatusPill('ready', 'Ready');
    const readyClass = dot.className;
    const readyText = label.textContent;

    // 3. Running state
    updateStatusPill('running', 'Evaluating submission...');
    const runningClass = dot.className;
    const runningText = label.textContent;

    // 4. Error state
    updateStatusPill('error', 'Runtime compilation error');
    const errorClass = dot.className;
    const errorText = label.textContent;

    // 5. App instance method integration
    app.updateStatusPill('ready', 'Engine Ready');
    const appReadyText = label.textContent;

    return {
        loadingClass, loadingText,
        readyClass, readyText,
        runningClass, runningText,
        errorClass, errorText,
        appReadyText
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert "status-loading" in res["loadingClass"]
    assert res["loadingText"] == "Loading Wasm runtime..."

    assert "status-ready" in res["readyClass"]
    assert res["readyText"] == "Ready"

    assert "status-running" in res["runningClass"]
    assert res["runningText"] == "Evaluating submission..."

    assert "status-error" in res["errorClass"]
    assert res["errorText"] == "Runtime compilation error"

    assert res["appReadyText"] == "Engine Ready"


def test_fullscreen_toggle_and_monaco_relayout(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify fullscreen toggle compatibility with standalone root and Monaco relayout."""
    script = """
    const storage = new SpanglingsStorage('test_fullscreen_key');
    const app = new SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    const root = document.getElementById('standalone-playground-root');
    const container = document.getElementById('spanglings-app');

    // Attach mock monaco editor
    app.monacoEditor = globalThis.window.monaco.editor.create(null, {});

    // Toggle fullscreen on
    app.toggleFullscreen();
    const isFullscreen1 = root.classList.contains('fullscreen');

    // Toggle fullscreen off
    document.fullscreenElement = root;
    app.toggleFullscreen();
    const isFullscreen2 = root.classList.contains('fullscreen');

    return {
        isFullscreen1,
        isFullscreen2,
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["isFullscreen1"] is True
    assert res["isFullscreen2"] is False


def test_playground_js_handles_url_query_params(repo_root: Path) -> None:
    """Verify playground.js parses ?mode=arcade, ?topic=, and ?exercise= query params."""
    js_path = repo_root / "docs" / "assets" / "playground" / "playground.js"
    content = js_path.read_text(encoding="utf-8")

    assert "URLSearchParams" in content
    assert "applyUrlParams" in content or "parseUrlParams" in content
    assert "manual/#" in content or "../manual/" in content


def test_standalone_header_links_to_manual(repo_root: Path) -> None:
    """Verify docs/playground/index.html header links to ../manual/."""
    html_path = repo_root / "docs" / "playground" / "index.html"
    content = html_path.read_text(encoding="utf-8")

    assert 'href="../manual/"' in content
    assert "Language Manual" in content


def test_playground_url_query_param_evaluation(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify SpanglingsPlaygroundApp.applyUrlParams handles mode, topic, and exercise params in Node."""
    script = """
    const storage = new SpanglingsStorage('test_url_params_key');
    const app = new SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    // 1. Test curriculum topic deep-linking (?topic=subjunctive)
    app.applyUrlParams('?topic=subjunctive');
    const frameAfterTopic = app.getCurrentFrame();
    const modeAfterTopic = app.currentMode;

    // 2. Test exercise deep-linking (?exercise=ser-estar-02)
    app.applyUrlParams('?exercise=ser-estar-02');
    const frameAfterExercise = app.getCurrentFrame();

    // 3. Test arcade mode deep-linking (?mode=arcade&topic=ser-estar)
    app.applyUrlParams('?mode=arcade&topic=ser-estar');
    const modeAfterArcade = app.currentMode;
    const arcadeTopicMode = app.arcadeEngine.mode;

    return {
        topicFrameTopic: frameAfterTopic ? frameAfterTopic.topic : null,
        modeAfterTopic,
        exerciseId: frameAfterExercise ? frameAfterExercise.id : null,
        modeAfterArcade,
        arcadeTopicMode,
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["topicFrameTopic"] == "subjunctive"
    assert res["modeAfterTopic"] == "curriculum"
    assert res["exerciseId"] == "ser-estar-02"
    assert res["modeAfterArcade"] == "arcade"
    assert res["arcadeTopicMode"] == "ser-estar"
