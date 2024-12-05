import polars as pl

from expression_lib import process_ibans

df = pl.DataFrame(
        {"ibans":["AT611904300234573201", "CY17002001280000001200527600", "TOTOT"]}
)
print(df)

res = df.with_columns(
    validated=process_ibans('ibans').str.split_exact(',',2)\
        .struct.rename_fields(['valid_ibans', 'bank_id', 'branch_id'])
).unnest('validated')

print(res)

print('OK')
