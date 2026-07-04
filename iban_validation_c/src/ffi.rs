// This is experimental and subject to change
//  Modified ffi.rs with zero-copy optimization

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;

use iban_validation_rs::{
    Iban, ValidationError, validate_iban_get_numeric, validate_iban_str, validate_iban_with_data,
};

const MIN_IBAN_LEN: usize = iban_validation_rs::IBAN_MIN_LEN as usize;
const MAX_IBAN_LEN: usize = iban_validation_rs::IBAN_MAX_LEN as usize;

/// Maps a core validation error to the corresponding C error code.
/// Single source of truth so adding a `ValidationError` variant only needs one update.
#[inline]
fn map_validation_error(err: ValidationError) -> c_int {
    match err {
        ValidationError::TooShort(_) => IbanErrorCode::TooShort as c_int,
        ValidationError::MissingCountry => IbanErrorCode::MissingCountry as c_int,
        ValidationError::InvalidCountry => IbanErrorCode::InvalidCountry as c_int,
        ValidationError::StructureIncorrectForCountry => {
            IbanErrorCode::StructureIncorrectForCountry as c_int
        }
        ValidationError::InvalidSizeForCountry => IbanErrorCode::InvalidSize as c_int,
        ValidationError::ModuloIncorrect => IbanErrorCode::ModuloFailed as c_int,
        ValidationError::InvalidChecksum => IbanErrorCode::InvalidChecksum as c_int,
    }
}

/// Error codes for IBAN validation (unchanged)
#[repr(C)]
pub enum IbanErrorCode {
    Valid = 1,
    Invalid = 0, // non plausible length or non utf-8 characters
    TooShort = -1,
    MissingCountry = -2,
    InvalidCountry = -3,
    StructureIncorrectForCountry = -4,
    InvalidSize = -5,
    ModuloFailed = -6,
    InvalidChecksum = -7,
}

/// A zero-copy string view for C strings.
///
/// Legacy/NUL-terminated-buffer path (backs `iban_get_view`). Not recommended for
/// buffers that aren't NUL-terminated (e.g. DuckDB's `string_t`) - use
/// `iban_validate_short`/`iban_validate_span` instead for that calling convention.
#[repr(C)]
pub struct StringView {
    ptr: *const c_char,
    len: usize,
}

/// A zero-copy string view for C strings
#[repr(C)]
pub struct IbanValidationResult {
    is_valid: bool, // is it a valid iban
    bank_s: u8,     // bank id starting point
    bank_e: u8,     // bank id end point, when zero it is not available
    branch_s: u8,   // branch id starting point
    branch_e: u8,   // branch id end point, when zero it is not available
}

impl StringView {
    /// Safely converts to a &str reference without copying
    #[inline]
    unsafe fn as_str<'a>(&self) -> Option<&'a str> {
        if self.ptr.is_null() {
            return None;
        }
        let bytes = unsafe { slice::from_raw_parts(self.ptr as *const u8, self.len) };
        std::str::from_utf8(bytes).ok()
    }

    /// Creates a StringView from a C string
    #[inline]
    unsafe fn from_c_string(ptr: *const c_char) -> Option<StringView> {
        if ptr.is_null() {
            return None;
        }

        // Find the null terminator, scanning from byte 0 so a string shorter than
        // MAX_IBAN_LEN can never cause an out-of-bounds read.
        let mut len = 0;
        while len <= MAX_IBAN_LEN {
            if unsafe { *ptr.add(len) } == 0 {
                break;
            }
            len += 1;
        }
        if !(MIN_IBAN_LEN..=MAX_IBAN_LEN).contains(&len) {
            return None;
        }
        Some(StringView { ptr, len })
    }
}

