//! The CLI reads whatever a data pipeline hands it, so the properties that matter are
//! that no input can panic it, and that the output stays machine readable line by line.

use iban_validation_cli::{Args, CSV_HEADER, Format, format_line, normalize, run};
use iban_validation_rs::CountrySet;
use proptest::prelude::*;

fn args(format: Format, print_format: bool) -> Args {
    Args {
        format,
        print_format,
        ..Args::default()
    }
}

fn run_str(args: &Args, input: &str) -> String {
    let mut out: Vec<u8> = Vec::new();
    run(args, input.as_bytes(), &mut out).expect("in-memory run cannot fail");
    String::from_utf8(out).expect("output is utf-8")
}

proptest! {
    /// Arbitrary UTF-8, including multi-byte characters and long lines, must never panic
    /// the rendering path in either format or country set.
    #[test]
    fn format_line_never_panics(s in ".{0,200}") {
        let _ = format_line(&s, Format::Text, CountrySet::Registry);
        let _ = format_line(&s, Format::Csv, CountrySet::WithNonRegistry);
    }

    /// Same for the whole read loop, over inputs with several lines and mixed content.
    #[test]
    fn run_never_panics(lines in prop::collection::vec(".{0,64}", 0..8)) {
        let input = lines.join("\n");
        for format in [Format::Text, Format::Csv] {
            for print_format in [false, true] {
                let _ = run_str(&args(format, print_format), &input);
            }
        }
    }

    /// One output line per non-empty input line (plus the csv header), so downstream
    /// tools can zip input and output.
    #[test]
    fn one_output_line_per_non_empty_input_line(
        lines in prop::collection::vec("[ 0-9A-Za-z]{0,40}", 0..8)
    ) {
        let expected = lines.iter().filter(|l| !l.trim().is_empty()).count();
        let input = lines.join("\n");

        let text = run_str(&args(Format::Text, false), &input);
        prop_assert_eq!(text.lines().count(), expected);

        let csv = run_str(&args(Format::Csv, false), &input);
        prop_assert_eq!(csv.lines().count(), expected + 1);
    }

    /// Rendered lines never embed a newline, which is what makes the line counting above
    /// hold no matter what the input contained.
    #[test]
    fn rendered_lines_are_single_lines(s in ".{0,80}") {
        for format in [Format::Text, Format::Csv] {
            let line = format_line(&s.replace(['\n', '\r'], ""), format, CountrySet::Registry);
            prop_assert!(!line.contains('\n'));
            prop_assert!(!line.contains('\r'));
        }
    }

    /// Every csv row keeps the header's field count, once quoted fields are accounted
    /// for by using inputs that cannot introduce a quote or a comma.
    #[test]
    fn csv_rows_match_the_header_arity(s in "[ 0-9A-Za-z]{0,40}") {
        let expected = CSV_HEADER.split(',').count();
        let line = format_line(s.trim(), Format::Csv, CountrySet::Registry);
        prop_assert_eq!(line.split(',').count(), expected);
    }

    /// Normalizing is idempotent: feeding a normalized line back through changes nothing.
    #[test]
    fn normalize_is_idempotent(s in "[ 0-9A-Za-z]{0,60}", print_format in any::<bool>()) {
        let once = normalize(&s, print_format).into_owned();
        let twice = normalize(&once, print_format).into_owned();
        prop_assert_eq!(once, twice);
    }

    /// Print format only ever removes spaces, so it can only turn an invalid line into a
    /// valid one, never the reverse.
    #[test]
    fn print_format_only_adds_acceptance(s in "[ 0-9A-Za-z]{0,40}") {
        let strict = normalize(&s, false);
        let lenient = normalize(&s, true);
        let strict_valid = iban_validation_rs::Iban::new(&strict).is_ok();
        let lenient_valid = iban_validation_rs::Iban::new(&lenient).is_ok();
        prop_assert!(!strict_valid || lenient_valid);
    }

    /// Opting into the non-registry countries can only widen what is accepted.
    #[test]
    fn non_registry_only_adds_acceptance(s in "[0-9A-Za-z]{0,40}") {
        let registry = iban_validation_rs::Iban::new_with(&s, CountrySet::Registry).is_ok();
        let with_non_registry =
            iban_validation_rs::Iban::new_with(&s, CountrySet::WithNonRegistry).is_ok();
        prop_assert!(!registry || with_non_registry);
    }
}

/// Every example IBAN shipped with the core crate must come back as valid through the
/// CLI's own read loop, in both output formats.
#[test]
fn all_registry_examples_validate_through_the_cli() {
    let examples = include_str!("../../iban_validation_rs/data/IBAN Examples.txt");
    // The first line is the file's title, not an IBAN.
    let ibans: Vec<&str> = examples
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    assert!(
        ibans.len() > 50,
        "expected the full example list, got {}",
        ibans.len()
    );

    let input = ibans.join("\n");

    let mut out: Vec<u8> = Vec::new();
    let report = run(&args(Format::Text, false), input.as_bytes(), &mut out)
        .expect("in-memory run cannot fail");
    let text = String::from_utf8(out).expect("output is utf-8");
    assert_eq!(
        report.invalid,
        0,
        "some example IBANs did not validate:\n{}",
        text.lines()
            .filter(|l| l.contains("\tinvalid\t"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert_eq!(report.total, ibans.len());

    let csv = run_str(&args(Format::Csv, false), &input);
    assert_eq!(csv.lines().count(), ibans.len() + 1);
}

/// The same list in print format: grouping the digits in fours must not change any verdict.
#[test]
fn registry_examples_validate_in_print_format() {
    let examples = include_str!("../../iban_validation_rs/data/IBAN Examples.txt");
    let spaced: Vec<String> = examples
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|iban| {
            iban.as_bytes()
                .chunks(4)
                .map(|c| std::str::from_utf8(c).expect("ascii chunk").to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    let mut out: Vec<u8> = Vec::new();
    let report = run(
        &args(Format::Text, true),
        spaced.join("\n").as_bytes(),
        &mut out,
    )
    .expect("in-memory run cannot fail");
    assert_eq!(report.invalid, 0);
    assert_eq!(report.total, spaced.len());
}
