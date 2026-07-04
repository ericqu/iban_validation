//! A short example illustrating a simple library usage
//!
//! ```
//! extern crate iban_validation_rs;
//! use iban_validation_rs::{validate_iban_str, Iban};
//!
//! // This function attempts to create an IBAN from the input string and displays the IBAN, bank ID, and branch ID if successful — or an error message if the creation fails.
//! fn display_iban_or_error(s: &str){
//!     match Iban::new(s) {
//!         Ok(iban) => {
//!             println!("IBAN: {}", iban.get_iban());
//!             match iban.iban_bank_id {
//!                 Some(bank_id) => println!("Bank ID: {}", bank_id),
//!                 None => println!("Bank ID: Not available"),
//!             }
//!             match iban.iban_branch_id {
//!                 Some(branch_id) => println!("Branch ID: {}", branch_id),
//!                 None => println!("Branch ID: Not available"),
//!             }
//!         }
//!         Err(e) => println!("Failed to create IBAN due to {:?} for input: {:?}", e, s),
//!     }
//! }
//!
//! fn main() {
//!     println!("okay? {:?}", validate_iban_str("DE44500105175407324931"));
//!     display_iban_or_error("DE44500105175407324931");
//!     display_iban_or_error("FR1234");
//! }
//! ```
//!
//! ## `non_registry` feature
//!
//! Off by default. When enabled, makes an additional 22 countries available
//! for validation: IBAN-shaped account numbers that are not published in the
//! official SWIFT IBAN registry (community-sourced from schwifty's
//! `overwrite.json`). Their structure specs carry weaker guarantees than the
//! default, registry-backed country set.
//!
//! Compiling in the feature does not change the default behavior: functions
//! such as [`validate_iban_str`] and [`Iban::new`] remain registry-only. To
//! opt into the non-registry country set at runtime, use the `_with` variants
//! ([`validate_iban_str_with`], [`validate_iban_str_print_with`],
//! [`Iban::new_with`]) with [`CountrySet::WithNonRegistry`].

use iban_definition::get_iban_fields_with;
use std::error::Error;
use std::fmt;

mod iban_definition;
#[cfg(feature = "non_registry")]
pub use iban_definition::NON_REGISTRY_COUNTRIES;
pub use iban_definition::{IBAN_MAX_LEN, IBAN_MIN_LEN, is_non_registry_country};

/// Selects which country set validation should accept.
///
/// `Registry` (the default) only accepts the official SWIFT IBAN registry
/// countries. `WithNonRegistry` additionally accepts the opt-in, community-sourced
/// countries gated behind the `non_registry` Cargo feature; when that feature is
/// not compiled in, `WithNonRegistry` behaves identically to `Registry`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CountrySet {
    #[default]
    Registry,
    WithNonRegistry,
}

impl From<bool> for CountrySet {
    /// `true` maps to [`CountrySet::WithNonRegistry`], `false` to the default
    /// [`CountrySet::Registry`]. Lets callers write `allow_non_registry.into()`.
    fn from(allow_non_registry: bool) -> Self {
        if allow_non_registry {
            CountrySet::WithNonRegistry
        } else {
            CountrySet::Registry
        }
    }
}

type ValidatorFn = fn(u8) -> Result<usize, ValidationLetterError>;

/// indicate which information is expected from the Iban Registry and in the record.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IbanFields {
    /// two-letter country codes as per ISO 3166-1
    pub ctry_cd: [u8; 2],
    /// position of bank identifier starting point
    pub bank_id_pos_s: Option<usize>,
    /// position of bank identifier end point
    pub bank_id_pos_e: Option<usize>,
    /// position of branch identifier starting point
    pub branch_id_pos_s: Option<usize>,
    /// position of branch identifier end point
    pub branch_id_pos_e: Option<usize>,
    /// array of validation functions for each position (generated from the python code)
    iban_struct_validators: &'static [ValidatorFn],
}

