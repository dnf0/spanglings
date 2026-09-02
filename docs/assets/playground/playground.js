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
 * 16 Showdown pairs available in the Rapid Arcade Arena.
 * @type {Array<{ slug: string, title: string, topic: string }>}
 */
export const SHOWDOWN_PAIRS = [
  { slug: "ser-estar", title: "Ser vs. Estar", topic: "ser-estar" },
  { slug: "por-para", title: "Por vs. Para", topic: "por-para" },
  { slug: "pret-imp", title: "Preterite vs. Imperfect", topic: "pret-imp" },
  { slug: "subj-ind", title: "Subjunctive vs. Indicative", topic: "subj-ind" },
  { slug: "saber-conocer", title: "Saber vs. Conocer", topic: "saber-conocer" },
  { slug: "pedir-preguntar", title: "Pedir vs. Preguntar", topic: "pedir-preguntar" },
  { slug: "llevar-traer", title: "Llevar vs. Traer", topic: "llevar-traer" },
  { slug: "muy-mucho", title: "Muy vs. Mucho", topic: "muy-mucho" },
  { slug: "bien-bueno", title: "Bien vs. Bueno", topic: "bien-bueno" },
  { slug: "tu-usted", title: "Tú vs. Usted", topic: "tu-usted" },
  { slug: "lo-le", title: "Lo vs. Le (Direct vs. Indirect)", topic: "lo-le" },
  { slug: "haber-estar", title: "Haber vs. Estar (Existence vs Location)", topic: "haber-estar" },
  { slug: "tener-haber", title: "Tener vs. Haber (Possession vs Auxiliary)", topic: "tener-haber" },
  { slug: "ir-irse", title: "Ir vs. Irse (Motion vs Departure)", topic: "ir-irse" },
  { slug: "sino-pero", title: "Sino vs. Pero (Contrast vs Exception)", topic: "sino-pero" },
  { slug: "para-que-porque", title: "Para qué vs. Por qué / Porque", topic: "para-que-porque" },
];

/**
 * 5 Specialized Drill Engines available in the Rapid Arcade Arena.
 * @type {Array<{ slug: string, title: string, topic: string, description: string }>}
 */
export const SPECIALIZED_ENGINES = [
  {
    slug: "regimen",
    title: "Prepositional Regimen Engine",
    topic: "regimen",
    description: "Master verb-bound prepositions (soñar con, depender de, fijarse en).",
  },
  {
    slug: "irregulars",
    title: "High-Frequency Irregulars Engine",
    topic: "irregulars",
    description: "Rapid drill for radical stem-changers and irregular preterite/subjunctive forms.",
  },
  {
    slug: "false-friends",
    title: "False Friends & Cognates Engine",
    topic: "false-friends",
    description: "Avoid deceptive cognates (embarazada, constipado, éxito, realizar).",
  },
  {
    slug: "se-matrix",
    title: "Se Matrix & Reflexive Dynamics",
    topic: "se-matrix",
    description: "Disambiguate reflexive, reciprocal, passive, accidental, and middle 'se'.",
  },
  {
    slug: "connectors",
    title: "Logical Discourse Connectors",
    topic: "connectors",
    description: "Fluid transitions (sin embargo, por lo tanto, a pesar de que, ya que).",
  },
];

/**
 * Single-key hotkey mapping for instant arcade answers.
 * Option 1: 1 / j / J
 * Option 2: 2 / k / K
 * Option 3: 3 / l / L
 * Option 4: 4 / ; / :
 * @type {Record<string, number>}
 */
export const HOTKEY_MAP = {
  "1": 0,
  j: 0,
  J: 0,
  "2": 1,
  k: 1,
  K: 1,
  "3": 2,
  l: 2,
  L: 2,
  "4": 3,
  ";": 3,
  ":": 3,
};

/**
 * Calculates speed bonus points (+1 point per 15ms under 1500ms, max 100).
 *
 * @param {number} responseTimeMs - Response time in milliseconds.
 * @returns {number} Speed bonus points from 0 to 100.
 */
export function calculateSpeedBonus(responseTimeMs) {
  if (typeof responseTimeMs !== "number" || isNaN(responseTimeMs)) {
    return 0;
  }
  if (responseTimeMs <= 0) {
    return 100;
  }
  if (responseTimeMs >= 1500) {
    return 0;
  }
  const diff = 1500 - responseTimeMs;
  return Math.min(100, Math.max(0, Math.floor(diff / 15)));
}

