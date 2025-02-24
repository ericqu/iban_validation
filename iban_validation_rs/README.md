# Iban Validation rs
Facilitate validation of ibans and selecting Bank_id and Branch_id in Rust.

See documentation for implementation details and a short example.

## Use Cases
The package is not a general-purpose library to parse IBANs. The intention is not for a user-facing library (in other words, for backends, not frontends). Hence, the 'print' format, loosely documented in the Iban Registry, is not implemented. Further, both the input and output of the library are intended to be in the 'electronic' format. BBAN (Basic Bank Account Number) validation only validates that the length, the position of the bank identifier, and the branch identifiers are correct. Further country-specific validations are not performed.

In contrast, the intention is to provide a quick, correct validation of the IBAN. Ideally, using minimal memory and CPU and reading the input only once. To integrate easily with other packages, it aims to keep dependencies low. A Python script pre-processed data for the library to decouple the main library and limit code change when a new version of the IBAN registry is released.

For now, parallelisation is not in the scope of the core library as usage through other libraries is likely to provide that, like in the polars plugin.

# Changes
 - 0.14: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.
 - 0.13: Updated to latest [Iban Register](https://www.swift.com/standards/data-standards/iban-international-bank-account-number) v99 from Dec 2024