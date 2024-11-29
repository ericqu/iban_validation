use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use core_iban_valid;

fn process_iban_str(value: &str, iban_output: &mut (Option<String>, Option<String>, Option<String>)) {
    match core_iban_valid::Iban::new(value) {
        Ok(iban_obj) => {
            iban_output.0 = Some(iban_obj.get_iban().to_string());
            iban_output.1 = iban_obj.iban_bank_id.map(|x| x.to_string());
            iban_output.2 = iban_obj.iban_branch_id.map(|x| x.to_string());
        }
        Err(_) => {
            iban_output.0 = None;
            iban_output.1 = None;
            iban_output.2 = None;
        }
    }
}

fn iban_struct(_input_fields: &[Field]) -> PolarsResult<Field> {
    let iban = Field::new("iban".into(), DataType::String);
    let bank_id = Field::new("bank_id".into(), DataType::String);
    let branch_id = Field::new("branch_id".into(), DataType::String);

    let struct_type = DataType::Struct(vec![iban, bank_id, branch_id]);
    Ok(Field::new("iban_struct".into(), struct_type))
}

pub fn iban_struct_ca(data: &ChunkedArray<String>) -> (ChunkedArray<String>, ChunkedArray<String>, ChunkedArray<String>) {
    let mut iban_builder =
        PrimitiveChunkedBuilder::<String>::new("iban".into(), data.len());
    let mut bank_id_builder =
        PrimitiveChunkedBuilder::<String>::new("bank_id".into(), data.len());    
    let mut branch_id_builder =
        PrimitiveChunkedBuilder::<String>::new("branch_id".into(), data.len());

    for s in data.iter() {
        match s {
            Some(s) => {
                iban_builder.append_value(0);
                bank_id_builder.append_value(0);
                branch_id_builder.append_value(0);
            }
            None => {
                iban_builder.append_null();
                bank_id_builder.append_null();
                branch_id_builder.append_null();
            }
        }
    }

    (iban_builder.finish(), bank_id_builder.finish(), branch_id_builder.finish())
}


#[polars_expr(output_type_func=iban_struct)]
fn process_ibans(inputs: &[Series]) -> PolarsResult<Series> {
    let input = inputs[0].str()?;

    let n_threads = 2;
    let splits = split_offsets(input.len(), n_threads);

    let chunks: Vec<(Vec<_>, Vec<_>)> = splits
        .into_iter()
        .map(|(offset, len)| -> (Vec<_>, Vec<_>) {
            let sliced = input.slice(offset as i64, len);
            let (iban, bank_id, branch_id) = iban_struct_ca(&sliced);
            (iban.downcast_iter().cloned().collect::<Vec<_>>(),
             bank_id.downcast_iter().cloned().collect::<Vec<_>>(),
             branch_id.downcast_iter().cloned().collect::<Vec<_>>())
        })
        .collect();

    let (unzipped1, unzipped2, unzipped3 ): (Vec<Vec<_>>, Vec<Vec<_>>, Vec<Vec<_>>) = chunks.into_iter().unzip();
    let all_iban = StringChunked::from_chunk_iter("iban", unzipped1.into_iter().flatten());
    let all_bank_id = StringChunked::from_chunk_iter("bank_id", unzipped2.into_iter().flatten());
    let all_branch_id = StringChunked::from_chunk_iter("branch_id", unzipped3.into_iter().flatten());
    let s = StructChunked::from_series("iban_struct".into(), all_iban.len(), &[all_iban.into_series(), all_bank_id.into_series(), all_branch_id.into_series() ])?;
    Ok(s.into_series())
}