/**
 * Normalizes arcade mode slug or alias (e.g. 'ser-vs-estar' -> 'ser-estar').
 *
 * @param {string} mode - Mode slug or alias.
 * @returns {string} Normalized mode slug.
 */
export function normalizeModeSlug(mode) {
  if (!mode || typeof mode !== "string") return "all";
  const clean = mode.trim().toLowerCase();
  if (clean === "all" || clean === "mixed") return "all";

  const aliases = {
    "ser-vs-estar": "ser-estar",
    "por-vs-para": "por-para",
    "imperfect-vs-preterite": "pret-imp",
    "preterite-vs-imperfect": "pret-imp",
    "subjunctive-vs-indicative": "subj-ind",
    "indicative-vs-subjunctive": "subj-ind",
    "saber-vs-conocer": "saber-conocer",
    "pedir-vs-preguntar": "pedir-preguntar",
    "llevar-vs-traer": "llevar-traer",
    "muy-vs-mucho": "muy-mucho",
    "bien-vs-bueno": "bien-bueno",
    "tu-vs-usted": "tu-usted",
    "lo-vs-le": "lo-le",
    "haber-vs-estar": "haber-estar",
    "tener-vs-haber": "tener-haber",
    "ir-vs-irse": "ir-irse",
    "sino-vs-pero": "sino-pero",
    "para-que-vs-porque": "para-que-porque",
    "para-que-porque": "para-que-porque",
  };
  return aliases[clean] || clean;
}

/**
 * Filters arcade items by selected mode.
 *
 * @param {Array<object>} arcadeItems - Complete arcade items array.
 * @param {string} selectedMode - Selected mode slug (e.g. 'all', 'ser-estar', 'regimen').
 * @returns {Array<object>} Filtered arcade items.
 */
export function filterArcadePool(arcadeItems, selectedMode) {
  if (!Array.isArray(arcadeItems)) return [];
  const target = normalizeModeSlug(selectedMode);
  if (target === "all") {
    return [...arcadeItems];
  }

  return arcadeItems.filter((item) => {
    const itemTopic = (item.topic || "").toLowerCase();
    const itemMode = (item.mode || "").toLowerCase();
    return (
      itemTopic === target ||
      item.id?.toLowerCase().startsWith(target) ||
      itemMode === target
    );
  });
}

/**
 * Evaluates an arcade choice submission, computing score, speed bonus, and dual-layer feedback.
 *
 * @param {object} item - Arcade item object.
 * @param {number} selectedIndex - Index of selected option.
 * @param {number} [responseTimeMs=0] - Elapsed response time in ms.
 * @returns {object} Evaluation descriptor.
 */
export function evaluateArcadeChoice(item, selectedIndex, responseTimeMs = 0) {
  if (!item || typeof selectedIndex !== "number") {
    return {
      isCorrect: false,
      baseScore: 0,
      speedBonus: 0,
      totalScore: 0,
      responseTimeMs: responseTimeMs || 0,
      selectedIndex: -1,
      selectedOption: null,
      correctIndex: item?.correct_index ?? 0,
      correctOption: item?.correct_option || "",
      meaning: item?.meaning || item?.plain_english || "",
      rule: item?.rule || item?.explanation || "",
      triggerSentence: item?.trigger_sentence || item?.template || item?.prompt || "",
    };
  }

  const isCorrect = selectedIndex === item.correct_index;
  const speedBonus = isCorrect ? calculateSpeedBonus(responseTimeMs) : 0;
  const baseScore = isCorrect ? 100 : 0;
  const totalScore = baseScore + speedBonus;
  const options = Array.isArray(item.options) ? item.options : [];
  const selectedOption =
    options[selectedIndex] !== undefined ? options[selectedIndex] : null;

  return {
    isCorrect,
    baseScore,
    speedBonus,
    totalScore,
    responseTimeMs: typeof responseTimeMs === "number" ? Math.round(responseTimeMs) : 0,
    selectedIndex,
    selectedOption,
    correctIndex: item.correct_index,
    correctOption: item.correct_option || (options[item.correct_index] || ""),
    meaning: item.meaning || item.plain_english || "Mental model context unavailable.",
    rule: item.rule || item.explanation || "Grammar rule explanation unavailable.",
    triggerSentence: item.trigger_sentence || item.template || item.prompt || "",
  };
}

