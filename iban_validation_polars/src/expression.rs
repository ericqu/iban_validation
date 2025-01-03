use iban_validation_rs;
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

fn process_iban_str_str(value: &str, iban_valid: &mut String) {
    *iban_valid = String::from("");

    match iban_validation_rs::Iban::new(value) {
        Ok(valid_iban) => {
            iban_valid.push_str(valid_iban.get_iban());
            iban_valid.push_str(",");
            iban_valid.push_str(
                valid_iban
                    .iban_bank_id
                    .map(|x| x.to_string())
                    .unwrap_or(String::from(""))
                    .as_str(),
            );
            iban_valid.push_str(",");
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
fn process_ibans(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(process_iban_str_str);
    Ok(out.into_series())
}