/// Optimized IBAN validation using zero-copy approach
///
/// @param iban_str A null-terminated C string containing the IBAN to validate
/// @param result the results needed to build the branch_id and bank_id (when available)
/// @param len Length of the string (if known), pass 0 to auto-detect length
/// @return Status code (see IbanErrorCode enum values)
///
/// # Safety
/// The input must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_validate_short(
    iban_str: *const c_char,
    len: usize,
    result: *mut IbanValidationResult,
) -> c_int {
    // Safety check for null pointer
    if iban_str.is_null() {
        unsafe {
            (*result).is_valid = false;
            (*result).bank_s = 0;
            (*result).bank_e = 0;
            (*result).branch_s = 0;
            (*result).branch_e = 0;
        };
        return IbanErrorCode::MissingCountry as c_int;
    }

    let actual_len = match len {
        0 => {
            // Find the null terminator (caller guarantees a NUL-terminated string
            // when len == 0, per this function's safety contract). The resulting
            // actual_len is exactly the number of bytes before that NUL, so slicing
            // it below is always in-bounds - no separate range check needed here;
            // out-of-range lengths are handled downstream via MissingCountry /
            // InvalidSizeForCountry, same as before this fix.
            let mut i = 0;
            while unsafe { *iban_str.add(i) != 0 } {
                i += 1;
            }
            i
        }
        n => {
            // Explicit caller-supplied length: this is untrusted and must never be
            // coerced to a different value (the previous `_ => 15` fallback did
            // exactly that, causing an out-of-bounds read when `n` didn't match the
            // caller's actual buffer size). Reject out-of-range lengths outright,
            // before ever constructing a slice over the buffer.
            if !(MIN_IBAN_LEN..=MAX_IBAN_LEN).contains(&n) {
                unsafe {
                    (*result).is_valid = false;
                    (*result).bank_s = 0;
                    (*result).bank_e = 0;
                    (*result).branch_s = 0;
                    (*result).branch_e = 0;
                };
                return IbanErrorCode::InvalidSize as c_int;
            }
            n
        }
    };

    // Create a byte slice without copying or allocation
    let bytes = unsafe { slice::from_raw_parts(iban_str as *const u8, actual_len) };

    // Convert to &str without copying - only validates UTF-8
    let iban_rust_str = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return IbanErrorCode::Invalid as c_int,
    };

    // Validate the IBAN
    match validate_iban_get_numeric(iban_rust_str) {
        Ok((true, bank_s, bank_e, branch_s, branch_e)) => {
            unsafe {
                (*result).is_valid = true;
                (*result).bank_s = bank_s;
                (*result).bank_e = bank_e;
                (*result).branch_s = branch_s;
                (*result).branch_e = branch_e;
            };
            IbanErrorCode::Valid as c_int
        }
        Ok((false, _, _, _, _)) => IbanErrorCode::Invalid as c_int,
        Err(err) => map_validation_error(err),
    }
}

/// Optimized IBAN validation using zero-copy approach
///
/// @param iban_str A null-terminated C string containing the IBAN to validate
/// @param len Length of the string (if known), pass 0 to auto-detect length
/// @return Status code (see IbanErrorCode enum values)
///
/// # Safety
/// The input must be a valid null-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_validate_optimized(iban_str: *const c_char, len: usize) -> c_int {
    // Safety check for null pointer
    if iban_str.is_null() {
        return IbanErrorCode::MissingCountry as c_int;
    }

    let actual_len = match len {
        0 => {
            // Find the null terminator (caller guarantees a NUL-terminated string
            // when len == 0, per this function's safety contract). See
            // iban_validate_short for why no separate range check is needed here.
            let mut i = 0;
            while unsafe { *iban_str.add(i) != 0 } {
                i += 1;
            }
            i
        }
        n => {
            // Explicit caller-supplied length: reject out-of-range values outright
            // rather than coercing them (see iban_validate_short for rationale).
            if !(MIN_IBAN_LEN..=MAX_IBAN_LEN).contains(&n) {
                return IbanErrorCode::InvalidSize as c_int;
            }
            n
        }
    };

    // Create a byte slice without copying or allocation
    let bytes = unsafe { slice::from_raw_parts(iban_str as *const u8, actual_len) };

    // Convert to &str without copying - only validates UTF-8
    let iban_rust_str = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return IbanErrorCode::Invalid as c_int,
    };

    // Validate the IBAN
    match validate_iban_str(iban_rust_str) {
        Ok(true) => IbanErrorCode::Valid as c_int,
        Ok(false) => IbanErrorCode::Invalid as c_int,
        Err(err) => map_validation_error(err),
    }
}

