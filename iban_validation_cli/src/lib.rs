//! Command line front end for [`iban_validation_rs`].
//!
//! Reads IBANs, one per line, from standard input or from a file, and reports for each
//! whether it is valid together with the bank and branch identifiers when the country
//! registry defines them.
//!
//! The binary is intentionally a thin wrapper: everything it does is exposed here as
//! plain functions so it can be tested without spawning a process, and so the argument
//! parsing stays dependency free (the core crate has no dependencies, and this crate
//! keeps that property).

use std::borrow::Cow;
use std::fmt;
use std::io::{BufRead, Write};

use iban_validation_rs::{CountrySet, Iban, ValidationError, get_source_file, get_version};

/// How each result line is rendered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Format {
    /// Tab separated, human readable: `<iban>\t<valid|invalid>\t<bank>\t<branch>`.
    #[default]
    Text,
    /// Comma separated with a header line, for piping into a data tool.
    Csv,
}

/// The options a run is driven by, as produced by [`parse_args`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Args {
    /// File to read, or `None` to read standard input.
    pub path: Option<String>,
    /// Output rendering.
    pub format: Format,
    /// Suppress per-line output; only the exit code reports the outcome.
    pub quiet: bool,
    /// Which country set validation accepts.
    pub country_set: CountrySet,
    /// Accept print format input (spaces are removed before validating).
    pub print_format: bool,
}

/// What the command line asked for.
#[derive(Clone, Debug, PartialEq)]
pub enum Invocation {
    /// Validate, using these options.
    Run(Args),
    /// Print the usage text and exit successfully.
    Help,
    /// Print the version text and exit successfully.
    Version,
}

/// Counts collected over a run, used to pick the process exit code.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Number of non-empty lines that were checked.
    pub total: usize,
    /// How many of them validated.
    pub valid: usize,
    /// How many of them did not.
    pub invalid: usize,
}

/// Usage text, also shown when the arguments do not parse.
pub const USAGE: &str = "\
iban_validation_cli - validate IBANs and extract bank and branch identifiers

USAGE:
    iban_validation_cli [OPTIONS] [FILE]

    FILE is read one IBAN per line; empty lines are skipped. When FILE is absent
    or is `-`, standard input is read instead.

OPTIONS:
    -f, --format <text|csv>  Output format (default: text)
    -q, --quiet              Print nothing; report the outcome through the exit code
        --print-format       Accept print format input (spaces are removed first)
        --non-registry       Also accept the 22 community-sourced, non-registry countries
    -h, --help               Print this help
    -V, --version            Print version information

EXIT CODES:
    0  every IBAN read was valid (including when nothing was read)
    1  at least one IBAN was invalid
    2  the arguments or the input file could not be used

EXAMPLES:
    echo DE44500105175407324931 | iban_validation_cli
    iban_validation_cli --format csv ibans.txt > checked.csv
";

/// Version text for `--version`, naming the registry the country data was generated from.
pub fn version_text() -> String {
    format!(
        "iban_validation_cli {} (iban_validation_rs {}, registry {})",
        env!("CARGO_PKG_VERSION"),
        get_version(),
        get_source_file().trim()
    )
}

/// Parse the command line arguments, excluding the program name.
///
/// Returns the usage message as the error when an argument is unknown, missing its
/// value, or repeated in a conflicting way.
pub fn parse_args<I, S>(args: I) -> Result<Invocation, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut parsed = Args::default();
    let mut positional: Option<String> = None;
    let mut args = args.into_iter().map(Into::into);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Invocation::Help),
            "-V" | "--version" => return Ok(Invocation::Version),
            "-q" | "--quiet" => parsed.quiet = true,
            "--print-format" => parsed.print_format = true,
            "--non-registry" => parsed.country_set = CountrySet::WithNonRegistry,
            "-f" | "--format" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("missing value for {arg}"))?;
                parsed.format = parse_format(&value)?;
            }
            other if other.starts_with("--format=") => {
                parsed.format = parse_format(&other["--format=".len()..])?;
            }
            "-" => set_positional(&mut positional, "-".to_string())?,
            other if other.starts_with('-') && other.len() > 1 => {
                return Err(format!("unknown option: {other}"));
            }
            other => set_positional(&mut positional, other.to_string())?,
        }
    }

    parsed.path = match positional {
        Some(path) if path == "-" => None,
        other => other,
    };

    Ok(Invocation::Run(parsed))
}

