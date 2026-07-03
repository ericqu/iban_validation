use iban_validation_rs::{Iban, validate_iban_str, validate_iban_str_print};
use proptest::prelude::*;

/// A handful of known-valid IBANs, one per distinct total length/structure, used as seeds
/// for the structured (near-valid) property tests below.
const SEED_IBANS: &[&str] = &[
    "DE89370400440532013000",
    "GB29NWBK60161331926819",
    "AL47212110090000000235698741",
    "MT84MALT011000012345MTLCAST001S",
    "FR1420041010050500013M02606",
    "AT611904300234573201",
];

proptest! {
    /// Arbitrary UTF-8 input (including empty, multi-byte, and very long strings) must
    /// never panic any of the three public validation entry points, regardless of
    /// whether the result is Ok or Err.
    #[test]
    fn never_panics_on_arbitrary_utf8(s in ".{0,200}") {
        let _ = validate_iban_str(&s);
        let _ = validate_iban_str_print(&s);
        let _ = Iban::new(&s);
    }

    /// Same "never panics" property, but for a random-length ASCII alphanumeric string,
    /// which is much more likely to pass the initial length/country checks and drive
    /// deeper into the mod97 loop and Iban's bank/branch slicing.
    #[test]
    fn never_panics_on_random_alphanumeric(s in "[0-9A-Za-z]{0,64}") {
        let _ = validate_iban_str(&s);
        let _ = validate_iban_str_print(&s);
        let _ = Iban::new(&s);
    }

    /// Structured near-valid strategy: a real country prefix with the exact expected total
    /// length, but a random alphanumeric tail. Exercises the mod97 loop and
    /// Iban::extract_identifier slicing without being rejected early by the length/country
    /// checks; must never panic irrespective of whether the checksum happens to be valid.
    #[test]
    fn never_panics_on_correct_length_random_tail(
        seed_idx in 0..SEED_IBANS.len(),
        tail in "[0-9A-Za-z]{1,10}",
    ) {
        let seed = SEED_IBANS[seed_idx];
        let country = &seed[..2];
        let tail_needed = seed.len() - 2;
        let filled: String = tail.chars().cycle().take(tail_needed).collect();
        let candidate = format!("{country}{filled}");

        let _ = validate_iban_str(&candidate);
        let _ = validate_iban_str_print(&candidate);
        let _ = Iban::new(&candidate);
    }

    /// Inserting a modest number of spaces at arbitrary positions around a known-valid
    /// IBAN must not change the print-format validation result compared to validating the
    /// original, space-free IBAN directly. The total inserted spaces is kept well under
    /// `validate_iban_str_print`'s internal raw-length guard (see
    /// `flexible_rejects_excessively_long_input` in tests/validation_tests.rs for the
    /// pathological-length case, where exceeding that guard is the expected behavior).
    #[test]
    fn print_format_ignores_spaces_regardless_of_position(
        seed_idx in 0..SEED_IBANS.len(),
        gaps in prop::collection::vec(0usize..=2, 0..10),
    ) {
        let seed = SEED_IBANS[seed_idx];
        let expected = validate_iban_str(seed);

        let mut padded = String::new();
        for (i, byte) in seed.bytes().enumerate() {
            if let Some(gap) = gaps.get(i) {
                padded.push_str(&" ".repeat(*gap));
            }
            padded.push(byte as char);
        }
        if let Some(gap) = gaps.get(seed.len()) {
            padded.push_str(&" ".repeat(*gap));
        }

        prop_assert_eq!(validate_iban_str_print(&padded), expected);
    }
}
