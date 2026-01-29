# Iban Validation WASM
Present iban_validation in a javascript propose a demo html file.

## Changes
 - 0.1.26: added compile time checks, and updated to rust 1.93
 - 0.1.25: added forbidden checksums in the validation
 - 0.1.23: upgraded to latest Iban register (version 101), only change Portugal (no branch anymore). updated to rust 1.92.0.
 - 0.1.22: upgraded to latest Iban register (version 100), only Albania (AL) and Poland (PL) have changes affecting this project. updated to rust 1.91.1.
 - 0.1.21: upgraded to polars 0.52.0, rust 1.91, improved internal data structure. Enable modern CPU instruction on x86 (x86-64-v3) and Mac (M1) for python, polars and c packages. 
 - 0.1.20: technical update upgraded to polars 0.51.0, rust 1.90
 - 0.1.19: technical update upgraded to polars 0.50.0, rust 1.89
 - 0.1.18: technical update upgraded to polars 0.49.1, pyo3 0.25, rust 1.88
 - 0.1.17: memory usage reduced.
 - 0.1.16: initial release of the WASM/JS wrapper.