fn parse_format(value: &str) -> Result<Format, String> {
    match value {
        "text" => Ok(Format::Text),
        "csv" => Ok(Format::Csv),
        other => Err(format!("unknown format: {other} (expected text or csv)")),
    }
}

fn set_positional(slot: &mut Option<String>, value: String) -> Result<(), String> {
    match slot {
        Some(existing) => Err(format!(
            "unexpected extra argument: {value} (already reading {existing})"
        )),
        None => {
            *slot = Some(value);
            Ok(())
        }
    }
}

/// Normalize one input line: surrounding whitespace is always removed, and inner spaces
/// are removed as well when print format input was requested.
///
/// Borrowing is kept whenever nothing has to be removed, so the common electronic-format
/// case does not allocate.
pub fn normalize<'a>(line: &'a str, print_format: bool) -> Cow<'a, str> {
    let trimmed = line.trim();
    if print_format && trimmed.contains(' ') {
        Cow::Owned(trimmed.replace(' ', ""))
    } else {
        Cow::Borrowed(trimmed)
    }
}

/// Render one already normalized IBAN as an output line, without the trailing newline.
///
/// Valid IBANs report their bank and branch identifiers, using `-` (text) or an empty
/// field (csv) for the countries where the registry defines none. Invalid ones report
/// the reason from [`ValidationError`].
pub fn format_line(iban: &str, format: Format, country_set: CountrySet) -> String {
    match Iban::new_with(iban, country_set) {
        Ok(parsed) => match format {
            Format::Text => format!(
                "{}\tvalid\t{}\t{}",
                iban,
                parsed.iban_bank_id.unwrap_or("-"),
                parsed.iban_branch_id.unwrap_or("-")
            ),
            Format::Csv => format!(
                "{},valid,{},{},",
                csv_field(iban),
                csv_field(parsed.iban_bank_id.unwrap_or("")),
                csv_field(parsed.iban_branch_id.unwrap_or("")),
            ),
        },
        Err(err) => match format {
            Format::Text => format!("{iban}\tinvalid\t{err}"),
            Format::Csv => format!(
                "{},invalid,,,{}",
                csv_field(iban),
                csv_field(&err.to_string())
            ),
        },
    }
}

/// Quote a CSV field only when it needs it, doubling any embedded quote.
fn csv_field(value: &str) -> Cow<'_, str> {
    if value.contains([',', '"', '\n', '\r']) {
        Cow::Owned(format!("\"{}\"", value.replace('"', "\"\"")))
    } else {
        Cow::Borrowed(value)
    }
}

/// The CSV header, written before the first row in [`Format::Csv`].
pub const CSV_HEADER: &str = "iban,valid,bank_id,branch_id,error";

/// Validate every line of `input`, writing the results to `output`.
///
/// Empty (or whitespace only) lines are skipped and are not counted. Lines that are not
/// valid UTF-8 are reported as an I/O error, as the caller cannot meaningfully validate
/// them.
pub fn run<R: BufRead, W: Write>(args: &Args, input: R, output: &mut W) -> std::io::Result<Report> {
    let mut report = Report::default();

    if !args.quiet && args.format == Format::Csv {
        writeln!(output, "{CSV_HEADER}")?;
    }

    for line in input.lines() {
        let line = line?;
        let candidate = normalize(&line, args.print_format);
        if candidate.is_empty() {
            continue;
        }

        report.total += 1;
        let is_valid = Iban::new_with(&candidate, args.country_set).is_ok();
        if is_valid {
            report.valid += 1;
        } else {
            report.invalid += 1;
        }

        if !args.quiet {
            writeln!(
                output,
                "{}",
                format_line(&candidate, args.format, args.country_set)
            )?;
        }
    }

    output.flush()?;
    Ok(report)
}

/// Exit code matching a [`Report`]: `0` when nothing was invalid, `1` otherwise.
pub const fn exit_code(report: &Report) -> i32 {
    if report.invalid == 0 { 0 } else { 1 }
}

/// Exit code used when the arguments or the input file could not be used.
pub const USAGE_EXIT_CODE: i32 = 2;

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Csv => write!(f, "csv"),
        }
    }
}

