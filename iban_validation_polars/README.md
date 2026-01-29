# IBAN Validation for Polars (iban_validation_polars)
A high-performance Polars plugin for validating IBANs (International Bank Account Numbers) and extracting bank/branch identifiers leveraging Polars Multi-threaded feature and performance.

## Quick Start
Example:
```python
import polars as pl
from iban_validation_polars import process_ibans
import os

inputfile = r'iban_validation_rs/data/IBAN Examples.txt'
outputfile = r'iban_validation_polars/examples/test_file.csv'

# File generation 
df = pl.read_csv(inputfile).sample(10000000, with_replacement=True)
df.write_csv(outputfile)
print('writing to file complete')

# using the library
df = pl.scan_csv(outputfile)\
    .with_columns(
    validated=process_ibans('IBAN Examples').str.split_exact(',',2)\
        .struct.rename_fields(['valid_ibans', 'bank_id', 'branch_id'])
).unnest('validated').sort(by='IBAN Examples', descending=True)

# show some results
print(df.collect(streaming=True))

# cleanup
os.remove(outputfile)
```
## Features
✅ Fast IBAN validation (Rust-powered)

✅ Bank identifier extraction

✅ Branch identifier extraction

✅ Support for all IBAN countries (SWIFT Registry v99)

✅ Zero-copy operations where possible

## Installation
```bash 
pip install iban-validation-polars
```

## Performance Benchmarks
This polars plugin was the principal objective of this library; the benchmarks [here](../iban_validation_bench_py/README.md) highlight how much faster it is to use the plugin than to call the Python library with ```map_element``` (about 100 times faster).

## API Reference
`process_ibans(column: pl.Expr) -> pl.Expr`
Validates IBANs and extracts bank/branch information in a single operation.
Parameters:
 - column: A Polars expression containing IBAN strings in electronic format (no spaces)
    Electronic format only: No spaces or special characters.
    Case insensitive: Both uppercase and lowercase accepted when the IBAN definition allows it.
    Length: 15-34 characters depending on country, as per the registry definition.

Returns:

 - pl.Expr: A struct expression containing:
    - validated_iban (str): The valid IBAN if validation passes, empty string if invalid
    - bank_code (str): Bank identifier code (when relevant and valid) otherwise empty
    - branch_code (str): Branch identifier code (when relevant and valid) otherwise empty

## Common Use cases
 - Data Cleaning pipeline
 - Data validation report
 - Extracting valid IBANs only (filter for non-empty valid IBANs)

## Error Handling
This pluging does not raise exception under normal operation.

## Credits
Cheers to the [pyo3-polars project](https://github.com/pola-rs/pyo3-polars)! It made this library possible.

## Changes
 - 0.1.26: added compile time checks, and updated to rust 1.93, dropping python 3.9, adding python 3.14
 - 0.1.25: added forbidden checksums in the validation
 - 0.1.24: update to the python interface only
 - 0.1.23: upgraded to latest Iban register (version 101), only change Portugal (no branch anymore). updated to rust 1.92.0.
 - 0.1.22: upgraded to latest Iban register (version 100), only Albania (AL) and Poland (PL) have changes affecting this project. updated to rust 1.91.1.
 - 0.1.21: upgraded to polars 0.52.0, rust 1.91, improved internal data structure. Enable modern CPU instruction on x86 (x86-64-v3) and Mac (M1) for python, polars and c packages.
 - 0.1.20: technical update upgraded to polars 0.51.0, rust 1.90
 - 0.1.19: technical update upgraded to polars 0.50.0, rust 1.89. Improved Polars documentation.
 - 0.1.18: technical update upgraded to polars 0.49.1, pyo3 0.25, rust 1.88
 - 0.1.17: memory usage reduced.
 - 0.1.16: improved performance, added territories for GB and FR, and more tests, added WASM (experimental for now), added fuzzer.
 - 0.1.15: improved performance (char to bytes) and improved c wrapper doc.
 - 0.1.13: technical update to polars 0.48.1 and pyo3 0.24.
 - 0.1.11: eliminated rust dependencies (rust code generated from Python instead of Hash and Serde).
 - 0.1.9: improve mod97 perf (reduce memory needed).
 - 0.1.8: improve mod97 perf (cpu memory tradeoff).
 - 0.1.7: improve performance related to the Iban structure again.
 - 0.1.6: improve performance related to the Iban structure.
 - 0.1.5: add support for Python 3.13.
 - 0.1.4: technical update; updated polars dependency to polars 0.46.0, pyo3-polars 0.20, and py03 0.23.