/**
 * Spanglings Rapid Arcade Arena Engine.
 * Manages item pools, rapid single-key input, scoring, streak multipliers,
 * dual-layer pedagogical feedback, mistake tracking, and state persistence.
 */
export class SpanglingsArcadeEngine {
  /**
   * @param {object} [options={}]
   * @param {object} [options.bundle=null] - Playground bundle containing arcade_items.
   * @param {SpanglingsStorage} [options.storage=null] - Storage engine.
   * @param {Function} [options.onStateChange=null] - State transition listener.
   */
  constructor(options = {}) {
    this.bundle = options.bundle || null;
    this.storage = options.storage || new SpanglingsStorage();
    this.onStateChange =
      typeof options.onStateChange === "function" ? options.onStateChange : null;

    this.mode = "all";
    this.roundLength = 10;
    this.items = [];
    this.currentIndex = 0;
    this.score = 0;
    this.streak = 0;
    this.bestStreak = 0;
    this.startTime = 0;
    this.responses = [];
    this.missedItems = [];
    this.state = "idle"; // "idle" | "question" | "feedback" | "summary"
    this.lastEvaluation = null;
  }

  /**
   * Starts a new arcade round.
   *
   * @param {string} [mode="all"] - Mode identifier or showdown slug.
   * @param {number} [roundLength=10] - Number of items per round (0 = endless).
   * @param {Array<object>} [itemsOverride=null] - Explicit list of items (e.g. for replay).
   */
  startRound(mode = "all", roundLength = 10, itemsOverride = null) {
    this.mode = mode;
    this.roundLength = typeof roundLength === "number" ? roundLength : 10;

    if (Array.isArray(itemsOverride)) {
      this.items = [...itemsOverride];
    } else {
      const allItems = this.bundle?.arcade_items || [];
      const pool = filterArcadePool(allItems, mode);
      this.items = pool.length > 0 ? [...pool] : [...allItems];

      // Fisher-Yates shuffle
      for (let i = this.items.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [this.items[i], this.items[j]] = [this.items[j], this.items[i]];
      }

      if (this.roundLength > 0 && this.items.length > this.roundLength) {
        this.items = this.items.slice(0, this.roundLength);
      }
    }

    this.currentIndex = 0;
    this.score = 0;
    this.streak = 0;
    this.bestStreak = 0;
    this.responses = [];
    this.missedItems = [];
    this.lastEvaluation = null;

    if (this.items.length > 0) {
      this.state = "question";
      this.startTime =
        typeof performance !== "undefined" ? performance.now() : Date.now();
    } else {
      this.state = "idle";
    }

    this._notify();
  }

  /**
   * Returns current active question item.
   * @returns {object|null}
   */
  getCurrentItem() {
    return this.items[this.currentIndex] || null;
  }

  /**
   * Submits user choice for current question.
   *
   * @param {number} selectedIndex - Chosen option index.
   * @param {number} [responseTimeMsOverride=null] - Optional elapsed time override.
   * @returns {object|null} Evaluation result.
   */
  submitChoice(selectedIndex, responseTimeMsOverride = null) {
    if (this.state !== "question") return null;
    const item = this.getCurrentItem();
    if (!item) return null;

    let timeMs = responseTimeMsOverride;
    if (timeMs === null || typeof timeMs === "undefined") {
      const now =
        typeof performance !== "undefined" ? performance.now() : Date.now();
      timeMs = Math.max(0, now - this.startTime);
    }

    const evalResult = evaluateArcadeChoice(item, selectedIndex, timeMs);
    this.lastEvaluation = evalResult;
    this.score += evalResult.totalScore;

    if (evalResult.isCorrect) {
      this.streak += 1;
      this.bestStreak = Math.max(this.bestStreak, this.streak);
    } else {
      this.streak = 0;
      this.missedItems.push({ item, ...evalResult });
    }

    this.responses.push(evalResult);
    this.state = "feedback";
    this._notify();
    return evalResult;
  }

  /**
   * Advances to next question or concludes round if complete.
   * @returns {object|null} Next item or summary object.
   */
  nextQuestion() {
    if (this.state !== "feedback") return null;

    this.currentIndex += 1;
    if (this.currentIndex < this.items.length) {
      this.state = "question";
      this.lastEvaluation = null;
      this.startTime =
        typeof performance !== "undefined" ? performance.now() : Date.now();
      this._notify();
      return this.getCurrentItem();
    }

    this.state = "summary";
    const summary = this.getSummary();
    if (this.storage && typeof this.storage.recordArcadeSession === "function") {
      this.storage.recordArcadeSession(
        this.score,
        this.bestStreak,
        summary.totalQuestions,
        summary.correctQuestions
      );
    }
    this._notify();
    return summary;
  }

