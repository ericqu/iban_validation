# Iban Validation
A set of projects to facilitate validation of ibans and getting the bank identifier and branch identifier in Rust, Python and Polars.

## Structure
The primary validation logic is written in Rust in the iban_validation_rs project. There is a Criterion benchmark to validate if changes are affecting performance positively. Two projects depend on it: the iban_validation_py, a Python wrapper using Maturin to compile, which is intended to be published in PyPI and conda (TODO). A small example in Python is included. The iban_validation_polars is a wrapper into a Polaris plugin, compiling through Maturin and intended to be published on Pypi and conda; likewise, a short example is provided.

## Use Cases
The package is not a general-purpose library to parse IBANs. The intention is not for a user-facing library (in other words, for backends, not frontends). Hence, the 'print' format, loosely documented in the Iban Registry, is not implemented. Further, both the input and output of the library are intended to be in the 'electronic' format. BBAN (Basic Bank Account Number) validation only validates that the length, the position of the bank identifier, and the branch identifiers are correct. Further country-specific validations are not performed. 

In contrast, the intention is to provide a quick, correct validation of the IBAN. Ideally, using minimal memory and CPU and reading the input only once. To integrate easily with other packages, it aims to keep dependencies low. A Python script pre-processed data for the library to decouple the main library and limit code change when a new version of the IBAN registry is released.

## Credits
Some of the Makefile were inspired by the makefiles on the [Polars project](https://github.com/pola-rs/polars).

## Comparison with similar libraries
In the iban_validation_bench_rs, benchmark of similar crates published on crates.io is presented. While this library is the fastest other libraries can provide additional features that are more relevant to your use-cases. See [details](iban_validation_bench_rs/README.md). Similar benchmarking was done on Python libraries see [details](iban_validation_bench_py/README.md).

## Changes
 - 0.1.11: eliminated rust dependecies (rust code generated from Python instead of Hash and Serde)
 - 0.1.9: improve mod97 perf (reduce memory needed)
 - 0.1.8: improve mod97 perf (cpu memory tradeoff).
 - 0.1.7: improve performance related to the Iban structure again.
 - 0.1.6: improve performance related to the Iban structure.
 - 0.1.5: improve documentation and add support to Python 3.13.
 - 0.1.4: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.
 - 0.1.3: Updated to latest [Iban Register](https://www.swift.com/standards/data-standards/iban-international-bank-account-number) v99 from Dec 2024