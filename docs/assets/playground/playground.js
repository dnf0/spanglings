/**
 * Spanglings Interactive Split-Pane Workspace & Monaco Editor Controller
 *
 * Implements the full client-side curriculum workspace, Monaco editor integration,
 * accent insertion toolbar, progressive 3-tier hints, live compiler diagnostics,
 * dual-layer pedagogical feedback cards, and state persistence synchronization.
 *
 * @module SpanglingsPlayground
 */

import { SpanglingsStorage } from "./storage.js";

/**
 * Standard Spanish diacritics and inverted punctuation helper characters.
 * @type {string[]}
 */
export const ACCENT_CHARS = ["á", "é", "í", "ó", "ú", "ñ", "ü", "¿", "¡"];

/**
 * Monaco Editor CDN base path.
 * @type {string}
 */
export const MONACO_CDN_BASE =
  "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs";

/**
 * Normalizes Spanish text for comparison and diacritic handling.
 *
 * @param {string} str - Raw input string.
 * @param {boolean} [stripAccents=false] - Whether to strip combining diacritics.
 * @returns {string} Normalized string.
 */
export function normalizeSpanish(str, stripAccents = false) {
  if (!str || typeof str !== "string") {
    return "";
  }
  const trimmed = str.trim();
  if (stripAccents) {
    return trimmed
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .toLowerCase();
  }
  return trimmed;
}

/**
 * Calculates string replacement and new cursor position for accent insertion.
 *
 * @param {string} text - Current full text.
 * @param {number} selectionStart - Start offset of selection/cursor.
 * @param {number} selectionEnd - End offset of selection/cursor.
 * @param {string} char - Accent character to insert.
 * @returns {{ text: string, selectionStart: number, selectionEnd: number }}
 */
export function insertAccentAtCursor(text, selectionStart, selectionEnd, char) {
  const str = typeof text === "string" ? text : "";
  const start =
    typeof selectionStart === "number"
      ? Math.max(0, Math.min(selectionStart, str.length))
      : str.length;
  const end =
    typeof selectionEnd === "number"
      ? Math.max(start, Math.min(selectionEnd, str.length))
      : start;

  const before = str.slice(0, start);
  const after = str.slice(end);
  const updated = before + char + after;
  const newPos = start + char.length;

  return {
    text: updated,
    selectionStart: newPos,
    selectionEnd: newPos,
  };
}

/**
 * Extracts target answer from user submission (supporting both target-only and full sentence input).
 *
 * @param {string} template - Sentence frame template containing '____'.
 * @param {string} userInput - Raw user input.
 * @returns {string} Extracted answer candidate.
 */
export function extractAnswerFromSubmission(template, userInput) {
  if (!userInput || typeof userInput !== "string") {
    return "";
  }
  const rawInput = userInput.trim();
  if (!template || !template.includes("____")) {
    return rawInput;
  }

  // Create regex pattern from template, replacing slot variables and blank
  const escaped = template.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const withSlots = escaped.replace(/\\\{[a-zA-Z0-9_]+\\\}/g, ".*?");
  const regexPattern =
    "^" +
    withSlots.replace(
      /____/g,
      "([a-zA-ZáéíóúÁÉÍÓÚñÑüÜ]+(?:\\s+[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ]+)?)"
    ) +
    "[.?!]?$";

  try {
    const rx = new RegExp(regexPattern, "i");
    const match = rawInput.match(rx);
    if (match && match[1]) {
      return match[1].trim();
    }
  } catch {
    // Fall back to direct input if regex fails
  }

  // Also check simple prefix/suffix slice if slots weren't matched
  const blankIdx = template.indexOf("____");
  if (blankIdx !== -1) {
    const prefix = template.slice(0, blankIdx).trim();
    if (rawInput.startsWith(prefix) && rawInput.length > prefix.length) {
      let remainder = rawInput.slice(prefix.length).trim();
      // Remove trailing punctuation or suffix words if template ends
      const suffix = template.slice(blankIdx + 4).trim();
      const firstSuffixWord = suffix.split(/\s+/)[0];
      if (firstSuffixWord && remainder.includes(firstSuffixWord)) {
        remainder = remainder.slice(0, remainder.indexOf(firstSuffixWord)).trim();
      }
      if (remainder) {
        return remainder;
      }
    }
  }

  return rawInput;
}

/**
 * Generates a progressive 3-tier pedagogical hint for a sentence frame.
 *
 * - Tier 1: Grammatical category and formula cue.
 * - Tier 2: Mental model communicative context.
 * - Tier 3: First and last letter structural pattern mask (non-spoiling).
 *
 * @param {object} frame - Sentence frame object.
 * @param {number} tier - Hint tier index (0 = none, 1, 2, 3).
 * @returns {object|null} Hint descriptor or null if tier is 0.
 */
