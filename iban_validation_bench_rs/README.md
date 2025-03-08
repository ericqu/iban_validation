## Benchmark iban_validation_rs against similar libraries
To give a perspective on how the rust crate performs with regards to other similar crates. The other crates can have additional feature that iban_validation_rs does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

Here is the output from Criterion:

```
iban_validate_sd        time:   [106.96 ns 107.04 ns 107.14 ns]
iban_short_sd           time:   [133.89 ns 134.15 ns 134.40 ns]
iban_parser_sd          time:   [877.51 ns 880.07 ns 882.81 ns]
schwifty_sd             time:   [53.839 µs 54.002 µs 54.176 µs]
iban_validation_rs_sd   time:   [57.994 ns 58.075 ns 58.156 ns]
```

- iban_validate_sd refers to [iban_validate](https://crates.io/crates/iban_validate) in version 5.0
- iban_short_sd refers to [iban](https://crates.io/crates/iban) in version 0.1.7
- iban_parser_sd refers to [iban_parser](https://crates.io/crates/iban_parser) in version 0.2.2
- schwifty_sd refers to [schwifty](https://crates.io/crates/schwifty) in version 0.3.2 (the rust crate not the python package)
- iban_validation_rs_sd refers to this package [iban_validation_rs](https://crates.io/crates/iban_validation_rs) in version 0.1.6

For the details look in the criterion directory in the iban_validation_bench_rs package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [crates.io](https://crates.io/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 