/// Re-exported so callers of the library do not have to depend on the core crate just to
/// name the error type they get back from [`Iban::new_with`].
pub type Error = ValidationError;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_args(args: &[&str]) -> Result<Invocation, String> {
        parse_args(args.iter().copied())
    }

    #[test]
    fn no_arguments_reads_stdin_as_text() {
        assert_eq!(
            run_args(&[]),
            Ok(Invocation::Run(Args {
                path: None,
                format: Format::Text,
                quiet: false,
                country_set: CountrySet::Registry,
                print_format: false,
            }))
        );
    }

    #[test]
    fn dash_is_stdin() {
        let Ok(Invocation::Run(args)) = run_args(&["-"]) else {
            panic!("expected a run invocation");
        };
        assert_eq!(args.path, None);
    }

    #[test]
    fn positional_is_the_file_to_read() {
        let Ok(Invocation::Run(args)) = run_args(&["ibans.txt"]) else {
            panic!("expected a run invocation");
        };
        assert_eq!(args.path.as_deref(), Some("ibans.txt"));
    }

    #[test]
    fn flags_are_recognised_in_both_spellings() {
        for flag in ["-q", "--quiet"] {
            let Ok(Invocation::Run(args)) = run_args(&[flag]) else {
                panic!("expected a run invocation");
            };
            assert!(args.quiet);
        }
        for flag in ["-h", "--help"] {
            assert_eq!(run_args(&[flag]), Ok(Invocation::Help));
        }
        for flag in ["-V", "--version"] {
            assert_eq!(run_args(&[flag]), Ok(Invocation::Version));
        }
    }

    #[test]
    fn format_accepts_separate_and_joined_values() {
        for spelling in [
            vec!["-f", "csv"],
            vec!["--format", "csv"],
            vec!["--format=csv"],
        ] {
            let Ok(Invocation::Run(args)) = parse_args(spelling.iter().copied()) else {
                panic!("expected a run invocation");
            };
            assert_eq!(args.format, Format::Csv);
        }
    }

    #[test]
    fn opt_in_flags_are_off_by_default() {
        let Ok(Invocation::Run(args)) = run_args(&["-"]) else {
            panic!("expected a run invocation");
        };
        assert_eq!(args.country_set, CountrySet::Registry);
        assert!(!args.print_format);

        let Ok(Invocation::Run(args)) = run_args(&["--non-registry", "--print-format"]) else {
            panic!("expected a run invocation");
        };
        assert_eq!(args.country_set, CountrySet::WithNonRegistry);
        assert!(args.print_format);
    }

    #[test]
    fn bad_arguments_are_rejected() {
        assert!(run_args(&["--nope"]).is_err());
        assert!(run_args(&["--format"]).is_err());
        assert!(run_args(&["--format", "yaml"]).is_err());
        assert!(run_args(&["a.txt", "b.txt"]).is_err());
    }

    #[test]
    fn normalize_only_allocates_when_it_must() {
        assert!(matches!(
            normalize("  DE44500105175407324931  ", false),
            Cow::Borrowed("DE44500105175407324931")
        ));
        assert!(matches!(
            normalize("DE44 5001 0517 5407 3249 31", false),
            Cow::Borrowed("DE44 5001 0517 5407 3249 31")
        ));
        assert_eq!(
            normalize(" DE44 5001 0517 5407 3249 31 ", true),
            "DE44500105175407324931"
        );
    }

    #[test]
    fn csv_fields_are_quoted_only_when_needed() {
        assert_eq!(csv_field("DE44"), "DE44");
        assert_eq!(csv_field("a,b"), "\"a,b\"");
        assert_eq!(csv_field("a\"b"), "\"a\"\"b\"");
    }

    #[test]
    fn exit_code_follows_the_invalid_count() {
        assert_eq!(exit_code(&Report::default()), 0);
        assert_eq!(
            exit_code(&Report {
                total: 2,
                valid: 2,
                invalid: 0
            }),
            0
        );
        assert_eq!(
            exit_code(&Report {
                total: 2,
                valid: 1,
                invalid: 1
            }),
            1
        );
    }

    #[test]
    fn version_text_names_the_registry() {
        let text = version_text();
        assert!(text.starts_with("iban_validation_cli "));
        assert!(text.contains(get_version()));
        assert!(text.contains("iban_registry_v"));
    }
}