/// indicate what types of error the iban validation can detect
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    /// the test Iban is too short for the country
    TooShort(usize),
    /// There is no country in the IBAN
    MissingCountry,
    /// There is no valid country in the IBAN
    InvalidCountry,
    /// Does not follow the structure for the country
    StructureIncorrectForCountry,
    /// The size of the IBAN is not what it should be for the country
    InvalidSizeForCountry,
    /// the modulo mod97 computation for the IBAN is invalid.
    ModuloIncorrect,
    /// to avoid ambiguities some checksum are invalids (00, 01 and 99).
    InvalidChecksum,
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::TooShort(len) => write!(
                f,
                "The input Iban is too short to be an IBAN {len} (minimum length is 4)"
            ),
            ValidationError::MissingCountry => write!(
                f,
                "The input Iban does not appear to start with 2 letters representing a two-letter country code"
            ),
            ValidationError::InvalidCountry => write!(
                f,
                "the input Iban the first two-letter do not match a valid country"
            ),
            ValidationError::StructureIncorrectForCountry => write!(
                f,
                "The characters founds in the input Iban do not follow the country's Iban structure"
            ),
            ValidationError::InvalidSizeForCountry => write!(
                f,
                "The length of the input Iban does match the length for that country"
            ),
            ValidationError::ModuloIncorrect => write!(
                f,
                "The calculated mod97 for the iban indicates an incorrect Iban"
            ),
            ValidationError::InvalidChecksum => {
                write!(f, "The checksum is invalid it can not be 00, 01 or 99")
            }
        }
    }
}
impl Error for ValidationError {}

/// potential error for the per letter validation
#[derive(Debug, PartialEq)]
enum ValidationLetterError {
    NotPartOfRequiredSet,
}

