import polars as pl
from iban_validation_polars import process_ibans
import time

import os

inputfile = r"iban_validation_rs/data/IBAN Examples.txt"
generatedfile = r"iban_validation_polars/examples/test_file.csv"
sample_size = 10000000

# generate a csv file for testing
df = pl.read_csv(inputfile).sample(sample_size, with_replacement=True)
df.write_csv(generatedfile)
print("writing to file complete")

start = time.perf_counter()
df = (
    pl.scan_csv(generatedfile)
    .with_columns(
        validated=process_ibans("IBAN Examples")
        .str.split_exact(",", 2)
        .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
    )
    .unnest("validated")
    .sort(by="IBAN Examples", descending=True)
)
# trigger the processing
print(df.collect())
duration = time.perf_counter() - start
print(f'process_ibans for {sample_size} took {duration:.6f}')

# start = time.perf_counter()
# df = (
#     pl.scan_csv(generatedfile)
#     .with_columns(
#         validated=process_ibans_num("IBAN Examples")
#         .str.split_exact(",", 2)
#         .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
#     )
#     .unnest("validated")
#     .sort(by="IBAN Examples", descending=True)
# )
# # trigger the processing
# print(df.collect())
# duration = time.perf_counter() - start
# print(f'process_ibans_num for {sample_size} took {duration:.6f}')

# cleanup
os.remove(generatedfile)
