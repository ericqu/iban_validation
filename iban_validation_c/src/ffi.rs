use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use iban_validation_rs::{validate_iban_str, Iban, ValidationError};

/// Error codes for IBAN validation
#[repr(C)]
pub enum IbanErrorCode {
    /// IBAN is valid
    Valid = 1,
    /// IBAN is invalid for unspecified reason
    Invalid = 0,
    /// IBAN is too short
    TooShort = -1,
    /// IBAN is missing country code
    MissingCountry = -2,
    /// IBAN has invalid country code
    InvalidCountry = -3,
    /// IBAN structure is incorrect for the country
    StructureIncorrect = -4,
    /// IBAN length is invalid for the country
    InvalidSize = -5,
    /// IBAN checksum (mod-97) is incorrect
    ModuloFailed = -6,
}

/// Validates an IBAN string and returns a status code
/// 
/// @param iban_str A null-terminated C string containing the IBAN to validate
/// @return Status code (see IbanErrorCode enum values)
#[unsafe(no_mangle)]
pub extern "C" fn iban_validate(iban_str: *const c_char) -> c_int {
    // Safety check for null pointer
    if iban_str.is_null() {
        return IbanErrorCode::MissingCountry as c_int;
    }

    // Convert C string to Rust string
    let iban_cstr = unsafe { CStr::from_ptr(iban_str) };
    let iban_rust_str = match iban_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return IbanErrorCode::StructureIncorrect as c_int,
    };

    // Validate the IBAN
    match validate_iban_str(iban_rust_str) {
        Ok(true) => IbanErrorCode::Valid as c_int,
        Ok(false) => IbanErrorCode::Invalid as c_int,
        Err(err) => match err {
            ValidationError::TooShort(_) => IbanErrorCode::TooShort as c_int,
            ValidationError::MissingCountry => IbanErrorCode::MissingCountry as c_int,
            ValidationError::InvalidCountry => IbanErrorCode::InvalidCountry as c_int,
            ValidationError::StructureIncorrectForCountry => IbanErrorCode::StructureIncorrect as c_int,
            ValidationError::InvalidSizeForCountry => IbanErrorCode::InvalidSize as c_int,
            ValidationError::ModuloIncorrect => IbanErrorCode::ModuloFailed as c_int,
        },
    }
}

/// Represents the IBAN data structure for C
#[repr(C)]
pub struct IbanData {
    iban: *mut c_char,
    bank_id: *mut c_char,
    branch_id: *mut c_char,
}

/// Creates a new IBAN structure from a string
/// 
/// @param iban_str A null-terminated C string containing the IBAN
/// @return A pointer to an IbanData structure if valid, NULL otherwise
/// 
/// Note: The caller is responsible for freeing the memory by calling iban_free
#[unsafe(no_mangle)]
pub extern "C" fn iban_new(iban_str: *const c_char) -> *mut IbanData {
    // Safety check for null pointer
    if iban_str.is_null() {
        return ptr::null_mut();
    }

    // Convert C string to Rust string
    let iban_cstr = unsafe { CStr::from_ptr(iban_str) };
    let iban_rust_str = match iban_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // Create the Iban struct
    let iban = match Iban::new(iban_rust_str) {
        Ok(iban) => iban,
        Err(_) => return ptr::null_mut(),
    };

    // Allocate memory for the C struct
    let mut iban_data = Box::new(IbanData {
        iban: ptr::null_mut(),
        bank_id: ptr::null_mut(),
        branch_id: ptr::null_mut(),
    });

    // Copy the IBAN string
    match CString::new(iban.get_iban()) {
        Ok(s) => iban_data.iban = s.into_raw(),
        Err(_) => return ptr::null_mut(),
    }

    // Copy the bank ID if available
    if let Some(bank_id) = iban.iban_bank_id {
        match CString::new(bank_id) {
            Ok(s) => iban_data.bank_id = s.into_raw(),
            Err(_) => {
                // Clean up the already allocated iban string
                unsafe {
                    let _ = CString::from_raw(iban_data.iban);
                }
                return ptr::null_mut();
            }
        }
    }

    // Copy the branch ID if available
    if let Some(branch_id) = iban.iban_branch_id {
        match CString::new(branch_id) {
            Ok(s) => iban_data.branch_id = s.into_raw(),
            Err(_) => {
                // Clean up the already allocated strings
                unsafe {
                    let _ = CString::from_raw(iban_data.iban);
                    if !iban_data.bank_id.is_null() {
                        let _ = CString::from_raw(iban_data.bank_id);
                    }
                }
                return ptr::null_mut();
            }
        }
    }

    Box::into_raw(iban_data)
}

/// Frees the memory allocated for an IbanData structure
/// 
/// @param iban_data Pointer to the IbanData structure to free
#[unsafe(no_mangle)]
pub extern "C" fn iban_free(iban_data: *mut IbanData) {
    if iban_data.is_null() {
        return;
    }

    unsafe {
        let data = Box::from_raw(iban_data);
        
        // Free all the C strings
        if !data.iban.is_null() {
            let _ = CString::from_raw(data.iban);
        }
        
        if !data.bank_id.is_null() {
            let _ = CString::from_raw(data.bank_id);
        }
        
        if !data.branch_id.is_null() {
            let _ = CString::from_raw(data.branch_id);
        }
        
        // Box will be dropped here, freeing the IbanData struct
    }
}

/// Gets error message for a specific error code
/// 
/// @param error_code The error code returned by iban_validate
/// @return A null-terminated string with the error message (do not free this string)
#[unsafe(no_mangle)]
pub extern "C" fn iban_error_message(error_code: c_int) -> *const c_char {
    // Use static strings that persist for the program's lifetime
    static VALID: &'static [u8] = b"Valid IBAN\0";
    static INVALID: &'static [u8] = b"Invalid IBAN\0";
    static TOO_SHORT: &'static [u8] = b"IBAN is too short\0";
    static MISSING_COUNTRY: &'static [u8] = b"IBAN is missing country code\0";
    static INVALID_COUNTRY: &'static [u8] = b"IBAN has invalid country code\0";
    static STRUCTURE_INCORRECT: &'static [u8] = b"IBAN structure is incorrect for the country\0";
    static INVALID_SIZE: &'static [u8] = b"IBAN length is invalid for the country\0";
    static MODULO_FAILED: &'static [u8] = b"IBAN checksum (mod-97) is incorrect\0";
    static UNKNOWN: &'static [u8] = b"Unknown error code\0";
    
    let bytes = match error_code {
        1 => VALID,
        0 => INVALID,
        -1 => TOO_SHORT,
        -2 => MISSING_COUNTRY,
        -3 => INVALID_COUNTRY,
        -4 => STRUCTURE_INCORRECT,
        -5 => INVALID_SIZE,
        -6 => MODULO_FAILED,
        _ => UNKNOWN,
    };
    
    bytes.as_ptr() as *const c_char
}