export function generateProgressiveHint(frame, tier) {
  if (!frame || typeof tier !== "number" || tier <= 0) {
    return null;
  }

  const target = (frame.target || "").trim();

  if (tier === 1) {
    const cue =
      frame.formula_cue ||
      `Verb: ${frame.target_verb || "conjugation"} | Subject: ${frame.target_subject || "pronoun"}`;
    return {
      tier: 1,
      title: "💡 Tier 1: Grammar Cue",
      content: cue,
    };
  }

  if (tier === 2) {
    const meaning =
      frame.meaning ||
      frame.plain_english ||
      "Focus on the communicative nuance, intent, and contextual trigger of the sentence.";
    return {
      tier: 2,
      title: "🧠 Tier 2: Communicative Context",
      content: meaning,
    };
  }

  if (tier >= 3) {
    let pattern = "";
    if (target.length <= 1) {
      pattern = target;
    } else if (target.length === 2) {
      pattern = `${target[0]} _`;
    } else {
      pattern = `${target[0]} ${"_ ".repeat(target.length - 2)}${target[target.length - 1]}`;
    }

    return {
      tier: 3,
      title: "📐 Tier 3: Structural Pattern",
      pattern: pattern.trim(),
      content: `Target is a ${target.length}-letter word matching: ${pattern.trim()}`,
    };
  }

  return null;
}

/**
 * Evaluates user exercise submission across Forgiving, Strict, and Off accent modes.
 *
 * @param {object} frame - Sentence frame object.
 * @param {string} userInput - User input text from Monaco editor or input box.
 * @param {string} [accentMode="Forgiving"] - Accent strictness mode ("Forgiving"|"Strict"|"Off").
 * @returns {object} Evaluation result object.
 */
export function evaluateExercise(frame, userInput, accentMode = "Forgiving") {
  if (!frame) {
    return {
      isValid: false,
      score: 0,
      feedback: "No active exercise selected.",
      accentError: false,
      accentWarning: null,
      meaning: "",
      rule: "",
      expected: "",
      actual: "",
    };
  }

  const target = (frame.target || "").trim();
  const rawInput = (userInput || "").trim();
  const extractedAnswer = extractAnswerFromSubmission(frame.template, rawInput);
  const answer = extractedAnswer || rawInput;

  const exactMatch = answer === target;
  const foldedTarget = normalizeSpanish(target, true);
  const foldedAnswer = normalizeSpanish(answer, true);
  const foldMatch = foldedAnswer === foldedTarget;

  const meaning = frame.meaning || frame.plain_english || "";
  const rule = frame.rule || frame.explanation || "";

  // 1. Exact match
  if (exactMatch) {
    return {
      isValid: true,
      score: 100,
      feedback: "✓ CORRECT!",
      accentError: false,
      accentWarning: null,
      meaning,
      rule,
      expected: target,
      actual: answer,
    };
  }

  // 2. Fold match (spelling matches without accents)
  if (foldMatch) {
    if (accentMode === "Strict") {
      return {
        isValid: false,
        score: 0,
        feedback: `✗ Accent mismatch: Expected '${target}', got '${answer}'. Strict mode requires exact diacritics.`,
        accentError: true,
        accentWarning: null,
        meaning,
        rule,
        expected: target,
        actual: answer,
      };
    }

    if (accentMode === "Off") {
      return {
        isValid: true,
        score: 100,
        feedback: "✓ CORRECT!",
        accentError: false,
        accentWarning: null,
        meaning,
        rule,
        expected: target,
        actual: answer,
      };
    }

    // Default: Forgiving mode -> Valid with helpful reminder warning
    return {
      isValid: true,
      score: 100,
      feedback: "✓ CORRECT!",
      accentError: false,
      accentWarning: `ℹ Note: Remember the accent mark on '${target}' (you typed '${answer}').`,
      meaning,
      rule,
      expected: target,
      actual: answer,
    };
  }

  // 3. Incorrect answer
  return {
    isValid: false,
    score: 0,
    feedback: `✗ INCORRECT. Expected '${target}', got '${answer}'.`,
    accentError: false,
    accentWarning: null,
    meaning,
    rule,
    expected: target,
    actual: answer,
  };
}

/**
 * Builds the syllabus hierarchy model, grouping exercises by 24 topics with progress metrics.
 *
 * @param {object} bundleData - Loaded playground bundle JSON.
 * @param {SpanglingsStorage} storage - SpanglingsStorage instance.
 * @returns {Array<object>} Syllabus topics array with frames and metrics.
 */
