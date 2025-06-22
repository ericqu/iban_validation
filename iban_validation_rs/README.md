# Iban Validation rs
Facilitate validation of ibans and getting the bank identifier and branch identifier in Rust.

See documentation for implementation details and a short example.

## Use Cases
The package is not a general-purpose library to parse IBANs. The intention is not for a user-facing library (in other words, for backends, not frontends). Hence, the 'print' format, loosely documented in the Iban Registry, is not implemented. Further, both the input and output of the library are intended to be in the 'electronic' format. BBAN (Basic Bank Account Number) validation only validates that the length, the position of the bank identifier, and the branch identifiers are correct. Further country-specific validations are not performed.

In contrast, the intention is to provide a quick, correct validation of the IBAN. Ideally, using minimal memory and CPU and reading the input only once. To integrate easily with other packages, it aims to keep dependencies low. A Python script pre-processed data for the library to decouple the main library and limit code change when a new version of the IBAN registry is released.

For now, parallelisation is not in the scope of the core library as usage through other libraries is likely to provide that, like in the polars plugin.

# Changes
 - 0.1.17: memory usage reduced.
 - 0.1.16: improved performance, added territories for GB and FR, and more tests, added WASM (experimental for now), added fuzzer.
 - 0.1.15: improved performance (char to bytes) and improved c wrapper doc.
 - 0.1.14: fixed error for country code IQ (using pdf instead of technical input file).
 - 0.1.11: eliminated rust dependecies (rust code generated from Python instead of Hash and Serde).
 - 0.1.9: improve mod97 perf (reduce memory needed).
 - 0.1.8: improve mod97 perf (cpu memory tradeoff).
 - 0.1.7: improve performance related to the Iban structure again.
 - 0.1.6: improve performance related to the Iban structure.
 - 0.1.5: improved documentation.
 - 0.1.4: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.
 - 0.1.3: Updated to latest [Iban Register](https://www.swift.com/standards/data-standards/iban-international-bank-account-number) v99 from Dec 2024.