/// Zero-copy IBAN validation over an explicit byte span.
///
/// Purpose-built for callers whose buffers are NOT NUL-terminated (e.g. DuckDB's
/// `duckdb_string_t`/`string_t`, which is a pointer+length pair with no guaranteed
/// trailing NUL). Unlike `iban_validate_short`, `len == 0` always means "empty input"
/// (rejected immediately) and never triggers NUL-scanning. Never reads more than
/// `len` bytes from `iban_str`.
///
/// @param iban_str Pointer to `len` bytes of IBAN data (need not be NUL-terminated)
/// @param len Exact number of valid bytes at `iban_str`
/// @param result the results needed to build the branch_id and bank_id (when available)
/// @return Status code (see IbanErrorCode enum values)
///
/// # Safety
/// If `len > 0`, `iban_str` must point to at least `len` readable bytes. If `len == 0`,
/// `iban_str` is never dereferenced and may be null or dangling. `result` must point to
/// a valid `IbanValidationResult`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_validate_span(
    iban_str: *const c_char,
    len: usize,
    result: *mut IbanValidationResult,
) -> c_int {
    unsafe {
        (*result).is_valid = false;
        (*result).bank_s = 0;
        (*result).bank_e = 0;
        (*result).branch_s = 0;
        (*result).branch_e = 0;
    };

    if !(MIN_IBAN_LEN..=MAX_IBAN_LEN).contains(&len) {
        return IbanErrorCode::InvalidSize as c_int;
    }

    if iban_str.is_null() {
        return IbanErrorCode::MissingCountry as c_int;
    }

    // Create a byte slice without copying or allocation - bounded strictly by `len`
    let bytes = unsafe { slice::from_raw_parts(iban_str as *const u8, len) };

    // Convert to &str without copying - only validates UTF-8
    let iban_rust_str = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return IbanErrorCode::Invalid as c_int,
    };

    match validate_iban_get_numeric(iban_rust_str) {
        Ok((true, bank_s, bank_e, branch_s, branch_e)) => {
            unsafe {
                (*result).is_valid = true;
                (*result).bank_s = bank_s;
                (*result).bank_e = bank_e;
                (*result).branch_s = branch_s;
                (*result).branch_e = branch_e;
            };
            IbanErrorCode::Valid as c_int
        }
        Ok((false, _, _, _, _)) => IbanErrorCode::Invalid as c_int,
        Err(err) => map_validation_error(err),
    }
}

/// Represents the IBAN data structure for C with zero-copy considerations
#[repr(C)]
pub struct IbanDataView {
    iban: StringView,      /* The IBAN as a string view */
    bank_id: StringView,   /* Bank identifier as a string view */
    branch_id: StringView, /* Branch identifier as a string view */
}

/// Modified version of IbanData for when we need to own strings
#[repr(C)]
pub struct IbanData {
    iban: *mut c_char,
    bank_id: *mut c_char,
    branch_id: *mut c_char,
}

/// Gets IBAN information without copying strings
/// Note: The returned data is only valid while iban_str is valid
///
/// Legacy/NUL-terminated-buffer path. Not recommended for buffers that aren't
/// NUL-terminated (e.g. DuckDB's `string_t`) - use `iban_validate_short`/
/// `iban_validate_span` instead for that calling convention.
///
/// @param iban_str A null-terminated C string containing the IBAN
/// @param out_data Pointer to an IbanDataView structure to fill
/// @return 1 if valid, 0 or negative error code otherwise
///
/// # Safety
/// This requires valid pointers and the caller must ensure iban_str remains valid
/// while the returned view is in use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iban_get_view(
    iban_str: *const c_char,
    out_data: *mut IbanDataView,
) -> c_int {
    // Safety check for null pointers
    if iban_str.is_null() || out_data.is_null() {
        return IbanErrorCode::MissingCountry as c_int;
    }

    // Get string view
    let view = match unsafe { StringView::from_c_string(iban_str) } {
        Some(v) => v,
        None => return IbanErrorCode::Invalid as c_int,
    };

    // Convert to &str without copying
    let iban_rust_str = match unsafe { view.as_str() } {
        Some(s) => s,
        None => return IbanErrorCode::Invalid as c_int,
    };

    // Create the Iban struct
    let iban_fields = match validate_iban_with_data(iban_rust_str) {
        Ok((ibf, _)) => ibf,
        Err(err) => return map_validation_error(err),
    };

    // Fill out the data view structure
    unsafe { (*out_data).iban = view };

    // For bank_id and branch_id, we need to handle them differently since they're
    // extracted substrings that don't directly reference parts of the input string

    // For a zero-copy solution, we need to extract positions of bank_id and branch_id
    // from the original IBAN string if possible
    if let (Some(start), Some(end)) = (iban_fields.bank_id_pos_s, iban_fields.bank_id_pos_e) {
        unsafe {
            (*out_data).bank_id = StringView {
                ptr: iban_str.add(start + 3), // Point to the substring in the original string
                len: 1 + end - start,
            }
        };
    }

    if let (Some(start), Some(end)) = (iban_fields.branch_id_pos_s, iban_fields.branch_id_pos_e) {
        unsafe {
            (*out_data).branch_id = StringView {
                ptr: iban_str.add(start + 3), // Point to the substring in the original string
                len: 1 + end - start,
            }
        };
    }

    IbanErrorCode::Valid as c_int
}

