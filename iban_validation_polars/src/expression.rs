use iban_validation_rs::CountrySet;
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProcessKwargs {
    #[serde(default)]
    allow_non_registry: bool,
}

#[inline]
fn country_set(allow_non_registry: bool) -> CountrySet {
    if allow_non_registry {
        CountrySet::WithNonRegistry
    } else {
        CountrySet::Registry
    }
}

fn process_iban_str_str(value: &str, set: CountrySet, iban_valid: &mut String) {
    *iban_valid = String::from("");

    match iban_validation_rs::Iban::new_with(value, set) {
        Ok(valid_iban) => {
            iban_valid.push_str(valid_iban.get_iban());
            iban_valid.push(',');
            iban_valid.push_str(
                valid_iban
                    .iban_bank_id
                    .map(|x| x.to_string())
                    .unwrap_or(String::from(""))
                    .as_str(),
            );
            iban_valid.push(',');
            iban_valid.push_str(
                valid_iban
                    .iban_branch_id
                    .map(|x| x.to_string())
                    .unwrap_or(String::from(""))
                    .as_str(),
            );
        }
        Err(_) => {
            *iban_valid = String::from("");
        }
    }
}

#[polars_expr(output_type=String)]
fn process_ibans(inputs: &[Series], kwargs: ProcessKwargs) -> PolarsResult<Series> {
    let set = country_set(kwargs.allow_non_registry);
    let ca = inputs[0].str()?;
    let out: StringChunked =
        ca.apply_into_string_amortized(|value, out| process_iban_str_str(value, set, out));
    Ok(out.into_series())
}

/// Classifies each IBAN as "registry", "non_registry", or "invalid", regardless of
/// caller-side `allow_non_registry` choice for `process_ibans` - this lets pipelines
/// filter/group on the country set in pure Polars.
fn country_status_str(value: &str, out: &mut String) {
    let status =
        match iban_validation_rs::validate_iban_str_with(value, CountrySet::WithNonRegistry) {
            Ok(true) => {
                let cc: Option<[u8; 2]> =
                    value.as_bytes().get(0..2).and_then(|s| s.try_into().ok());
                match cc {
                    Some(cc) if iban_validation_rs::is_non_registry_country(cc) => "non_registry",
                    _ => "registry",
                }
            }
            _ => "invalid",
        };
    *out = status.to_string();
}

#[polars_expr(output_type=String)]
fn country_status(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(country_status_str);
    Ok(out.into_series())
}
