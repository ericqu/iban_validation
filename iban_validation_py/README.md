# iban_validation_py
A package to facilitate validation of IBANs and getting the bank identifier and branch identifier in Python.

## Short examples

There are four ways to interact with the API:
 - Validate the iban with `validate_iban` this does not indicate what is incorrect when the iban in invalid.
 - Validate the iban with `validate_iban_with_error` does the same and give an error message when the iban is invalid.
 - Validate the iban with `validate_iban_error_code` which returns an error code (0 for valid, 1-6 for specific errors).
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

# Using error codes
error_code = iban_validation_py.validate_iban_error_code('AL47212110090000000235698741')
assert(error_code == iban_validation_py.ERROR_VALID)

error_code = iban_validation_py.validate_iban_error_code('AL47212110090000000235698741VV')
assert(error_code == iban_validation_py.ERROR_INVALID_SIZE)

# Check specific error types
if error_code == iban_validation_py.ERROR_TOO_SHORT:
    print("IBAN is too short")
elif error_code == iban_validation_py.ERROR_MISSING_COUNTRY:
    print("IBAN missing country code")
elif error_code == iban_validation_py.ERROR_INVALID_COUNTRY:
    print("Invalid country code")
elif error_code == iban_validation_py.ERROR_STRUCTURE_INCORRECT:
    print("IBAN structure doesn't match country requirements")
elif error_code == iban_validation_py.ERROR_INVALID_SIZE:
    print("IBAN length doesn't match country requirements")
elif error_code == iban_validation_py.ERROR_MODULO_INCORRECT:
    print("IBAN checksum is incorrect")

result = iban_validation_py.validate_print_iban('AL47 2121 1009 0000 0002 3569 8741')
assert result is True

# # Valid IBAN
iban = IbanValidation('AL47212110090000000235698741')
assert('AL47212110090000000235698741' == iban.stored_iban)
assert('212' == iban.iban_bank_id)
assert('11009' == iban.iban_branch_id)
```

## Error Codes

The `validate_iban_error_code` function returns the following error codes:

- `ERROR_VALID` (0): The IBAN is valid
- `ERROR_TOO_SHORT` (1): The IBAN is too short (minimum length is 4)
- `ERROR_MISSING_COUNTRY` (2): The IBAN doesn't start with a two-letter country code
- `ERROR_INVALID_COUNTRY` (3): The country code is not recognized
- `ERROR_STRUCTURE_INCORRECT` (4): The IBAN structure doesn't match the country's requirements
- `ERROR_INVALID_SIZE` (5): The IBAN length doesn't match the expected length for that country
- `ERROR_MODULO_INCORRECT` (6): The IBAN checksum (mod97) is incorrect

## Credit
Cheers to the [Pyo3 Maturin](https://github.com/PyO3/maturin) project! It made this package possible.

## Changes
 - 0.1.26: added compile time checks, and updated to rust 1.93
 - 0.1.25: added forbidden checksums in the validation
 - 0.1.24: added `validate_iban_error_code` function to return specific error codes for validation failures.
 - 0.1.23: upgraded to latest Iban register (version 101), only change Portugal (no branch anymore). updated to rust 1.92.0.
 - 0.1.22: upgraded to latest Iban register (version 100), only Albania (AL) and Poland (PL) have changes affecting this project. updated to rust 1.91.1.
 - 0.1.21: upgraded to polars 0.52.0, rust 1.91, improved internal data structure. Enable modern CPU instruction on x86 (x86-64-v3) and Mac (M1) for python, polars and c packages.
 - 0.1.20: technical update upgraded to polars 0.51.0, rust 1.90
 - 0.1.19: technical update upgraded to polars 0.50.0, rust 1.89
 - 0.1.18: technical update upgraded to polars 0.49.1, pyo3 0.25, rust 1.88
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