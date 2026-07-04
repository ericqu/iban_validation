# Iban Validation WASM
Present iban_validation in a javascript propose a demo html file.

## Non-registry countries

By default, only the official SWIFT IBAN registry countries validate. Pass
`true` as the optional `allow_non_registry` argument to `validate_iban_js` or
`parse_iban_js` to additionally accept 22 IBAN-shaped account numbers that are
**not** in the official registry (community-sourced, weaker guarantees):

```js
validate_iban_js("AO49012345678901234567890");        // throws (default, registry-only)
validate_iban_js("AO49012345678901234567890", true);   // true

const parsed = parse_iban_js("AO49012345678901234567890", true);
parsed.is_non_registry; // true

non_registry_countries_js(); // ["AO", "BF", ...]
```

The argument defaults to `false`/omitted, so existing calls are unaffected.

## Changes
 - 0.1.29: added an opt-in `allow_non_registry` argument (default `false`) to `validate_iban_js` and `parse_iban_js`, plus `JsIban.is_non_registry` and `non_registry_countries_js()`.
 - 0.1.28: upgraded to polars 0.54.4, rust 1.96.1, update to iban registry version 102 from Jun 2026 (no significant changes for this package)
 - 0.1.27: upgraded to polars 0.53.0, rust 1.93.1
 - 0.1.26: added user_friendly iban validation (handle spaces), added compile time checks, and updated to rust 1.93, dropping python 3.9, adding python 3.14
 - 0.1.25: added forbidden checksums in the validation
 - 0.1.23: upgraded to latest Iban register (version 101), only change Portugal (no branch anymore). updated to rust 1.92.0.
 - 0.1.22: upgraded to latest Iban register (version 100), only Albania (AL) and Poland (PL) have changes affecting this project. updated to rust 1.91.1.
 - 0.1.21: upgraded to polars 0.52.0, rust 1.91, improved internal data structure. Enable modern CPU instruction on x86 (x86-64-v3) and Mac (M1) for python, polars and c packages. 
 - 0.1.20: technical update upgraded to polars 0.51.0, rust 1.90
 - 0.1.19: technical update upgraded to polars 0.50.0, rust 1.89
 - 0.1.18: technical update upgraded to polars 0.49.1, pyo3 0.25, rust 1.88
 - 0.1.17: memory usage reduced.
 - 0.1.16: initial release of the WASM/JS wrapper.
