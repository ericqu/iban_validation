"""Type stubs for the `iban_validation_py` extension module.

Kept in sync by hand with `src/lib.rs`; every symbol added to the `#[pymodule]`
there needs an entry here.
"""

from typing import Final

__version__: Final[str]

# Error codes returned by `validate_iban_error_code` /
# `validate_print_iban_error_code`.
ERROR_VALID: Final[int]
ERROR_TOO_SHORT: Final[int]
ERROR_MISSING_COUNTRY: Final[int]
ERROR_INVALID_COUNTRY: Final[int]
ERROR_STRUCTURE_INCORRECT: Final[int]
ERROR_INVALID_SIZE: Final[int]
ERROR_MODULO_INCORRECT: Final[int]
ERROR_INVALID_CHECKSUM: Final[int]

# Two-letter codes of the opt-in, non-registry countries (see `allow_non_registry`).
NON_REGISTRY_COUNTRIES: Final[frozenset[str]]

def validate_iban(iban_t: str, *, allow_non_registry: bool = False) -> bool:
    """Indicate if the iban is valid or not."""

def validate_print_iban(iban_t: str, *, allow_non_registry: bool = False) -> bool:
    """Indicate if the iban in 'print' format (with spaces) is valid or not."""

def validate_iban_with_error(
    iban_t: str, *, allow_non_registry: bool = False
) -> tuple[bool, str]:
    """Validate the IBAN and return (valid, explanation); the message is empty when valid."""

def validate_print_iban_with_error(
    iban_t: str, *, allow_non_registry: bool = False
) -> tuple[bool, str]:
    """Validate the IBAN in 'print' format and return (valid, explanation)."""

def validate_iban_error_code(iban_t: str, *, allow_non_registry: bool = False) -> int:
    """Validate the IBAN and return 0 when valid, otherwise one of the `ERROR_*` codes."""

def validate_print_iban_error_code(
    iban_t: str, *, allow_non_registry: bool = False
) -> int:
    """Validate the IBAN in 'print' format and return 0 or one of the `ERROR_*` codes."""

def iban_source_file() -> str:
    """Get the original registry source file used to build the validation data."""

class IbanValidation:
    """Result of validating a single IBAN; every attribute is None when it is invalid."""

    def __init__(self, s: str, *, allow_non_registry: bool = False) -> None: ...
    @property
    def stored_iban(self) -> str | None:
        """The IBAN when it is a valid one, otherwise None."""

    @property
    def iban_bank_id(self) -> str | None:
        """The bank ID when the IBAN is valid and the country defines one."""

    @property
    def iban_branch_id(self) -> str | None:
        """The branch ID when the IBAN is valid and the country defines one."""

    @property
    def is_non_registry(self) -> bool | None:
        """Whether the matched country is a non-registry one (None when the IBAN is invalid)."""
