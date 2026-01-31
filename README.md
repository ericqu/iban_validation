# Iban Validation
A set of libraries to validate IBANs and extract bank and branch identifiers in Rust, Python, and Polars, with a focus on correctness, performance, and minimal memory usage.

## Structure
The primary validation logic is written in Rust in the iban_validation_rs project. There is a Criterion benchmark to validate if changes are affecting performance positively. Two projects depend on it: the iban_validation_py, a Python wrapper using Maturin to compile, which is intended to be published in PyPI. A small example in Python is included. The iban_validation_polars is a wrapper into a Polaris plugin, compiling through Maturin and published on Pypi, a short example is provided.

## Design Goals

This project is designed as a **validation engine**, not a user-facing IBAN formatting or parsing library.

The primary goals are:

- **Fast validation** of IBANs using a single pass over the input
- **Zero or minimal allocation** during validation
- **Deterministic performance** with explicit input limits
- **Accurate, country-specific structure validation** based on the official IBAN registry
- **Low dependency footprint** to ease integration in larger systems and FFI contexts

These goals intentionally favor backend and batch-processing use cases over convenience-oriented APIs.

## Validation Modes

The library provides **two distinct validation paths**, each with different trade-offs:

### Electronic (Strict) IBAN Validation

This mode expects an IBAN in *electronic format* (no spaces, no separators).

Characteristics:
- Known length upfront
- Early rejection on incorrect length
- Tight inner loop with minimal branching
- Highest possible performance

This mode is recommended for:
- Backend systems
- Batch validation
- Data pipelines
- Situations where the IBAN is already normalized

### Print / User-Friendly IBAN Validation

This mode accepts IBANs in *print format*, where spaces may appear between characters.

Characteristics:
- Streaming validation with space filtering
- Single-pass processing without allocation
- Explicit input length limits for safety
- Slightly lower performance due to mandatory per-byte inspection

Because spaces must be inspected and skipped, this mode is **inherently slower** than strict validation. This cost is fundamental and not an implementation artifact.

This mode is intended for:
- Ingesting user-provided data
- Transitional systems where normalization is not guaranteed


## Use Cases
The package is **not a general-purpose IBAN parsing or formatting library**. It is intentionally **not user-facing** and is designed primarily for backend systems.
Further, both the input and output of the library are intended to be in the 'electronic' format. BBAN (Basic Bank Account Number) validation only validates that the length, the position of the bank identifier, and the branch identifiers are correct. Further country-specific validations are not performed. 

BBAN (Basic Bank Account Number) validation only verifies length and the positions of bank and branch identifiers. Country-specific BBAN semantic checks are intentionally out of scope.

In contrast, IBAN validation aims to be:
- fast
- correct
- allocation-free where possible
- based on official registry data

The input is read only once, and validation is performed without constructing intermediate normalized strings unless explicitly required by the caller.

In contrast, the intention is to provide a quick, correct validation of the IBAN. Ideally, using minimal memory and CPU and reading the input only once. To integrate easily with other packages, it aims to keep dependencies low. A Python script pre-processed data for the library to decouple the main library and limit code change when a new version of the IBAN registry is released.

## Performance Philosophy

This project treats performance characteristics as part of the public contract.

In particular:
- Validation time is proportional to the number of bytes inspected
- Input normalization (such as space removal) is avoided where possible
- When normalization is required, its cost is explicit and documented
- Benchmarks are included to detect performance regressions over time

As a result, the library may appear lower-level than typical IBAN validation utilities, but it provides predictable behavior suitable for high-throughput systems.


## Credits
Some of the Makefile were inspired by the makefiles on the [Polars project](https://github.com/pola-rs/polars).

## Comparison with similar libraries
In the iban_validation_bench_rs, benchmark of similar crates published on crates.io is presented. 
While this library prioritizes performance and correctness, other libraries may provide higher-level conveniences such as automatic normalization or formatting, which may be more suitable for frontend or interactive use cases.
See [details](iban_validation_bench_rs/README.md). Similar benchmarking was done on Python libraries see [details](iban_validation_bench_py/README.md).

## WASM
While experimental the library can be tested as JS/WASM here: https://ericqu.github.io/iban_validation/

## Changes
 - 0.1.26: added user_friendly iban validation (handle spaces), added compile time checks, and updated to rust 1.93, dropping python 3.9, adding python 3.14
 - 0.1.25: added forbidden checksums in the validation
 - 0.1.24: update to the python interface only
 - 0.1.23: upgraded to latest Iban register (version 101), only change Portugal (no branch anymore). updated to rust 1.92.0.
 - 0.1.22: upgraded to latest Iban register (version 100), only Albania (AL) and Poland (PL) have changes affecting this project. updated to rust 1.91.1.
 - 0.1.21: upgraded to polars 0.52.0, rust 1.91, improved internal data structure. Enable modern CPU instruction on x86 (x86-64-v3) and Mac (M1) for python, polars and c packages.
 - 0.1.20: technical update upgraded to polars 0.51.0, rust 1.90
 - 0.1.19: technical update upgraded to polars 0.50.0, rust 1.89
 - 0.1.18: technical update upgraded to polars 0.49.1, pyo3 0.25, rust 1.88
 - 0.1.17: memory usage reduced.
 - 0.1.16: improved performance, added territories for GB and FR, and more tests, added WASM (experimental for now), added fuzzer.
 - 0.1.15: improved performance (char to bytes) and improved c wrapper doc.
 - 0.1.14: fixed error for country code IQ (using pdf instead of technical input file).
 - 0.1.13: technical update to polars 0.48.1 and pyo3 0.24.
 - 0.1.12: added the c/c++ wrapper.
 - 0.1.11: eliminated rust dependecies (rust code generated from Python instead of Hash and Serde).
 - 0.1.9: improve mod97 perf (reduce memory needed).
 - 0.1.8: improve mod97 perf (cpu memory tradeoff).
 - 0.1.7: improve performance related to the Iban structure again.
 - 0.1.6: improve performance related to the Iban structure.
 - 0.1.5: improve documentation and add support to Python 3.13.
 - 0.1.4: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.
 - 0.1.3: Updated to latest [Iban Register](https://www.swift.com/standards/data-standards/iban-international-bank-account-number) v99 from Dec 2024.
