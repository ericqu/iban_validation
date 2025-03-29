# iban_validation_polars
A package to facilitate validation of IBANs and getting bank identifier and branch identifier as a Polars plugin.

Leveraging Polars Multi-threaded feature to split IBANs

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
## Credits
Cheers to the [pyo3-polars project](https://github.com/pola-rs/pyo3-polars)! It made this library possible.

## Changes
 - 0.1.11: eliminated rust dependecies (rust code generated from Python instead of Hash and Serde)
 - 0.1.9: improve mod97 perf (reduce memory needed)
 - 0.1.8: improve mod97 perf (cpu memory tradeoff).
 - 0.1.7: improve performance related to the Iban structure again.
 - 0.1.6: improve performance related to the Iban structure.
 - 0.1.5: add support to Python 3.13
 - 0.1.4: technical update; updated polars dependency to polars 0.46.0, pyo3-polars 0.20, and py03 0.23.