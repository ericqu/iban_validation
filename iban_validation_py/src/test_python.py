import iban_validation_py
from iban_validation_py import IbanValidation

print("testing version ", iban_validation_py.__version__)


def test_validate_iban():
    assert iban_validation_py.validate_iban("AL47212110090000000235698741") is True
    assert iban_validation_py.validate_iban("AL47212110090000000235698741VV") is False
    assert iban_validation_py.validate_iban("AL47212110090000000235658741") is False

    result, message = iban_validation_py.validate_iban_with_error(
        "AL47212110090000000235698741VV"
    )
    assert result is False
    assert (
        message
        == "IBAN Validation failed: The length of the input Iban does match the length for that country"
    )


def test_iban():
    # # Valid IBAN
    iban = IbanValidation("AL47212110090000000235698741")
    assert "AL47212110090000000235698741" == iban.stored_iban
    assert "212" == iban.iban_bank_id
    assert "1100" == iban.iban_branch_id

    # # Invalid IBAN
    invalid_iban = IbanValidation("AL47212110090000000235658741")
    assert invalid_iban.stored_iban is None
    assert invalid_iban.iban_bank_id is None
    assert invalid_iban.iban_branch_id is None

    # # Invalid IBAN
    invalid_iban = IbanValidation("")
    assert invalid_iban.stored_iban is None
    assert invalid_iban.iban_bank_id is None
    assert invalid_iban.iban_branch_id is None

    # # Invalid IBAN
    invalid_iban = IbanValidation("HN88CABF00000000000250005469HN88CABF00000000000250005469")
    assert invalid_iban.stored_iban is None
    assert invalid_iban.iban_bank_id is None
    assert invalid_iban.iban_branch_id is None

    # # Invalid IBAN
    invalid_iban = IbanValidation("HN88ZZZZ00000000000250005469HN88CABF00000000000250005469")
    assert invalid_iban.stored_iban is None
    assert invalid_iban.iban_bank_id is None
    assert invalid_iban.iban_branch_id is None

    iban = IbanValidation("AE070331234567890123456")
    assert "AE070331234567890123456" == iban.stored_iban
    assert "033" == iban.iban_bank_id
    assert iban.iban_branch_id is None

    iban = IbanValidation("AT611904300234573201")
    assert "AT611904300234573201" == iban.stored_iban
    assert "19043" == iban.iban_bank_id
    assert iban.iban_branch_id is None

    iban = IbanValidation("CY17002001280000001200527600")
    assert "CY17002001280000001200527600" == iban.stored_iban
    assert "002" == iban.iban_bank_id
    assert "00128" == iban.iban_branch_id


test_validate_iban()
test_iban()