export function buildSyllabusModel(bundleData, storage) {
  if (!bundleData || !Array.isArray(bundleData.topics)) {
    return [];
  }

  const allFrames = Array.isArray(bundleData.frames) ? bundleData.frames : [];
  const state = storage ? storage.getState() : { completed_exercises: [], srs: {} };
  const completedSet = new Set(state.completed_exercises || []);

  return bundleData.topics.map((topic) => {
    const topicFrames = allFrames.filter((f) => f.topic === topic.slug);
    let completedCount = 0;

    const decoratedFrames = topicFrames.map((frame) => {
      const isCompleted = completedSet.has(frame.id);
      if (isCompleted) {
        completedCount += 1;
      }
      const isDue = storage && typeof storage.isDueForReview === "function"
        ? storage.isDueForReview(frame.id)
        : false;

      return {
        ...frame,
        isCompleted,
        isDue,
      };
    });

    const totalCount = topicFrames.length;
    const percent = totalCount > 0 ? Math.round((completedCount / totalCount) * 100) : 0;

    return {
      ...topic,
      totalCount,
      completedCount,
      percent,
      isAllCompleted: totalCount > 0 && completedCount === totalCount,
      frames: decoratedFrames,
    };
  });
}

/**
 * Main Controller class for the Spanglings Interactive Playground.
 */
export class SpanglingsPlaygroundApp {
  /**
   * @param {object} options
   * @param {object} options.bundle - Pre-loaded playground bundle JSON data.
   * @param {SpanglingsStorage} [options.storage] - SpanglingsStorage persistence engine.
   * @param {string} [options.containerId="spanglings-app"] - Mount root ID.
   */
  constructor(options = {}) {
    this.bundle = options.bundle || null;
    this.storage = options.storage || new SpanglingsStorage();
    this.containerId = options.containerId || "spanglings-app";

    this.currentExerciseId = null;
    this.currentHintTier = 0;
    this.currentMode = "curriculum";
    this.accentMode = "Forgiving";
    this.searchQuery = "";
    this.collapsedTopics = new Set();
    this.monacoEditor = null;
    this.isMonacoReady = false;

    // Load initial accent mode from state if present
    const savedState = this.storage.getState();
    if (savedState && savedState.accent_mode) {
      this.accentMode = savedState.accent_mode;
    }

    if (this.bundle) {
      this._selectInitialExercise();
    }
  }

  /**
   * Selects the initial exercise based on current_exercise in storage or first uncompleted frame.
   * @private
   */
  _selectInitialExercise() {
    const state = this.storage.getState();
    const allFrames = this.bundle.frames || [];

    if (state.current_exercise && allFrames.some((f) => f.id === state.current_exercise)) {
      this.currentExerciseId = state.current_exercise;
      return;
    }

    const firstUncompleted = allFrames.find(
      (f) => !state.completed_exercises.includes(f.id)
    );
    this.currentExerciseId = firstUncompleted ? firstUncompleted.id : (allFrames[0] ? allFrames[0].id : null);
  }

  /**
   * Returns the current sentence frame object.
   * @returns {object|null}
   */
  getCurrentFrame() {
    if (!this.bundle || !this.bundle.frames || !this.currentExerciseId) {
      return null;
    }
    return this.bundle.frames.find((f) => f.id === this.currentExerciseId) || null;
  }

  /**
   * Selects an exercise by its ID.
   *
   * @param {string} exerciseId - Sentence frame ID (e.g. "subjunctive-01").
   */
  selectExercise(exerciseId) {
    if (!this.bundle || !this.bundle.frames) return;
    const frame = this.bundle.frames.find((f) => f.id === exerciseId);
    if (!frame) return;

    this.currentExerciseId = exerciseId;
    this.currentHintTier = 0;

    // Update storage current_exercise
    const state = this.storage.getState();
    state.current_exercise = exerciseId;
    this.storage.save();

    this.render();
  }

  /**
   * Advances to the next progressive hint tier (up to tier 3).
   * @returns {object|null}
   */
  nextHint() {
    const frame = this.getCurrentFrame();
    if (!frame) return null;

    this.currentHintTier = Math.min(3, this.currentHintTier + 1);
    const hint = generateProgressiveHint(frame, this.currentHintTier);
    this.renderHintView(hint);
    return hint;
  }

  /**
   * Submits the current exercise input for evaluation.
   *
   * @param {string} [inputOverride=null] - Optional user input string (if testing headless).
   * @returns {object} Evaluation result.
   */
  submitExercise(inputOverride = null) {
    const frame = this.getCurrentFrame();
    if (!frame) {
      return { isValid: false, feedback: "No exercise selected." };
    }

    let input = inputOverride;
    if (input === null) {
      input = this.getEditorValue();
    }

    const result = evaluateExercise(frame, input, this.accentMode);

    if (result.isValid) {
      // Record in storage
      this.storage.recordReview(frame.topic, frame.id, 4, true, this.currentHintTier);
    } else {
      this.storage.updateSrs(frame.id, 1);
      if (frame.topic) {
        this.storage.updateConceptMastery(frame.topic, 1);
      }
    }

    this.renderDiagnosticsView(result);
    this.renderSyllabusView();
    return result;
  }

