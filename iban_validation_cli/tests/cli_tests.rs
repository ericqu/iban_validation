//! End to end tests: the library `run` loop against in-memory input, and the installed
//! binary against real stdin, files and exit codes.

use std::io::Write;
use std::process::{Command, Stdio};

use iban_validation_cli::{Args, CSV_HEADER, Format, Report, run};
use iban_validation_rs::CountrySet;

const VALID_DE: &str = "DE44500105175407324931";
const VALID_AL: &str = "AL47212110090000000235698741";
/// Registry country, but the mod97 check fails.
const INVALID: &str = "AL4721211009000000023569874Q";
/// Community-sourced country, only accepted with `--non-registry`.
const NON_REGISTRY: &str = "AO49012345678901234567890";

fn run_str(args: &Args, input: &str) -> (Report, String) {
    let mut out: Vec<u8> = Vec::new();
    let report = run(args, input.as_bytes(), &mut out).expect("in-memory run cannot fail");
    (report, String::from_utf8(out).expect("output is utf-8"))
}

fn text_args() -> Args {
    Args::default()
}

// ---------------------------------------------------------------------------
// library level
// ---------------------------------------------------------------------------

#[test]
fn valid_iban_reports_bank_and_branch() {
    let (report, out) = run_str(&text_args(), VALID_AL);
    assert_eq!(
        report,
        Report {
            total: 1,
            valid: 1,
            invalid: 0
        }
    );
    // AL defines both a bank and a branch identifier.
    assert_eq!(out, format!("{VALID_AL}\tvalid\t212\t11009\n"));
}

#[test]
fn missing_branch_identifier_is_rendered_as_a_dash() {
    let (_, out) = run_str(&text_args(), VALID_DE);
    assert_eq!(out, format!("{VALID_DE}\tvalid\t50010517\t-\n"));
}

#[test]
fn invalid_iban_reports_the_reason() {
    let (report, out) = run_str(&text_args(), INVALID);
    assert_eq!(
        report,
        Report {
            total: 1,
            valid: 0,
            invalid: 1
        }
    );
    assert!(out.starts_with(&format!("{INVALID}\tinvalid\t")), "{out}");
    assert!(out.contains("mod97"), "{out}");
}

#[test]
fn empty_and_whitespace_lines_are_skipped() {
    let (report, out) = run_str(&text_args(), &format!("\n  \n{VALID_DE}\n\n"));
    assert_eq!(
        report,
        Report {
            total: 1,
            valid: 1,
            invalid: 0
        }
    );
    assert_eq!(out.lines().count(), 1);
}

#[test]
fn surrounding_whitespace_is_trimmed_without_print_format() {
    let (report, _) = run_str(&text_args(), &format!("  {VALID_DE}\t\n"));
    assert_eq!(report.valid, 1);
}

#[test]
fn inner_spaces_need_print_format() {
    let spaced = "DE44 5001 0517 5407 3249 31";

    let (report, _) = run_str(&text_args(), spaced);
    assert_eq!(report.invalid, 1);

    let args = Args {
        print_format: true,
        ..Args::default()
    };
    let (report, out) = run_str(&args, spaced);
    assert_eq!(report.valid, 1);
    // The normalized (electronic) form is what gets echoed back.
    assert!(out.starts_with(VALID_DE), "{out}");
}

#[test]
fn non_registry_countries_are_opt_in() {
    let (report, _) = run_str(&text_args(), NON_REGISTRY);
    assert_eq!(report.invalid, 1);

    let args = Args {
        country_set: CountrySet::WithNonRegistry,
        ..Args::default()
    };
    let (report, _) = run_str(&args, NON_REGISTRY);
    assert_eq!(report.valid, 1);
}

#[test]
fn csv_output_has_a_header_and_one_row_per_iban() {
    let args = Args {
        format: Format::Csv,
        ..Args::default()
    };
    let (report, out) = run_str(&args, &format!("{VALID_DE}\n{INVALID}\n"));
    assert_eq!(report.total, 2);

    let mut lines = out.lines();
    assert_eq!(lines.next(), Some(CSV_HEADER));
    assert_eq!(
        lines.next(),
        Some(format!("{VALID_DE},valid,50010517,,").as_str())
    );
    let invalid_row = lines.next().expect("an invalid row");
    assert!(
        invalid_row.starts_with(&format!("{INVALID},invalid,,,")),
        "{invalid_row}"
    );
    assert_eq!(lines.next(), None);

    // Every row keeps the header's field count.
    let fields = CSV_HEADER.split(',').count();
    for line in out.lines() {
        assert_eq!(line.split(',').count(), fields, "{line}");
    }
}

