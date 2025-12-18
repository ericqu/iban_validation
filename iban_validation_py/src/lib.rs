use pyo3::prelude::*;

/// indicate if the iban is valid or not
#[pyfunction]
fn validate_iban(iban_t: &str) -> PyResult<bool> {
    match iban_validation_rs::validate_iban_str(iban_t) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// indicate if the iban is valid or not and provide an explanation when there is an error
/// Validate the IBAN and return a tuple of (bool, String)
#[pyfunction]
fn validate_iban_with_error(iban_t: &str) -> PyResult<(bool, String)> {
    match iban_validation_rs::validate_iban_str(iban_t) {
        Ok(_) => Ok((true, String::new())),
        Err(e) => Ok((false, format!("IBAN Validation failed: {e}"))),
    }
}

/// Error codes for IBAN validation failures
enum IbanErrorCode {
    Valid = 0,
    TooShort = 1,
    MissingCountry = 2,
    InvalidCountry = 3,
    StructureIncorrectForCountry = 4,
    InvalidSizeForCountry = 5,
    ModuloIncorrect = 6,
}

/// Validate the IBAN and return an error code
/// Returns 0 if valid, or a specific error code (1-6) for different validation failures
#[pyfunction]
fn validate_iban_error_code(iban_t: &str) -> PyResult<i32> {
    match iban_validation_rs::validate_iban_str(iban_t) {
        Ok(_) => Ok(IbanErrorCode::Valid as i32),
        Err(e) => {
            let error_code = match e {
                iban_validation_rs::ValidationError::TooShort(_) => IbanErrorCode::TooShort,
                iban_validation_rs::ValidationError::MissingCountry => IbanErrorCode::MissingCountry,
                iban_validation_rs::ValidationError::InvalidCountry => IbanErrorCode::InvalidCountry,
                iban_validation_rs::ValidationError::StructureIncorrectForCountry => IbanErrorCode::StructureIncorrectForCountry,
                iban_validation_rs::ValidationError::InvalidSizeForCountry => IbanErrorCode::InvalidSizeForCountry,
                iban_validation_rs::ValidationError::ModuloIncorrect => IbanErrorCode::ModuloIncorrect,
            };
            Ok(error_code as i32)
        }
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
}

#[pymethods]
impl IbanValidation {
    /// Constructor for PyIban. Returns a dictionary-like object for Python.
    #[new]
    pub fn new(s: &str) -> PyResult<Self> {
        match iban_validation_rs::Iban::new(s) {
            Ok(iban) => Ok(Self {
                stored_iban: Some(iban.get_iban().to_string()),
                iban_bank_id: Some(iban.iban_bank_id.map(|x| x.to_string())),
                iban_branch_id: Some(iban.iban_branch_id.map(|x| x.to_string())),
            }),
            Err(_) => Ok(Self {
                stored_iban: None,
                iban_bank_id: None,
                iban_branch_id: None,
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
}

/// wrap the class and function into the module for python
#[pymodule]
fn iban_validation_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_iban, m)?)?;
    m.add_function(wrap_pyfunction!(validate_iban_with_error, m)?)?;
    m.add_function(wrap_pyfunction!(validate_iban_error_code, m)?)?;
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

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