  /**
   * Advances to the next uncompleted exercise.
   */
  nextExercise() {
    if (!this.bundle || !this.bundle.frames) return;
    const allFrames = this.bundle.frames;
    const currentIdx = allFrames.findIndex((f) => f.id === this.currentExerciseId);
    const state = this.storage.getState();

    // Look for next uncompleted frame after current
    let nextFrame = allFrames.slice(currentIdx + 1).find((f) => !state.completed_exercises.includes(f.id));
    if (!nextFrame) {
      // Wrap around
      nextFrame = allFrames.find((f) => !state.completed_exercises.includes(f.id));
    }
    if (!nextFrame && allFrames.length > 0) {
      // Next sequential if all completed
      nextFrame = allFrames[(currentIdx + 1) % allFrames.length];
    }

    if (nextFrame) {
      this.selectExercise(nextFrame.id);
    }
  }

  /**
   * Resets the current exercise editor to its template blank state.
   */
  resetCurrentExercise() {
    const frame = this.getCurrentFrame();
    if (!frame) return;

    this.currentHintTier = 0;
    this.setEditorValue(frame.template || "");
    this.renderDiagnosticsView(null);
    this.renderHintView(null);
  }

  /**
   * Resets all progress with user confirmation.
   */
  resetAll() {
    this.storage.resetAll();
    this.currentHintTier = 0;
    this._selectInitialExercise();
    this.render();
    this.showToast("All progress has been reset to defaults.", "info");
  }

