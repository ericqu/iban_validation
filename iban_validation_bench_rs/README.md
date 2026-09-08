## Benchmark iban_validation_rs against similar libraries
To give a perspective on how the rust crate performs with regards to other similar crates. The other crates can have additional features that iban_validation_rs does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

Here is the output from Criterion:

- iban_validate_sd[^1] time:   [107.54 ns __108.03 ns__ 108.58 ns]
- iban_short_sd[^2] time:   [129.97 ns __130.28 ns__ 130.58 ns]
- iban_parser_sd[^3] time:   [784.23 ns __785.85 ns__ 787.60 ns]
- schwifty_sd[^4] time:   [43.750 µs __43.798 µs__ 43.849 µs]
- iban_validation_rs_sd[^5] time:   [28.451 ns __28.489 ns__ 28.528 ns]
- iban_check_sd[^6]  time:   [98.327 ns __98.604 ns__ 98.843 ns]
- use_iban_sd[^7] time:   [156.11 ns __156.65 ns__ 157.27 ns]

[^1]: iban_validate_sd refers to [iban_validate](https://crates.io/crates/iban_validate) in version 5.0
[^2]: iban_short_sd refers to [iban](https://crates.io/crates/iban) in version 0.2.0
[^3]: iban_parser_sd refers to [iban_parser](https://crates.io/crates/iban_parser) in version 0.2.2
[^4]: schwifty_sd refers to [schwifty](https://crates.io/crates/schwifty) in version 0.3.2 (the rust crate not the python package)
[^5]: iban_validation_rs_sd refers to this package [iban_validation_rs](https://crates.io/crates/iban_validation_rs) in version 0.1.29
[^6]: iban_check_sd refers to [iban_check](https://docs.rs/iban-check/latest/iban_check/) in version 0.1.0
[^7]: use_iban_sd refers to [use_iban](https://github.com/RustUse/use-finance/tree/main/crates/use-iban) in version 0.1.0 part of use-finance

For the details look in the criterion directory in the iban_validation_bench_rs package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [crates.io](https://crates.io/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