  /**
   * Calculates round summary metrics.
   * @returns {object}
   */
  getSummary() {
    const totalQuestions = this.responses.length;
    const correctQuestions = this.responses.filter((r) => r.isCorrect).length;
    const totalTime = this.responses.reduce(
      (sum, r) => sum + (r.responseTimeMs || 0),
      0
    );
    const accuracy =
      totalQuestions > 0
        ? Number(((correctQuestions / totalQuestions) * 100).toFixed(1))
        : 0;
    const avgResponseTimeMs =
      totalQuestions > 0 ? Math.round(totalTime / totalQuestions) : 0;

    return {
      score: this.score,
      totalQuestions,
      correctQuestions,
      accuracy,
      bestStreak: this.bestStreak,
      avgResponseTimeMs,
      missedItems: [...this.missedItems],
      isPerfect: this.missedItems.length === 0 && totalQuestions > 0,
    };
  }

  /**
   * Replays missed questions only.
   */
  replayMissedItems() {
    if (this.missedItems.length === 0) return;
    const missed = this.missedItems.map((m) => m.item);
    this.startRound(this.mode, missed.length, missed);
  }

  /**
   * Handles keyboard shortcut inputs (1/j/J, 2/k/K, 3/l/L, 4/;/:, Space/Enter).
   *
   * @param {string} key - Event key string.
   * @returns {object|null} Handled action result or null.
   */
  handleKey(key) {
    if (this.state === "question") {
      if (Object.prototype.hasOwnProperty.call(HOTKEY_MAP, key)) {
        const choiceIdx = HOTKEY_MAP[key];
        const item = this.getCurrentItem();
        if (
          item &&
          Array.isArray(item.options) &&
          choiceIdx < item.options.length
        ) {
          return this.submitChoice(choiceIdx);
        }
      }
    } else if (this.state === "feedback") {
      if (key === " " || key === "Enter" || key === "Space") {
        return this.nextQuestion();
      }
    }
    return null;
  }

