import iban_validation_py
from iban_validation_py import IbanValidation

print("testing version ", iban_validation_py.__version__)

VALID_IBAN = "AL47212110090000000235698741"
#                   AL47 2121 1009 0000 0002 3569 8741
VALID_IBAN_PRINT = "AL47 2121 1009 0000 0002 3569 8741"

INVALID_IBAN = "AL47212110090000000235658741"
INVALID_IBAN_PRINT = "AL47 2121 1009 0000 0002 3569 741"


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


def test_validate_print_iban():
    assert iban_validation_py.validate_print_iban(VALID_IBAN_PRINT) is True
    assert iban_validation_py.validate_print_iban(INVALID_IBAN_PRINT) is False
    assert iban_validation_py.validate_print_iban("") is False


def test_validate_print_iban_with_error():
    result, message = iban_validation_py.validate_print_iban_with_error(
        VALID_IBAN_PRINT
    )
    assert result is True
    assert message == ""

    result, message = iban_validation_py.validate_print_iban_with_error(
        INVALID_IBAN_PRINT
    )
    assert result is False
    assert message.startswith("IBAN Validation failed:")


def test_iban():
    # # Valid IBAN
    iban = IbanValidation("AL47212110090000000235698741")
    assert "AL47212110090000000235698741" == iban.stored_iban
    assert "212" == iban.iban_bank_id
    assert "11009" == iban.iban_branch_id

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
    invalid_iban = IbanValidation(
        "HN88CABF00000000000250005469HN88CABF00000000000250005469"
    )
    assert invalid_iban.stored_iban is None
    assert invalid_iban.iban_bank_id is None
    assert invalid_iban.iban_branch_id is None

    # # Invalid IBAN
    invalid_iban = IbanValidation(
        "HN88ZZZZ00000000000250005469HN88CABF00000000000250005469"
    )
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


def test_non_registry_default_rejected():
    non_registry_iban = "AO49012345678901234567890"
    assert iban_validation_py.validate_iban(non_registry_iban) is False
    assert (
        iban_validation_py.validate_iban_error_code(non_registry_iban)
        == iban_validation_py.ERROR_INVALID_COUNTRY
    )

    invalid_iban = IbanValidation(non_registry_iban)
    assert invalid_iban.stored_iban is None
    assert invalid_iban.is_non_registry is None


def test_non_registry_opt_in_accepted():
    non_registry_iban = "AO49012345678901234567890"
    assert (
        iban_validation_py.validate_iban(non_registry_iban, allow_non_registry=True)
        is True
    )
    assert (
        iban_validation_py.validate_iban_error_code(
            non_registry_iban, allow_non_registry=True
        )
        == iban_validation_py.ERROR_VALID
    )

    iban = IbanValidation(non_registry_iban, allow_non_registry=True)
    assert iban.stored_iban == non_registry_iban
    assert iban.is_non_registry is True

    # MA: bank 1-5, branch 6-10 within the BBAN
    ma_iban = IbanValidation("MA36012345678901234567890123", allow_non_registry=True)
    assert ma_iban.iban_bank_id == "01234"
    assert ma_iban.iban_branch_id == "56789"


def test_non_registry_country_set_membership():
    assert "AO" in iban_validation_py.NON_REGISTRY_COUNTRIES
    assert "MA" in iban_validation_py.NON_REGISTRY_COUNTRIES
    assert "DE" not in iban_validation_py.NON_REGISTRY_COUNTRIES
    assert len(iban_validation_py.NON_REGISTRY_COUNTRIES) == 22


def test_is_non_registry_getter_for_registry_country():
    iban = IbanValidation(VALID_IBAN, allow_non_registry=True)
    assert iban.is_non_registry is False


test_validate_iban()
test_iban()
test_validate_print_iban()
test_validate_print_iban_with_error()
test_non_registry_default_rejected()
test_non_registry_opt_in_accepted()
test_non_registry_country_set_membership()
test_is_non_registry_getter_for_registry_country()
