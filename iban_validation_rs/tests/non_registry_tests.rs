#![cfg(feature = "non_registry")]
use iban_validation_rs::*;

/// (country, example_iban) pairs from iban_validation_preprocess/iban_registry_non_registry.csv
const NON_REGISTRY_EXAMPLES: &[(&str, &str)] = &[
    ("AO", "AO49012345678901234567890"),
    ("BF", "BF23012345678901234567890123"),
    ("BJ", "BJ11012345678901234567890123"),
    ("CF", "CF5701234567890123456789012"),
    ("CG", "CG5401234567890123456789012"),
    ("CI", "CI68AB2345678901234567890123"),
    ("CM", "CM3601234567890123456789012"),
    ("CV", "CV10012345678901234567890"),
    ("DZ", "DZ090123456789012345678901"),
    ("GA", "GA3601234567890123456789012"),
    ("GQ", "GQ8501234567890123456789012"),
    ("GW", "GW68012345678901234567890"),
    ("IR", "IR850123456789012345678901"),
    ("KM", "KM6101234567890123456789012"),
    ("MA", "MA36012345678901234567890123"),
    ("MG", "MG6101234567890123456789012"),
    ("ML", "ML03012345678901234567890123"),
    ("MZ", "MZ05012345678901234567890"),
    ("NE", "NE78AB2345678901234567890123"),
    ("SN", "SN06AB2345678901234567890123"),
    ("TD", "TD0701234567890123456789012"),
    ("TG", "TG18AB2345678901234567890123"),
];

#[test]
fn all_non_registry_examples_validate() {
    for (cc, iban) in NON_REGISTRY_EXAMPLES {
        assert_eq!(
            validate_iban_str_with(iban, CountrySet::WithNonRegistry),
            Ok(true),
            "expected {cc} example {iban} to validate"
        );
    }
}

/// The compile-time feature alone must not change the default, registry-only
/// behavior of the plain (non-`_with`) API: opting into the non-registry
/// country set is always an explicit, per-call runtime choice.
#[test]
fn registry_only_mode_rejects_non_registry_examples_even_with_feature_on() {
    for (cc, iban) in NON_REGISTRY_EXAMPLES {
        assert_eq!(
            validate_iban_str(iban),
            Err(ValidationError::InvalidCountry),
            "expected {cc} example {iban} to be rejected in registry-only mode"
        );
        assert_eq!(
            validate_iban_str_with(iban, CountrySet::Registry),
            Err(ValidationError::InvalidCountry),
            "expected {cc} example {iban} to be rejected in registry-only mode"
        );
    }
}

#[test]
fn bank_and_branch_extraction_when_positions_known() {
    // MA: bank 1-5, branch 6-10 within the BBAN
    let iban = Iban::new_with("MA36012345678901234567890123", CountrySet::WithNonRegistry).unwrap();
    assert_eq!(iban.iban_bank_id, Some("01234"));
    assert_eq!(iban.iban_branch_id, Some("56789"));
}

#[test]
fn bank_and_branch_are_none_when_positions_unknown() {
    // AO has no bank/branch position data
    let iban = Iban::new_with("AO49012345678901234567890", CountrySet::WithNonRegistry).unwrap();
    assert_eq!(iban.iban_bank_id, None);
    assert_eq!(iban.iban_branch_id, None);
}

#[test]
fn structure_violation_is_rejected() {
    // MA structure is all digits after the check digits; put a letter where a digit is required.
    let iban = "MA3A012345678901234567890123";
    assert_eq!(
        validate_iban_str_with(iban, CountrySet::WithNonRegistry),
        Err(ValidationError::StructureIncorrectForCountry)
    );
}

#[test]
fn checksum_violation_is_rejected() {
    // AO valid example is AO49...; corrupt the check digits so mod97 fails.
    let iban = "AO48012345678901234567890";
    assert_eq!(
        validate_iban_str_with(iban, CountrySet::WithNonRegistry),
        Err(ValidationError::ModuloIncorrect)
    );
}

#[test]
fn is_non_registry_country_discriminates_correctly() {
    for (_, iban) in NON_REGISTRY_EXAMPLES {
        let cc: [u8; 2] = iban.as_bytes()[0..2].try_into().unwrap();
        assert!(is_non_registry_country(cc), "{iban} should be non-registry");
    }
    assert!(!is_non_registry_country(*b"DE"));
    assert!(!is_non_registry_country(*b"ZZ"));
}

#[test]
fn non_registry_countries_constant_matches_examples() {
    assert_eq!(NON_REGISTRY_COUNTRIES.len(), NON_REGISTRY_EXAMPLES.len());
    for (cc, _) in NON_REGISTRY_EXAMPLES {
        let cc_bytes: [u8; 2] = cc.as_bytes().try_into().unwrap();
        assert!(
            NON_REGISTRY_COUNTRIES.contains(&cc_bytes),
            "{cc} missing from NON_REGISTRY_COUNTRIES"
        );
    }
}
