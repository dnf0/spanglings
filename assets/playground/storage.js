/**
 * Spanglings Client-Side State Persistence Engine (SpanglingsStorage)
 *
 * Provides a zero-backend, browser localStorage state management engine that
 * maintains 100% JSON schema parity and algorithmic fidelity with the Rust CLI
 * AppState (`src/core/state.rs`) and PortableStateBackup (`src/cli/commands/sync.rs`).
 *
 * @module SpanglingsStorage
 */

const STORAGE_KEY = "spanglings_state_v1";
const APP_VERSION = "0.5.4";
const MAX_STABILITY_LN = 4.110874; // ln(61.0) for concept mastery stability scaling

/**
 * Creates a clean default AppState object matching Rust AppState::default().
 *
 * @returns {object} Default state conforming to AppState schema.
 */
function createDefaultState() {
  return {
    version: 1,
    completed_exercises: [],
    current_exercise: null,
    accent_mode: "Forgiving",
    srs: {},
    stats: {},
    activity_history: {},
    evaluated_level: null,
    concept_mastery: {},
    tour_completed: false,
    arcade_stats: {
      high_score: 0,
      total_duels: 0,
      best_streak: 0,
      accuracy: 0.0,
    },
  };
}

/**
 * In-memory fallback storage if localStorage is not accessible (e.g. Node tests / private browsing).
 */
class MemoryStorage {
  constructor() {
    this.store = {};
  }
  getItem(key) {
    return Object.prototype.hasOwnProperty.call(this.store, key) ? this.store[key] : null;
  }
  setItem(key, value) {
    this.store[key] = String(value);
  }
  removeItem(key) {
    delete this.store[key];
  }
  clear() {
    this.store = {};
  }
}

const memoryStorageFallback = new MemoryStorage();

/**
 * SpanglingsStorage manages persistence, SRS reviews, concept mastery calculations,
 * and state import/export for the web playground.
 */
class SpanglingsStorage {
  /**
   * @param {string} storageKey - Key used for localStorage persistence.
   */
  constructor(storageKey = STORAGE_KEY) {
    this.storageKey = storageKey;
    this._state = null;
  }

  /**
   * Resolves the active storage backend (localStorage or fallback).
   * @returns {Storage|MemoryStorage}
   */
  _getBackend() {
    try {
      const storage =
        (typeof globalThis !== "undefined" && globalThis.localStorage) ||
        (typeof window !== "undefined" && window.localStorage);
      if (storage) {
        const testKey = "__spanglings_probe__";
        storage.setItem(testKey, "1");
        storage.removeItem(testKey);
        return storage;
      }
    } catch {
      // Access denied / quota exceeded / SecurityError in iframe
    }
    return memoryStorageFallback;
  }

