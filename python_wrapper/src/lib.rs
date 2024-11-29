use pyo3::prelude::*; // For Python bindings
use core_iban_valid; 
// use pyo3::exceptions::PyValueError;

/// indicate if the iban is valid or not
#[pyfunction]
fn validate_iban(iban_t: &str) -> PyResult<bool> {
    match core_iban_valid::validate_iban_str(iban_t) {
        Ok(_) => Ok(true),
        Err(e) => {
            eprintln!("IBAN Validation failed: {}", e);
            Ok(false)
        }
    }
}

#[pyclass]
pub struct PyVIban {
    stored_iban: Option<String>,
    iban_bank_id: Option<Option<String>>, // Outer Option for validation
    iban_branch_id: Option<Option<String>>,
}

#[pymethods]
impl PyVIban {
    /// Constructor for PyIban. Returns a dictionary-like object for Python.
    #[new]
    pub fn new(s: &str) -> PyResult<Self> {
        match core_iban_valid::Iban::new(s) {
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


#[pymodule]
fn py_viban(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_iban, m)?)?;
    m.add_class::<PyVIban>()?;
    Ok(())
}