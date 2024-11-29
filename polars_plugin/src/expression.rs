use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use core_iban_valid;

// fn process_iban_str(value: &str, iban_output: &mut (Option<String>, Option<String>, Option<String>)) {
//     match core_iban_valid::Iban::new(value) {
//         Ok(iban_obj) => {
//             iban_output.0 = Some(iban_obj.get_iban().to_string());
//             iban_output.1 = iban_obj.iban_bank_id.map(|x| x.to_string());
//             iban_output.2 = iban_obj.iban_branch_id.map(|x| x.to_string());
//         }
//         Err(_) => {
//             iban_output.0 = None;
//             iban_output.1 = None;
//             iban_output.2 = None;
//         }
//     }
// }

fn process_iban_str_str(value: &str, iban_valid: &mut String) {
    *iban_valid = String::from("");

    match core_iban_valid::Iban::new(value) {
        Ok(valid_iban) => {
            iban_valid.push_str(valid_iban.get_iban());
            iban_valid.push_str(",");
            iban_valid.push_str(valid_iban.iban_bank_id.map(|x| x.to_string()).unwrap_or(String::from("")).as_str());
            iban_valid.push_str(",");
            iban_valid.push_str(valid_iban.iban_branch_id.map(|x| x.to_string()).unwrap_or(String::from("")).as_str());
            // eprintln!("in process_iban_str_str {}", iban_valid);
        }
        Err(_) => {
                *iban_valid = String::from("");
            }
        }
    }

// fn iban_struct(_input_fields: &[Field]) -> PolarsResult<Field> {
//     let iban = Field::new("iban".into(), DataType::String);
//     let bank_id = Field::new("bank_id".into(), DataType::String);
//     let branch_id = Field::new("branch_id".into(), DataType::String);

//     let struct_type = DataType::Struct(vec![iban, bank_id, branch_id]);
//     Ok(Field::new("iban_struct".into(), struct_type))
// }

// pub fn iban_struct_ca(data: &ChunkedArray<String>) -> (ChunkedArray<String>, ChunkedArray<String>, ChunkedArray<String>) {
//     let mut iban_builder =
//         PrimitiveChunkedBuilder::<String>::new("iban".into(), data.len());
//     let mut bank_id_builder =
//         PrimitiveChunkedBuilder::<String>::new("bank_id".into(), data.len());    
//     let mut branch_id_builder =
//         PrimitiveChunkedBuilder::<String>::new("branch_id".into(), data.len());

//     for s in data.iter() {
//         match s {
//             Some(s) => {
//                 iban_builder.append_value(0);
//                 bank_id_builder.append_value(0);
//                 branch_id_builder.append_value(0);
//             }
//             None => {
//                 iban_builder.append_null();
//                 bank_id_builder.append_null();
//                 branch_id_builder.append_null();
//             }
//         }
//     }

//     (iban_builder.finish(), bank_id_builder.finish(), branch_id_builder.finish())
// }


// #[polars_expr(output_type_func=iban_struct)]
// fn process_ibans(inputs: &[Series]) -> PolarsResult<Series> {
//     let ca = inputs[0].str().unwrap();

//     ca.

// }

#[polars_expr(output_type=String)]
fn process_ibans(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(process_iban_str_str);
    Ok(out.into_series())
}


