use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

fn process_iban_str_str(value: &str, iban_valid: &mut String) {
    *iban_valid = String::from("");

    match iban_validation_rs::Iban::new(value) {
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
fn process_ibans(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(process_iban_str_str);
    Ok(out.into_series())
}

// fn process_iban_str_num(value: &str, iban_valid: &mut String) {
//     *iban_valid = String::from("");

//     match iban_validation_rs::validate_iban_get_numeric(value) {
//         Ok((_, bank_s, bank_e, branch_s, branch_e)) => {
//             iban_valid.push_str(value);
//             iban_valid.push(',');
//             if bank_e > bank_s {
//                 iban_valid.push_str(&value[bank_s as usize..bank_e as usize]);
//             }
//             iban_valid.push(',');
//             if branch_e > branch_s {
//                 iban_valid.push_str(&value[branch_s as usize..branch_e as usize]);
//             }
//         }
//         Err(_) => {
//             *iban_valid = String::from("");
//         }
//     }
// }

// #[polars_expr(output_type=String)]
// fn process_ibans_num(inputs: &[Series]) -> PolarsResult<Series> {
//     let ca = inputs[0].str()?;
//     let out: StringChunked = ca.apply_into_string_amortized(process_iban_str_num);
//     Ok(out.into_series())
// }
