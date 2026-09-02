/* tslint:disable */
/* eslint-disable */

/**
 * Calculates next interval, repetitions, and ease factor using the SM-2 algorithm.
 */
export function calculate_sm2_review_wasm(ease_factor: number, interval: number, repetitions: number, grade: number): string;

/**
 * Evaluates a user choice for an arcade drill item with speed scoring and dual-layer feedback.
 */
export function evaluate_arcade_choice_wasm(item_id: string, user_choice: string, elapsed_ms: bigint): string;

/**
 * Evaluates a user submission against an exercise or frame ID without filesystem access.
 */
export function evaluate_exercise_wasm(frame_id: string, user_input: string): string;

/**
 * Returns the arcade catalog JSON containing showdowns and specialized engine drills.
 */
export function get_arcade_catalog_json(mode: string): string;

/**
 * Returns the complete curriculum catalog JSON embedded directly in the WebAssembly binary.
 */
export function get_curriculum_catalog_json(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly calculate_sm2_review_wasm: (a: number, b: number, c: number, d: number) => [number, number];
    readonly evaluate_arcade_choice_wasm: (a: number, b: number, c: number, d: number, e: bigint) => [number, number];
    readonly evaluate_exercise_wasm: (a: number, b: number, c: number, d: number) => [number, number];
    readonly get_arcade_catalog_json: (a: number, b: number) => [number, number];
    readonly get_curriculum_catalog_json: () => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