  /**
   * Loads state from storage, normalizes it with defaults, and updates the memory cache.
   *
   * @returns {object} Normalized state object.
   */
  load() {
    const backend = this._getBackend();
    let raw = null;
    try {
      raw = backend.getItem(this.storageKey);
    } catch {
      raw = memoryStorageFallback.getItem(this.storageKey);
    }

    if (!raw) {
      this._state = createDefaultState();
      this.save(this._state);
      return JSON.parse(JSON.stringify(this._state));
    }

    try {
      const parsed = JSON.parse(raw);
      if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
        this._state = createDefaultState();
        this.save(this._state);
        return JSON.parse(JSON.stringify(this._state));
      }

      this._state = this._normalizeState(parsed);
      return JSON.parse(JSON.stringify(this._state));
    } catch {
      this._state = createDefaultState();
      this.save(this._state);
      return JSON.parse(JSON.stringify(this._state));
    }
  }

  /**
   * Ensures all schema fields exist and are of correct types.
   *
   * @param {object} rawState - Raw loaded state object.
   * @returns {object} Normalized state.
   */
  _normalizeState(rawState) {
    const defaultState = createDefaultState();
    const normalized = {
      version: typeof rawState.version === "number" ? rawState.version : defaultState.version,
      completed_exercises: Array.isArray(rawState.completed_exercises)
        ? Array.from(new Set(rawState.completed_exercises))
        : defaultState.completed_exercises,
      current_exercise:
        typeof rawState.current_exercise === "string" ? rawState.current_exercise : null,
      accent_mode:
        rawState.accent_mode === "Strict" || rawState.accent_mode === "Off"
          ? rawState.accent_mode
          : "Forgiving",
      srs: rawState.srs && typeof rawState.srs === "object" ? rawState.srs : {},
      stats: rawState.stats && typeof rawState.stats === "object" ? rawState.stats : {},
      activity_history:
        rawState.activity_history && typeof rawState.activity_history === "object"
          ? rawState.activity_history
          : {},
      evaluated_level:
        rawState.evaluated_level && typeof rawState.evaluated_level === "object"
          ? rawState.evaluated_level
          : null,
      concept_mastery:
        rawState.concept_mastery && typeof rawState.concept_mastery === "object"
          ? rawState.concept_mastery
          : {},
      tour_completed: Boolean(rawState.tour_completed),
      arcade_stats: {
        ...defaultState.arcade_stats,
        ...(rawState.arcade_stats && typeof rawState.arcade_stats === "object"
          ? rawState.arcade_stats
          : {}),
      },
    };
    return normalized;
  }

  /**
   * Saves state to storage.
   *
   * @param {object} [state=null] - Optional state object to save (defaults to cached state).
   */
  save(state = null) {
    if (state !== null) {
      this._state = state;
    }
    if (!this._state) {
      this._state = createDefaultState();
    }
    const backend = this._getBackend();
    try {
      backend.setItem(this.storageKey, JSON.stringify(this._state));
    } catch {
      memoryStorageFallback.setItem(this.storageKey, JSON.stringify(this._state));
    }
  }

  /**
   * Retrieves the current state (loading if necessary).
   *
   * @returns {object} Current state.
   */
  getState() {
    if (!this._state) {
      this.load();
    }
    return this._state;
  }

  /**
   * Checks if an exercise has been marked as completed.
   *
   * @param {string} exerciseId - Exercise unique identifier.
   * @returns {boolean}
   */
  isCompleted(exerciseId) {
    const state = this.getState();
    return state.completed_exercises.includes(exerciseId);
  }

  /**
   * Records activity for a given date in the activity heatmap.
   *
   * @param {string} [dateStr=null] - Date formatted as YYYY-MM-DD (defaults to today UTC).
   */
  recordActivity(dateStr = null) {
    const state = this.getState();
    const dateKey = dateStr || new Date().toISOString().slice(0, 10);
    state.activity_history[dateKey] = (state.activity_history[dateKey] || 0) + 1;
  }

  /**
   * Marks an exercise as completed, updating attempts, completion timestamp, and activity.
   *
   * @param {string} exerciseId - Exercise unique identifier.
   * @param {number} [hintsUsed=0] - Number of hints used during the session.
   * @param {Date} [now=null] - Timestamp of completion.
   */
  markCompleted(exerciseId, hintsUsed = 0, now = null) {
    const state = this.getState();
    if (!state.completed_exercises.includes(exerciseId)) {
      state.completed_exercises.push(exerciseId);
    }

    const timestamp = (now || new Date()).toISOString();
    const currentStat = state.stats[exerciseId] || {
      attempts: 0,
      completed_at: null,
      hints_used: 0,
    };

    state.stats[exerciseId] = {
      attempts: currentStat.attempts + 1,
      completed_at: timestamp,
      hints_used: (currentStat.hints_used || 0) + hintsUsed,
    };

    this.recordActivity(timestamp.slice(0, 10));
    this.save();
  }

  /**
   * Unmarks an exercise completion.
   *
   * @param {string} exerciseId - Exercise unique identifier.
   */
  unmarkCompleted(exerciseId) {
    const state = this.getState();
    state.completed_exercises = state.completed_exercises.filter((id) => id !== exerciseId);
    if (state.stats[exerciseId]) {
      state.stats[exerciseId].completed_at = null;
    }
    this.save();
  }

  /**
   * Marks the interactive tour as completed.
   */
  markTourCompleted() {
    const state = this.getState();
    state.tour_completed = true;
    this.save();
  }

  /**
   * Checks if an exercise is due for spaced-repetition review.
   *
   * @param {string} exerciseId - Exercise unique identifier.
   * @param {Date} [now=null] - Current time.
   * @returns {boolean}
   */
  isDueForReview(exerciseId, now = null) {
    const state = this.getState();
    const item = state.srs[exerciseId];
    if (!item || !item.next_review_due) {
      return false;
    }
    const currentTime = (now || new Date()).getTime();
    const dueTime = new Date(item.next_review_due).getTime();
    return dueTime <= currentTime;
  }

  /**
   * Updates SRS spaced repetition data using the SuperMemo SM-2 algorithm.
   * Parity matched with `src/core/srs.rs::calculate_sm2_review`.
   *
   * @param {string} exerciseId - Exercise unique identifier.
   * @param {number} quality - Rating from 0 (complete blackout) to 5 (perfect recall).
   * @param {Date} [now=null] - Review timestamp.
   * @returns {object} Updated SRS item.
   */
  updateSrs(exerciseId, quality, now = null) {
    const state = this.getState();
    const currentDate = now || new Date();
    const currentIso = currentDate.toISOString();

    const currentItem = state.srs[exerciseId] || {
      repetitions: 0,
      interval_days: 0,
      ease_factor: 2.5,
      next_review_due: currentIso,
      last_reviewed: null,
    };

    const q = Math.max(0, Math.min(5, Math.floor(quality)));
    const qDiff = 5.0 - q;
    let newEf = currentItem.ease_factor + (0.1 - qDiff * (0.08 + qDiff * 0.02));
    if (newEf < 1.3) {
      newEf = 1.3;
    }

    let newReps = 0;
    let newInterval = 1;

    if (q < 3) {
      newReps = 0;
      newInterval = 1;
    } else {
      switch (currentItem.repetitions) {
        case 0:
          newReps = 1;
          newInterval = 1;
          break;
        case 1:
          newReps = 2;
          newInterval = 6;
          break;
        default: {
          newReps = currentItem.repetitions + 1;
          const calculated = Math.round(currentItem.interval_days * newEf);
          newInterval = Math.max(1, Math.min(3650, calculated));
          break;
        }
      }
    }

    const nextDueDate = new Date(currentDate.getTime() + newInterval * 24 * 60 * 60 * 1000);

    const updatedItem = {
      repetitions: newReps,
      interval_days: newInterval,
      ease_factor: Number(newEf.toFixed(4)),
      next_review_due: nextDueDate.toISOString(),
      last_reviewed: currentIso,
    };

    state.srs[exerciseId] = updatedItem;
    this.recordActivity(currentIso.slice(0, 10));
    this.save();
    return updatedItem;
  }

  /**
   * Updates concept mastery tracking matching `src/core/state.rs::update_concept_mastery`.
   *
   * @param {string} conceptId - Concept slug or identifier.
   * @param {number} quality - Rating from 0 to 5.
   * @param {Date} [now=null] - Practice timestamp.
   * @returns {object} Updated ConceptMastery item.
   */
  updateConceptMastery(conceptId, quality, now = null) {
    const state = this.getState();
    const currentDate = now || new Date();
    const currentIso = currentDate.toISOString();

    const entry = state.concept_mastery[conceptId] || {
      concept_id: conceptId,
      mastery_score: 0.0,
      repetitions: 0,
      interval_days: 0,
      ease_factor: 2.5,
      total_reviews: 0,
      lapses: 0,
      last_practiced: null,
    };

    entry.last_practiced = currentIso;
    entry.total_reviews += 1;

    const q = Math.max(0, Math.min(5, Math.floor(quality)));
    const qDiff = 5.0 - q;
    let newEf = entry.ease_factor + (0.1 - qDiff * (0.08 + qDiff * 0.02));
    if (newEf < 1.3) {
      newEf = 1.3;
    }
    entry.ease_factor = Number(newEf.toFixed(4));

    if (q < 3) {
      entry.lapses += 1;
      entry.repetitions = Math.max(0, entry.repetitions - 1);
      switch (entry.repetitions) {
        case 0:
          entry.interval_days = 0;
          break;
        case 1:
          entry.interval_days = 1;
          break;
        case 2:
          entry.interval_days = 6;
          break;
        default: {
          const calc = Math.round(entry.interval_days / entry.ease_factor);
          entry.interval_days = Math.max(0, Math.min(3650, calc));
          break;
        }
      }
    } else {
      entry.repetitions += 1;
      switch (entry.repetitions) {
        case 1:
          entry.interval_days = 1;
          break;
        case 2:
          entry.interval_days = 6;
          break;
        default: {
          const calc = Math.round(entry.interval_days * entry.ease_factor);
          entry.interval_days = Math.max(1, Math.min(3650, calc));
          break;
        }
      }
    }

    if (entry.repetitions === 0 || entry.interval_days === 0) {
      entry.mastery_score = 0.0;
    } else {
      const repFactor = Math.min(1.0, entry.repetitions / 6.0);
      const stabilityFactor = Math.min(
        1.0,
        Math.log(1.0 + entry.interval_days) / MAX_STABILITY_LN
      );
      const easeScale = entry.ease_factor / 2.5;
      const score = repFactor * stabilityFactor * easeScale;
      entry.mastery_score = Number(Math.max(0.0, Math.min(1.0, score)).toFixed(4));
    }

    state.concept_mastery[conceptId] = entry;
    this.save();
    return entry;
  }

  /**
   * Combined helper to record an exercise review, updating completion, SRS, and concept mastery.
   *
   * @param {string} conceptId - Concept slug or topic identifier.
   * @param {string} exerciseId - Exercise unique identifier.
   * @param {number} [quality=4] - Rating from 0 to 5.
   * @param {boolean} [isCorrect=true] - Whether the submission was correct.
   * @param {number} [hintsUsed=0] - Number of hints used.
   */
  recordReview(conceptId, exerciseId, quality = 4, isCorrect = true, hintsUsed = 0) {
    if (isCorrect) {
      this.markCompleted(exerciseId, hintsUsed);
    }
    this.updateSrs(exerciseId, quality);
    if (conceptId) {
      this.updateConceptMastery(conceptId, quality);
    }
  }

  /**
   * Updates arcade stats summary.
   *
   * @param {number} points - Score achieved.
   * @param {number} streak - Max streak in session.
   * @param {number} questionsTotal - Number of questions answered.
   * @param {number} questionsCorrect - Number of correct questions.
   */
  recordArcadeSession(points, streak, questionsTotal = 1, questionsCorrect = 1) {
    const state = this.getState();
    const stats = state.arcade_stats;

    stats.high_score = Math.max(stats.high_score, points);
    stats.best_streak = Math.max(stats.best_streak, streak);
    stats.total_duels += 1;

    // Rolling accuracy estimation
    const sessionAcc = questionsTotal > 0 ? questionsCorrect / questionsTotal : 0.0;
    if (stats.total_duels === 1) {
      stats.accuracy = Number(sessionAcc.toFixed(4));
    } else {
      stats.accuracy = Number(((stats.accuracy * (stats.total_duels - 1) + sessionAcc) / stats.total_duels).toFixed(4));
    }

    this.save();
  }

  /**
   * Exports state as a formatted JSON string compatible with `spanglings sync --import`.
   *
   * @returns {string} Serialized PortableStateBackup JSON.
   */
  exportJson() {
    const state = this.getState();
    const backup = {
      version: APP_VERSION,
      exported_at: new Date().toISOString(),
      completed_count: state.completed_exercises.length,
      srs_items_count: Object.keys(state.srs).length,
      state: state,
    };
    return JSON.stringify(backup, null, 2);
  }

  /**
   * Imports a state JSON string (supporting both PortableStateBackup and raw AppState formats),
   * validates and merges into current state.
   *
   * @param {string} jsonString - State JSON string.
   * @returns {boolean} True if successfully imported and merged, false otherwise.
   */
  importJson(jsonString) {
    if (!jsonString || typeof jsonString !== "string") {
      return false;
    }

    let parsed;
    try {
      parsed = JSON.parse(jsonString);
    } catch {
      return false;
    }

    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
      return false;
    }

    // Determine if wrapped in PortableStateBackup or raw AppState
    const incomingState = parsed.state && typeof parsed.state === "object" ? parsed.state : parsed;

    if (!Array.isArray(incomingState.completed_exercises) && typeof incomingState.version !== "number") {
      return false;
    }

    const currentState = this.getState();

    // 1. Merge completed exercises
    if (Array.isArray(incomingState.completed_exercises)) {
      const combined = new Set([
        ...currentState.completed_exercises,
        ...incomingState.completed_exercises.filter((id) => typeof id === "string"),
      ]);
      currentState.completed_exercises = Array.from(combined);
    }

    // 2. Merge SRS items (prefer higher repetitions or more recent reviews)
    if (incomingState.srs && typeof incomingState.srs === "object") {
      for (const [id, incomingItem] of Object.entries(incomingState.srs)) {
        if (!incomingItem || typeof incomingItem !== "object") continue;
        const existingItem = currentState.srs[id];
        if (!existingItem) {
          currentState.srs[id] = incomingItem;
        } else {
          const incomingReps = incomingItem.repetitions || 0;
          const existingReps = existingItem.repetitions || 0;
          const incomingReviewed = incomingItem.last_reviewed ? new Date(incomingItem.last_reviewed).getTime() : 0;
          const existingReviewed = existingItem.last_reviewed ? new Date(existingItem.last_reviewed).getTime() : 0;

          if (incomingReps > existingReps || (incomingReps === existingReps && incomingReviewed > existingReviewed)) {
            currentState.srs[id] = incomingItem;
          }
        }
      }
    }

    // 3. Merge stats
    if (incomingState.stats && typeof incomingState.stats === "object") {
      for (const [id, incomingStat] of Object.entries(incomingState.stats)) {
        if (!incomingStat || typeof incomingStat !== "object") continue;
        const existingStat = currentState.stats[id] || { attempts: 0, completed_at: null, hints_used: 0 };
        currentState.stats[id] = {
          attempts: Math.max(existingStat.attempts || 0, incomingStat.attempts || 0),
          completed_at: incomingStat.completed_at || existingStat.completed_at,
          hints_used: Math.max(existingStat.hints_used || 0, incomingStat.hints_used || 0),
        };
      }
    }

    // 4. Merge activity history (max count per date)
    if (incomingState.activity_history && typeof incomingState.activity_history === "object") {
      for (const [dateStr, count] of Object.entries(incomingState.activity_history)) {
        if (typeof count === "number") {
          currentState.activity_history[dateStr] = Math.max(
            currentState.activity_history[dateStr] || 0,
            count
          );
        }
      }
    }

    // 5. Merge concept mastery
    if (incomingState.concept_mastery && typeof incomingState.concept_mastery === "object") {
      for (const [id, incomingMastery] of Object.entries(incomingState.concept_mastery)) {
        if (!incomingMastery || typeof incomingMastery !== "object") continue;
        const existingMastery = currentState.concept_mastery[id];
        if (!existingMastery) {
          currentState.concept_mastery[id] = incomingMastery;
        } else if ((incomingMastery.total_reviews || 0) >= (existingMastery.total_reviews || 0)) {
          currentState.concept_mastery[id] = incomingMastery;
        }
      }
    }

    // 6. Scalar / optional state fields
    if (incomingState.accent_mode) {
      currentState.accent_mode = incomingState.accent_mode;
    }
    if (incomingState.current_exercise) {
      currentState.current_exercise = incomingState.current_exercise;
    }
    if (incomingState.evaluated_level) {
      currentState.evaluated_level = incomingState.evaluated_level;
    }
    if (incomingState.tour_completed) {
      currentState.tour_completed = true;
    }
    if (incomingState.arcade_stats && typeof incomingState.arcade_stats === "object") {
      currentState.arcade_stats.high_score = Math.max(
        currentState.arcade_stats.high_score || 0,
        incomingState.arcade_stats.high_score || 0
      );
      currentState.arcade_stats.best_streak = Math.max(
        currentState.arcade_stats.best_streak || 0,
        incomingState.arcade_stats.best_streak || 0
      );
      currentState.arcade_stats.total_duels = Math.max(
        currentState.arcade_stats.total_duels || 0,
        incomingState.arcade_stats.total_duels || 0
      );
      if (typeof incomingState.arcade_stats.accuracy === "number") {
        currentState.arcade_stats.accuracy = incomingState.arcade_stats.accuracy;
      }
    }

    this.save(currentState);
    return true;
  }

  /**
   * Resets progress for a single exercise.
   *
   * @param {string} exerciseId - Exercise unique identifier.
   */
  resetExercise(exerciseId) {
    const state = this.getState();
    state.completed_exercises = state.completed_exercises.filter((id) => id !== exerciseId);
    delete state.srs[exerciseId];
    delete state.stats[exerciseId];
    this.save();
  }

  /**
   * Resets the entire playground state to clean defaults.
   */
  resetAll() {
    this._state = createDefaultState();
    this.save(this._state);
  }
}

// Attach to browser global if window is present
if (typeof window !== "undefined") {
  window.SpanglingsStorage = SpanglingsStorage;
  window.createDefaultState = createDefaultState;
}

// Support CommonJS if needed
if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    STORAGE_KEY,
    APP_VERSION,
    createDefaultState,
    SpanglingsStorage,
  };
}

// Standard ES module export
export { STORAGE_KEY, APP_VERSION, createDefaultState, SpanglingsStorage };