  /**
   * @private
   */
  _notify() {
    if (this.onStateChange) {
      this.onStateChange(this.state, this);
    }
  }
}

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

    // Initialize Rapid Arcade Arena Engine
    this.arcadeEngine = new SpanglingsArcadeEngine({
      bundle: this.bundle,
      storage: this.storage,
      onStateChange: () => this.renderArcadeView(),
    });

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
      this.arcadeEngine.bundle = this.bundle;
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
    this.renderArcadeView();
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
        this.renderArcadeView();
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

    // Global keyboard shortcuts
    if (typeof window !== "undefined") {
      window.addEventListener("keydown", (e) => {
        if (this.currentMode === "arcade") {
          const activeTag = document.activeElement?.tagName?.toLowerCase();
          if (activeTag === "input" || activeTag === "textarea" || activeTag === "select") {
            return;
          }
          const handled = this.arcadeEngine.handleKey(e.key);
          if (handled !== null) {
            e.preventDefault();
          }
          return;
        }

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

  /**
   * Renders the Rapid Arcade Arena view based on engine state.
   */
  renderArcadeView() {
    if (typeof document === "undefined") return;
    const arcadeContainer = document.getElementById("arcade-arena-container");
    if (!arcadeContainer) return;

    const state = this.arcadeEngine.state;
    const stats = this.storage.getState().arcade_stats || {
      high_score: 0,
      best_streak: 0,
      total_duels: 0,
      accuracy: 0.0,
    };

    if (state === "idle") {
      arcadeContainer.innerHTML = `
        <div class="arcade-setup-card">
          <div class="arcade-setup-header">
            <div class="arcade-title">⚡ Rapid Arcade Arena</div>
            <div class="arcade-subtitle">Single-key binary showdowns & specialized drills. Test muscle memory with instant keyboard feedback.</div>
          </div>

          <div class="arcade-config-grid">
            <div class="arcade-config-group">
              <label for="arcade-mode-select" class="arcade-config-label">Select Drill Arena / Showdown Pair:</label>
              <select id="arcade-mode-select" class="arcade-select">
                <option value="all">⚡ All Drills (Mixed Pool - ${this.bundle?.arcade_items?.length || 262} items)</option>
                <optgroup label="Showdown Duels (16 Pairs)">
                  ${SHOWDOWN_PAIRS.map(
                    (p) =>
                      `<option value="${p.slug}" ${this.arcadeEngine.mode === p.slug ? "selected" : ""}>⚔ ${p.title}</option>`
                  ).join("")}
                </optgroup>
                <optgroup label="Specialized Drill Engines (5 Engines)">
                  ${SPECIALIZED_ENGINES.map(
                    (e) =>
                      `<option value="${e.slug}" ${this.arcadeEngine.mode === e.slug ? "selected" : ""}>🔧 ${e.title}</option>`
                  ).join("")}
                </optgroup>
              </select>
            </div>

            <div class="arcade-config-group">
              <label for="arcade-length-select" class="arcade-config-label">Round Length:</label>
              <select id="arcade-length-select" class="arcade-select">
                <option value="10" ${this.arcadeEngine.roundLength === 10 ? "selected" : ""}>10 Questions (Sprint)</option>
                <option value="20" ${this.arcadeEngine.roundLength === 20 ? "selected" : ""}>20 Questions (Standard)</option>
                <option value="0" ${this.arcadeEngine.roundLength === 0 ? "selected" : ""}>Endless Mode (Continuous)</option>
              </select>
            </div>
          </div>

          <div class="arcade-stats-summary-card">
            <div class="arcade-stat-badge">
              <span class="stat-label">High Score</span>
              <span class="stat-val">🏆 ${stats.high_score}</span>
            </div>
            <div class="arcade-stat-badge">
              <span class="stat-label">Best Streak</span>
              <span class="stat-val">🔥 ${stats.best_streak}</span>
            </div>
            <div class="arcade-stat-badge">
              <span class="stat-label">Total Duels</span>
              <span class="stat-val">⚔ ${stats.total_duels}</span>
            </div>
            <div class="arcade-stat-badge">
              <span class="stat-label">Career Accuracy</span>
              <span class="stat-val">🎯 ${(stats.accuracy * 100).toFixed(1)}%</span>
            </div>
          </div>

          <div class="arcade-start-action">
            <button id="arcade-start-btn" class="arcade-action-btn primary">⚡ Start Arcade Drill</button>
          </div>
        </div>
      `;

      const modeSelect = document.getElementById("arcade-mode-select");
      const lenSelect = document.getElementById("arcade-length-select");
      const startBtn = document.getElementById("arcade-start-btn");

      modeSelect?.addEventListener("change", (e) => {
        this.arcadeEngine.mode = e.target.value;
      });
      lenSelect?.addEventListener("change", (e) => {
        this.arcadeEngine.roundLength = parseInt(e.target.value, 10) || 0;
      });
      startBtn?.addEventListener("click", () => {
        const mode = modeSelect ? modeSelect.value : "all";
        const len = lenSelect ? parseInt(lenSelect.value, 10) : 10;
        this.arcadeEngine.startRound(mode, len);
      });
      return;
    }

    if (state === "question" || state === "feedback") {
      const item = this.arcadeEngine.getCurrentItem();
      if (!item) return;

      const evalRes = this.arcadeEngine.lastEvaluation;
      const isFeedback = state === "feedback" && evalRes !== null;

      const currentNum = this.arcadeEngine.currentIndex + 1;
      const totalNum = this.arcadeEngine.items.length;
      const progressLabel =
        this.arcadeEngine.roundLength === 0
          ? `Question ${currentNum} (Endless)`
          : `Question ${currentNum} of ${totalNum}`;

      const hotkeyTags = [
        { num: "1", alpha: "J" },
        { num: "2", alpha: "K" },
        { num: "3", alpha: "L" },
        { num: "4", alpha: ";" },
      ];

      // Formatted trigger sentence: replace ____ with high-contrast blank element
      let formattedSentence = item.trigger_sentence || item.template || "";
      if (isFeedback && evalRes) {
        const spanClass = evalRes.isCorrect
          ? "blank-filled-correct"
          : "blank-filled-wrong";
        formattedSentence = formattedSentence.replace(
          /____/g,
          `<span class="arcade-blank ${spanClass}">${evalRes.correctOption}</span>`
        );
      } else {
        formattedSentence = formattedSentence.replace(
          /____/g,
          `<span class="arcade-blank">____</span>`
        );
      }

      let feedbackBannerHtml = "";
      if (isFeedback && evalRes) {
        if (evalRes.isCorrect) {
          const speedBonusHtml =
            evalRes.speedBonus > 0
              ? ` <span class="arcade-speed-bonus">+ ${evalRes.speedBonus} speed bonus</span>`
              : "";
          feedbackBannerHtml = `
            <div class="arcade-feedback-banner correct">
              ✓ CORRECT! (+${evalRes.baseScore} base${speedBonusHtml}) [${evalRes.responseTimeMs}ms]
            </div>
          `;
        } else {
          feedbackBannerHtml = `
            <div class="arcade-feedback-banner incorrect">
              ✗ INCORRECT! Expected: <strong>${evalRes.correctOption}</strong> [${evalRes.responseTimeMs}ms]
            </div>
          `;
        }
      }

      let dualLayerHtml = "";
      if (isFeedback && evalRes) {
        dualLayerHtml = `
          <div class="arcade-dual-layer-card">
            <div class="card-layer meaning">
              <div class="layer-heading">💡 Meaning / Context</div>
              <div class="layer-body">${evalRes.meaning}</div>
            </div>
            <div class="card-layer rule">
              <div class="layer-heading">📐 Grammar Rule</div>
              <div class="layer-body">${evalRes.rule}</div>
            </div>
          </div>
        `;
      }

      const options = Array.isArray(item.options) ? item.options : [];

      arcadeContainer.innerHTML = `
        <div class="arcade-card">
          <div class="arcade-hud">
            <div class="hud-left">
              <span class="hud-progress">${progressLabel}</span>
              <span class="hud-topic">${item.prompt_cue || item.topic || "Arcade"}</span>
            </div>
            <div class="hud-right">
              <span class="hud-score">🏆 ${this.arcadeEngine.score} pts</span>
              <span class="arcade-streak-badge ${this.arcadeEngine.streak >= 3 ? "on-fire" : ""}">
                🔥 Streak: ${this.arcadeEngine.streak}
              </span>
            </div>
          </div>

          ${feedbackBannerHtml}

          <div class="arcade-question-card">
            <div class="arcade-trigger-sentence">${formattedSentence}</div>
          </div>

          <div class="arcade-options-grid options-${options.length}">
            ${options
              .map((opt, idx) => {
                let btnStateClass = "";
                if (isFeedback && evalRes) {
                  if (idx === evalRes.selectedIndex) {
                    btnStateClass = evalRes.isCorrect
                      ? "selected-correct"
                      : "selected-wrong";
                  } else if (idx === evalRes.correctIndex) {
                    btnStateClass = "show-correct";
                  }
                }
                const tag = hotkeyTags[idx] || { num: `${idx + 1}`, alpha: "" };
                return `
                  <button class="arcade-choice-btn ${btnStateClass}" data-index="${idx}" ${isFeedback ? "disabled" : ""}>
                    <span class="arcade-hotkey-badge">${tag.num} / ${tag.alpha}</span>
                    <span class="arcade-choice-text">${opt}</span>
                  </button>
                `;
              })
              .join("")}
          </div>

          ${dualLayerHtml}

          <div class="arcade-footer-action">
            ${
              isFeedback
                ? `<button id="arcade-next-btn" class="arcade-action-btn next">Next Question ➔ (Space / Enter)</button>`
                : `<div class="arcade-hotkey-guide">Press <strong>1</strong> / <strong>2</strong> (or <strong>J</strong> / <strong>K</strong>) for rapid single-key input</div>`
            }
          </div>
        </div>
      `;

      if (!isFeedback) {
        arcadeContainer.querySelectorAll(".arcade-choice-btn").forEach((btn) => {
          btn.addEventListener("click", (e) => {
            const idx = parseInt(e.currentTarget.getAttribute("data-index"), 10);
            if (!isNaN(idx)) {
              this.arcadeEngine.submitChoice(idx);
            }
          });
        });
      } else {
        document.getElementById("arcade-next-btn")?.addEventListener("click", () => {
          this.arcadeEngine.nextQuestion();
        });
      }
      return;
    }

    if (state === "summary") {
      const summary = this.arcadeEngine.getSummary();

      let perfectBannerHtml = "";
      if (summary.isPerfect) {
        perfectBannerHtml = `
          <div class="arcade-perfect-banner">
            ✨ Perfect Run! 100% Accuracy — No mistakes to review! ✨
          </div>
        `;
      }

      let mistakesHtml = "";
      if (summary.missedItems.length > 0) {
        mistakesHtml = `
          <div class="arcade-mistakes-card">
            <div class="mistakes-header">
              <span>❌ Review Missed Questions (${summary.missedItems.length})</span>
            </div>
            <div class="mistakes-list">
              ${summary.missedItems
                .map((m, idx) => {
                  return `
                    <div class="arcade-mistake-item">
                      <div class="mistake-sentence">
                        <span class="mistake-idx">${idx + 1}.</span>
                        <span>${m.triggerSentence}</span>
                      </div>
                      <div class="mistake-answers-row">
                        <span class="mistake-label">You chose:</span>
                        <span class="arcade-mistake-wrong">${m.selectedOption || "None"}</span>
                        <span class="mistake-label">Correct:</span>
                        <span class="arcade-mistake-correct">${m.correctOption}</span>
                      </div>
                      <div class="mistake-explanation">
                        <span class="mistake-why">💡 Why:</span>
                        <span>${m.meaning} — ${m.rule}</span>
                      </div>
                    </div>
                  `;
                })
                .join("")}
            </div>
            <div class="mistakes-actions">
              <button id="arcade-replay-missed-btn" class="arcade-action-btn replay">↺ Replay Missed Items Only</button>
            </div>
          </div>
        `;
      }

      arcadeContainer.innerHTML = `
        <div class="arcade-summary-card">
          <div class="arcade-summary-header">
            <div class="summary-title">⚡ Arcade Drill Summary</div>
            <div class="summary-subtitle">Session completed across ${summary.totalQuestions} questions.</div>
          </div>

          <div class="arcade-stats-summary-grid">
            <div class="arcade-stat-box">
              <span class="stat-box-label">Final Score</span>
              <span class="stat-box-val">🏆 ${summary.score}</span>
            </div>
            <div class="arcade-stat-box">
              <span class="stat-box-label">Accuracy</span>
              <span class="stat-box-val">🎯 ${summary.accuracy}%</span>
              <span class="stat-box-sub">(${summary.correctQuestions}/${summary.totalQuestions})</span>
            </div>
            <div class="arcade-stat-box">
              <span class="stat-box-label">Best Streak</span>
              <span class="stat-box-val">🔥 ${summary.bestStreak}</span>
            </div>
            <div class="arcade-stat-box">
              <span class="stat-box-label">Avg Response</span>
              <span class="stat-box-val">⚡ ${summary.avgResponseTimeMs}ms</span>
            </div>
          </div>

          ${perfectBannerHtml}
          ${mistakesHtml}

          <div class="arcade-summary-actions">
            <button id="arcade-play-again-btn" class="arcade-action-btn primary">⚡ Play Again</button>
            <button id="arcade-switch-curriculum-btn" class="arcade-action-btn secondary">📚 Switch to Curriculum Workspace</button>
          </div>
        </div>
      `;

      document
        .getElementById("arcade-replay-missed-btn")
        ?.addEventListener("click", () => {
          this.arcadeEngine.replayMissedItems();
        });

      document
        .getElementById("arcade-play-again-btn")
        ?.addEventListener("click", () => {
          this.arcadeEngine.startRound(
            this.arcadeEngine.mode,
            this.arcadeEngine.roundLength
          );
        });

      document
        .getElementById("arcade-switch-curriculum-btn")
        ?.addEventListener("click", () => {
          this.currentMode = "curriculum";
          document.getElementById("mode-curriculum-btn")?.classList.add("active");
          document.getElementById("mode-arcade-btn")?.classList.remove("active");
          document
            .getElementById(this.containerId)
            ?.classList.remove("arcade-mode");
        });
    }
  }
}

// Auto-initialize if running in a browser environment with mount element
if (typeof window !== "undefined") {
  window.SpanglingsPlaygroundApp = SpanglingsPlaygroundApp;
  window.SpanglingsArcadeEngine = SpanglingsArcadeEngine;
  window.calculateSpeedBonus = calculateSpeedBonus;
  window.normalizeModeSlug = normalizeModeSlug;
  window.filterArcadePool = filterArcadePool;
  window.evaluateArcadeChoice = evaluateArcadeChoice;
  window.SHOWDOWN_PAIRS = SHOWDOWN_PAIRS;
  window.SPECIALIZED_ENGINES = SPECIALIZED_ENGINES;
  window.HOTKEY_MAP = HOTKEY_MAP;

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

