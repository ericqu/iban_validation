## Benchmark iban_validation_rs against similar libraries
To give a perspective on how the rust crate performs with regards to other similar crates. The other crates can have additional features that iban_validation_rs does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

Here is the output from Criterion:

- iban_validate_sd[^1] time:   [105.99 ns __106.16 ns__ 106.32 ns]
- iban_short_sd[^2] time:   [142.59 ns __142.97 ns__ 143.32 ns]
- iban_parser_sd[^3] time:   [792.82 ns __796.30 ns__ 799.99 ns]
- schwifty_sd[^4] time:   [43.374 µs __43.443 µs__ 43.513 µs]
- iban_validation_rs_sd[^5] time:   [28.427 ns __28.474 ns__ 28.525 ns]
- iban_check_sd[^6]  time:   [96.397 ns __96.529 ns__ 96.644 ns]
- use_iban_sd[^7] time:   [149.82 ns __150.46 ns__ 151.14 ns]

[^1]: iban_validate_sd refers to [iban_validate](https://crates.io/crates/iban_validate) in version 5.0
[^2]: iban_short_sd refers to [iban](https://crates.io/crates/iban) in version 0.2.0
[^3]: iban_parser_sd refers to [iban_parser](https://crates.io/crates/iban_parser) in version 0.2.2
[^4]: schwifty_sd refers to [schwifty](https://crates.io/crates/schwifty) in version 0.3.2 (the rust crate not the python package)
[^5]: iban_validation_rs_sd refers to this package [iban_validation_rs](https://crates.io/crates/iban_validation_rs) in version 0.1.28
[^6]: iban_check_sd refers to [iban_check](https://docs.rs/iban-check/latest/iban_check/) in version 0.1.0
[^7]: use_iban_sd refers to [use_iban](https://github.com/RustUse/use-finance/tree/main/crates/use-iban) in version 0.1.0 part of use-finance

For the details look in the criterion directory in the iban_validation_bench_rs package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [crates.io](https://crates.io/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
