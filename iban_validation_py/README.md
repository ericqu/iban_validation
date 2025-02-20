# iban_validation_py
A package to facilitate validation of IBANs and selecting Bank_id and Branch_id in Python.

## Short examples

There are three way to interact with the API:
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
# Changes
 - 0.14: technical update; updated polars dependency to polars 0.46.0, and py03 0.23 impacting only the Python packages.