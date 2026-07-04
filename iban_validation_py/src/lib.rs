use pyo3::prelude::*;
use pyo3::types::PyFrozenSet;

/// Error codes for IBAN validation failures
enum IbanErrorCode {
    Valid = 0,
    TooShort = 1,
    MissingCountry = 2,
    InvalidCountry = 3,
    StructureIncorrectForCountry = 4,
    InvalidSizeForCountry = 5,
    ModuloIncorrect = 6,
    InvalidChecksum = 7,
}

/// Maps a core validation error to the corresponding `IbanErrorCode`.
/// Single source of truth so adding a `ValidationError` variant only needs one update.
#[inline]
fn map_validation_error(err: iban_validation_rs::ValidationError) -> IbanErrorCode {
    match err {
        iban_validation_rs::ValidationError::TooShort(_) => IbanErrorCode::TooShort,
        iban_validation_rs::ValidationError::MissingCountry => IbanErrorCode::MissingCountry,
        iban_validation_rs::ValidationError::InvalidCountry => IbanErrorCode::InvalidCountry,
        iban_validation_rs::ValidationError::StructureIncorrectForCountry => {
            IbanErrorCode::StructureIncorrectForCountry
        }
        iban_validation_rs::ValidationError::InvalidSizeForCountry => {
            IbanErrorCode::InvalidSizeForCountry
        }
        iban_validation_rs::ValidationError::ModuloIncorrect => IbanErrorCode::ModuloIncorrect,
        iban_validation_rs::ValidationError::InvalidChecksum => IbanErrorCode::InvalidChecksum,
    }
}

