//! Thin entry point: parse the arguments, pick the input, hand everything to the library.

use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use iban_validation_cli::{
    Args, Invocation, USAGE, USAGE_EXIT_CODE, exit_code, parse_args, run, version_text,
};

fn main() -> ExitCode {
    match real_main() {
        Ok(code) => code,
        Err(message) => {
            eprintln!("iban-validate: {message}");
            ExitCode::from(USAGE_EXIT_CODE as u8)
        }
    }
}

fn real_main() -> Result<ExitCode, String> {
    let args = match parse_args(std::env::args().skip(1))? {
        Invocation::Help => {
            print!("{USAGE}");
            return Ok(ExitCode::SUCCESS);
        }
        Invocation::Version => {
            println!("{}", version_text());
            return Ok(ExitCode::SUCCESS);
        }
        Invocation::Run(args) => args,
    };

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let report = validate(&args, &mut out).map_err(|err| match &args.path {
        Some(path) => format!("{path}: {err}"),
        None => format!("<stdin>: {err}"),
    })?;
    out.flush().map_err(|err| err.to_string())?;

    Ok(ExitCode::from(exit_code(&report) as u8))
}

fn validate<W: Write>(args: &Args, out: &mut W) -> io::Result<iban_validation_cli::Report> {
    match &args.path {
        Some(path) => run(args, BufReader::new(File::open(path)?), out),
        None => {
            let stdin = io::stdin();
            run(args, stdin.lock(), out)
        }
    }
}
