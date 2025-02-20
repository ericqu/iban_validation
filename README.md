# Iban Validation
A set of projects to facilitate validation of ibans and selecting Bank_id and Branch_id in Rust, Python and Polars.

## Structure
The main validation logic is written in Rust in the iban_validation_rs project. There is a Criterion benchmark to validate if changes are affecting performance positively. There are two project depending on it, the iban_validation_py which a python wrapper using Maturin to compile, it is intended to be published in pypi and conda (TODO). As small example in python is included. The iban_validation_polars is a wrapper into a Polars plugin, also compiling through Maturin and intended to be published on pypi and conda; likewise a short example is provided.

# Credits
Some of the Makefile were inspired by the makefiles on the [Polars project](https://github.com/pola-rs/polars)

# Changes
 - 0.14: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.
 - 0.13: Updated to latest [Iban Register](https://www.swift.com/standards/data-standards/iban-international-bank-account-number) v99 from Dec 2024