/// Indicate if the iban is valid or not
#[pyfunction]
#[pyo3(signature = (iban_t, *, allow_non_registry = false))]
fn validate_iban(iban_t: &str, allow_non_registry: bool) -> PyResult<bool> {
    match iban_validation_rs::validate_iban_str_with(iban_t, allow_non_registry.into()) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Indicate if the iban is valid or not
#[pyfunction]
#[pyo3(signature = (iban_t, *, allow_non_registry = false))]
fn validate_print_iban(iban_t: &str, allow_non_registry: bool) -> PyResult<bool> {
    match iban_validation_rs::validate_iban_str_print_with(iban_t, allow_non_registry.into()) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Indicate if the iban is valid or not and provide an explanation when there is an error
/// Validate the IBAN and return a tuple of (bool, String)
#[pyfunction]
#[pyo3(signature = (iban_t, *, allow_non_registry = false))]
fn validate_iban_with_error(iban_t: &str, allow_non_registry: bool) -> PyResult<(bool, String)> {
    match iban_validation_rs::validate_iban_str_with(iban_t, allow_non_registry.into()) {
        Ok(_) => Ok((true, String::new())),
        Err(e) => Ok((false, format!("IBAN Validation failed: {e}"))),
    }
}

/// Validate the IBAN in 'print' (use-friendly / with extra spaces) and return an error code
/// Validate the IBAN and return a tuple of (bool, String)
#[pyfunction]
#[pyo3(signature = (iban_t, *, allow_non_registry = false))]
fn validate_print_iban_with_error(
    iban_t: &str,
    allow_non_registry: bool,
) -> PyResult<(bool, String)> {
    match iban_validation_rs::validate_iban_str_print_with(iban_t, allow_non_registry.into()) {
        Ok(_) => Ok((true, String::new())),
        Err(e) => Ok((false, format!("IBAN Validation failed: {e}"))),
    }
}

/// Validate the IBAN and return an error code
/// Returns 0 if valid, or a specific error code (1-6) for different validation failures
#[pyfunction]
#[pyo3(signature = (iban_t, *, allow_non_registry = false))]
fn validate_iban_error_code(iban_t: &str, allow_non_registry: bool) -> PyResult<i32> {
    match iban_validation_rs::validate_iban_str_with(iban_t, allow_non_registry.into()) {
        Ok(_) => Ok(IbanErrorCode::Valid as i32),
        Err(e) => Ok(map_validation_error(e) as i32),
    }
}

/// Validate the IBAN in 'print' (use-friendly / with extra spaces) and return an error code
/// Returns 0 if valid, or a specific error code (1-6) for different validation failures
#[pyfunction]
#[pyo3(signature = (iban_t, *, allow_non_registry = false))]
fn validate_print_iban_error_code(iban_t: &str, allow_non_registry: bool) -> PyResult<i32> {
    match iban_validation_rs::validate_iban_str_print_with(iban_t, allow_non_registry.into()) {
        Ok(_) => Ok(IbanErrorCode::Valid as i32),
        Err(e) => Ok(map_validation_error(e) as i32),
    }
}

/// Get original source file used
#[pyfunction]
fn iban_source_file() -> PyResult<&'static str> {
    Ok(iban_validation_rs::get_source_file())
}

/// Provide a python class to encapsulate the results
#[pyclass]
pub struct IbanValidation {
    /// the IBAN when it is a valid one, otherwise None
    stored_iban: Option<String>,
    /// the Bank Id when there is a valid one, and when it is a valid IBAN
    iban_bank_id: Option<Option<String>>, // Outer Option for validation
    /// the branch ID when there is a valid one and whn there is a valid IBAN
    iban_branch_id: Option<Option<String>>,
    /// whether the matched country is one of the non-registry countries, when valid
    is_non_registry: Option<bool>,
}

#[pymethods]
impl IbanValidation {
    /// Constructor for PyIban. Returns a dictionary-like object for Python.
    #[new]
    #[pyo3(signature = (s, *, allow_non_registry = false))]
    pub fn new(s: &str, allow_non_registry: bool) -> PyResult<Self> {
        match iban_validation_rs::Iban::new_with(s, allow_non_registry.into()) {
            Ok(iban) => {
                let cc: [u8; 2] = iban.get_iban().as_bytes()[0..2].try_into().unwrap();
                Ok(Self {
                    stored_iban: Some(iban.get_iban().to_string()),
                    iban_bank_id: Some(iban.iban_bank_id.map(|x| x.to_string())),
                    iban_branch_id: Some(iban.iban_branch_id.map(|x| x.to_string())),
                    is_non_registry: Some(iban_validation_rs::is_non_registry_country(cc)),
                })
            }
            Err(_) => Ok(Self {
                stored_iban: None,
                iban_bank_id: None,
                iban_branch_id: None,
                is_non_registry: None,
            }),
        }
    }

    /// Expose the stored IBAN.
    #[getter]
    pub fn stored_iban(&self) -> Option<String> {
        self.stored_iban.clone()
    }

    /// Expose the bank ID if available.
    #[getter]
    pub fn iban_bank_id(&self) -> Option<Option<String>> {
        self.iban_bank_id.clone()
    }

    /// Expose the branch ID if available.
    #[getter]
    pub fn iban_branch_id(&self) -> Option<Option<String>> {
        self.iban_branch_id.clone()
    }

    /// Whether the matched country is a non-registry country (None when the IBAN is invalid).
    #[getter]
    pub fn is_non_registry(&self) -> Option<bool> {
        self.is_non_registry
    }
}

/// wrap the class and function into the module for python
#[pymodule]
fn iban_validation_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_iban, m)?)?;
    m.add_function(wrap_pyfunction!(validate_iban_with_error, m)?)?;
    m.add_function(wrap_pyfunction!(validate_iban_error_code, m)?)?;
    m.add_function(wrap_pyfunction!(validate_print_iban, m)?)?;
    m.add_function(wrap_pyfunction!(validate_print_iban_with_error, m)?)?;
    m.add_function(wrap_pyfunction!(validate_print_iban_error_code, m)?)?;
    m.add_function(wrap_pyfunction!(iban_source_file, m)?)?;
    m.add_class::<IbanValidation>()?;
    // Add error code constants to the module
    m.add("ERROR_VALID", 0)?;
    m.add("ERROR_TOO_SHORT", 1)?;
    m.add("ERROR_MISSING_COUNTRY", 2)?;
    m.add("ERROR_INVALID_COUNTRY", 3)?;
    m.add("ERROR_STRUCTURE_INCORRECT", 4)?;
    m.add("ERROR_INVALID_SIZE", 5)?;
    m.add("ERROR_MODULO_INCORRECT", 6)?;

    // Two-letter codes of the opt-in, non-registry countries (see `allow_non_registry`).
    let non_registry_countries = iban_validation_rs::NON_REGISTRY_COUNTRIES
        .iter()
        .map(|cc| std::str::from_utf8(cc).unwrap());
    m.add(
        "NON_REGISTRY_COUNTRIES",
        PyFrozenSet::new(m.py(), non_registry_countries)?,
    )?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
