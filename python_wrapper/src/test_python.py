import py_viban
from py_viban import PyVIban
print (py_viban.__version__)

print(py_viban.validate_iban('AL47212110090000000235698741'))
print(py_viban.validate_iban('AL47212110090000000235698741VV'))
print(py_viban.validate_iban('AL47212110090000000235658741'))

# Valid IBAN
iban = PyVIban('AL47212110090000000235698741')
print("IBAN:", iban.stored_iban)
print("Bank ID:", iban.iban_bank_id)
print("Branch ID:", iban.iban_branch_id)

# Invalid IBAN
invalid_iban = PyVIban('AL47212110090000000235658741')
print("IBAN:", invalid_iban.stored_iban)  # None
print("Bank ID:", invalid_iban.iban_bank_id)  # None
print("Branch ID:", invalid_iban.iban_branch_id)  # None

iban = PyVIban('AE070331234567890123456')
print("IBAN:", iban.stored_iban)
print("Bank ID:", iban.iban_bank_id)
print("Branch ID:", iban.iban_branch_id)

iban = PyVIban('AT611904300234573201')
print("IBAN:", iban.stored_iban)
print("Bank ID:", iban.iban_bank_id)
print("Branch ID:", iban.iban_branch_id)

iban = PyVIban('CY17002001280000001200527600')
print("IBAN:", iban.stored_iban)
print("Bank ID:", iban.iban_bank_id)
print("Branch ID:", iban.iban_branch_id)