/// Original iban_validate function (unchanged - kept for compatibility)
/// # Safety
/// This requires valid pointers and the caller must ensure iban_str remains valid
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
        Err(_) => return IbanErrorCode::StructureIncorrectForCountry as c_int,
    };

    // Validate the IBAN
    match validate_iban_str(iban_rust_str) {
        Ok(true) => IbanErrorCode::Valid as c_int,
        Ok(false) => IbanErrorCode::Invalid as c_int,
        Err(err) => map_validation_error(err),
    }
}

/// Original iban_new function (unchanged - kept for compatibility)
/// # Safety
/// This requires valid pointers and the caller must ensure iban_str remains valid
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

/// Original iban_free function (unchanged)
/// # Safety
/// This requires valid pointer
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
    static INVALID_CHECKSUM: &[u8] = b"IBAN checksum is invalid (00, 01, or 99 not allowed)\0";
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
        -7 => INVALID_CHECKSUM,
        _ => UNKNOWN,
    };

    bytes.as_ptr() as *const c_char
}

/// Returns the library version as a C string
///
/// @return A null-terminated string with the version (do not free this string)
#[unsafe(no_mangle)]
pub extern "C" fn iban_version() -> *const c_char {
    // Define a static version string that persists for the program's lifetime
    // This uses the version from Cargo.toml during compile time
    static VERSION: &[u8] = concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes();

    VERSION.as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_optimized_valid_ibans() {
        let valid_ibans = [
            "DE89370400440532013000",      // Germany
            "GB29NWBK60161331926819",      // UK
            "FR1420041010050500013M02606", // France
            "CH9300762011623852957",       // Switzerland
            "NL91ABNA0417164300",          // Netherlands
        ];

        for iban in valid_ibans.iter() {
            let c_iban = CString::new(*iban).unwrap();
            let result = unsafe { iban_validate_optimized(c_iban.as_ptr(), 0) };
            assert_eq!(
                result,
                IbanErrorCode::Valid as i32,
                "IBAN {} should be valid",
                iban
            );
        }
    }
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
    fn test_invalid_ibans_optimized() {
        let test_cases = [
            ("DE8937040044053201300", IbanErrorCode::InvalidSize as i32), // Too short for Germany
            (
                "XX89370400440532013000",
                IbanErrorCode::InvalidCountry as i32,
            ), // Invalid country code
            (
                "DE00370400440532013000",
                IbanErrorCode::InvalidChecksum as i32,
            ), // Invalid checksum
            ("DE96370400440532013000", IbanErrorCode::ModuloFailed as i32), // failed modulo
            ("D", IbanErrorCode::MissingCountry as i32),                  // Too short overall
            ("", IbanErrorCode::MissingCountry as i32),                   // Empty string
            (
                "DE893704004405320130001234",
                IbanErrorCode::InvalidSize as i32,
            ), // Too long for Germany
        ];

        for (iban, expected_error) in test_cases.iter() {
            let c_iban = CString::new(*iban).unwrap();
            let result = unsafe { iban_validate_optimized(c_iban.as_ptr(), 0) };
            assert_eq!(
                result, *expected_error,
                "IBAN {} should return error code {}",
                iban, expected_error
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
            (
                "DE00370400440532013000",
                IbanErrorCode::InvalidChecksum as i32,
            ), // Invalid checksum
            ("DE96370400440532013000", IbanErrorCode::ModuloFailed as i32), // failed modulo
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
        let result = unsafe { iban_validate_optimized(ptr::null(), 0) };
        assert_eq!(
            result,
            IbanErrorCode::MissingCountry as i32,
            "Null pointer should return MissingCountry error"
        );
    }

    // Test iban_new functionality
    #[test]
    fn test_iban_new_optimized() {
        // Test with a valid IBAN
        let valid_iban_str = "DE89370400440532013000"; // German IBAN with bank code 37040044
        let c_iban = CString::new(valid_iban_str).unwrap();

        let iban = Box::new(IbanDataView {
            iban: StringView {
                ptr: c_iban.as_ptr(),
                len: c_iban.count_bytes(),
            },
            bank_id: StringView {
                ptr: std::ptr::null(),
                len: 0,
            },
            branch_id: StringView {
                ptr: std::ptr::null(),
                len: 0,
            },
        });

        let iban_view: *mut IbanDataView = Box::into_raw(iban);

        let result = unsafe { iban_get_view(c_iban.as_ptr(), iban_view) };
        assert_eq!(result, IbanErrorCode::Valid as i32);
        assert_eq!(
            unsafe { (*iban_view).iban.as_str().unwrap() },
            valid_iban_str
        );
        assert_eq!(
            unsafe { (*iban_view).bank_id.as_str().unwrap() },
            "37040044"
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

    #[test]
    fn test_version() {
        let version_ptr = iban_version();
        let version = unsafe { CStr::from_ptr(version_ptr).to_str().unwrap() };

        // This test verifies that we can retrieve the version without crashing
        // The actual version value will depend on what's in Cargo.toml
        assert!(!version.is_empty(), "Version string should not be empty");

        // Optional: You can assert the specific version if you want to ensure it matches
        // assert_eq!(version, "0.1.0");  // Replace with your actual version
    }

    #[test]
    fn test_valid_ibans_short() {
        let test_cases = [
            ("GB82WEST12345698765432", "UK IBAN"),
            ("DE89370400440532013000", "German IBAN"),
            ("FR1420041010050500013M02606", "French IBAN"),
            ("IT60X0542811101000000123456", "Italian IBAN"),
            ("ES9121000418450200051332", "Spanish IBAN"),
            ("NL91ABNA0417164300", "Dutch IBAN"),
            ("BE68539007547034", "Belgian IBAN"),
            ("CH9300762011623852957", "Swiss IBAN"),
            ("AT611904300234573201", "Austrian IBAN"),
            ("LU280019400644750000", "Luxembourg IBAN"),
            ("IE29AIBK93115212345678", "Irish IBAN"),
            ("PT50000201231234567890154", "Portuguese IBAN"),
            ("SE4550000000058398257466", "Swedish IBAN"),
            ("DK5000400440116243", "Danish IBAN"),
            ("NO9386011117947", "Norwegian IBAN"),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!(
                "{}: {} -> Status: {}, Valid: {}",
                description, iban, status, result.is_valid
            );

            assert_eq!(
                status,
                IbanErrorCode::Valid as i32,
                "Expected valid IBAN for {}: {}",
                description,
                iban
            );
            assert!(
                result.is_valid,
                "Result should indicate valid IBAN for {}",
                description
            );
        }
    }

    #[test]
    fn test_invalid_ibans_checksum_failures() {
        let test_cases = [
            ("GB82WEST12345698765433", "UK IBAN with wrong checksum"),
            ("DE89370400440532013001", "German IBAN with wrong checksum"),
            (
                "FR1420041010050500013M02607",
                "French IBAN with wrong checksum",
            ),
            (
                "IT60X0542811101000000123457",
                "Italian IBAN with wrong checksum",
            ),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: {} -> Status: {}", description, iban, status);

            assert_eq!(
                status,
                IbanErrorCode::ModuloFailed as i32,
                "Expected modulo failure for {}: {}",
                description,
                iban
            );
            assert!(
                !result.is_valid,
                "Result should indicate invalid IBAN for {}",
                description
            );
        }
    }

    #[test]
    fn test_invalid_country_codes() {
        let test_cases = [
            ("XX82WEST12345698765432", "Invalid country code XX"),
            ("ZZ12345678901234567890", "Invalid country code ZZ"),
            ("G182WEST12345698765432", "Malformed country code G1"),
            ("1B82WEST12345698765432", "Numeric first character"),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: {} -> Status: {}", description, iban, status);

            assert_eq!(
                status,
                IbanErrorCode::InvalidCountry as i32,
                "Expected invalid country for {}: {}",
                description,
                iban
            );
        }
    }

    #[test]
    fn test_structure_incorrect() {
        let test_cases = [
            ("GB82WEST123456987654321", "UK IBAN too long"),
            ("GB82WEST1234569876543", "UK IBAN too short"),
            ("DE89370400440532013", "German IBAN too short"),
            ("DE893704004405320130001", "German IBAN too long"),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: {} -> Status: {}", description, iban, status);

            // Could be StructureIncorrectForCountry or InvalidSize
            assert!(
                status == IbanErrorCode::StructureIncorrectForCountry as i32
                    || status == IbanErrorCode::InvalidSize as i32,
                "Expected structure error for {}: {}",
                description,
                iban
            );
        }
    }

    #[test]
    fn test_too_short() {
        let test_cases = [
            ("", "Empty string"),
            ("G", "Single character"),
            ("GB", "Two characters"),
            ("GB8", "Three characters"),
            ("GB82", "Four characters - minimal but incorrect"),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: '{}' -> Status: {}", description, iban, status);

            // Could be or InvalidSize MissingCountry
            assert!(
                status == IbanErrorCode::MissingCountry as i32
                    || status == IbanErrorCode::InvalidSize as i32,
                "Expected structure error for {}: {}",
                description,
                iban
            );
        }
    }

    #[test]
    fn test_invalid_characters() {
        let test_cases = [
            ("GBXXWEST12345698765432", "Non-numeric check digits"),
            ("GB82WEST12345698765@32", "Special character @ in account"),
            ("GB82WEST123456987654#2", "Special character # in account"),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: {} -> Status: {}", description, iban, status);

            assert_eq!(
                status,
                IbanErrorCode::StructureIncorrectForCountry as i32,
                "Expected invalid error for {}: {}",
                description,
                iban
            );
        }
    }

    #[test]
    fn test_null_pointer_short() {
        let mut result = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };

        let status = unsafe { iban_validate_short(ptr::null(), 0, &mut result) };

        println!("NULL pointer -> Status: {}", status);

        assert_eq!(status, IbanErrorCode::MissingCountry as i32);
        assert!(!result.is_valid);
        assert_eq!(result.bank_s, 0);
        assert_eq!(result.bank_e, 0);
        assert_eq!(result.branch_s, 0);
        assert_eq!(result.branch_e, 0);
    }

    #[test]
    fn test_explicit_length() {
        let iban = "GB82WEST12345698765432";
        let c_string = CString::new(iban).expect("CString::new failed");
        let mut result = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };

        // Test with correct explicit length
        let status = unsafe { iban_validate_short(c_string.as_ptr(), iban.len(), &mut result) };
        println!("Explicit correct length -> Status: {}", status);
        assert_eq!(status, IbanErrorCode::Valid as i32);
        assert!(result.is_valid);

        // Test with wrong explicit length (too short)
        let mut result2 = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };
        let status2 = unsafe { iban_validate_short(c_string.as_ptr(), 10, &mut result2) };
        println!("Explicit wrong length (10) -> Status: {}", status2);
        // Should fail due to incorrect length - exact error depends on implementation
        assert_eq!(status2, IbanErrorCode::InvalidSize as i32);
    }

    #[test]
    fn test_bank_branch_extraction() {
        let test_cases = [
            ("GB82WEST12345698765432", "GB"),
            ("DE89370400440532013000", "DE"),
            ("FR1420041010050500013M02606", "FR"),
            ("IT60X0542811101000000123456", "IT"),
        ];

        for (iban, country) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            if status == IbanErrorCode::Valid as i32 {
                println!("{} {}:", country, iban);
                println!("  Bank: positions {}-{}", result.bank_s, result.bank_e);
                println!(
                    "  Branch: positions {}-{}",
                    result.branch_s, result.branch_e
                );

                // Validate that positions are reasonable
                assert!(
                    result.bank_s >= 4,
                    "Bank start should be after country and check digits"
                );
                if result.bank_e > 0 {
                    assert!(
                        result.bank_e > result.bank_s,
                        "Bank end should be after bank start"
                    );
                    assert!(
                        result.bank_e <= iban.len() as u8,
                        "Bank end should be within IBAN length"
                    );

                    // Extract and display bank code
                    let bank_code = &iban[result.bank_s as usize..result.bank_e as usize];
                    println!("  Bank code: {}", bank_code);
                    assert!(!bank_code.is_empty(), "Bank code should not be empty");
                }

                if result.branch_e > 0 {
                    assert!(
                        result.branch_e > result.branch_s,
                        "Branch end should be after branch start"
                    );
                    assert!(
                        result.branch_e <= iban.len() as u8,
                        "Branch end should be within IBAN length"
                    );

                    // Extract and display branch code
                    let branch_code = &iban[result.branch_s as usize..result.branch_e as usize];
                    println!("  Branch code: {}", branch_code);
                    assert!(!branch_code.is_empty(), "Branch code should not be empty");
                }
            } else {
                panic!(
                    "Expected valid IBAN for {}: {}, got status: {}",
                    country, iban, status
                );
            }
        }
    }

    #[test]
    /// only upper case accepted. not a user-facing library.
    fn test_case_sensitivity() {
        let test_cases = [
            ("gb82west12345698765432", "Lowercase"),
            ("Gb82West12345698765432", "Mixed case"),
            ("GB82west12345698765432", "Mixed case 2"),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: {} -> Status: {}", description, iban, status);

            // Could be or StructureIncorrectForCountry or InvalidCountry
            assert!(
                status == IbanErrorCode::InvalidCountry as i32
                    || status == IbanErrorCode::StructureIncorrectForCountry as i32,
                "Expected structure error for {}: {}",
                description,
                iban
            );
        }
    }

    #[test]
    fn test_invalid_utf8() {
        // This test requires creating invalid UTF-8, which is tricky in safe Rust
        // We'll simulate it by creating a raw byte array and converting to *const c_char
        let invalid_utf8 = vec![b'G', b'B', b'8', b'2', 0xFF, 0xFE, 0x00];
        let mut result = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };

        let status =
            unsafe { iban_validate_short(invalid_utf8.as_ptr() as *const i8, 0, &mut result) };

        println!("Invalid UTF-8 -> Status: {}", status);

        assert_eq!(status, IbanErrorCode::Invalid as i32);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_boundary_lengths() {
        // Test minimum and maximum plausible IBAN lengths
        let test_cases = [
            (
                "AD1200012030200359100100",
                "Andorra - 24 chars (one of shortest)",
            ),
            ("SM86U0322509800000000270100", "San Marino - 27 chars"),
            (
                "MT84MALT011000012345MTLCAST001S",
                "Malta - 31 chars (one of longest)",
            ),
        ];

        for (iban, description) in test_cases.iter() {
            let c_string = CString::new(*iban).expect("CString::new failed");
            let mut result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };

            let status = unsafe { iban_validate_short(c_string.as_ptr(), 0, &mut result) };

            println!("{}: {} -> Status: {}", description, iban, status);

            // These should be processed (might be valid or invalid based on checksum)
            assert_ne!(
                status,
                IbanErrorCode::TooShort as i32,
                "Should not be too short for {}",
                description
            );
        }
    }

    // Real, mod-97-verified maximum-length IBAN (RU, 33 chars) - the previous
    // `_ => 15` fallback would have silently truncated this to 15 bytes.
    const RU_MAX_LEN_IBAN: &str = "RU0204452560040702810412345678901";
    // Real minimum-length IBAN (NO, 15 chars).
    const NO_MIN_LEN_IBAN: &str = "NO9386011117947";

    #[test]
    fn test_ru_max_length_round_trip() {
        assert_eq!(RU_MAX_LEN_IBAN.len(), MAX_IBAN_LEN);
        let c_string = CString::new(RU_MAX_LEN_IBAN).unwrap();
        let mut result = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };

        let status =
            unsafe { iban_validate_short(c_string.as_ptr(), RU_MAX_LEN_IBAN.len(), &mut result) };
        assert_eq!(status, IbanErrorCode::Valid as i32);
        assert!(result.is_valid);
        assert_eq!((result.bank_s, result.bank_e), (4, 13));
        assert_eq!((result.branch_s, result.branch_e), (13, 18));

        let mut span_result = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };
        let span_status = unsafe {
            iban_validate_span(
                RU_MAX_LEN_IBAN.as_ptr() as *const c_char,
                RU_MAX_LEN_IBAN.len(),
                &mut span_result,
            )
        };
        assert_eq!(span_status, IbanErrorCode::Valid as i32);
        assert!(span_result.is_valid);
        assert_eq!((span_result.bank_s, span_result.bank_e), (4, 13));
        assert_eq!((span_result.branch_s, span_result.branch_e), (13, 18));
    }

    #[test]
    fn test_no_min_length_round_trip() {
        assert_eq!(NO_MIN_LEN_IBAN.len(), MIN_IBAN_LEN);
        let c_string = CString::new(NO_MIN_LEN_IBAN).unwrap();
        let mut result = IbanValidationResult {
            is_valid: false,
            bank_s: 0,
            bank_e: 0,
            branch_s: 0,
            branch_e: 0,
        };

        let status =
            unsafe { iban_validate_short(c_string.as_ptr(), NO_MIN_LEN_IBAN.len(), &mut result) };
        assert_eq!(status, IbanErrorCode::Valid as i32);
        assert!(result.is_valid);
    }

    /// Reproduces the DuckDB threat model precisely: a buffer sized *exactly* to
    /// `len` bytes (no over-allocation, no NUL guarantee), covering lengths both
    /// inside and outside the valid IBAN range. Before the fix, out-of-range
    /// lengths caused `iban_validate_short`/`iban_validate_optimized` to read 15
    /// bytes from a possibly-shorter buffer - an out-of-bounds read. Run this test
    /// under `cargo +nightly miri test -p iban_validation_c` to prove no UB remains.
    #[test]
    fn test_no_oob_read_for_arbitrary_lengths() {
        for &len in &[0usize, 1, 4, 10, 14, 15, 20, 33, 34, 50, 255] {
            let buf: Vec<u8> = (0..len).map(|i| b'A' + (i % 26) as u8).collect();
            assert_eq!(buf.len(), len);

            let mut span_result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };
            // `Vec::as_ptr()` is well-defined (non-null, dangling-but-valid) even for
            // an empty (len == 0) vector, and `iban_validate_span` never dereferences
            // it when the length check fails first.
            let ptr = buf.as_ptr() as *const c_char;
            let status = unsafe { iban_validate_span(ptr, len, &mut span_result) };
            if !(MIN_IBAN_LEN..=MAX_IBAN_LEN).contains(&len) {
                assert_eq!(
                    status,
                    IbanErrorCode::InvalidSize as i32,
                    "len={len} should be rejected as InvalidSize without reading past the buffer"
                );
                assert!(!span_result.is_valid);
            }

            // iban_validate_short/optimized only accept a real C string (NUL-terminated);
            // append a NUL so length auto-detection isn't exercised, and pass the exact
            // buffer length explicitly - the code path under test.
            let mut nul_terminated = buf.clone();
            nul_terminated.push(0);
            let mut short_result = IbanValidationResult {
                is_valid: false,
                bank_s: 0,
                bank_e: 0,
                branch_s: 0,
                branch_e: 0,
            };
            let short_status = unsafe {
                iban_validate_short(
                    nul_terminated.as_ptr() as *const c_char,
                    len,
                    &mut short_result,
                )
            };
            let optimized_status =
                unsafe { iban_validate_optimized(nul_terminated.as_ptr() as *const c_char, len) };
            // len == 0 means "auto-detect via NUL scan" for these two functions (a
            // different, pre-existing contract from iban_validate_span) - the
            // resulting actual_len is always in-bounds by construction, so no
            // InvalidSize is guaranteed here; the core validator reports its own
            // appropriate error (e.g. MissingCountry for an empty string). The
            // out-of-range rejection only applies to an explicit nonzero length.
            if len != 0 && !(MIN_IBAN_LEN..=MAX_IBAN_LEN).contains(&len) {
                assert_eq!(short_status, IbanErrorCode::InvalidSize as i32);
                assert_eq!(optimized_status, IbanErrorCode::InvalidSize as i32);
            }
        }
    }

    #[test]
    fn test_error_message_invalid_checksum() {
        let message_ptr = iban_error_message(IbanErrorCode::InvalidChecksum as i32);
        let message = unsafe { CStr::from_ptr(message_ptr).to_str().unwrap() };
        assert!(message.to_lowercase().contains("checksum"));
    }

    /// Cheap concurrency regression insurance: with no mutable global state today,
    /// this should always pass - it exists so a future change that adds shared
    /// state (e.g. a cache) breaks loudly instead of silently under concurrent use,
    /// matching the DuckDB requirement of calling this library from many threads.
    #[test]
    fn test_concurrent_validation_matches_single_threaded() {
        let inputs: Vec<(&str, i32)> = vec![
            ("DE89370400440532013000", IbanErrorCode::Valid as i32),
            ("GB29NWBK60161331926819", IbanErrorCode::Valid as i32),
            (RU_MAX_LEN_IBAN, IbanErrorCode::Valid as i32),
            (NO_MIN_LEN_IBAN, IbanErrorCode::Valid as i32),
            (
                "DE00370400440532013000",
                IbanErrorCode::InvalidChecksum as i32,
            ),
            (
                "XX89370400440532013000",
                IbanErrorCode::InvalidCountry as i32,
            ),
        ];

        let handles: Vec<_> = (0..8)
            .map(|_| {
                let inputs = inputs.clone();
                std::thread::spawn(move || {
                    for _ in 0..2000 {
                        for (iban, expected) in &inputs {
                            let mut result = IbanValidationResult {
                                is_valid: false,
                                bank_s: 0,
                                bank_e: 0,
                                branch_s: 0,
                                branch_e: 0,
                            };
                            let status = unsafe {
                                iban_validate_span(
                                    iban.as_ptr() as *const c_char,
                                    iban.len(),
                                    &mut result,
                                )
                            };
                            assert_eq!(status, *expected, "mismatch for {iban} under concurrency");
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("worker thread panicked");
        }
    }
}
