## Benchmark iban_validation_rs against similar libraries
To give a perspective on how the rust crate performs with regards to other similar crates. The other crates can have additional features that iban_validation_rs does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

Here is the output from Criterion:

- iban_validate_sd[^1] time:   [104.97 ns __105.16 ns__ 105.34 ns]
- iban_short_sd[^2] time:   [134.20 ns __134.47 ns__ 134.75 ns]
- iban_parser_sd[^3] time:   [859.62 ns __863.97 ns__ 868.44 ns]
- schwifty_sd[^4] time:   [52.999 µs __53.455 µs__ 54.176 µs]
- iban_validation_rs_sd[^5] time:   [27.753 ns __27.799 ns__ 27.848 ns]

[^1]: iban_validate_sd refers to [iban_validate](https://crates.io/crates/iban_validate) in version 5.0
[^2]: iban_short_sd refers to [iban](https://crates.io/crates/iban) in version 0.1.7
[^3]: iban_parser_sd refers to [iban_parser](https://crates.io/crates/iban_parser) in version 0.2.2
[^4]: schwifty_sd refers to [schwifty](https://crates.io/crates/schwifty) in version 0.3.2 (the rust crate not the python package)
[^5]: iban_validation_rs_sd refers to this package [iban_validation_rs](https://crates.io/crates/iban_validation_rs) in version 0.1.16

For the details look in the criterion directory in the iban_validation_bench_rs package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [crates.io](https://crates.io/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
