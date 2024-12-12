import polars as pl
from py_viban_pl import process_ibans
import os
# print('debug', pl.__version__)

inputfile = r'core_iban_valid/data/IBAN Examples.txt'
outputfile = r'polars_plugin/examples/test_file.csv'

# File generation 
df = pl.read_csv(inputfile).sample(10000000, with_replacement=True)
df.write_csv(outputfile)
print('writing to file complete')

df = pl.scan_csv(outputfile)\
    .with_columns(
    validated=process_ibans('IBAN Examples').str.split_exact(',',2)\
        .struct.rename_fields(['valid_ibans', 'bank_id', 'branch_id'])
).unnest('validated').sort(by='IBAN Examples', descending=True)

print(df.collect(streaming=True))

os.remove(outputfile)


