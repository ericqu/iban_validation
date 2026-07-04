/* tslint:disable */
/* eslint-disable */

export class JsIban {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly bank_id: string | undefined;
    readonly branch_id: string | undefined;
    readonly iban: string;
    /**
     * Whether the matched country is one of the non-registry countries.
     */
    readonly is_non_registry: boolean;
}

export function get_source_file_js(): string;

export function get_version_js(): string;

/**
 * Lists the two-letter codes of the opt-in, non-registry countries.
 */
export function non_registry_countries_js(): string[];

/**
 * Validates and parses an IBAN. `allowNonRegistry` (default `false`) additionally
 * accepts 22 IBAN-shaped account numbers that are not in the official SWIFT IBAN
 * registry (community-sourced, weaker guarantees).
 */
export function parse_iban_js(input: string, allow_non_registry?: boolean | null): JsIban;

/**
 * Validates an IBAN. `allowNonRegistry` (default `false`) additionally accepts 22
 * IBAN-shaped account numbers that are not in the official SWIFT IBAN registry
 * (community-sourced, weaker guarantees).
 */
export function validate_iban_js(input: string, allow_non_registry?: boolean | null): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_jsiban_free: (a: number, b: number) => void;
    readonly get_source_file_js: () => [number, number];
    readonly get_version_js: () => [number, number];
    readonly jsiban_bank_id: (a: number) => [number, number];
    readonly jsiban_branch_id: (a: number) => [number, number];
    readonly jsiban_iban: (a: number) => [number, number];
    readonly jsiban_is_non_registry: (a: number) => number;
    readonly non_registry_countries_js: () => [number, number];
    readonly parse_iban_js: (a: number, b: number, c: number) => [number, number, number];
    readonly validate_iban_js: (a: number, b: number, c: number) => [number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