#[test]
fn quiet_suppresses_output_but_still_counts() {
    let args = Args {
        quiet: true,
        ..Args::default()
    };
    let (report, out) = run_str(&args, &format!("{VALID_DE}\n{INVALID}\n"));
    assert_eq!(
        report,
        Report {
            total: 2,
            valid: 1,
            invalid: 1
        }
    );
    assert!(out.is_empty(), "{out}");

    // Not even the csv header is written when quiet.
    let args = Args {
        quiet: true,
        format: Format::Csv,
        ..Args::default()
    };
    let (_, out) = run_str(&args, VALID_DE);
    assert!(out.is_empty(), "{out}");
}

#[test]
fn input_without_a_trailing_newline_is_still_read() {
    let (report, _) = run_str(&text_args(), VALID_DE);
    assert_eq!(report.total, 1);
}

#[test]
fn empty_input_is_a_clean_run() {
    let (report, out) = run_str(&text_args(), "");
    assert_eq!(report, Report::default());
    assert!(out.is_empty());
}

// ---------------------------------------------------------------------------
// binary level
// ---------------------------------------------------------------------------

const BIN: &str = env!("CARGO_BIN_EXE_iban-validate");

struct Output {
    code: i32,
    stdout: String,
    stderr: String,
}

fn spawn(args: &[&str], stdin_data: &str) -> Output {
    let mut child = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary is built by cargo test");
    child
        .stdin
        .as_mut()
        .expect("stdin was piped")
        .write_all(stdin_data.as_bytes())
        .expect("writing to the child");
    let out = child.wait_with_output().expect("waiting for the child");
    Output {
        code: out.status.code().expect("the child was not signalled"),
        stdout: String::from_utf8(out.stdout).expect("stdout is utf-8"),
        stderr: String::from_utf8(out.stderr).expect("stderr is utf-8"),
    }
}

#[test]
fn stdin_all_valid_exits_zero() {
    let out = spawn(&[], &format!("{VALID_DE}\n{VALID_AL}\n"));
    assert_eq!(out.code, 0, "{}", out.stderr);
    assert_eq!(out.stdout.lines().count(), 2);
}

#[test]
fn one_invalid_exits_one() {
    let out = spawn(&[], &format!("{VALID_DE}\n{INVALID}\n"));
    assert_eq!(out.code, 1);
    assert!(out.stdout.contains("invalid"), "{}", out.stdout);
}

#[test]
fn dash_reads_stdin() {
    let out = spawn(&["-"], VALID_DE);
    assert_eq!(out.code, 0, "{}", out.stderr);
    assert!(out.stdout.starts_with(VALID_DE));
}

#[test]
fn a_file_argument_is_read() {
    let dir = std::env::temp_dir().join(format!("iban_cli_test_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("creating the temp dir");
    let path = dir.join("ibans.txt");
    std::fs::write(&path, format!("{VALID_DE}\n{VALID_AL}\n")).expect("writing the temp file");

    let out = spawn(&[path.to_str().expect("utf-8 temp path")], "");
    assert_eq!(out.code, 0, "{}", out.stderr);
    assert_eq!(out.stdout.lines().count(), 2);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_missing_file_is_a_usage_error() {
    let out = spawn(&["no/such/file.txt"], "");
    assert_eq!(out.code, 2);
    assert!(out.stderr.contains("no/such/file.txt"), "{}", out.stderr);
    assert!(out.stdout.is_empty(), "{}", out.stdout);
}

#[test]
fn an_unknown_option_is_a_usage_error() {
    let out = spawn(&["--definitely-not-an-option"], "");
    assert_eq!(out.code, 2);
    assert!(out.stderr.contains("unknown option"), "{}", out.stderr);
}

#[test]
fn help_and_version_exit_zero() {
    let help = spawn(&["--help"], "");
    assert_eq!(help.code, 0);
    assert!(help.stdout.contains("USAGE:"), "{}", help.stdout);

    let version = spawn(&["--version"], "");
    assert_eq!(version.code, 0);
    assert!(
        version.stdout.starts_with("iban-validate "),
        "{}",
        version.stdout
    );
}

#[test]
fn quiet_prints_nothing_and_keeps_the_exit_code() {
    let out = spawn(&["--quiet"], &format!("{VALID_DE}\n{INVALID}\n"));
    assert_eq!(out.code, 1);
    assert!(out.stdout.is_empty(), "{}", out.stdout);
}

#[test]
fn csv_flag_reaches_the_binary() {
    let out = spawn(&["--format", "csv"], VALID_DE);
    assert_eq!(out.code, 0, "{}", out.stderr);
    assert!(out.stdout.starts_with(CSV_HEADER), "{}", out.stdout);
}

#[test]
fn non_utf8_input_is_reported_as_an_error() {
    let mut child = Command::new(BIN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary is built by cargo test");
    child
        .stdin
        .as_mut()
        .expect("stdin was piped")
        .write_all(&[0xff, 0xfe, b'\n'])
        .expect("writing to the child");
    let out = child.wait_with_output().expect("waiting for the child");
    assert_eq!(out.status.code(), Some(2));
}
