import polars as pl

from expression_lib import process_ibans

df = pl.DataFrame(
        {"ibans":["AT611904300234573201", "CY17002001280000001200527600"]}
)
print(df)

res = df.with_columns(
    validated=process_ibans('ibans')
)
print(res)

print('OK')
