## Benchmark iban_validation_rs against similar libraries
To give a perspective on how the rust crate performs with regards to other similar crates. The other crates can have additional feature that iban_validation_rs does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

Here is the output from Criterion:

```
iban_validate_sd        time:   [107.07 ns 107.17 ns 107.29 ns]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) low severe
  5 (5.00%) high mild
  3 (3.00%) high severe

iban_short_sd           time:   [138.93 ns 139.38 ns 139.80 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

iban_parser_sd          time:   [884.90 ns 890.01 ns 895.08 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

schwifty_sd             time:   [54.516 µs 54.623 µs 54.741 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

iban_validation_rs_sd   time:   [59.322 ns 59.436 ns 59.559 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
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