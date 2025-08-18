## Benchmark iban_validation_rs against similar libraries
To give a perspective on how the rust crate performs with regards to other similar crates. The other crates can have additional features that iban_validation_rs does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

Here is the output from Criterion:

- iban_validate_sd[^1] time:   [111.55 ns __111.71 ns__ 111.88 ns]
- iban_short_sd[^2] time:   [140.44 ns __140.77 ns__ 141.14 ns]
- iban_parser_sd[^3] time:   [873.38 ns __875.81 ns__ 878.36 ns]
- schwifty_sd[^4] time:   [53.495 µs __53.629 µs__ 53.773 µs]
- iban_validation_rs_sd[^5] time:   [27.759 ns __27.790 ns__ 27.820 ns]

[^1]: iban_validate_sd refers to [iban_validate](https://crates.io/crates/iban_validate) in version 5.0
[^2]: iban_short_sd refers to [iban](https://crates.io/crates/iban) in version 0.2.0
[^3]: iban_parser_sd refers to [iban_parser](https://crates.io/crates/iban_parser) in version 0.2.2
[^4]: schwifty_sd refers to [schwifty](https://crates.io/crates/schwifty) in version 0.3.2 (the rust crate not the python package)
[^5]: iban_validation_rs_sd refers to this package [iban_validation_rs](https://crates.io/crates/iban_validation_rs) in version 0.1.19

For the details look in the criterion directory in the iban_validation_bench_rs package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [crates.io](https://crates.io/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
