# iban_validation_py
A package to facilitate validation of IBANs and getting the bank identifier and branch identifier in Python.

## Short examples

There are three ways to interact with the API:
 - Validate the iban with `validate_iban` this does not indicate what is incorrect when the iban in invalid.
 - Validate the iban with `validate_iban_with_error` does the same and give an error message when the iban is invalid.
 - create an `IbanValidation` which allows to select the validated iban, the branch_id and bank_id when relevant.

 See below code for illustration:

```python
import iban_validation_py
from iban_validation_py import IbanValidation

result = iban_validation_py.validate_iban('AL47212110090000000235698741')
assert(result is True)
result = iban_validation_py.validate_iban('AL47212110090000000235698741VV')
assert(result is False)
result, message = iban_validation_py.validate_iban_with_error('AL47212110090000000235698741VV')
assert(result is False)
assert(message == 'IBAN Validation failed: The length of the input Iban does match the length for that country')   

# # Valid IBAN
iban = IbanValidation('AL47212110090000000235698741')
assert('AL47212110090000000235698741' == iban.stored_iban)
assert('212' == iban.iban_bank_id)
assert('11009' == iban.iban_branch_id)
```
## Credit
Cheers to the [Pyo3 Maturin](https://github.com/PyO3/maturin) project! It made this package possible.

## Changes
 - 0.1.17: memory usage reduced.
 - 0.1.16: improved performance, added territories for GB and FR, and more tests, added WASM (experimental for now), added fuzzer.
 - 0.1.15: improved performance (char to bytes) and improved c wrapper doc.
 - 0.1.14: fixed error for country code IQ (using pdf instead of technicql input file).
 - 0.1.11: eliminated rust dependecies (rust code generated from Python instead of Hash and Serde).
 - 0.1.9: improve mod97 perf (reduce memory needed).
 - 0.1.8: improve mod97 perf (cpu memory tradeoff).
 - 0.1.7: improve performance related to the Iban structure again.
 - 0.1.6: improve performance related to the Iban structure.
 - 0.1.5: added support to Python 3.13.
 - 0.1.4: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.