/// internal utility
/// Check the character (byte) is a digit and return the value of that digit.
#[inline]
fn simple_contains_n(c: u8) -> Result<usize, ValidationLetterError> {
    if c.is_ascii_digit() {
        Ok((c - 48) as usize) // 48 is the ascii value of '0'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// check the character is an uppercase A-Z and return a value between 10-36
#[inline]
fn simple_contains_a(c: u8) -> Result<usize, ValidationLetterError> {
    if c.is_ascii_uppercase() {
        Ok((c - 55) as usize) // 55 is to get a 10 from a 'A'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// Check the character is alphanumeric an return the value (0-9 for digit,) 10-36 for letters.
#[inline]
fn simple_contains_c(c: u8) -> Result<usize, ValidationLetterError> {
    if c.is_ascii_digit() {
        Ok((c - 48) as usize)
    } else if c.is_ascii_uppercase() {
        Ok((c - 55) as usize)
    } else if c.is_ascii_lowercase() {
        Ok((c - 87) as usize) // 87 is to get a 10 from a 'a'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// const storage for the comprehensive modulo operation
const MFF_ARRAY: [[u8; 36]; 97] = generate_mff_array();

/// internal utility
/// build an array of precomputed modulo operations
/// the maximum should be 9635 (96 the largest previous, 35 a Z the largest possible)
const fn generate_mff_array() -> [[u8; 36]; 97] {
    let mut array: [[u8; 36]; 97] = [[0; 36]; 97];
    let mut pseudo_acc = 0u8;

    while pseudo_acc < 97 {
        let mut pseudo_newchar = 0u8;

        while pseudo_newchar < 36 {
            let mut result: u32 = pseudo_acc as u32;
            result *= if pseudo_newchar < 10 { 10 } else { 100 }; // Multiply by 10 (or 100 for two-digit numbers)
            // result =  if result == 0 {10} else {result * if pseudo_newchar < 10 {10} else {100}} ; // Multiply by 10 (or 100 for two-digit numbers)
            result = (result + pseudo_newchar as u32) % 97; // and add new digit
            array[pseudo_acc as usize][pseudo_newchar as usize] = result as u8;

            pseudo_newchar += 1;
        }

        pseudo_acc += 1;
    }
    array
}

/// Indicates which file was used a source
pub const fn get_source_file() -> &'static str {
    include_str!("../data/iban_sourcefile.txt")
}

/// Indicates the version used. to be used in other modules like the c wrapper where this infomration is not available.
pub const fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn validate_iban_with_data(input_iban: &str) -> Result<(&IbanFields, bool), ValidationError> {
    validate_iban_with_data_with(input_iban, CountrySet::Registry)
}

/// Same as [`validate_iban_with_data`], but the accepted country set is chosen at
/// runtime via `set`.
pub fn validate_iban_with_data_with(
    input_iban: &str,
    set: CountrySet,
) -> Result<(&IbanFields, bool), ValidationError> {
    let identified_country: [u8; 2] = match input_iban.get(..2) {
        Some(value) => value
            .as_bytes()
            .try_into()
            .map_err(|_| ValidationError::InvalidCountry)?,
        None => return Err(ValidationError::MissingCountry),
    };

    let iban_data: &IbanFields = match get_iban_fields_with(identified_country, set) {
        Some(pattern) => pattern,
        None => return Err(ValidationError::InvalidCountry),
    };

    let validators = &iban_data.iban_struct_validators;

    if validators.len() != input_iban.len() {
        return Err(ValidationError::InvalidSizeForCountry);
    }

    // if we have invalid character at the boundary before starting to check them
    if !input_iban.is_char_boundary(4) {
        return Err(ValidationError::StructureIncorrectForCountry);
    }

    // forbidden checksums: it cannot be 00, 01 or 99
    let check_code = &input_iban.as_bytes()[2..4];
    if matches!(check_code, &[b'0', b'0'] | &[b'0', b'1'] | &[b'9', b'9']) {
        return Err(ValidationError::InvalidChecksum);
    }

    let input_re = input_iban[4..].bytes().chain(input_iban[..4].bytes());

    let mut acc: usize = 0;

    for (validator, byte) in validators.iter().zip(input_re) {
        let m97digit =
            validator(byte).map_err(|_| ValidationError::StructureIncorrectForCountry)?;
        acc = MFF_ARRAY[acc][m97digit] as usize;
    }

    if acc == 1 {
        Ok((iban_data, true))
    } else {
        Err(ValidationError::ModuloIncorrect)
    }
}

/// Validate than an Iban is valid according to the registry information
/// return true when Iban is fine, otherwise returns Error.
pub fn validate_iban_str(input_iban: &str) -> Result<bool, ValidationError> {
    validate_iban_str_with(input_iban, CountrySet::Registry)
}

/// Same as [`validate_iban_str`], but the accepted country set is chosen at
/// runtime via `set`.
pub fn validate_iban_str_with(input_iban: &str, set: CountrySet) -> Result<bool, ValidationError> {
    validate_iban_with_data_with(input_iban, set).map(|(_, is_valid)| is_valid)
}

/// Validate an IBAN in user-friendly (print) format.
/// Spaces are allowed and ignored.
/// Other characters are rejected.
pub fn validate_iban_str_print(input: &str) -> Result<bool, ValidationError> {
    validate_iban_str_print_with(input, CountrySet::Registry)
}

/// Same as [`validate_iban_str_print`], but the accepted country set is chosen at
/// runtime via `set`.
pub fn validate_iban_str_print_with(input: &str, set: CountrySet) -> Result<bool, ValidationError> {
    const RAW_LIMIT: usize = 64;

    let mut raw = input.bytes();

    let mut logical = raw.by_ref().take(RAW_LIMIT + 1).filter(|b| *b != b' ');

    // Read country + check digits
    let mut head = [0u8; 4];
    for position in &mut head {
        *position = logical
            .next()
            .ok_or(ValidationError::InvalidSizeForCountry)?;
    }

    let identified_country: [u8; 2] = head[0..2]
        .try_into()
        .map_err(|_| ValidationError::InvalidCountry)?;

    // Forbidden checksums
    match &head[2..4] {
        [b'0', b'0'] | [b'0', b'1'] | [b'9', b'9'] => return Err(ValidationError::InvalidChecksum),
        _ => {}
    }

    let iban_data =
        get_iban_fields_with(identified_country, set).ok_or(ValidationError::InvalidCountry)?;

    let validators = iban_data.iban_struct_validators;

    // remaining characters + checksum moved to end
    let mut reordered = logical.chain(head);

    // Mod97 computation
    let mut acc: usize = 0;

    for validator in validators {
        let byte = reordered
            .next()
            .ok_or(ValidationError::InvalidSizeForCountry)?;

        let m97digit =
            validator(byte).map_err(|_| ValidationError::StructureIncorrectForCountry)?;
        acc = MFF_ARRAY[acc][m97digit] as usize;
    }

    // if remaining reordered characters: input too long
    if reordered.next().is_some() {
        return Err(ValidationError::InvalidSizeForCountry);
    }

    // Any remaining raw characters = raw input too long
    if raw.next().is_some() {
        return Err(ValidationError::InvalidSizeForCountry);
    }

    if acc == 1 {
        Ok(true)
    } else {
        Err(ValidationError::ModuloIncorrect)
    }
}

/// Validate than an Iban is valid according to the registry information
/// Give the results by numerical values (0,0 when the optional part is missing).
/// This is meant to be used in c wrapper when copying value is expensive.
pub fn validate_iban_get_numeric(
    input_iban: &str,
) -> Result<(bool, u8, u8, u8, u8), ValidationError> {
    validate_iban_get_numeric_with(input_iban, CountrySet::Registry)
}

/// Same as [`validate_iban_get_numeric`], but the accepted country set is chosen
/// at runtime via `set`.
pub fn validate_iban_get_numeric_with(
    input_iban: &str,
    set: CountrySet,
) -> Result<(bool, u8, u8, u8, u8), ValidationError> {
    let (iban_data, result) = validate_iban_with_data_with(input_iban, set)?;

    let (bank_s, bank_e) = match (iban_data.bank_id_pos_s, iban_data.bank_id_pos_e) {
        (Some(start), Some(end)) => (start + 3, end + 4),
        _ => (0, 0),
    };

    let (branch_s, branch_e) = match (iban_data.branch_id_pos_s, iban_data.branch_id_pos_e) {
        (Some(start), Some(end)) => (start + 3, end + 4),
        _ => (0, 0),
    };

    Ok((
        result,
        bank_s as u8,
        bank_e as u8,
        branch_s as u8,
        branch_e as u8,
    ))
}

/// Indicate how a valid Iban is stored.
/// A owned String for the iban, so that if the String we tested is out of scope we have our own copy. TODO is it an issue?
/// If valid for the country the slice of the Iban representing the bank_id bank identifier.
/// If valid for the country the slice of the Iban representing the branch_id Branch identifier.
#[derive(Debug)]
pub struct Iban<'a> {
    // /// owned String not accessible to ensure read-only through reader
    // stored_iban: String,
    stored_iban: &'a str,
    /// Bank identifier when relevant
    pub iban_bank_id: Option<&'a str>,
    /// Branch identifier when relevant
    pub iban_branch_id: Option<&'a str>,
}

/// building a valid Iban (validate and take the relavant slices).
impl<'a> Iban<'a> {
    pub fn new(s: &'a str) -> Result<Self, ValidationError> {
        Self::new_with(s, CountrySet::Registry)
    }

    /// Same as [`Iban::new`], but the accepted country set is chosen at runtime
    /// via `set`.
    pub fn new_with(s: &'a str, set: CountrySet) -> Result<Self, ValidationError> {
        let (iban_data, _) = validate_iban_with_data_with(s, set)?;

        let bank_id = Self::extract_identifier(s, iban_data.bank_id_pos_s, iban_data.bank_id_pos_e);
        let branch_id =
            Self::extract_identifier(s, iban_data.branch_id_pos_s, iban_data.branch_id_pos_e);

        Ok(Self {
            stored_iban: s,
            iban_bank_id: bank_id,
            iban_branch_id: branch_id,
        })
    }

    /// get read-only access to the Iban
    pub fn get_iban(&self) -> &str {
        self.stored_iban
    }

    /// helper function to fill the bank_id and branch_id
    #[inline]
    fn extract_identifier(
        s: &'a str,
        start_pos: Option<usize>,
        end_pos: Option<usize>,
    ) -> Option<&'a str> {
        match (start_pos, end_pos) {
            (Some(start), Some(end)) if start <= end && (4 + end) <= s.len() => {
                Some(&s[start + 3..end + 4])
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests exercising only the public API live in `tests/validation_tests.rs` and
    // `tests/property_tests.rs` as integration tests. This module keeps tests that need
    // access to private internals (char classifiers, the mod97 table, the generated
    // registry data).

    #[test]
    fn simple_contains_n_boundaries() {
        assert_eq!(simple_contains_n(b'0'), Ok(0));
        assert_eq!(simple_contains_n(b'9'), Ok(9));
        assert_eq!(
            simple_contains_n(b'/'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        assert_eq!(
            simple_contains_n(b':'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        assert_eq!(
            simple_contains_n(b'A'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        assert_eq!(
            simple_contains_n(0xFF),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
    }

    #[test]
    fn simple_contains_a_boundaries() {
        assert_eq!(simple_contains_a(b'A'), Ok(10));
        assert_eq!(simple_contains_a(b'Z'), Ok(35));
        assert_eq!(
            simple_contains_a(b'@'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        assert_eq!(
            simple_contains_a(b'['),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        // uppercase-only: lowercase letters must be rejected, unlike simple_contains_c
        assert_eq!(
            simple_contains_a(b'a'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        assert_eq!(
            simple_contains_a(b'0'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
    }

    #[test]
    fn simple_contains_c_boundaries_and_case_fold() {
        assert_eq!(simple_contains_c(b'0'), Ok(0));
        assert_eq!(simple_contains_c(b'9'), Ok(9));
        assert_eq!(simple_contains_c(b'A'), Ok(10));
        assert_eq!(simple_contains_c(b'Z'), Ok(35));
        // lowercase letters fold to the same value as their uppercase counterpart
        assert_eq!(simple_contains_c(b'a'), simple_contains_c(b'A'));
        assert_eq!(simple_contains_c(b'z'), simple_contains_c(b'Z'));
        assert_eq!(
            simple_contains_c(b'@'),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
        assert_eq!(
            simple_contains_c(b' '),
            Err(ValidationLetterError::NotPartOfRequiredSet)
        );
    }

    /// Independently recomputes the mod-97 folding step (ISO 7064 MOD 97-10) for every
    /// (accumulator, digit) pair and compares it against the compile-time-generated
    /// MFF_ARRAY, guarding against transcription errors in `generate_mff_array`.
    #[test]
    fn mff_array_matches_naive_mod97_fold() {
        for acc in 0u32..97 {
            for digit in 0u32..36 {
                let multiplier = if digit < 10 { 10 } else { 100 };
                let expected = (acc * multiplier + digit) % 97;
                assert_eq!(
                    MFF_ARRAY[acc as usize][digit as usize] as u32, expected,
                    "mismatch at acc={acc}, digit={digit}"
                );
            }
        }
    }

    /// Sanity-checks a table of per-country registry data: every definition's
    /// bank/branch positions must be internally consistent and within bounds, guarding
    /// against a bad regeneration from `iban_validation_preprocess/pre_process_registry.py`.
    fn assert_definitions_consistent(defs: &[IbanFields]) {
        for fields in defs {
            let len = fields.iban_struct_validators.len();
            assert!(len > 0, "{:?} has no validators", fields.ctry_cd);

            if let (Some(s), Some(e)) = (fields.bank_id_pos_s, fields.bank_id_pos_e) {
                assert!(s <= e, "{:?} bank_id start after end", fields.ctry_cd);
                assert!(e < len, "{:?} bank_id end out of bounds", fields.ctry_cd);
            }

            if let (Some(s), Some(e)) = (fields.branch_id_pos_s, fields.branch_id_pos_e) {
                assert!(s <= e, "{:?} branch_id start after end", fields.ctry_cd);
                assert!(e < len, "{:?} branch_id end out of bounds", fields.ctry_cd);
            }

            if let (Some(bank_e), Some(branch_s)) = (fields.bank_id_pos_e, fields.branch_id_pos_s) {
                assert!(
                    branch_s > bank_e,
                    "{:?} branch_id overlaps bank_id",
                    fields.ctry_cd
                );
            }
        }
    }

    #[test]
    fn registry_definitions_are_internally_consistent() {
        assert_definitions_consistent(&iban_definition::IBAN_DEFINITIONS);
    }

    /// Same sanity checks as `registry_definitions_are_internally_consistent`, applied
    /// to the opt-in, non-registry country definitions.
    #[test]
    #[cfg(feature = "non_registry")]
    fn non_registry_definitions_are_internally_consistent() {
        assert_definitions_consistent(&iban_definition::NON_REGISTRY_IBAN_DEFINITIONS);
    }

    #[test]
    fn get_iban_fields_looks_up_by_country_code() {
        assert!(iban_definition::get_iban_fields([b'D', b'E']).is_some());
        assert!(iban_definition::get_iban_fields([b'Z', b'Z']).is_none());
    }
}
