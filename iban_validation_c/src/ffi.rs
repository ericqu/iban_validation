use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use iban_validation_rs::{Iban, ValidationError, validate_iban_str};

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
///
/// # Safety
/// This calls unsafe code, when converting the C String. The input must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_validate(iban_str: *const c_char) -> c_int {
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
            ValidationError::StructureIncorrectForCountry => {
                IbanErrorCode::StructureIncorrect as c_int
            }
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
///
/// # Safety
/// This calls unsafe code, when converting the C String. The input must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_new(iban_str: *const c_char) -> *mut IbanData {
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
///
/// # Safety
/// This calls unsafe code, when converting the C String. The input must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_free(iban_data: *mut IbanData) {
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
    static VALID: &[u8] = b"Valid IBAN\0";
    static INVALID: &[u8] = b"Invalid IBAN\0";
    static TOO_SHORT: &[u8] = b"IBAN is too short\0";
    static MISSING_COUNTRY: &[u8] = b"IBAN is missing country code\0";
    static INVALID_COUNTRY: &[u8] = b"IBAN has invalid country code\0";
    static STRUCTURE_INCORRECT: &[u8] = b"IBAN structure is incorrect for the country\0";
    static INVALID_SIZE: &[u8] = b"IBAN length is invalid for the country\0";
    static MODULO_FAILED: &[u8] = b"IBAN checksum (mod-97) is incorrect\0";
    static UNKNOWN: &[u8] = b"Unknown error code\0";

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    // Test valid IBANs from different countries
    #[test]
    fn test_valid_ibans() {
        let valid_ibans = [
            "DE89370400440532013000",      // Germany
            "GB29NWBK60161331926819",      // UK
            "FR1420041010050500013M02606", // France
            "CH9300762011623852957",       // Switzerland
            "NL91ABNA0417164300",          // Netherlands
        ];

        for iban in valid_ibans.iter() {
            let c_iban = CString::new(*iban).unwrap();
            let result = unsafe { iban_validate(c_iban.as_ptr()) };
            assert_eq!(
                result,
                IbanErrorCode::Valid as i32,
                "IBAN {} should be valid",
                iban
            );
        }
    }

    // Test invalid IBANs with different error cases
    #[test]
    fn test_invalid_ibans() {
        let test_cases = [
            ("DE8937040044053201300", IbanErrorCode::InvalidSize as i32), // Too short for Germany
            (
                "XX89370400440532013000",
                IbanErrorCode::InvalidCountry as i32,
            ), // Invalid country code
            ("DE00370400440532013000", IbanErrorCode::ModuloFailed as i32), // Invalid checksum
            ("D", IbanErrorCode::MissingCountry as i32),                  // Too short overall
            ("", IbanErrorCode::MissingCountry as i32),                   // Empty string
            (
                "DE893704004405320130001234",
                IbanErrorCode::InvalidSize as i32,
            ), // Too long for Germany
        ];

        for (iban, expected_error) in test_cases.iter() {
            let c_iban = CString::new(*iban).unwrap();
            let result = unsafe { iban_validate(c_iban.as_ptr()) };
            assert_eq!(
                result, *expected_error,
                "IBAN {} should return error code {}",
                iban, expected_error
            );
        }
    }

    // Test null pointer handling
    #[test]
    fn test_null_pointer() {
        let result = unsafe { iban_validate(ptr::null()) };
        assert_eq!(
            result,
            IbanErrorCode::MissingCountry as i32,
            "Null pointer should return MissingCountry error"
        );
    }

    // Test iban_new functionality
    #[test]
    fn test_iban_new() {
        // Test with a valid IBAN
        let valid_iban = "DE89370400440532013000"; // German IBAN with bank code 37040044
        let c_iban = CString::new(valid_iban).unwrap();

        let iban_data_ptr = unsafe { iban_new(c_iban.as_ptr()) };
        assert!(
            !iban_data_ptr.is_null(),
            "iban_new should return a valid pointer for a valid IBAN"
        );

        // Check the data fields
        unsafe {
            let iban_data = &*iban_data_ptr;

            // Check the IBAN string
            let iban_str = CStr::from_ptr(iban_data.iban).to_str().unwrap();
            assert_eq!(iban_str, valid_iban);

            // Check the bank ID (for German IBANs it should be positions 4-11)
            if !iban_data.bank_id.is_null() {
                let bank_id = CStr::from_ptr(iban_data.bank_id).to_str().unwrap();
                assert_eq!(
                    bank_id, "37040044",
                    "Bank ID should match the expected value"
                );
            } else {
                panic!("Bank ID should not be null for a German IBAN");
            }

            // Clean up
            iban_free(iban_data_ptr);
        }
    }

    // Test iban_new with invalid IBAN
    #[test]
    fn test_iban_new_invalid() {
        let invalid_iban = "INVALID";
        let c_iban = CString::new(invalid_iban).unwrap();

        let iban_data_ptr = unsafe { iban_new(c_iban.as_ptr()) };
        assert!(
            iban_data_ptr.is_null(),
            "iban_new should return null for an invalid IBAN"
        );
    }

    // Test iban_new with null pointer
    #[test]
    fn test_iban_new_null() {
        let iban_data_ptr = unsafe { iban_new(ptr::null()) };
        assert!(
            iban_data_ptr.is_null(),
            "iban_new should return null for a null pointer"
        );
    }

    // Test iban_free with null pointer (should not crash)
    #[test]
    fn test_iban_free_null() {
        unsafe {
            iban_free(ptr::null_mut());
            // If we get here without crashing, the test passes
        }
    }

    // Test error messages
    #[test]
    fn test_error_messages() {
        let error_codes = [
            (1, "Valid IBAN"),                                   // IbanErrorCode::Valid
            (0, "Invalid IBAN"),                                 // IbanErrorCode::Invalid
            (-1, "IBAN is too short"),                           // IbanErrorCode::TooShort
            (-2, "IBAN is missing country code"),                // IbanErrorCode::MissingCountry
            (-3, "IBAN has invalid country code"),               // IbanErrorCode::InvalidCountry
            (-4, "IBAN structure is incorrect for the country"), // IbanErrorCode::StructureIncorrect
            (-5, "IBAN length is invalid for the country"),      // IbanErrorCode::InvalidSize
            (-6, "IBAN checksum (mod-97) is incorrect"),         // IbanErrorCode::ModuloFailed
            (99, "Unknown error code"),                          // Unknown error code
        ];

        for (code, expected_message) in error_codes.iter() {
            let message_ptr = iban_error_message(*code);
            let message = unsafe { CStr::from_ptr(message_ptr).to_str().unwrap() };
            assert_eq!(
                message, *expected_message,
                "Error message for code {} should match",
                code
            );
        }
    }

    // Integration test - validate and then parse IBAN
    #[test]
    fn test_validate_and_parse() {
        let valid_ibans = [
            "DE89370400440532013000", // Germany
            "GB29NWBK60161331926819", // UK
        ];

        for iban in valid_ibans.iter() {
            let c_iban = CString::new(*iban).unwrap();

            // First validate
            let result = unsafe { iban_validate(c_iban.as_ptr()) };
            assert_eq!(result, IbanErrorCode::Valid as i32);

            // Then parse if valid
            if result == IbanErrorCode::Valid as i32 {
                let iban_data_ptr = unsafe { iban_new(c_iban.as_ptr()) };
                assert!(!iban_data_ptr.is_null());

                // Clean up
                unsafe {
                    iban_free(iban_data_ptr);
                }
            }
        }
    }

    // Memory leak test - create and free multiple IBAN objects
    #[test]
    fn test_memory_management() {
        let valid_iban = "DE89370400440532013000";
        let c_iban = CString::new(valid_iban).unwrap();

        // Create and free multiple IBAN objects in a loop
        for _ in 0..100 {
            let iban_data_ptr = unsafe { iban_new(c_iban.as_ptr()) };
            assert!(!iban_data_ptr.is_null());
            unsafe {
                iban_free(iban_data_ptr);
            }
        }
        // If no memory leaks, this test should complete without issues
    }
}