  /**
   * Exports progress JSON file.
   */
  exportJson() {
    const jsonStr = this.storage.exportJson();
    const dateStr = new Date().toISOString().slice(0, 10);
    const filename = `spanglings-state-${dateStr}.json`;

    if (typeof window !== "undefined" && typeof document !== "undefined") {
      const blob = new Blob([jsonStr], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      this.showToast(`Exported progress to ${filename}`, "success");
    }
    return jsonStr;
  }

  /**
   * Imports a progress JSON backup.
   *
   * @param {string} jsonString - JSON backup string.
   * @returns {boolean}
   */
  importJson(jsonString) {
    const success = this.storage.importJson(jsonString);
    if (success) {
      this._selectInitialExercise();
      this.render();
      this.showToast("Progress successfully imported and merged!", "success");
    } else {
      this.showToast("Failed to import state: Invalid JSON format.", "error");
    }
    return success;
  }

  /**
   * Inserts an accent character at the current editor cursor.
   *
   * @param {string} char - Accent character.
   */
  insertAccent(char) {
    if (this.monacoEditor) {
      const selection = this.monacoEditor.getSelection();
      const op = {
        range: selection,
        text: char,
        forceMoveMarkers: true,
      };
      this.monacoEditor.executeEdits("accent-bar", [op]);
      this.monacoEditor.focus();
    } else {
      const textarea = document.getElementById("fallback-textarea");
      if (textarea) {
        const start = textarea.selectionStart;
        const end = textarea.selectionEnd;
        const updated = insertAccentAtCursor(textarea.value, start, end, char);
        textarea.value = updated.text;
        textarea.selectionStart = updated.selectionStart;
        textarea.selectionEnd = updated.selectionEnd;
        textarea.focus();
      }
    }
  }

  /**
   * Gets current editor value.
   * @returns {string}
   */
  getEditorValue() {
    if (this.monacoEditor) {
      return this.monacoEditor.getValue();
    }
    const textarea = document.getElementById("fallback-textarea");
    return textarea ? textarea.value : "";
  }

  /**
   * Sets editor text.
   * @param {string} val
   */
  setEditorValue(val) {
    if (this.monacoEditor) {
      this.monacoEditor.setValue(val);
    }
    const textarea = document.getElementById("fallback-textarea");
    if (textarea) {
      textarea.value = val;
    }
  }

  /**
   * Toggles fullscreen layout.
   */
  toggleFullscreen() {
    const container = document.getElementById(this.containerId);
    if (!container) return;

    if (!document.fullscreenElement) {
      container.classList.add("fullscreen");
      if (container.requestFullscreen) {
        container.requestFullscreen().catch(() => {});
      }
    } else {
      container.classList.remove("fullscreen");
      if (document.exitFullscreen) {
        document.exitFullscreen().catch(() => {});
      }
    }
  }

  /**
   * Displays temporary toast notification.
   *
   * @param {string} msg
   * @param {"success"|"error"|"info"} [type="info"]
   */
  showToast(msg, type = "info") {
    if (typeof document === "undefined") return;
    let toast = document.getElementById("playground-toast");
    if (!toast) {
      toast = document.createElement("div");
      toast.id = "playground-toast";
      toast.className = "playground-toast";
      document.body.appendChild(toast);
    }

    toast.textContent = msg;
    toast.className = `playground-toast show toast-${type}`;
    setTimeout(() => {
      toast.classList.remove("show");
    }, 3200);
  }

  /**
   * Asynchronously loads playground bundle from disk or URL.
   *
   * @param {string} [url="assets/playground/playground-bundle.json"]
   */
  async loadBundle(url = "assets/playground/playground-bundle.json") {
    try {
      const res = await fetch(url);
      if (!res.ok) throw new Error(`HTTP ${res.status} fetching bundle`);
      this.bundle = await res.json();
      this._selectInitialExercise();
      this.render();
    } catch (err) {
      console.error("Failed to load playground bundle:", err);
      this.showToast("Failed to load playground bundle.", "error");
    }
  }

  /**
   * Main render lifecycle.
   */
  render() {
    if (typeof document === "undefined") return;
    const container = document.getElementById(this.containerId);
    if (!container) return;

    if (!container.dataset.mounted) {
      this._mountSkeleton(container);
      container.dataset.mounted = "true";
      this._initMonacoOrFallback();
    }

    this.renderHeader();
    this.renderSyllabusView();
    this.renderEditorPrompt();
    this.renderReferenceCard();
  }

  /**
   * Mounts HTML skeleton structure into container.
   * @private
   */
  _mountSkeleton(container) {
    container.className = "playground-container";
    container.innerHTML = `
      <header class="playground-header">
        <div class="header-brand">
          <span>⚡ Spanglings</span>
          <span class="brand-badge">WASM Playground</span>
        </div>
        <div class="mode-switcher">
          <button id="mode-curriculum-btn" class="mode-btn active">📚 Curriculum Workspace</button>
          <button id="mode-arcade-btn" class="mode-btn">⚡ Rapid Arcade Arena</button>
        </div>
        <div class="header-actions">
          <select id="accent-mode-select" class="accent-mode-select" title="Accent Evaluation Strictness">
            <option value="Forgiving">Accents: Forgiving</option>
            <option value="Strict">Accents: Strict</option>
            <option value="Off">Accents: Off</option>
          </select>
          <button id="export-json-btn" class="header-btn" title="Export progress JSON">⬇ Export</button>
          <button id="import-json-btn" class="header-btn" title="Import progress JSON">⬆ Import</button>
          <input type="file" id="import-file-input" style="display: none;" accept=".json" />
          <button id="reset-all-btn" class="header-btn" title="Reset All Progress">⚠ Reset All</button>
          <button id="fullscreen-btn" class="header-btn" title="Toggle Fullscreen">⛶</button>
        </div>
      </header>

      <div class="playground-body">
        <aside class="syllabus-pane" id="syllabus-pane">
          <div class="syllabus-search-box">
            <input type="text" id="syllabus-search-input" class="syllabus-search-input" placeholder="Search 24 topics & concepts..." />
          </div>
          <div class="syllabus-overall-progress" id="syllabus-overall-progress">
            <div class="progress-labels">
              <span>Curriculum Progress</span>
              <span id="overall-progress-text">0%</span>
            </div>
            <div class="progress-bar-track">
              <div class="progress-bar-fill" id="overall-progress-fill"></div>
            </div>
          </div>
          <ul class="syllabus-tree" id="syllabus-tree"></ul>
        </aside>

        <main class="editor-pane" id="editor-pane">
          <div class="exercise-prompt-bar" id="exercise-prompt-bar">
            <div class="prompt-title-row">
              <span class="prompt-topic-name" id="prompt-topic-name">SUBJUNCTIVE</span>
              <span class="prompt-id" id="prompt-exercise-id" style="font-family: var(--font-mono); font-size: 0.75rem; color: var(--text-dim);"></span>
            </div>
            <div class="prompt-cue" id="prompt-cue">Fill in the missing Spanish verb form:</div>
            <div class="prompt-formula" id="prompt-formula"></div>
          </div>

          <div class="accent-toolbar">
            <span class="accent-label">Accents:</span>
            ${ACCENT_CHARS.map((ch) => `<button class="accent-btn" data-char="${ch}">${ch}</button>`).join("")}
          </div>

          <div class="editor-container" id="editor-container">
            <div id="monaco-editor-mount"></div>
            <textarea id="fallback-textarea" class="fallback-editor" style="display: none;" placeholder="Type your answer or fill in the blank..."></textarea>
          </div>

          <div class="editor-action-bar">
            <div class="action-left">
              <button id="hint-btn" class="hint-btn">? Hint (Tier 1)</button>
              <button id="reset-exercise-btn" class="header-btn">↺ Reset</button>
            </div>
            <div class="action-right">
              <button id="submit-btn" class="submit-btn">⚡ Submit (Ctrl+Enter)</button>
              <button id="next-btn" class="next-btn" disabled>Next ➔</button>
            </div>
          </div>
        </main>

        <section class="diagnostics-pane" id="diagnostics-pane">
          <div class="diagnostics-header">
            <span>Compiler Diagnostics</span>
          </div>

          <div id="diag-status-container">
            <div class="diag-status ready">○ Ready for compilation</div>
          </div>

          <div id="diag-feedback-container"></div>
          <div id="diag-accent-container"></div>

          <div id="hint-container"></div>

          <div class="dual-layer-card" id="dual-layer-card">
            <div class="card-layer meaning">
              <div class="layer-heading">💡 Meaning / Communicative Context</div>
              <div class="layer-body" id="card-meaning-text">Select an exercise to view pedagogical explanation.</div>
            </div>
            <div class="card-layer rule">
              <div class="layer-heading">📐 Grammar Rule / Structural Law</div>
              <div class="layer-body" id="card-rule-text">Structural conjugation guidelines will appear here.</div>
            </div>
          </div>

          <div class="reference-card-drawer" id="reference-card-drawer">
            <div class="reference-card-header" id="ref-card-header">
              <span>📖 Topic Cheat Sheet</span>
              <span id="ref-card-toggle-icon">▼</span>
            </div>
            <pre class="reference-card-content" id="ref-card-content"></pre>
          </div>
        </section>
      </div>

      <div class="arcade-arena-container" id="arcade-arena-container">
        <h2>⚡ Rapid Arcade Arena</h2>
        <p style="color: var(--text-muted);">Switch to Arcade mode for fast-paced single-key binary showdowns.</p>
      </div>
    `;

    this._bindEvents();
  }

  /**
   * Binds UI events and keyboard shortcuts.
   * @private
   */
  _bindEvents() {
    // Mode switcher
    const currBtn = document.getElementById("mode-curriculum-btn");
    const arcBtn = document.getElementById("mode-arcade-btn");
    const container = document.getElementById(this.containerId);

    if (currBtn && arcBtn) {
      currBtn.addEventListener("click", () => {
        this.currentMode = "curriculum";
        currBtn.classList.add("active");
        arcBtn.classList.remove("active");
        container?.classList.remove("arcade-mode");
      });
      arcBtn.addEventListener("click", () => {
        this.currentMode = "arcade";
        arcBtn.classList.add("active");
        currBtn.classList.remove("active");
        container?.classList.add("arcade-mode");
      });
    }

    // Accent mode selector
    const accentSelect = document.getElementById("accent-mode-select");
    if (accentSelect) {
      accentSelect.value = this.accentMode;
      accentSelect.addEventListener("change", (e) => {
        this.accentMode = e.target.value;
        const state = this.storage.getState();
        state.accent_mode = this.accentMode;
        this.storage.save();
      });
    }

    // Header actions
    document.getElementById("export-json-btn")?.addEventListener("click", () => this.exportJson());
    const fileInput = document.getElementById("import-file-input");
    document.getElementById("import-json-btn")?.addEventListener("click", () => fileInput?.click());
    fileInput?.addEventListener("change", (e) => {
      const file = e.target.files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (evt) => {
          this.importJson(evt.target.result);
          fileInput.value = "";
        };
        reader.readAsText(file);
      }
    });

    document.getElementById("reset-all-btn")?.addEventListener("click", () => {
      if (confirm("Are you sure you want to reset all curriculum and SRS progress?")) {
        this.resetAll();
      }
    });

    document.getElementById("fullscreen-btn")?.addEventListener("click", () => this.toggleFullscreen());

    // Accent toolbar buttons
    document.querySelectorAll(".accent-btn").forEach((btn) => {
      btn.addEventListener("click", (e) => {
        const char = e.currentTarget.getAttribute("data-char");
        if (char) {
          this.insertAccent(char);
        }
      });
    });

    // Editor actions
    document.getElementById("submit-btn")?.addEventListener("click", () => this.submitExercise());
    document.getElementById("next-btn")?.addEventListener("click", () => this.nextExercise());
    document.getElementById("hint-btn")?.addEventListener("click", () => this.nextHint());
    document.getElementById("reset-exercise-btn")?.addEventListener("click", () => this.resetCurrentExercise());

    // Search filter
    const searchInput = document.getElementById("syllabus-search-input");
    searchInput?.addEventListener("input", (e) => {
      this.searchQuery = e.target.value.toLowerCase().trim();
      this.renderSyllabusView();
    });

    // Reference card accordion toggle
    document.getElementById("ref-card-header")?.addEventListener("click", () => {
      const content = document.getElementById("ref-card-content");
      const icon = document.getElementById("ref-card-toggle-icon");
      if (content && icon) {
        const isHidden = content.style.display === "none";
        content.style.display = isHidden ? "block" : "none";
        icon.textContent = isHidden ? "▲" : "▼";
      }
    });

    // Global keyboard shortcut: Ctrl+Enter / Cmd+Enter to submit
    if (typeof window !== "undefined") {
      window.addEventListener("keydown", (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
          e.preventDefault();
          this.submitExercise();
        }
      });
    }
  }

  /**
   * Initializes Monaco Editor from CDN or falls back to standard textarea.
   * @private
   */
  _initMonacoOrFallback() {
    const fallback = document.getElementById("fallback-textarea");
    const monacoMount = document.getElementById("monaco-editor-mount");

    if (typeof window !== "undefined" && window.require) {
      window.require.config({ paths: { vs: MONACO_CDN_BASE } });
      window.require(
        ["vs/editor/editor.main"],
        () => {
          this.monacoEditor = window.monaco.editor.create(monacoMount, {
            value: this.getCurrentFrame()?.template || "",
            language: "markdown",
            theme: "vs-dark",
            fontSize: 14,
            fontFamily: "var(--font-mono)",
            minimap: { enabled: false },
            lineNumbers: "on",
            scrollBeyondLastLine: false,
            wordWrap: "on",
            automaticLayout: true,
          });

          this.monacoEditor.addCommand(
            window.monaco.KeyMod.CtrlCmd | window.monaco.KeyCode.Enter,
            () => this.submitExercise()
          );

          this.isMonacoReady = true;
          if (fallback) fallback.style.display = "none";
        },
        (err) => {
          console.warn("Monaco editor failed to load, using fallback textarea:", err);
          if (fallback) fallback.style.display = "block";
          if (monacoMount) monacoMount.style.display = "none";
        }
      );
    } else {
      // Direct fallback
      if (fallback) {
        fallback.style.display = "block";
        const frame = this.getCurrentFrame();
        fallback.value = frame ? frame.template : "";
      }
      if (monacoMount) {
        monacoMount.style.display = "none";
      }
    }
  }

  /**
   * Renders Header state.
   */
  renderHeader() {
    const select = document.getElementById("accent-mode-select");
    if (select) {
      select.value = this.accentMode;
    }
  }

  /**
   * Renders Syllabus tree view.
   */
  renderSyllabusView() {
    if (!this.bundle) return;
    const syllabus = buildSyllabusModel(this.bundle, this.storage);
    const treeEl = document.getElementById("syllabus-tree");
    if (!treeEl) return;

    let totalAll = 0;
    let completedAll = 0;

    treeEl.innerHTML = "";

    syllabus.forEach((topic) => {
      totalAll += topic.totalCount;
      completedAll += topic.completedCount;

      // Filter by search query
      const matchesTopic = !this.searchQuery || topic.title.toLowerCase().includes(this.searchQuery) || topic.slug.includes(this.searchQuery);
      const matchingFrames = topic.frames.filter((f) => {
        if (!this.searchQuery) return true;
        return (
          f.id.toLowerCase().includes(this.searchQuery) ||
          f.template.toLowerCase().includes(this.searchQuery) ||
          f.formula_cue.toLowerCase().includes(this.searchQuery)
        );
      });

      if (!matchesTopic && matchingFrames.length === 0) {
        return;
      }

      const isCollapsed = this.collapsedTopics.has(topic.slug);
      const groupLi = document.createElement("li");
      groupLi.className = `topic-group ${isCollapsed ? "collapsed" : ""}`;

      const headerDiv = document.createElement("div");
      headerDiv.className = "topic-header";
      headerDiv.innerHTML = `
        <div class="topic-header-left">
          <span class="topic-chevron">▼</span>
          <span class="topic-title" title="${topic.title}">${topic.title}</span>
        </div>
        <span class="topic-count">${topic.completedCount}/${topic.totalCount}</span>
      `;

      headerDiv.addEventListener("click", () => {
        if (this.collapsedTopics.has(topic.slug)) {
          this.collapsedTopics.delete(topic.slug);
        } else {
          this.collapsedTopics.add(topic.slug);
        }
        this.renderSyllabusView();
      });

      const exercisesUl = document.createElement("ul");
      exercisesUl.className = "topic-exercises";

      (matchingFrames.length > 0 ? matchingFrames : topic.frames).forEach((frame) => {
        const itemLi = document.createElement("li");
        const isActive = frame.id === this.currentExerciseId;
        itemLi.className = `exercise-item ${isActive ? "active" : ""} ${frame.isCompleted ? "completed" : ""}`;

        const statusIcon = frame.isCompleted ? "✓" : "○";
        const srsBadgeHtml = frame.isDue ? `<span class="srs-badge">SRS</span>` : "";

        itemLi.innerHTML = `
          <div>
            <span class="status-icon">${statusIcon}</span>
            <span>${frame.id}</span>
          </div>
          ${srsBadgeHtml}
        `;

        itemLi.addEventListener("click", () => this.selectExercise(frame.id));
        exercisesUl.appendChild(itemLi);
      });

      groupLi.appendChild(headerDiv);
      groupLi.appendChild(exercisesUl);
      treeEl.appendChild(groupLi);
    });

    // Update overall progress bar
    const overallPercent = totalAll > 0 ? Math.round((completedAll / totalAll) * 100) : 0;
    const progressFill = document.getElementById("overall-progress-fill");
    const progressText = document.getElementById("overall-progress-text");

    if (progressFill) progressFill.style.width = `${overallPercent}%`;
    if (progressText) progressText.textContent = `${overallPercent}% (${completedAll}/${totalAll})`;
  }

  /**
   * Renders Editor prompt and formula cue.
   */
  renderEditorPrompt() {
    const frame = this.getCurrentFrame();
    if (!frame) return;

    const topicEl = document.getElementById("prompt-topic-name");
    const idEl = document.getElementById("prompt-exercise-id");
    const cueEl = document.getElementById("prompt-cue");
    const formulaEl = document.getElementById("prompt-formula");
    const hintBtn = document.getElementById("hint-btn");
    const nextBtn = document.getElementById("next-btn");

    if (topicEl) topicEl.textContent = (frame.topic || "").toUpperCase();
    if (idEl) idEl.textContent = frame.id;
    if (cueEl) cueEl.textContent = `Fill in the blank: ${frame.template}`;
    if (formulaEl) formulaEl.textContent = `Target cue: ${frame.formula_cue || "Conjugate appropriately"}`;
    if (hintBtn) hintBtn.textContent = `? Hint (Tier ${Math.min(3, this.currentHintTier + 1)})`;

    // Next button state
    const isCompleted = this.storage.isCompleted(frame.id);
    if (nextBtn) {
      nextBtn.disabled = !isCompleted;
    }

    // Set editor template if blank
    this.setEditorValue(frame.template || "");
  }

  /**
   * Renders Diagnostics pane feedback.
   * @param {object|null} result
   */
  renderDiagnosticsView(result) {
    const statusContainer = document.getElementById("diag-status-container");
    const feedbackContainer = document.getElementById("diag-feedback-container");
    const accentContainer = document.getElementById("diag-accent-container");
    const meaningText = document.getElementById("card-meaning-text");
    const ruleText = document.getElementById("card-rule-text");
    const nextBtn = document.getElementById("next-btn");

    if (!result) {
      if (statusContainer) statusContainer.innerHTML = `<div class="diag-status ready">○ Ready for compilation</div>`;
      if (feedbackContainer) feedbackContainer.innerHTML = "";
      if (accentContainer) accentContainer.innerHTML = "";
      return;
    }

    if (result.isValid) {
      if (statusContainer) {
        statusContainer.innerHTML = `<div class="diag-status correct">✓ CORRECT! Score: 100</div>`;
      }
      if (feedbackContainer) {
        feedbackContainer.innerHTML = `<div class="diag-feedback-text" style="color: var(--success-color);">Great job! Submission matched target pattern.</div>`;
      }
      if (accentContainer) {
        if (result.accentWarning) {
          accentContainer.innerHTML = `<div class="diag-accent-warning">${result.accentWarning}</div>`;
        } else {
          accentContainer.innerHTML = "";
        }
      }
      if (nextBtn) nextBtn.disabled = false;
    } else {
      if (statusContainer) {
        statusContainer.innerHTML = `<div class="diag-status incorrect">✗ INCORRECT</div>`;
      }
      if (feedbackContainer) {
        feedbackContainer.innerHTML = `<div class="diag-feedback-text" style="color: var(--error-color);">${result.feedback}</div>`;
      }
      if (accentContainer) {
        accentContainer.innerHTML = "";
      }
    }

    if (meaningText) meaningText.textContent = result.meaning || "Communicative context unavailable.";
    if (ruleText) ruleText.textContent = result.rule || "Grammar rule details unavailable.";
  }

  /**
   * Renders Progressive Hint card.
   * @param {object|null} hint
   */
  renderHintView(hint) {
    const hintContainer = document.getElementById("hint-container");
    if (!hintContainer) return;

    if (!hint) {
      hintContainer.innerHTML = "";
      return;
    }

    const patternHtml = hint.pattern
      ? `<div class="hint-pattern-box">${hint.pattern}</div>`
      : "";

    hintContainer.innerHTML = `
      <div class="hint-tier-card">
        <div class="hint-tier-header">${hint.title}</div>
        <div>${hint.content}</div>
        ${patternHtml}
      </div>
    `;
  }

  /**
   * Renders Reference Card / Cheat Sheet.
   */
  renderReferenceCard() {
    const frame = this.getCurrentFrame();
    if (!frame || !this.bundle || !this.bundle.topics) return;

    const topicObj = this.bundle.topics.find((t) => t.slug === frame.topic);
    const contentEl = document.getElementById("ref-card-content");
    const meaningText = document.getElementById("card-meaning-text");
    const ruleText = document.getElementById("card-rule-text");

    if (topicObj) {
      if (contentEl) {
        contentEl.textContent = topicObj.card || "No cheat sheet available.";
      }
      if (meaningText && !document.querySelector?.(".diag-status.correct")) {
        meaningText.textContent = topicObj.meaning || topicObj.mental_model || "";
      }
      if (ruleText && !document.querySelector?.(".diag-status.correct")) {
        ruleText.textContent = topicObj.rule || topicObj.gloss || "";
      }
    }
  }
}

// Auto-initialize if running in a browser environment with mount element
if (typeof window !== "undefined") {
  window.SpanglingsPlaygroundApp = SpanglingsPlaygroundApp;
  if (typeof document !== "undefined" && typeof window.addEventListener === "function") {
    window.addEventListener("DOMContentLoaded", async () => {
      const appMount = document.getElementById("spanglings-app");
      if (appMount) {
        const app = new SpanglingsPlaygroundApp({ containerId: "spanglings-app" });
        await app.loadBundle();
        window.spanglingsPlayground = app;
      }
    });
  }
}
