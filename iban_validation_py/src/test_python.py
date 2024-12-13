import iban_validation_py
from iban_validation_py import IbanValidation
print ('testing version ', iban_validation_py.__version__)

def test_validate_iban():
    assert(iban_validation_py.validate_iban('AL47212110090000000235698741') is True)
    assert(iban_validation_py.validate_iban('AL47212110090000000235698741VV') is False)
    assert(iban_validation_py.validate_iban('AL47212110090000000235658741') is False)

def test_iban():
    # # Valid IBAN
    iban = IbanValidation('AL47212110090000000235698741')
    assert('AL47212110090000000235698741' == iban.stored_iban)
    assert('212' == iban.iban_bank_id)
    assert('11009' == iban.iban_branch_id)
    # print("IBAN:", iban.stored_iban)
    # print("Bank ID:", iban.iban_bank_id)
    # print("Branch ID:", iban.iban_branch_id)

    # # Invalid IBAN
    invalid_iban = IbanValidation('AL47212110090000000235658741')
    assert(invalid_iban.stored_iban is None)
    assert(invalid_iban.iban_bank_id is None)
    assert(invalid_iban.iban_branch_id is None)
    # print("IBAN:", invalid_iban.stored_iban)  # None
    # print("Bank ID:", invalid_iban.iban_bank_id)  # None
    # print("Branch ID:", invalid_iban.iban_branch_id)  # None

    iban = IbanValidation('AE070331234567890123456')
    assert('AE070331234567890123456' == iban.stored_iban)
    assert('033' == iban.iban_bank_id)
    assert(iban.iban_branch_id is None)
    # print("IBAN:", iban.stored_iban)
    # print("Bank ID:", iban.iban_bank_id)
    # print("Branch ID:", iban.iban_branch_id)

    iban = IbanValidation('AT611904300234573201')
    assert('AT611904300234573201' == iban.stored_iban)
    assert('19043' == iban.iban_bank_id)
    assert(iban.iban_branch_id is None)
    # print("IBAN:", iban.stored_iban)
    # print("Bank ID:", iban.iban_bank_id)
    # print("Branch ID:", iban.iban_branch_id)

    iban = IbanValidation('CY17002001280000001200527600')
    assert('CY17002001280000001200527600' == iban.stored_iban)
    assert('002' == iban.iban_bank_id)
    assert('00128' == iban.iban_branch_id)
    # print("IBAN:", iban.stored_iban)
    # print("Bank ID:", iban.iban_bank_id)
    # print("Branch ID:", iban.iban_branch_id)

test_validate_iban()
test_iban()